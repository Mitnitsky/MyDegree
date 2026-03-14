use crate::course::Course;
use crate::sap_client::{to_new_course_number, SapClient};
use regex::Regex;
use std::collections::HashMap;

/// Fetch all semester info from SAP.
pub async fn get_last_semesters(
    client: &SapClient,
) -> Result<Vec<serde_json::Value>, String> {
    let params = "SemesterSet?sap-client=700&$select=PiqYear,PiqSession,Begda,Endda";
    let raw = client.send_request(params, false).await?;
    let results = raw["d"]["results"]
        .as_array()
        .ok_or("No semesters found")?;

    let mut semesters = Vec::new();
    for r in results {
        let year: i64 = r["PiqYear"]
            .as_str()
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);
        let sem: i64 = r["PiqSession"]
            .as_str()
            .unwrap_or("0")
            .parse()
            .unwrap_or(0);
        if ![200, 201, 202].contains(&sem) {
            continue;
        }
        let begda = crate::sap_client::sap_date_format_iso(
            r["Begda"].as_str().unwrap_or(""),
        )
        .unwrap_or_default();
        let endda = crate::sap_client::sap_date_format_iso(
            r["Endda"].as_str().unwrap_or(""),
        )
        .unwrap_or_default();

        semesters.push(serde_json::json!({
            "year": year,
            "semester": sem,
            "start": begda,
            "end": endda,
        }));
    }

    semesters.sort_by(|a, b| {
        let ka = (
            a["year"].as_i64().unwrap_or(0),
            a["semester"].as_i64().unwrap_or(0),
        );
        let kb = (
            b["year"].as_i64().unwrap_or(0),
            b["semester"].as_i64().unwrap_or(0),
        );
        kb.cmp(&ka)
    });

    Ok(semesters)
}

/// Fetch all course OTJIDs for a given year/semester.
pub async fn get_sap_course_numbers(
    client: &SapClient,
    year: i64,
    semester: i64,
) -> Result<Vec<String>, String> {
    let query = format!(
        "SmObjectSet?sap-client=700&$skip=0&$top=10000&$filter=Peryr%20eq%20'{}'%20and%20Perid%20eq%20'{}'&$select=Otjid",
        year, semester
    );
    let raw = client.send_request(&query, false).await?;
    let results = raw["d"]["results"]
        .as_array()
        .ok_or("No course numbers")?;
    Ok(results
        .iter()
        .filter_map(|x| x["Otjid"].as_str().map(String::from))
        .collect())
}

/// Fetch detailed SAP data for a single course.
pub async fn get_sap_course(
    client: &SapClient,
    year: i64,
    semester: i64,
    course: &str,
) -> Result<serde_json::Value, String> {
    let select = [
        "Otjid", "Points", "Name", "StudyContentDescription", "OrgText",
        "ZzAcademicLevel", "ZzAcademicLevelText", "ZzSemesterNote",
        "Responsible", "Exams", "SmRelations", "SmPrereq",
    ]
    .join(",");
    let expand = ["Responsible", "Exams", "SmRelations", "SmPrereq"].join(",");
    let filter = format!(
        "Peryr eq '{}' and Perid eq '{}' and Otjid eq '{}'",
        year, semester, course
    );
    let query = format!(
        "SmObjectSet?sap-client=700&$filter={}&$select={}&$expand={}",
        urlencoding(&filter),
        select,
        expand
    );
    let raw = client.send_request(&query, false).await?;
    let results = raw["d"]["results"]
        .as_array()
        .ok_or("Invalid course results")?;
    if results.len() != 1 {
        return Err(format!(
            "Expected 1 result for {}, got {}",
            course,
            results.len()
        ));
    }
    Ok(results[0].clone())
}

/// Minimal URL-encode for filter strings.
fn urlencoding(s: &str) -> String {
    s.replace(' ', "%20").replace('\'', "%27")
}

