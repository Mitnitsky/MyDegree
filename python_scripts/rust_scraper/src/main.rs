mod course;
mod fetcher;
mod sap_client;

use clap::Parser;
use course::{Course, CoursesJson};
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use sap_client::SapClient;
use std::collections::HashMap;
use std::path::PathBuf;

const DEFAULT_CONCURRENCY: usize = 2;

#[derive(Parser, Debug)]
#[command(
    name = "course-scraper",
    about = "Fetch Technion course data from SAP and output courses.json"
)]
struct Args {
    /// Semester selector:
    ///   "all"       — fetch ALL available semesters, merge with newest-wins
    ///   "last-N"    — fetch the N most recent semesters, merge
    ///   "YEAR-SEM"  — fetch a single semester (e.g. "2025-200")
    year_and_semester: String,

    /// Output JSON file path.
    #[arg(short, long, default_value = "../db/courses.json")]
    output: PathBuf,

    /// Cache directory for SAP responses.
    #[arg(long)]
    cache_dir: Option<PathBuf>,

    /// Number of concurrent requests.
    #[arg(long, default_value_t = DEFAULT_CONCURRENCY)]
    concurrency: usize,

    /// Verbose logging.
    #[arg(short, long)]
    verbose: bool,

    /// Proxy server (e.g. "socks5://127.0.0.1:9050" for Tor).
    /// Also reads HTTP_PROXY / HTTPS_PROXY env vars.
    #[arg(long)]
    proxy: Option<String>,

    /// Tor control port for automatic circuit rotation (e.g. 9051).
    /// Requires Tor running with ControlPort enabled.
    #[arg(long)]
    tor_control_port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let start = std::time::Instant::now();

    let mut client = SapClient::new(args.cache_dir.clone(), args.concurrency, args.verbose);
    client.set_proxy(args.proxy.clone());
    if let Some(port) = args.tor_control_port {
        client.set_tor_control_port(port);
    }

    let semesters_to_fetch = resolve_semesters(&client, &args.year_and_semester).await?;
    eprintln!(
        "Will fetch {} semester(s): {}",
        semesters_to_fetch.len(),
        semesters_to_fetch
            .iter()
            .map(|(y, s)| format!("{}-{}", y, s))
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Fetch all semesters oldest-first so newer data overwrites older.
    // We key by the canonical 8-digit course number so that old 6-digit
    // OTJIDs (e.g. SM234123) and new 8-digit ones (SM02340123) map to the
    // same entry — newest semester wins, no duplicates.
    let mut merged_raw: HashMap<String, serde_json::Value> = HashMap::new();

    for &(year, semester) in &semesters_to_fetch {
        let semester_raw = fetch_semester_raw(&client, year, semester, args.concurrency).await?;
        let count = semester_raw.len();
        for (_otjid, raw) in semester_raw {
            // Normalize the key: strip "SM" prefix, convert 6→8 digits.
            let raw_num = raw["Otjid"]
                .as_str()
                .unwrap_or("")
                .strip_prefix("SM")
                .unwrap_or(raw["Otjid"].as_str().unwrap_or(""));
            let canonical = sap_client::to_new_course_number(raw_num);
            merged_raw.insert(canonical, raw);
        }
        eprintln!("  {}-{}: {} courses fetched", year, semester, count);
    }

    eprintln!(
        "Merged total: {} unique courses across all semesters",
        merged_raw.len()
    );

    // Build global name lookup from all fetched courses.
    let course_names = build_name_map(&merged_raw);

    // Convert to Course structs.
    let mut course_ids: Vec<String> = merged_raw.keys().cloned().collect();
    course_ids.sort();

    let mut courses: Vec<Course> = Vec::new();
    for cid in &course_ids {
        if let Some(raw) = merged_raw.get(cid) {
            match fetcher::sap_to_course(raw, &course_names) {
                Ok(c) => courses.push(c),
                Err(e) => {
                    let clean = cid.strip_prefix("SM").unwrap_or(cid);
                    eprintln!("Failed to parse {}: {}", clean, e);
                }
            }
        }
    }

    // Compute followed_by across all courses.
    fetcher::compute_followed_by(&mut courses);

    // Write output.
    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let wrapper = CoursesJson {
        courses,
        number_aliases: None,
    };
    let json = serde_json::to_string_pretty(&wrapper)?;
    std::fs::write(&args.output, json)?;

    let elapsed = start.elapsed();
    eprintln!(
        "Wrote {} courses to {} in {:.2} minutes",
        wrapper.courses.len(),
        args.output.display(),
        elapsed.as_secs_f64() / 60.0
    );
    Ok(())
}

/// Resolve the semester selector string into a sorted list of (year, semester)
/// pairs, ordered oldest→newest so that newer data overwrites older.
async fn resolve_semesters(
    client: &SapClient,
    selector: &str,
) -> Result<Vec<(i64, i64)>, Box<dyn std::error::Error>> {
    if selector == "all" {
        let semesters = fetcher::get_last_semesters(client).await?;
        if semesters.is_empty() {
            return Err("No semesters returned from SAP".into());
        }
        let mut pairs: Vec<(i64, i64)> = semesters
            .iter()
            .map(|s| {
                (
                    s["year"].as_i64().unwrap(),
                    s["semester"].as_i64().unwrap(),
                )
            })
            .collect();
        // oldest first so newest overwrites
        pairs.sort();
        return Ok(pairs);
    }

    let parts: Vec<&str> = selector.split('-').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid selector: {}", selector).into());
    }

