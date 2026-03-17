use crate::course::CourseDB;

/// Normalize a course number to 8-digit format.
/// Old 6-digit numbers like "234123" become "02340123".
/// Special case: "9730XX" becomes "970300XX".
pub fn normalize_course_number(num: &str) -> String {
    let trimmed = num.trim();
    if trimmed.len() == 6 && trimmed.bytes().all(|b| b.is_ascii_digit()) {
        if trimmed.starts_with("9730") {
            return format!("970300{}", &trimmed[4..]);
        }
        return format!("0{}0{}", &trimmed[..3], &trimmed[3..]);
    }
    trimmed.to_string()
}

/// Find courses whose number contains the given substring.
pub fn find_course<'a>(course_number: &str, db: &'a CourseDB) -> Vec<&'a crate::course::CourseDBEntry> {
    if course_number.len() < 3 {
        return vec![];
    }
    let normalized = normalize_course_number(course_number);
    db.courses
        .iter()
        .filter(|e| e.number.contains(course_number) || e.number.contains(&normalized))
        .collect()
}

/// Department info derived from the 3-digit course number prefix.
#[derive(Debug, Clone, PartialEq)]
pub struct Department {
    pub prefix: &'static str,
    pub name_he: &'static str,
    pub name_en: &'static str,
}

/// All known Technion departments by course number prefix.
pub const DEPARTMENTS: &[Department] = &[
    Department { prefix: "001", name_he: "הנדסת תעשייה וניהול", name_en: "Industrial Engineering" },
    Department { prefix: "003", name_he: "הנדסת מכונות", name_en: "Mechanical Engineering" },
    Department { prefix: "004", name_he: "הנדסת חשמל ומחשבים", name_en: "Electrical & Computer Eng." },
    Department { prefix: "005", name_he: "הנדסה כימית", name_en: "Chemical Engineering" },
    Department { prefix: "006", name_he: "ביוטכנולוגיה ומזון", name_en: "Biotechnology & Food Eng." },
    Department { prefix: "008", name_he: "הנדסת אווירונאוטיקה וחלל", name_en: "Aerospace Engineering" },
    Department { prefix: "009", name_he: "הנדסת תעשייה וניהול", name_en: "IE&M" },
    Department { prefix: "010", name_he: "מתמטיקה", name_en: "Mathematics" },
    Department { prefix: "011", name_he: "פיזיקה", name_en: "Physics" },
    Department { prefix: "012", name_he: "כימיה", name_en: "Chemistry" },
    Department { prefix: "013", name_he: "ביולוגיה", name_en: "Biology" },
    Department { prefix: "019", name_he: "מתמטיקה שימושית", name_en: "Applied Mathematics" },
    Department { prefix: "020", name_he: "אדריכלות ובינוי ערים", name_en: "Architecture" },
    Department { prefix: "021", name_he: "חינוך למדע וטכנולוגיה", name_en: "Science Education" },
    Department { prefix: "023", name_he: "מדעי המחשב", name_en: "Computer Science" },
    Department { prefix: "027", name_he: "רפואה", name_en: "Medicine" },
    Department { prefix: "031", name_he: "הנדסת חומרים", name_en: "Materials Engineering" },
    Department { prefix: "032", name_he: "מדעים הומניסטיים ואמנויות", name_en: "Humanities & Arts" },
    Department { prefix: "033", name_he: "הנדסה ביו-רפואית", name_en: "Biomedical Engineering" },
    Department { prefix: "039", name_he: "מוסיקה", name_en: "Music" },
    Department { prefix: "061", name_he: "יזמות", name_en: "Entrepreneurship" },
    Department { prefix: "064", name_he: "ננוטכנולוגיה", name_en: "Nanotechnology" },
    Department { prefix: "073", name_he: "מערכות אוטונומיות", name_en: "Autonomous Systems" },
    Department { prefix: "074", name_he: "הנדסת מערכות", name_en: "Systems Engineering" },
    Department { prefix: "085", name_he: "הנדסת פולימרים", name_en: "Polymer Engineering" },
    Department { prefix: "510", name_he: "אנרגיה", name_en: "Energy" },
    Department { prefix: "520", name_he: "הנדסה ימית", name_en: "Marine Engineering" },
    Department { prefix: "610", name_he: "מדיה מקושרת", name_en: "Connected Media" },
    Department { prefix: "970", name_he: "מכינה", name_en: "Preparatory" },
];