/// Parse the SAP prerequisite items into the grouped format used by courses.json.
///
/// The SAP SmPrereq results contain items with Bracket, ModuleId, Operator fields.
/// We parse the text representation `(A ו-B) או (C)` into `[["A","B"],["C"]]`.
fn parse_prerequisites(sap_course: &serde_json::Value) -> Vec<Vec<String>> {
    let items = match sap_course["SmPrereq"]["results"].as_array() {
        Some(a) => a,
        None => return vec![vec![]],
    };

    if items.is_empty() {
        return vec![vec![]];
    }

    // Build the flat prereq string exactly as the Python does
    let mut prereq = String::new();
    let mut last_has_module = false;
    let mut last_has_operator = false;

    for item in items {
        let bracket = item["Bracket"].as_str().unwrap_or("");
        let module_id = item["ModuleId"].as_str().unwrap_or("").trim_start_matches('0');
        let operator = item["Operator"].as_str().unwrap_or("");

        // Buggy entry: two consecutive course ids without operator
        if last_has_module && !last_has_operator && !bracket.is_empty() || false {
            // Only add space in the very specific case from Python
        }
        if last_has_module && !last_has_operator && bracket.is_empty() && !module_id.is_empty()
        {
            prereq.push(' ');
        }

        prereq.push_str(bracket);
        if !module_id.is_empty() {
            prereq.push_str(item["ModuleId"].as_str().unwrap_or(""));
        }
        match operator {
            "AND" => prereq.push_str(" ו-"),
            "OR" => prereq.push_str(" או "),
            "" => {}
            other => {
                eprintln!("Warning: unknown operator {}", other);
            }
        }

        last_has_module = !module_id.is_empty();
        last_has_operator = !operator.is_empty();
    }

    // Clean up: `(X)` → `X`
    let re_single_paren = Regex::new(r"^\(([^()]+)\)$").unwrap();
    let re_paren_digits = Regex::new(r"\((\d+)\)").unwrap();
    prereq = re_paren_digits.replace_all(&prereq, "$1").to_string();
    prereq = re_single_paren.replace(&prereq, "$1").to_string();

    prereq_string_to_groups(&prereq)
}

/// Convert a prerequisite string like `"A ו-B או C ו-D"` into grouped format.
/// "או" separates OR-groups, "ו-" separates AND within groups.
/// Parenthesized groups `(A ו-B) או (C)` are also handled.
fn prereq_string_to_groups(prereq: &str) -> Vec<Vec<String>> {
    if prereq.trim().is_empty() {
        return vec![vec![]];
    }

    // Split by " או " for OR groups
    let or_groups: Vec<&str> = prereq.split(" או ").collect();
    let mut result: Vec<Vec<String>> = Vec::new();

    for group in or_groups {
        let group = group.trim();
        // Remove outer parens if present
        let group = group
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(group);

        // Split by " ו-" for AND within group
        let courses: Vec<String> = group
            .split(" ו-")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if !courses.is_empty() {
            result.push(courses);
        }
    }

    if result.is_empty() {
        vec![vec![]]
    } else {
        result
    }
}

/// Parse relations from the SAP SmRelations results.
/// All course numbers are normalized to 8-digit canonical format.
fn parse_relations(
    sap_course: &serde_json::Value,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut identical = Vec::new(); // AZEC + AZID
    let mut including = Vec::new(); // AZCC
    let mut included = Vec::new(); // BZCC

    if let Some(items) = sap_course["SmRelations"]["results"].as_array() {
        for item in items {
            let raw = item["Otjid"]
                .as_str()
                .unwrap_or("")
                .strip_prefix("SM")
                .unwrap_or(item["Otjid"].as_str().unwrap_or(""));
            let num = to_new_course_number(raw);
            match item["ZzRelationshipKey"].as_str().unwrap_or("") {
                "AZEC" | "AZID" => identical.push(num),
                "AZCC" => including.push(num),
                "BZCC" => included.push(num),
                other => eprintln!("Warning: unknown relation {}", other),
            }
        }
    }
    (identical, including, included)
}

