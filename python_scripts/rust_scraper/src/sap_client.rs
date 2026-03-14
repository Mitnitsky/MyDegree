use regex::Regex;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::time::{sleep, Duration};

const REQUEST_TIMEOUT_SECS: u64 = 60;
/// Rotate Tor circuit every N non-cached requests to avoid IP bans
const ROTATE_EVERY: u64 = 100;

/// Shared SAP client that handles OData batch requests with caching, retries,
/// and automatic Tor circuit rotation.
pub struct SapClient {
    cache_dir: Option<PathBuf>,
    proxy: Option<String>,
    verbose: bool,
    /// Tor control port (e.g. 9051). If set, sends NEWNYM signal to rotate circuits.
    tor_control_port: Option<u16>,
    /// Count of non-cached requests sent (for rotation scheduling)
    request_count: AtomicU64,
}

impl SapClient {
    pub fn new(cache_dir: Option<PathBuf>, _concurrency: usize, verbose: bool) -> Self {
        let proxy = std::env::var("HTTPS_PROXY")
            .or_else(|_| std::env::var("HTTP_PROXY"))
            .ok()
            .filter(|s| !s.is_empty());

        Self {
            cache_dir,
            proxy,
            verbose,
            tor_control_port: None,
            request_count: AtomicU64::new(0),
        }
    }

    pub fn set_proxy(&mut self, proxy: Option<String>) {
        if let Some(p) = proxy.filter(|s| !s.is_empty()) {
            self.proxy = Some(p);
        }
    }

    /// Enable Tor circuit rotation via the control port.
    pub fn set_tor_control_port(&mut self, port: u16) {
        self.tor_control_port = Some(port);
    }

    fn cache_path(&self, query: &str) -> Option<PathBuf> {
        let dir = self.cache_dir.as_ref()?;
        std::fs::create_dir_all(dir).ok()?;

        let safe: String = query
            .chars()
            .map(|c| if "<>:\"/\\|?*".contains(c) { '_' } else { c })
            .take(64)
            .collect();

        let mut hasher = Sha256::new();
        hasher.update(query.as_bytes());
        let hash_bytes = hasher.finalize();
        let hash = u64::from_le_bytes(hash_bytes[..8].try_into().unwrap());

        Some(dir.join(format!("{}_{:x}.json", safe, hash)))
    }