    if parts[0] == "last" {
        let count: usize = parts[1].parse()?;
        let semesters = fetcher::get_last_semesters(client).await?;
        if semesters.len() < count {
            return Err(format!(
                "Asked for last {} semesters but only {} available",
                count,
                semesters.len()
            )
            .into());
        }
        let mut pairs: Vec<(i64, i64)> = semesters
            .iter()
            .take(count)
            .map(|s| {
                (
                    s["year"].as_i64().unwrap(),
                    s["semester"].as_i64().unwrap(),
                )
            })
            .collect();
        pairs.sort(); // oldest first
        return Ok(pairs);
    }

    let year: i64 = parts[0].parse()?;
    let semester: i64 = parts[1].parse()?;
    Ok(vec![(year, semester)])
}

/// Fetch raw SAP course data for one semester. Returns a map of OTJID → raw JSON.
/// Uses buffer_unordered for concurrency control with automatic Tor circuit rotation.
async fn fetch_semester_raw(
    client: &SapClient,
    year: i64,
    semester: i64,
    concurrency: usize,
) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
    eprintln!("Fetching semester {}-{}...", year, semester);

    let mut course_ids = fetcher::get_sap_course_numbers(client, year, semester).await?;
    course_ids.sort();
    eprintln!("  {}-{}: {} course IDs", year, semester, course_ids.len());

    let pb = ProgressBar::new(course_ids.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap(),
    );

    let results: Vec<_> = stream::iter(course_ids.iter().map(|cid| {
        let cid = cid.clone();
        let query = build_course_query(year, semester, &cid);
        let pb_ref = pb.clone();
        async move {
            let result = client.send_request(&query, true).await;
            pb_ref.inc(1);
            (cid, result)
        }
    }))
    .buffer_unordered(concurrency)
    .collect()
    .await;

    pb.finish_and_clear();

    let mut raw_map: HashMap<String, serde_json::Value> = HashMap::new();
    for (cid, result) in results {
        match result {
            Ok(raw) => {
                if let Some(arr) = raw["d"]["results"].as_array() {
                    if arr.len() == 1 {
                        raw_map.insert(cid, arr[0].clone());
                    }
                }
            }
            Err(e) => {
                eprintln!("  Failed to fetch {}: {}", cid, e);
            }
        }
    }

    Ok(raw_map)
}

/// Build a course-number → name lookup from all raw SAP results.
/// Keys are canonical 8-digit numbers.
fn build_name_map(raw_map: &HashMap<String, serde_json::Value>) -> HashMap<String, String> {
    let mut names = HashMap::new();
    for (canonical, raw) in raw_map {
        let name = raw["Name"]
            .as_str()
            .unwrap_or("")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !canonical.is_empty() {
            names.insert(canonical.clone(), name);
        }
    }
    names
}

fn build_course_query(year: i64, semester: i64, course: &str) -> String {
    let select = [
        "Otjid",
        "Points",
        "Name",
        "StudyContentDescription",
        "OrgText",
        "ZzAcademicLevel",
        "ZzAcademicLevelText",
        "ZzSemesterNote",
        "Responsible",
        "Exams",
        "SmRelations",
        "SmPrereq",
    ]
    .join(",");
    let expand = ["Responsible", "Exams", "SmRelations", "SmPrereq"].join(",");
    let filter = format!(
        "Peryr%20eq%20'{}'%20and%20Perid%20eq%20'{}'%20and%20Otjid%20eq%20'{}'",
        year, semester, course
    );
    format!(
        "SmObjectSet?sap-client=700&$filter={}&$select={}&$expand={}",
        filter, select, expand
    )
}
