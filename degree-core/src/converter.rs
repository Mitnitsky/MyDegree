use crate::course::CourseDB;

/// Find courses whose number contains the given substring.
pub fn find_course<'a>(course_number: &str, db: &'a CourseDB) -> Vec<&'a crate::course::CourseDBEntry> {
    if course_number.len() < 3 {
        return vec![];
    }
    db.courses
        .iter()
        .filter(|e| e.number.contains(course_number))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::course::CourseDBEntry;

    fn sample_db() -> CourseDB {
        CourseDB {
            content_hash: None,
            courses: vec![
                CourseDBEntry {
                    full_name: "14003: סטטיסטיקה".into(),
                    name: "סטטיסטיקה".into(),
                    number: "14003".into(),
                    points: 3.0,
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
}