    fn read_cache(&self, query: &str) -> Option<serde_json::Value> {
        let path = self.cache_path(query)?;
        if !path.exists() {
            return None;
        }
        let data = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&data).ok()
    }

    fn write_cache(&self, query: &str, value: &serde_json::Value) {
        if let Some(path) = self.cache_path(query) {
            let _ = std::fs::write(
                &path,
                serde_json::to_string_pretty(value).unwrap_or_default(),
            );
        }
    }

    /// Rotate Tor circuit by sending NEWNYM to the control port.
    async fn maybe_rotate_circuit(&self) {
        let count = self.request_count.fetch_add(1, Ordering::Relaxed);
        if count > 0 && count % ROTATE_EVERY == 0 {
            self.rotate_circuit_now().await;
        }
    }

    /// Force a Tor circuit rotation right now.
    async fn rotate_circuit_now(&self) {
        let port = match self.tor_control_port {
            Some(p) => p,
            None => return,
        };

        if self.verbose {
            eprintln!("Rotating Tor circuit...");
        }

        // Send NEWNYM via Tor control port
        let cmd = format!(
            "printf 'AUTHENTICATE\\r\\nSIGNAL NEWNYM\\r\\nQUIT\\r\\n' | nc -w5 localhost {}",
            port
        );
        let _ = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .output()
            .await;

        // Wait for new circuit to be established
        sleep(Duration::from_secs(5)).await;

        if self.verbose {
            // Verify new IP
            if let Ok(output) = tokio::process::Command::new("curl")
                .args(["--proxy", "socks5h://127.0.0.1:9050", "-s", "https://api.ipify.org"])
                .output()
                .await
            {
                let ip = String::from_utf8_lossy(&output.stdout);
                eprintln!("New Tor IP: {}", ip.trim());
            }
        }
    }

    /// Send a single OData GET via a $batch POST (with cache + retry).
    pub async fn send_request(
        &self,
        query: &str,
        allow_empty: bool,
    ) -> Result<serde_json::Value, String> {
        // Check cache first
        if let Some(cached) = self.read_cache(query) {
            return Ok(cached);
        }

        self.maybe_rotate_circuit().await;

        let max_retries = 10;
        let mut delay = 2u64;
        for attempt in 1..=max_retries {
            // Small delay between requests to be polite
            sleep(Duration::from_millis(300)).await;

            match self.send_request_once(query).await {
                Ok(val) => {
                    // Check for WAF block (HTML response instead of multipart)
                    if let Some(s) = val.as_str() {
                        if s.contains("Request Rejected") {
                            eprintln!("WAF blocked request, rotating circuit...");
                            self.rotate_circuit_now().await;
                            delay = 5;
                            continue;
                        }
                    }

                    if !allow_empty && val == serde_json::json!({"d": {"results": []}}) {
                        self.write_cache(query, &val);
                        return Ok(val);
                    }
                    self.write_cache(query, &val);
                    return Ok(val);
                }
                Err(e) => {
                    let is_connection_error = e.contains("exit exit status: 52")
                        || e.contains("exit exit status: 56")
                        || e.contains("Connection reset")
                        || e.contains("empty response");

                    if is_connection_error && self.tor_control_port.is_some() {
                        eprintln!(
                            "Connection error (attempt {}/{}), rotating Tor circuit: {}",
                            attempt, max_retries, e
                        );
                        self.rotate_circuit_now().await;
                    } else {
                        eprintln!(
                            "Error (attempt {}/{}): {} for {}",
                            attempt, max_retries, e, query
                        );
                    }

                    if attempt == max_retries {
                        return Err(format!("Failed after {} retries: {}", max_retries, e));
                    }
                    sleep(Duration::from_secs(delay)).await;
                    delay = (delay * 2).min(30);
                }
            }
        }
        unreachable!()
    }

    /// Low-level: send one `$batch` POST with a single GET sub-request.
    async fn send_request_once(
        &self,
        query: &str,
    ) -> Result<serde_json::Value, String> {
        if self.verbose {
            eprintln!("Sending request: {}", query);
        }

        let url =
            "https://portalex.technion.ac.il/sap/opu/odata/sap/Z_CM_EV_CDIR_DATA_SRV/$batch?sap-client=700";

        let body = format!(
            "--batch_1d12-afbf-e3c7\r\n\
             Content-Type: application/http\r\n\
             Content-Transfer-Encoding: binary\r\n\
             \r\n\
             GET {} HTTP/1.1\r\n\
             sap-cancel-on-close: true\r\n\
             X-Requested-With: X\r\n\
             sap-contextid-accept: header\r\n\
             Accept: application/json\r\n\
             Accept-Language: he\r\n\
             DataServiceVersion: 2.0\r\n\
             MaxDataServiceVersion: 2.0\r\n\
             \r\n\
             \r\n\
             --batch_1d12-afbf-e3c7--\r\n",
            query
        );

        // Write body to temp file to avoid shell escaping issues with \r\n
        let tmp_dir = std::env::temp_dir();
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let body_file = tmp_dir.join(format!("sap_body_{}_{}.bin", std::process::id(), unique_id));
        std::fs::write(&body_file, body.as_bytes()).map_err(|e| e.to_string())?;
        let body_arg = format!("@{}", body_file.display());

        let mut args = vec![
            "-s".to_string(),
            "--max-time".to_string(),
            REQUEST_TIMEOUT_SECS.to_string(),
            "-X".to_string(),
            "POST".to_string(),
            url.to_string(),
            "-H".to_string(),
            "Content-Type: multipart/mixed;boundary=batch_1d12-afbf-e3c7".to_string(),
            "-H".to_string(),
            "Accept: multipart/mixed".to_string(),
            "-H".to_string(),
            "Accept-Language: he".to_string(),
            "-H".to_string(),
            "DataServiceVersion: 2.0".to_string(),
            "-H".to_string(),
            "MaxDataServiceVersion: 2.0".to_string(),
            "-H".to_string(),
            "X-Requested-With: X".to_string(),
            "-H".to_string(),
            "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36".to_string(),
            "-H".to_string(),
            "Origin: https://portalex.technion.ac.il".to_string(),
            "-H".to_string(),
            "Referer: https://portalex.technion.ac.il/ovv/".to_string(),
            "--data-binary".to_string(),
            body_arg,
        ];

        if let Some(proxy) = &self.proxy {
            args.push("--proxy".to_string());
            args.push(proxy.clone());
        }

        let output = tokio::process::Command::new("curl")
            .args(&args)
            .output()
            .await
            .map_err(|e| format!("Failed to run curl: {}", e))?;

        let _ = std::fs::remove_file(&body_file);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("curl failed (exit {}): {}", output.status, stderr));
        }

        let text = String::from_utf8_lossy(&output.stdout).to_string();
        if text.is_empty() {
            return Err("curl returned empty response".to_string());
        }

        // Check for WAF block
        if text.contains("Request Rejected") || text.contains("F5_Logo") {
            return Err("WAF blocked request".to_string());
        }

        // Parse multipart response — extract the JSON from the single sub-response
        let text = text.replace("\r\n", "\n");
        let chunks: Vec<&str> = text.trim().split("\n\n").collect();
        if chunks.len() < 3 {
            return Err(format!(
                "Invalid response: expected 3+ chunks, got {}. Response: {}",
                chunks.len(),
                &text[..text.len().min(300)]
            ));
        }

        let json_str = chunks[2].split('\n').next().unwrap_or("");
        serde_json::from_str(json_str).map_err(|e| format!("JSON parse error: {}", e))
    }
}