/// Parse adjoining (linked) courses from semester notes text.
fn parse_adjoining(semester_note: &str) -> Vec<String> {
    let re_split =
        Regex::new(r"(?m)^(?:מקצוע צמוד|מקצועות צמודים):").unwrap();
    let parts: Vec<&str> = re_split.splitn(semester_note, 2).collect();
    if parts.len() < 2 {
        return Vec::new();
    }

    let content = parts[1].trim();

    let re_nums = Regex::new(r"\d{5,8}").unwrap();
    let mut courses: Vec<String> = Vec::new();

    for m in re_nums.find_iter(content) {
        let mut num = m.as_str().to_string();
        if num.len() <= 6 {
            num = format!("{:0>6}", num);
            num = to_new_course_number(&num);
        } else {
            num = format!("{:0>8}", num);
        }
        courses.push(num);

        // Stop at sentence boundary
        let rest = &content[m.end()..];
        if rest.starts_with('.') || rest.starts_with("\n\n") {
            break;
        }
    }

    courses
}

/// Build a `Course` (in courses.json format) from raw SAP data.
/// `course_names` is used to resolve "number: name" strings for relations.
/// All course numbers are normalized to 8-digit canonical format.
pub fn sap_to_course(
    sap_course: &serde_json::Value,
    course_names: &HashMap<String, String>,
) -> Result<Course, String> {
    let raw_number = sap_course["Otjid"]
        .as_str()
        .unwrap_or("")
        .strip_prefix("SM")
        .unwrap_or(sap_course["Otjid"].as_str().unwrap_or(""));

    let number = to_new_course_number(raw_number);
    let name = sap_course["Name"]
        .as_str()
        .unwrap_or("")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    let points_str = sap_course["Points"].as_str().unwrap_or("0");
    let points: f64 = points_str
        .trim_end_matches('0')
        .trim_end_matches('.')
        .parse()
        .unwrap_or(0.0);
    // Re-parse to get canonical float
    let points: f64 = if points == 0.0 {
        points_str.parse().unwrap_or(0.0)
    } else {
        points
    };

    let mut prerequisites = parse_prerequisites(sap_course);
    // Normalize and resolve course numbers in prerequisites
    for group in prerequisites.iter_mut() {
        for entry in group.iter_mut() {
            let trimmed = to_new_course_number(entry.trim());
            if let Some(cname) = course_names.get(&trimmed) {
                *entry = format!("{}: {}", trimmed, cname);
            } else {
                *entry = trimmed;
            }
        }
    }

    let (identical_nums, including_nums, included_nums) = parse_relations(sap_course);
    let adjoining_nums = parse_adjoining(
        sap_course["ZzSemesterNote"].as_str().unwrap_or(""),
    );

    // Resolve to "number: name" format (numbers are already canonical from parsers)
    let resolve = |nums: &[String]| -> Vec<String> {
        nums.iter()
            .map(|n| {
                let canonical = to_new_course_number(n);
                match course_names.get(&canonical) {
                    Some(cname) => format!("{}: {}", canonical, cname),
                    None => canonical,
                }
            })
            .collect()
    };

    let linked = resolve(&adjoining_nums);
    let identical = resolve(&identical_nums);
    let including = resolve(&including_nums);
    let inclusive = resolve(&included_nums);

    let full_name = format!("{}: {}", number, name);

    Ok(Course {
        full_name,
        name,
        number,
        points,
        prerequisites,
        linked,
        identical,
        overlapping: Vec::new(), // SAP doesn't distinguish overlapping vs identical
        inclusive,
        including,
        followed_by: Vec::new(), // Computed after all courses are loaded
    })
}

/// Compute `followed_by` for all courses based on prerequisites.
pub fn compute_followed_by(courses: &mut [Course]) {
    // Build a map: course_number → index
    let num_to_idx: HashMap<String, usize> = courses
        .iter()
        .enumerate()
        .map(|(i, c)| (c.number.clone(), i))
        .collect();

    // For each course, look at its prerequisites and add itself to those courses' followed_by
    let mut additions: Vec<(usize, String)> = Vec::new();

    for course in courses.iter() {
        for group in &course.prerequisites {
            for prereq_entry in group {
                // Extract course number from "number: name" format
                let prereq_num = prereq_entry.split(':').next().unwrap_or("").trim();
                if let Some(&idx) = num_to_idx.get(prereq_num) {
                    additions.push((idx, course.full_name.clone()));
                }
            }
        }
    }

    for (idx, followed_by_entry) in additions {
        if !courses[idx].followed_by.contains(&followed_by_entry) {
            courses[idx].followed_by.push(followed_by_entry);
        }
    }
}
