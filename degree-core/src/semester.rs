use serde::{Deserialize, Serialize};

use crate::course::{Course, CourseDBEntry, EXEMPTION_INDEX, f64_from_any};
use crate::utils::math_round_10;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Semester {
    pub name: String,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub average: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub points: f64,
    pub courses: Vec<Course>,
}

impl Semester {
    pub fn new(name: &str, initial_courses: usize) -> Self {
        let mut courses = Vec::with_capacity(initial_courses);
        for _ in 0..initial_courses {
            courses.push(Course::default());
        }
        Self {
            name: name.to_string(),
            average: 0.0,
            points: 0.0,
            courses,
        }
    }

    pub fn add_empty_course(&mut self) {
        self.courses.push(Course::default());
    }

    /// Add an existing course (from DB or paste). Returns the index where it was placed.
    pub fn add_existing_course(&mut self, entry: &CourseDBEntry, grade: Option<&str>) -> usize {
        // Try to fill an empty slot first
        for i in 0..self.courses.len() {
            if self.courses[i].is_empty() {
                self.courses[i].name = entry.name.clone();
                self.courses[i].points = entry.points;
                self.courses[i].number = entry.number.clone();
                match grade {
                    Some(g) => {
                        let parsed = g.parse::<f64>().unwrap_or(0.0);
                        if parsed.is_nan() || parsed == 0.0 {
                            if g.contains("פטור") {
                                self.courses[i].course_type = EXEMPTION_INDEX;
                            }
                            self.courses[i].grade = 0.0;
                        } else {
                            self.courses[i].grade = parsed;
                        }
                    }
                    None => {
                        self.courses[i].grade = 0.0;
                    }
                }
                self.calculate_average();
                self.calculate_points();
                return i;
            }
        }
        // No empty slot — append
        self.courses.push(Course::from_db_entry(entry, grade));
        self.calculate_average();
        self.calculate_points();
        self.courses.len() - 1
    }

    pub fn remove_course(&mut self, index: usize) {
        if index < self.courses.len() {
            self.courses.remove(index);
        }
        self.calculate_average();
        self.calculate_points();
    }

    pub fn calculate_average(&mut self) {
        let mut points = 0.0_f64;
        let mut binary_points = 0.0_f64;
        let mut total_grade = 0.0_f64;

        for course in &self.courses {
            let cp = course.points;
            let cg = course.grade;
            if cg != 0.0
                && (course.binary || cp != 0.0)
                && course.course_type != EXEMPTION_INDEX
            {
                if course.binary {
                    binary_points += cp;
                } else {
                    total_grade += cg * cp;
                }
                points += cp;
            }
        }

        let points_graded = points - binary_points;
        if points_graded != 0.0 {
            let avg = total_grade / points_graded;
            // If the result is an integer, keep it; otherwise round to 1 decimal
            if avg == avg.floor() {
                self.average = avg;
            } else {
                self.average = math_round_10(avg, -1);
            }
        } else {
            self.average = 0.0;
        }
    }

    pub fn calculate_points(&mut self) {
        self.points = 0.0;
        for course in &self.courses {
            self.points += course.points;
        }
    }

    pub fn has_course(&self, course_number: &str) -> bool {
        self.courses.iter().any(|c| c.number == course_number)
    }

    pub fn is_summer(&self) -> bool {
        self.name.contains("קיץ")
    }
}

/// Check if a course exists in semesters up to `stop_index`.
/// Returns the semester display number where found, or `None`.
pub enum FoundInSemester {
    Regular(usize),
    Summer,
    NotFound,
}

pub fn course_exist_in_semesters(
    semesters: &[Semester],
    course_number: &str,
    stop_index: Option<usize>,
) -> FoundInSemester {
    let stop = stop_index.unwrap_or(semesters.len().saturating_sub(1));
    let mut summer_count = 0usize;
    let mut result = FoundInSemester::NotFound;

    for (i, sem) in semesters.iter().enumerate() {
        if i > stop {
            break;
        }
        if sem.is_summer() {
            summer_count += 1;
        }
        if sem.has_course(course_number) {
            if sem.is_summer() {
                result = FoundInSemester::Summer;
            } else {
                result = FoundInSemester::Regular(i + 1 - summer_count);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_semester() {
        let sem = Semester::new("1", 5);
        assert_eq!(sem.courses.len(), 5);
        assert_eq!(sem.name, "1");
        assert_eq!(sem.average, 0.0);
    }

    #[test]
    fn test_calculate_average_basic() {
        let mut sem = Semester::new("1", 0);
        sem.courses.push(Course {
            name: "Math".into(),
            number: "12345".into(),
            points: 3.0,
            grade: 90.0,
            course_type: 0,
            ..Default::default()
        });
        sem.courses.push(Course {
            name: "Physics".into(),
            number: "12346".into(),
            points: 4.0,
            grade: 80.0,
            course_type: 0,
            ..Default::default()
        });
        sem.calculate_average();
        // (90*3 + 80*4) / (3+4) = (270+320)/7 = 590/7 ≈ 84.3
        assert!((sem.average - 84.3).abs() < 0.01);
    }

    #[test]
    fn test_calculate_average_excludes_exemptions() {
        let mut sem = Semester::new("1", 0);
        sem.courses.push(Course {
            name: "Math".into(),
            number: "12345".into(),
            points: 3.0,
            grade: 90.0,
            course_type: 0,
            ..Default::default()
        });
        sem.courses.push(Course {
            name: "English".into(),
            number: "12350".into(),
            points: 3.0,
            grade: 85.0,
            course_type: EXEMPTION_INDEX,
            ..Default::default()
        });
        sem.calculate_average();
        assert!((sem.average - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_average_binary_courses() {
        let mut sem = Semester::new("1", 0);
        sem.courses.push(Course {
            name: "Sport".into(),
            number: "39400".into(),
            points: 1.0,
            grade: 100.0,
            course_type: 0,
            binary: true,
            ..Default::default()
        });
        sem.courses.push(Course {
            name: "Math".into(),
            number: "12345".into(),
            points: 3.0,
            grade: 85.0,
            course_type: 0,
            ..Default::default()
        });
        sem.calculate_average();
        // Binary doesn't contribute to grade average, only graded courses
        assert!((sem.average - 85.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_points() {
        let mut sem = Semester::new("1", 0);
        sem.courses.push(Course {
            points: 3.0,
            ..Default::default()
        });
        sem.courses.push(Course {
            points: 4.5,
            ..Default::default()
        });
        sem.calculate_points();
        assert!((sem.points - 7.5).abs() < 0.01);
    }

    #[test]
    fn test_has_course() {
        let mut sem = Semester::new("1", 0);
        sem.courses.push(Course {
            number: "12345".into(),
            ..Default::default()
        });
        assert!(sem.has_course("12345"));
        assert!(!sem.has_course("99999"));
    }

    #[test]
    fn test_remove_course() {
        let mut sem = Semester::new("1", 0);
        sem.courses.push(Course {
            name: "A".into(),
            points: 3.0,
            ..Default::default()
        });
        sem.courses.push(Course {
            name: "B".into(),
            points: 4.0,
            ..Default::default()
        });
        sem.remove_course(0);
        assert_eq!(sem.courses.len(), 1);
        assert_eq!(sem.courses[0].name, "B");
    }
}