/// Parse SAP date string like `/Date(1234567890000)/` → chrono-less YYYY-MM-DD.
pub fn sap_date_to_ymd(date_str: &str) -> Result<(i32, u32, u32), String> {
    let re = Regex::new(r"^/Date\((\d+)\)/$").unwrap();
    let caps = re
        .captures(date_str)
        .ok_or_else(|| format!("Invalid date: {}", date_str))?;
    let ts_ms: i64 = caps[1].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
    let ts = ts_ms / 1000;

    // Simple timestamp → date conversion (UTC)
    let days_since_epoch = ts / 86400;
    let (y, m, d) = days_to_ymd(days_since_epoch);
    Ok((y, m, d))
}

pub fn sap_date_format(date_str: &str) -> Result<String, String> {
    let (y, m, d) = sap_date_to_ymd(date_str)?;
    Ok(format!("{:02}-{:02}-{:04}", d, m, y))
}

pub fn sap_date_format_iso(date_str: &str) -> Result<String, String> {
    let (y, m, d) = sap_date_to_ymd(date_str)?;
    Ok(format!("{:04}-{:02}-{:02}", y, m, d))
}

pub fn sap_date_weekday(date_str: &str) -> Result<usize, String> {
    let re = Regex::new(r"^/Date\((\d+)\)/$").unwrap();
    let caps = re
        .captures(date_str)
        .ok_or_else(|| format!("Invalid date: {}", date_str))?;
    let ts_ms: i64 = caps[1].parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
    let ts = ts_ms / 1000;
    // weekday: 0=Thu for epoch, we need (day_of_week+1)%7 matching Python's (weekday()+1)%7
    let days_since_epoch = ts.div_euclid(86400);
    // Jan 1 1970 = Thursday = weekday 3 (Mon=0)
    let py_weekday = ((days_since_epoch % 7) + 3) % 7; // 0=Mon
    Ok(((py_weekday + 1) % 7) as usize) // 0=Sun
}

fn days_to_ymd(days_since_epoch: i64) -> (i32, u32, u32) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days_since_epoch + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m, d)
}

/// Convert old 6-digit course number to new 8-digit format.
pub fn to_new_course_number(course: &str) -> String {
    let re1 = Regex::new(r"^9730(\d\d)$").unwrap();
    if let Some(caps) = re1.captures(course) {
        return format!("970300{}", &caps[1]);
    }

    let re2 = Regex::new(r"^(\d{3})(\d{3})$").unwrap();
    if let Some(caps) = re2.captures(course) {
        return format!("0{}0{}", &caps[1], &caps[2]);
    }

    course.to_string()
}
