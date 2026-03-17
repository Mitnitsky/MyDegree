use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::converter::find_course;
use crate::course::{CourseDB, CourseType, EXEMPTION_INDEX, default_course_types, f64_from_any, usize_from_any, bool_from_any};
use crate::semester::Semester;
use crate::utils::math_round_10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserState {
    #[serde(default, deserialize_with = "usize_from_any")]
    pub summer_semesters: usize,
    #[serde(default, deserialize_with = "usize_from_any")]
    pub active_semester: usize,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub degree_average: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub degree_points: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub degree_points_done: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub degree_points_left: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub degree_points_to_choose: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub degree_graded_points: f64,
    #[serde(default, deserialize_with = "bool_from_any")]
    pub english_exemption: bool,
    #[serde(default)]
    pub semesters: Vec<Semester>,
    #[serde(default = "default_course_types")]
    pub course_types: Vec<CourseType>,
}

impl Default for UserState {
    fn default() -> Self {
        Self {
            summer_semesters: 0,
            active_semester: 0,
            degree_average: 0.0,
            degree_points: 0.0,
            degree_points_done: 0.0,
            degree_points_left: 0.0,
            degree_points_to_choose: 0.0,
            degree_graded_points: 0.0,
            english_exemption: false,
            semesters: Vec::new(),
            course_types: default_course_types(),
        }
    }
}

fn field_display_name(field: &str) -> &str {
    match field {
        "grade" => "ציון",
        "points" => "נקודות",
        "type" => "סוג קורס",
        "binary" => "עובר/נכשל",
        "existsInDB" => "קיים במאגר",
        "average" => "ממוצע",
        "name" | "number" => field,
        _ => field,
    }
}

/// Sanitize raw UserState JSON before deserialization.
/// Walks the semesters→courses structure, detects invalid values (empty strings,
/// wrong types in numeric fields), fixes them, and returns human-readable Hebrew warnings.
/// This ensures deserialization never fails and users know what was auto-fixed.
pub fn sanitize_user_json(json: &str) -> (String, Vec<String>) {
    let mut warnings: Vec<String> = Vec::new();

    let mut root: Value = match serde_json::from_str(json) {
        Ok(v) => v,
        Err(_) => return (json.to_string(), vec!["לא ניתן לקרוא את הנתונים".into()]),
    };

    let numeric_course_fields = ["grade", "points", "type"];

    if let Some(semesters) = root.get_mut("semesters").and_then(|s| s.as_array_mut()) {
        for (sem_idx, sem) in semesters.iter_mut().enumerate() {
            let sem_name = sem.get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("")
                .to_string();
            let sem_label = if sem_name.is_empty() {
                format!("סמסטר {}", sem_idx + 1)
            } else {
                format!("סמסטר {}", sem_name)
            };

            // Check semester-level numeric fields
            for field in &["average", "points"] {
                if let Some(val) = sem.get(field) {
                    if let Value::String(s) = val {
                        let t = s.trim();
                        if t.is_empty() || t.parse::<f64>().is_err() {
                            warnings.push(format!(
                                "{}: {} לא תקין, אופס ל-0",
                                sem_label, field_display_name(field)
                            ));
                            sem[field] = Value::Number(serde_json::Number::from(0));
                        }
                    }
                }
            }

            if let Some(courses) = sem.get_mut("courses").and_then(|c| c.as_array_mut()) {
                for (course_idx, course) in courses.iter_mut().enumerate() {
                    let course_name = course.get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("")
                        .to_string();
                    let course_number = course.get("number")
                        .and_then(|n| match n {
                            Value::String(s) => Some(s.as_str()),
                            Value::Number(_n) => Some(""),
                            _ => None,
                        })
                        .unwrap_or("")
                        .to_string();

                    let course_label = if !course_name.is_empty() {
                        if !course_number.is_empty() {
                            format!("'{}' ({})", course_name, course_number)
                        } else {
                            format!("'{}'", course_name)
                        }
                    } else if !course_number.is_empty() {
                        format!("קורס {}", course_number)
                    } else {
                        format!("שורה {}", course_idx + 1)
                    };

                    let is_binary = course.get("binary")
                        .map(|v| match v {
                            Value::Bool(b) => *b,
                            Value::String(s) => s == "true" || s == "1",
                            Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
                            _ => false,
                        })
                        .unwrap_or(false);

                    for field in &numeric_course_fields {
                        if let Some(val) = course.get(*field) {
                            let needs_fix = match val {
                                Value::String(s) => {
                                    let t = s.trim();
                                    t.is_empty() || t.parse::<f64>().is_err()
                                }
                                Value::Null => true,
                                Value::Bool(_) => false,
                                Value::Number(_) => false,
                                _ => true,
                            };
                            if needs_fix {
                                // Silent fix: grade field (0 = not yet graded), binary courses, or empty/unnamed courses
                                let is_empty_course = course_name.is_empty() && course_number.is_empty();
                                let silent = is_empty_course || *field == "grade";
                                if !silent {
                                    warnings.push(format!(
                                        "{}, {}: {} לא תקין, אופס ל-0",
                                        sem_label, course_label, field_display_name(field)
                                    ));
                                }
                                course[*field] = Value::Number(serde_json::Number::from(0));
                            }
                        }
                    }
                }
            }
        }
    }

    let sanitized = serde_json::to_string(&root).unwrap_or_else(|_| json.to_string());
    (sanitized, warnings)
}

