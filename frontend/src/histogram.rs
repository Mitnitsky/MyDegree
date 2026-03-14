use serde::Deserialize;
use std::collections::BTreeMap;
use gloo_net::http::Request;

const HISTOGRAM_BASE_URL: &str = "https://michael-maltsev.github.io/technion-histograms";

#[derive(Clone, Debug, Deserialize)]
pub struct GradeStats {
    #[serde(default)]
    pub students: Option<serde_json::Value>,
    #[serde(default, rename = "passFail")]
    pub pass_fail: Option<String>,
    #[serde(default, rename = "passPercent")]
    pub pass_percent: Option<String>,
    #[serde(default)]
    pub min: Option<serde_json::Value>,
    #[serde(default)]
    pub max: Option<serde_json::Value>,
    #[serde(default)]
    pub average: Option<serde_json::Value>,
    #[serde(default)]
    pub median: Option<serde_json::Value>,
}

#[derive(Clone, Debug)]
pub struct StaffInfo {
    pub title: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct ExamEntry {
    pub entry_name: String,
    pub display_name: String,
    pub semester_name: String,
    pub semester_number: String,
    pub staff: Option<String>,
    pub stats: GradeStats,
}

#[derive(Clone, Debug)]
pub struct SemesterGroup {
    pub label: String,
    pub semester_number: String,
    pub entries: Vec<ExamEntry>,
}

#[derive(Clone, Debug)]
pub struct HistogramData {
    pub resolved_number: String,
    pub semesters: Vec<SemesterGroup>,
}

fn get_course_number_variants(course_number: &str) -> Vec<String> {
    let num = course_number.to_string();
    let mut variants = vec![num.clone()];

    if num.len() < 8 {
        variants.push(format!("{:0>8}", num));
        if num.len() == 6 {
            let first_half = &num[..3];
            let second_half = &num[3..];
            variants.push(format!("0{}0{}", first_half, second_half));
        }
    }
    variants
}

fn convert_semester_to_text(semester: &str) -> String {
    let semesters = ["חורף", "אביב", "קיץ"];
    // Handle both "2024/2" and "202402" formats
    let (year, sem_num) = if semester.contains('/') {
        let year: i32 = semester[..4].parse().unwrap_or(0);
        let sem: usize = semester[5..].parse::<usize>().unwrap_or(1) - 1;
        (year, sem)
    } else if semester.len() == 6 {
        let year: i32 = semester[..4].parse().unwrap_or(0);
        let sem: usize = semester[4..].parse::<usize>().unwrap_or(1) - 1;
        (year, sem)
    } else {
        return semester.to_string();
    };

    if sem_num == 0 {
        format!("{} {}-{}", semesters.get(sem_num).unwrap_or(&""), year, year + 1)
    } else {
        format!("{} {}", semesters.get(sem_num).unwrap_or(&""), year + 1)
    }
}

fn convert_exam_name(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "final_a" => "סופי מועד א'".to_string(),
        "final_b" => "סופי מועד ב'".to_string(),
        "finals" => "סופי".to_string(),
        "exam_a" => "מבחן מועד א'".to_string(),
        "exam_b" => "מבחן מועד ב'".to_string(),
        _ => name.to_string(),
    }
}

pub fn build_image_url(resolved_number: &str, semester_number: &str, entry_name: &str) -> String {
    format!("{}/{}/{}/{}.png", HISTOGRAM_BASE_URL, resolved_number, semester_number, entry_name)
}

pub async fn fetch_histogram(course_number: &str) -> Option<HistogramData> {
    let variants = get_course_number_variants(course_number);

    for variant in &variants {
        let url = format!("{}/{}/index.json", HISTOGRAM_BASE_URL, variant);
        if let Ok(resp) = Request::get(&url).send().await {
            if resp.ok() {
                if let Ok(json) = resp.json::<BTreeMap<String, serde_json::Value>>().await {
                    let semesters = parse_histogram_json(&json);
                    return Some(HistogramData {
                        resolved_number: variant.clone(),
                        semesters,
                    });
                }
            }
        }
    }
    None
}

fn parse_histogram_json(json: &BTreeMap<String, serde_json::Value>) -> Vec<SemesterGroup> {
    let mut groups: Vec<SemesterGroup> = Vec::new();

    for (semester_key, semester_data) in json {
        let label = convert_semester_to_text(semester_key);
        let mut entries = Vec::new();
        let mut staff_str = String::new();

        if let Some(obj) = semester_data.as_object() {
            // First pass: extract staff
            for (key, val) in obj {
                if key == "Staff" || key.starts_with("Staff") {
                    if let Some(arr) = val.as_array() {
                        if let Some(first) = arr.first() {
                            let title = first.get("title").and_then(|v| v.as_str()).unwrap_or("");
                            let name = first.get("name").and_then(|v| v.as_str()).unwrap_or("");
                            staff_str = format!("{}: {}", title, name);
                        }
                    }
                }
            }
            // Second pass: extract exam entries
            for (key, val) in obj {
                if key == "Staff" || key.starts_with("Staff_") {
                    continue;
                }
                // The value can be either an object (direct stats) or an array of stats
                let stats_val = if val.is_object() {
                    Some(val.clone())
                } else if let Some(arr) = val.as_array() {
                    arr.first().cloned()
                } else {
                    None
                };

                if let Some(stats_json) = stats_val {
                    if let Ok(stats) = serde_json::from_value::<GradeStats>(stats_json) {
                        entries.push(ExamEntry {
                            entry_name: key.clone(),
                            display_name: convert_exam_name(key),
                            semester_name: label.clone(),
                            semester_number: semester_key.clone(),
                            staff: if staff_str.is_empty() { None } else { Some(staff_str.clone()) },
                            stats,
                        });
                    }
                }
            }
        }

        if !entries.is_empty() {
            groups.push(SemesterGroup {
                label,
                semester_number: semester_key.clone(),
                entries,
            });
        }
    }

    // Sort by semester descending
    groups.sort_by(|a, b| b.semester_number.cmp(&a.semester_number));
    groups
}
