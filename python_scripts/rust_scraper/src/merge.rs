//! merge-courses: Clone michael-maltsev/technion-sap-info-fetcher gh-pages,
//! parse the semester JSON files, convert to our CourseDB format, merge with
//! an existing old DB (deduplicating 6→8 digit numbers), compute number_aliases
//! for search, and write the merged output.
//!
//! Usage:
//!   merge-courses --old-db path/to/old/courses.json -o path/to/merged/courses.json
//!   merge-courses -o path/to/merged/courses.json   # no old DB, just convert gh-pages

use clap::Parser;
use degree_core::course::{CourseDB, CourseDBEntry};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "merge-courses", about = "Fetch & merge Technion course data into courses.json")]
struct Args {
    /// Path to old courses.json to merge with (optional).
    #[arg(long)]
    old_db: Option<PathBuf>,

    /// Output merged JSON file.
    #[arg(short, long)]
    output: PathBuf,

    /// Keep the cloned repo instead of cleaning up.
    #[arg(long)]
    keep_clone: bool,
}

/// Source format from michael-maltsev's fetcher: each course has `general` + `schedule`.
#[derive(Debug, Deserialize)]
struct SourceCourse {
    general: SourceGeneral,
}

/// Hebrew-keyed general info for a course.
#[derive(Debug, Deserialize)]
struct SourceGeneral {
    #[serde(rename = "מספר מקצוע", default)]
    number: String,
    #[serde(rename = "שם מקצוע", default)]
    name: String,
    #[serde(rename = "נקודות", default)]
    points: serde_json::Value,
    #[serde(rename = "מקצועות קדם", default)]
    prerequisites: serde_json::Value,
    #[serde(rename = "מקצועות צמודים", default)]
    linked: Option<String>,
    #[serde(rename = "מקצועות ללא זיכוי נוסף", default)]
    identical: Option<String>,
    #[serde(rename = "מקצועות ללא זיכוי נוסף (מוכלים)", default)]
    inclusive: Option<String>,
    #[serde(rename = "מקצועות ללא זיכוי נוסף (מכילים)", default)]
    including: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // 1. Clone gh-pages
    let clone_dir = std::env::temp_dir().join("technion-sap-courses-merge");
    if clone_dir.exists() {
        std::fs::remove_dir_all(&clone_dir)?;
    }
    eprintln!("Cloning technion-sap-info-fetcher gh-pages...");
    let status = Command::new("git")
        .args([
            "clone",
            "--branch", "gh-pages",
            "--single-branch",
            "--depth", "1",
            "https://github.com/michael-maltsev/technion-sap-info-fetcher.git",
            clone_dir.to_str().unwrap(),
        ])
        .status()?;
    if !status.success() {
        return Err("git clone failed".into());
    }