impl UserState {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn active_semester(&self) -> Option<&Semester> {
        self.semesters.get(self.active_semester)
    }

    pub fn active_semester_mut(&mut self) -> Option<&mut Semester> {
        let idx = self.active_semester;
        self.semesters.get_mut(idx)
    }

    pub fn add_semester(&mut self, initial_courses: usize) {
        self.summer_semesters = self.count_summer_semesters();
        let name = (self.semesters.len() + 1).to_string();
        self.semesters.push(Semester::new(&name, initial_courses));
        self.rename_semesters();
    }

    pub fn remove_semester(&mut self) {
        if self.semesters.len() == 1 {
            self.semesters.clear();
            self.summer_semesters = 0;
            return;
        }
        if self.active_semester < self.semesters.len() {
            self.semesters.remove(self.active_semester);
        }
        self.rename_semesters();
        self.summer_semesters = self.count_summer_semesters();
    }

    pub fn rename_semesters(&mut self) {
        let mut summer_count = 0usize;
        for (i, sem) in self.semesters.iter_mut().enumerate() {
            if sem.is_summer() {
                summer_count += 1;
            } else {
                sem.name = (1 + i - summer_count).to_string();
            }
        }
    }

    pub fn count_summer_semesters(&self) -> usize {
        self.semesters.iter().filter(|s| s.is_summer()).count()
    }

    pub fn toggle_semester_type(&mut self, index: usize) {
        if let Some(sem) = self.semesters.get_mut(index) {
            if sem.is_summer() {
                sem.name = "0".to_string();
            } else {
                sem.name = "קיץ".to_string();
            }
        }
        self.rename_semesters();
        self.summer_semesters = self.count_summer_semesters();
    }

    pub fn add_course_type(&mut self, name: &str) {
        if name.is_empty() {
            return;
        }
        if self.course_types.iter().any(|ct| ct.name == name) {
            return;
        }
        self.course_types.push(CourseType::new(name));
    }

    pub fn delete_course_type(&mut self, index: usize) {
        if index >= self.course_types.len() {
            return;
        }
        // Reset courses that had this category
        for sem in &mut self.semesters {
            for course in &mut sem.courses {
                if course.course_type == index {
                    course.course_type = 0;
                } else if course.course_type > index {
                    course.course_type -= 1;
                }
            }
        }
        self.course_types.remove(index);
    }

    pub fn move_course(&mut self, course_index: usize, direction: &str) {
        let idx = self.active_semester;
        if let Some(sem) = self.semesters.get_mut(idx) {
            let len = sem.courses.len();
            match direction {
                "up" if course_index > 0 => {
                    sem.courses.swap(course_index, course_index - 1);
                }
                "down" if course_index + 1 < len => {
                    sem.courses.swap(course_index, course_index + 1);
                }
                _ => {}
            }
        }
    }