/// Get the Hebrew department name from a course number.
pub fn department_name(course_number: &str) -> &'static str {
    if course_number.len() >= 3 {
        let prefix = &course_number[..3];
        for dept in DEPARTMENTS {
            if dept.prefix == prefix {
                return dept.name_he;
            }
        }
    }
    "אחר"
}

/// Course status relative to user's progress.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CourseStatus {
    Done,
    Overlap,
    Available,
    Blocked,
}

fn completed_contains(num: &str, completed: &std::collections::HashSet<String>) -> bool {
    completed.contains(num) || completed.contains(&normalize_course_number(num))
}

/// Determine a course's status given a set of completed course numbers.
/// The completed set should contain normalized 8-digit numbers.
pub fn course_status(
    course: &crate::course::CourseDBEntry,
    completed: &std::collections::HashSet<String>,
) -> CourseStatus {
    if completed_contains(&course.number, completed) {
        return CourseStatus::Done;
    }
    // Check if an overlapping, identical, or inclusive course was already completed
    let has_overlap = course.overlapping.iter()
        .chain(course.identical.iter())
        .chain(course.inclusive.iter())
        .any(|entry| {
            let num = entry.split(':').next().unwrap_or("").trim();
            !num.is_empty() && completed_contains(num, completed)
        });
    if has_overlap {
        return CourseStatus::Overlap;
    }
    // Check if all prerequisite groups are satisfied
    let all_met = course.prerequisites.iter().all(|or_group| {
        or_group.is_empty() || or_group.iter().any(|prereq| {
            let num = prereq.split(':').next().unwrap_or("").trim();
            completed_contains(num, completed)
        })
    });
    if all_met { CourseStatus::Available } else { CourseStatus::Blocked }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::course::CourseDBEntry;

    fn sample_db() -> CourseDB {
        CourseDB {
            courses: vec![
                CourseDBEntry {
                    full_name: "14003: סטטיסטיקה".into(),
                    name: "סטטיסטיקה".into(),
                    number: "14003".into(),
                    points: 3.0,
                    faculty: String::new(),
                    prerequisites: vec![],
                    linked: vec![],
                    identical: vec![],
                    overlapping: vec![],
                    inclusive: vec![],
                    including: vec![],
                    followed_by: vec![],
                },
                CourseDBEntry {
                    full_name: "14004: נתוח מערכות".into(),
                    name: "נתוח מערכות".into(),
                    number: "14004".into(),
                    points: 3.0,
                    faculty: String::new(),
                    prerequisites: vec![],
                    linked: vec![],
                    identical: vec![],
                    overlapping: vec![],
                    inclusive: vec![],
                    including: vec![],
                    followed_by: vec![],
                },
            ],
        }
    }

    #[test]
    fn test_find_by_number() {
        let db = sample_db();
        let results = find_course("14003", &db);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "סטטיסטיקה");
    }

    #[test]
    fn test_find_partial() {
        let db = sample_db();
        let results = find_course("1400", &db);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_find_too_short() {
        let db = sample_db();
        let results = find_course("14", &db);
        assert!(results.is_empty());
    }

    #[test]
    fn test_normalize_6_to_8_digit() {
        assert_eq!(normalize_course_number("234123"), "02340123");
        assert_eq!(normalize_course_number("014003"), "00140003");
        assert_eq!(normalize_course_number("236319"), "02360319");
    }

    #[test]
    fn test_normalize_special_9730() {
        assert_eq!(normalize_course_number("973001"), "97030001");
        assert_eq!(normalize_course_number("973099"), "97030099");
    }

    #[test]
    fn test_normalize_already_8_digit() {
        assert_eq!(normalize_course_number("02340123"), "02340123");
        assert_eq!(normalize_course_number("00140003"), "00140003");
    }

    #[test]
    fn test_normalize_short_numbers() {
        assert_eq!(normalize_course_number("12345"), "12345");
        assert_eq!(normalize_course_number("1234"), "1234");
    }

    #[test]
    fn test_course_status_with_old_numbers() {
        let course = CourseDBEntry {
            full_name: "02340123: אלגוריתמים".into(),
            name: "אלגוריתמים".into(),
            number: "02340123".into(),
            points: 3.0,
            faculty: String::new(),
            prerequisites: vec![],
            linked: vec![],
            identical: vec![],
            overlapping: vec![],
            inclusive: vec![],
            including: vec![],
            followed_by: vec![],
        };
        // User has old 6-digit number
        let mut completed = std::collections::HashSet::new();
        completed.insert("02340123".to_string()); // normalized form
        assert_eq!(course_status(&course, &completed), CourseStatus::Done);
    }
}