    // 2. Find course JSON files (not unfiltered, not min)
    let re_file = Regex::new(r"^courses_\d{4}_\d{3}\.json$").unwrap();
    let mut files: Vec<PathBuf> = std::fs::read_dir(&clone_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            let name = p.file_name().unwrap_or_default().to_string_lossy();
            re_file.is_match(&name)
        })
        .collect();
    files.sort();
    eprintln!("Found {} semester files", files.len());

    // 3. Parse all source files, merge newest-wins (files sorted oldest→newest)
    let mut new_courses: HashMap<String, CourseDBEntry> = HashMap::new();

    for file in &files {
        let fname = file.file_name().unwrap_or_default().to_string_lossy();
        let data = std::fs::read_to_string(file)?;
        let sources: Vec<SourceCourse> = serde_json::from_str(&data)?;
        let count = sources.len();

        for src in sources {
            let number = src.general.number.trim().to_string();
            if number.is_empty() {
                continue;
            }
            let name = src.general.name.trim().to_string();
            let points = parse_points(&src.general.points);
            let prerequisites = parse_prerequisites(&src.general.prerequisites);
            let linked = parse_course_list(src.general.linked.as_deref());
            let identical = parse_course_list(src.general.identical.as_deref());
            let inclusive = parse_course_list(src.general.inclusive.as_deref());
            let including = parse_course_list(src.general.including.as_deref());

            new_courses.insert(number.clone(), CourseDBEntry {
                full_name: format!("{}: {}", number, name),
                name,
                number,
                points,
                prerequisites,
                linked,
                identical,
                overlapping: vec![],
                inclusive,
                including,
                followed_by: vec![],
            });
        }
        eprintln!("  {}: {} courses", fname, count);
    }
    eprintln!("New source: {} unique courses", new_courses.len());

    // 4. Load old DB if provided
    let old_courses: HashMap<String, CourseDBEntry> = if let Some(ref old_path) = args.old_db {
        let data = std::fs::read_to_string(old_path)?;
        let db = CourseDB::from_json(&data)
            .ok_or_else(|| format!("Failed to parse old DB: {}", old_path.display()))?;
        eprintln!("Old DB: {} courses from {}", db.courses.len(), old_path.display());
        db.courses.into_iter().map(|c| (c.number.clone(), c)).collect()
    } else {
        HashMap::new()
    };

    // 5. Merge: new wins, convert old 5-6 digit → 8 digit, record aliases
    let mut merged: HashMap<String, CourseDBEntry> = new_courses.clone();
    let mut aliases: HashMap<String, String> = HashMap::new();

    for (old_num, old_course) in &old_courses {
        if let Some(new_num) = old_to_new_number(old_num) {
            aliases.insert(old_num.clone(), new_num.clone());
            if merged.contains_key(&new_num) {
                // New version exists — skip old
                continue;
            }
            // No new version, convert old to 8-digit
            let mut course = old_course.clone();
            course.number = new_num.clone();
            course.full_name = format!("{}: {}", new_num, course.name);
            merged.insert(new_num, course);
        } else if !merged.contains_key(old_num) {
            // Can't convert and not already present — keep as-is
            merged.insert(old_num.clone(), old_course.clone());
        }
    }

    eprintln!(
        "Merged: {} courses ({} aliases)",
        merged.len(),
        aliases.len()
    );

    // 6. Build name map (owned) and resolve all references to "number: name"
    let name_map: HashMap<String, String> = merged
        .iter()
        .map(|(num, c)| (num.clone(), c.name.clone()))
        .collect();

    for course in merged.values_mut() {
        resolve_refs(&mut course.prerequisites, &name_map, &aliases);
        resolve_list(&mut course.linked, &name_map, &aliases);
        resolve_list(&mut course.identical, &name_map, &aliases);
        resolve_list(&mut course.inclusive, &name_map, &aliases);
        resolve_list(&mut course.including, &name_map, &aliases);
    }

    // 7. Compute followed_by
    // Collect prerequisite → follower mappings first to avoid borrow issues
    let mut fb_map: HashMap<String, Vec<String>> = HashMap::new();
    for course in merged.values() {
        for group in &course.prerequisites {
            for entry in group {
                let prereq_num = entry.split(':').next().unwrap_or("").trim();
                if merged.contains_key(prereq_num) {
                    fb_map.entry(prereq_num.to_string())
                        .or_default()
                        .push(course.full_name.clone());
                }
            }
        }
    }
    for (num, followers) in &fb_map {
        if let Some(course) = merged.get_mut(num) {
            for f in followers {
                if !course.followed_by.contains(f) {
                    course.followed_by.push(f.clone());
                }
            }
        }
    }

    // 8. Build output
    let mut courses_list: Vec<CourseDBEntry> = merged.into_values().collect();
    courses_list.sort_by(|a, b| a.number.cmp(&b.number));

    let db = CourseDB {
        courses: courses_list,
    };

    // Serialize with number_aliases as an extra field
    let mut output = serde_json::to_value(&db)?;
    if !aliases.is_empty() {
        output["number_aliases"] = serde_json::to_value(&aliases)?;
    }

    if let Some(parent) = args.output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&output)?;
    std::fs::write(&args.output, &json)?;

    eprintln!(
        "Wrote {} courses to {}",
        db.courses.len(),
        args.output.display(),
    );

    // 9. Cleanup
    if !args.keep_clone {
        let _ = std::fs::remove_dir_all(&clone_dir);
    }

    Ok(())
}