    /// The main degree recalculation — ports `calculateUserInfo` from store.js.
    pub fn recalculate(&mut self, course_db: &CourseDB) {
        if self.semesters.is_empty() {
            return;
        }
        if self.active_semester >= self.semesters.len() {
            self.active_semester = self.semesters.len() - 1;
        }

        let mandatory_index: usize = 0;
        let english_exemption_points: f64 = if self.english_exemption { 3.0 } else { 0.0 };

        // Reset degree-level stats
        self.degree_points_done = english_exemption_points;
        self.degree_average = 0.0;
        self.degree_points_to_choose = self.degree_points - self.degree_points_done;
        self.degree_points_left = self.degree_points - self.degree_points_done;

        // Reset course type stats
        if mandatory_index < self.course_types.len() {
            self.course_types[mandatory_index].points_left =
                self.course_types[mandatory_index].points_required - english_exemption_points;
        }
        if EXEMPTION_INDEX < self.course_types.len() {
            self.course_types[EXEMPTION_INDEX].points_left = english_exemption_points;
        }
        for ct in &mut self.course_types {
            ct.average = 0.0;
            ct.points_done = 0.0;
            ct.total_points = 0.0;
            if ct.name != "חובה" && ct.name != "פטור" {
                ct.points_left = ct.points_required;
            }
            if ct.name == "פטור" {
                ct.total_points = english_exemption_points;
            }
        }

        let mut exemption_points = 0.0_f64;
        let mut failed_points = 0.0_f64;
        let mut binary_points = 0.0_f64;

        // courses_done: name -> (number, grade, binary)
        let mut courses_done: HashMap<String, (String, f64, bool)> = HashMap::new();

        // Iterate semesters in reverse (latest first)
        for sem in self.semesters.iter_mut().rev() {
            sem.calculate_average();
            sem.calculate_points();

            for course in &sem.courses {
                let course_has_number = course.number.len() > 2;
                let course_already_done = courses_done.contains_key(&course.name) && course_has_number;

                let is_sport = course.name.contains("ספורט") || course.name.contains("גופני");

                // Check if we should process this course
                let skip_main = course_already_done
                    && !is_sport
                    && courses_done.get(&course.name).map_or(false, |(num, grade, binary)| {
                        *num == course.number && (grade.round() as i64 != 0 || *binary)
                    });

                if skip_main {
                    continue;
                }

                let course_points = course.points;
                let course_grade = course.grade.round() as i64;

                // Accumulate total_points and points_left per category
                let skip_points = course_already_done
                    && !is_sport
                    && courses_done.get(&course.name).map_or(false, |(num, _, _)| {
                        *num == course.number
                    });

                if !skip_points {
                    if course.course_type < self.course_types.len() {
                        self.course_types[course.course_type].total_points += course_points;
                        if !self.course_types[course.course_type].name.contains("פטור") {
                            self.course_types[course.course_type].points_left -= course_points;
                        }
                    }
                    self.degree_points_to_choose -= course_points;
                }

                // Determine if course counts toward degree completion
                let counts_for_degree = (!course_already_done
                    && (course.binary || course_grade > 0))
                    || (!course_already_done && course.course_type == EXEMPTION_INDEX)
                    || (is_sport && (course.binary || course_grade > 0))
                    || (course_already_done
                        && courses_done.get(&course.name).map_or(false, |(_, g, b)| {
                            g.round() as i64 == 0 || *b
                        }));

                if counts_for_degree {
                    // Add to weighted average (exclude exemptions, binary, and ungraded)
                    if course.course_type != EXEMPTION_INDEX && !course.binary && course.grade != 0.0 {
                        self.degree_average += course_points * course.grade;
                        if course.course_type < self.course_types.len() {
                            self.course_types[course.course_type].average +=
                                course_points * course.grade;
                            self.course_types[course.course_type].points_done += course_points;
                        }
                    }

                    self.degree_points_left -= course_points;

                    if course_grade >= 55
                        || course.course_type == EXEMPTION_INDEX
                        || course.binary
                    {
                        if course.course_type == EXEMPTION_INDEX {
                            exemption_points += course_points;
                        }
                        if course.binary && course.course_type != EXEMPTION_INDEX {
                            binary_points += course_points;
                        }
                        self.degree_points_done += course_points;
                    } else if course_grade != 0 {
                        failed_points += course_points;
                    }
                }

                // Mark course as done, including overlapping/identical/inclusive
                let course_info = find_course(&course.number, course_db);
                courses_done.insert(
                    course.name.clone(),
                    (course.number.clone(), course.grade, course.binary),
                );

                if let Some(info) = course_info.first() {
                    for key in info.overlapping.iter()
                        .chain(info.identical.iter())
                        .chain(info.inclusive.iter())
                    {
                        let parts: Vec<&str> = key.splitn(2, ':').collect();
                        if parts.len() == 2 {
                            let num = parts[0].trim().to_string();
                            let name = parts[1].trim().to_string();
                            courses_done.insert(name, (num, course.grade, course.binary));
                        }
                    }
                }
            }
        }

        // Final degree average
        let degree_points_with_grade = self.degree_points_done
            - english_exemption_points
            - exemption_points
            - binary_points
            + failed_points;

        if degree_points_with_grade != 0.0 {
            self.degree_average /= degree_points_with_grade;
            self.degree_average = math_round_10(self.degree_average, -1);
        } else {
            self.degree_average = 0.0;
        }
        self.degree_graded_points = degree_points_with_grade;

        // Per-category averages
        for ct in &mut self.course_types {
            if ct.points_done > 0.0 {
                ct.average /= ct.points_done;
                ct.average = math_round_10(ct.average, -1);
            }
        }

        self.degree_points_left = self.degree_points - self.degree_points_done;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::course::Course;

    fn empty_db() -> CourseDB {
        CourseDB {
            courses: vec![],
        }
    }

    #[test]
    fn test_default_state() {
        let state = UserState::default();
        assert_eq!(state.semesters.len(), 0);
        assert_eq!(state.course_types.len(), 6);
        assert_eq!(state.degree_average, 0.0);
    }

    #[test]
    fn test_add_remove_semester() {
        let mut state = UserState::default();
        state.add_semester(5);
        assert_eq!(state.semesters.len(), 1);
        assert_eq!(state.semesters[0].name, "1");

        state.add_semester(5);
        assert_eq!(state.semesters.len(), 2);
        assert_eq!(state.semesters[1].name, "2");

        state.active_semester = 0;
        state.remove_semester();
        assert_eq!(state.semesters.len(), 1);
        assert_eq!(state.semesters[0].name, "1");
    }

    #[test]
    fn test_recalculate_basic() {
        let mut state = UserState::default();
        state.degree_points = 120.0;
        state.add_semester(0);

        state.semesters[0].courses.push(Course {
            name: "Math".into(),
            number: "12345".into(),
            points: 3.0,
            grade: 90.0,
            course_type: 0,
            ..Default::default()
        });
        state.semesters[0].courses.push(Course {
            name: "Physics".into(),
            number: "12346".into(),
            points: 4.0,
            grade: 80.0,
            course_type: 0,
            ..Default::default()
        });

        state.recalculate(&empty_db());

        // degree_average = (90*3 + 80*4) / (3+4) = 590/7 ≈ 84.3
        assert!((state.degree_average - 84.3).abs() < 0.1);
        assert!((state.degree_points_done - 7.0).abs() < 0.1);
    }

    #[test]
    fn test_recalculate_with_exemption() {
        let mut state = UserState::default();
        state.degree_points = 120.0;
        state.english_exemption = true;
        state.add_semester(0);

        state.semesters[0].courses.push(Course {
            name: "Math".into(),
            number: "12345".into(),
            points: 3.0,
            grade: 90.0,
            course_type: 0,
            ..Default::default()
        });

        state.recalculate(&empty_db());

        // english_exemption adds 3 points to done
        assert!((state.degree_points_done - 6.0).abs() < 0.1);
        assert!((state.degree_average - 90.0).abs() < 0.1);
    }

    #[test]
    fn test_toggle_semester_type() {
        let mut state = UserState::default();
        state.add_semester(0);
        state.add_semester(0);
        assert_eq!(state.semesters[0].name, "1");

        state.toggle_semester_type(0);
        assert!(state.semesters[0].is_summer());
        assert_eq!(state.semesters[1].name, "1"); // renumbered
        assert_eq!(state.summer_semesters, 1);
    }

    #[test]
    fn test_sanitize_empty_grade_and_points() {
        let json = r#"{
            "semesters": [{
                "name": "1",
                "average": "",
                "points": "3",
                "courses": [
                    {"name": "חשבון", "number": "104012", "grade": "", "points": "3.5", "type": "0"},
                    {"name": "פיזיקה", "number": "114071", "grade": "85", "points": "", "type": ""}
                ]
            }]
        }"#;
        let (sanitized, warnings) = sanitize_user_json(json);
        assert!(!warnings.is_empty());
        // Grade is silently fixed (empty = not yet graded), no warning for it
        assert!(!warnings.iter().any(|w| w.contains("חשבון") && w.contains("ציון")));
        // But points and type for פיזיקה should still warn
        assert!(warnings.iter().any(|w| w.contains("פיזיקה") && w.contains("נקודות")));
        assert!(warnings.iter().any(|w| w.contains("פיזיקה") && w.contains("סוג קורס")));
        // Semester average was empty
        assert!(warnings.iter().any(|w| w.contains("סמסטר 1") && w.contains("ממוצע")));
        // The sanitized JSON should parse without errors
        let _user: UserState = serde_json::from_str(&sanitized).unwrap();
    }

    #[test]
    fn test_sanitize_clean_data_no_warnings() {
        let json = r#"{
            "semesters": [{
                "name": "1",
                "average": 90.0,
                "points": 3.0,
                "courses": [
                    {"name": "Math", "number": "12345", "grade": 90, "points": 3.0, "type": 0}
                ]
            }]
        }"#;
        let (_sanitized, warnings) = sanitize_user_json(json);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_sanitize_unnamed_course_silently_fixed() {
        let json = r#"{
            "semesters": [{
                "name": "2",
                "courses": [
                    {"name": "", "number": "", "grade": "", "points": "", "type": ""}
                ]
            }]
        }"#;
        let (sanitized, warnings) = sanitize_user_json(json);
        // Empty/unnamed courses should be fixed silently — no warnings
        assert!(warnings.is_empty(), "Expected no warnings for unnamed course, got: {:?}", warnings);
        // But values should still be fixed to 0
        let root: serde_json::Value = serde_json::from_str(&sanitized).unwrap();
        let grade = root["semesters"][0]["courses"][0]["grade"].as_i64().unwrap();
        assert_eq!(grade, 0);
    }
}