// --- Parsing helpers ---

fn parse_points(val: &serde_json::Value) -> f64 {
    match val {
        serde_json::Value::Number(n) => n.as_f64().unwrap_or(0.0),
        serde_json::Value::String(s) => s.parse().unwrap_or(0.0),
        _ => 0.0,
    }
}

fn parse_prerequisites(val: &serde_json::Value) -> Vec<Vec<String>> {
    let s = match val {
        serde_json::Value::String(s) => s.clone(),
        _ => return vec![vec![]],
    };
    if s.trim().is_empty() {
        return vec![vec![]];
    }

    let or_groups: Vec<&str> = s.split(" או ").collect();
    let mut result = Vec::new();
    for group in or_groups {
        let group = group.trim();
        let group = group
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(group);
        let courses: Vec<String> = group
            .split(" ו-")
            .map(|c| c.trim().to_string())
            .filter(|c| !c.is_empty())
            .collect();
        if !courses.is_empty() {
            result.push(courses);
        }
    }
    if result.is_empty() { vec![vec![]] } else { result }
}

fn parse_course_list(val: Option<&str>) -> Vec<String> {
    let s = match val {
        Some(s) if !s.trim().is_empty() => s,
        _ => return vec![],
    };
    let re = Regex::new(r"\d{8}").unwrap();
    re.find_iter(s).map(|m| m.as_str().to_string()).collect()
}

/// Convert old 5-6 digit course number to new 8-digit format.
fn old_to_new_number(num: &str) -> Option<String> {
    let re_9730 = Regex::new(r"^9730(\d\d)$").unwrap();
    if let Some(caps) = re_9730.captures(num) {
        return Some(format!("970300{}", &caps[1]));
    }

    let re_6 = Regex::new(r"^(\d{3})(\d{3})$").unwrap();
    if let Some(caps) = re_6.captures(num) {
        return Some(format!("0{}0{}", &caps[1], &caps[2]));
    }

    // 5-digit: pad to 6 then convert
    if num.len() == 5 {
        let padded = format!("0{}", num);
        if let Some(caps) = re_6.captures(&padded) {
            return Some(format!("0{}0{}", &caps[1], &caps[2]));
        }
    }

    // Already 8-digit or unknown format
    if num.len() == 8 && num.chars().all(|c| c.is_ascii_digit()) {
        return Some(num.to_string());
    }

    None
}

/// Update a list of "number" or "number: name" references to use 8-digit numbers.
fn resolve_list(
    list: &mut Vec<String>,
    name_map: &HashMap<String, String>,
    aliases: &HashMap<String, String>,
) {
    for entry in list.iter_mut() {
        let num = entry.split(':').next().unwrap_or("").trim().to_string();
        let canonical = aliases.get(&num).cloned().unwrap_or(num);
        if let Some(name) = name_map.get(&canonical) {
            *entry = format!("{}: {}", canonical, name);
        } else {
            *entry = canonical;
        }
    }
}

/// Update prerequisite groups to use 8-digit numbers with names.
fn resolve_refs(
    groups: &mut Vec<Vec<String>>,
    name_map: &HashMap<String, String>,
    aliases: &HashMap<String, String>,
) {
    for group in groups.iter_mut() {
        for entry in group.iter_mut() {
            let num = entry.split(':').next().unwrap_or("").trim().to_string();
            let re = Regex::new(r"^\d{5,8}$").unwrap();
            if re.is_match(&num) {
                let canonical = aliases.get(&num).cloned().unwrap_or(num);
                if let Some(name) = name_map.get(&canonical) {
                    *entry = format!("{}: {}", canonical, name);
                } else {
                    *entry = canonical;
                }
            }
        }
    }
}
