use serde::{Deserialize, Serialize};

pub const EXEMPTION_INDEX: usize = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Course {
    #[serde(default, rename = "existsInDB")]
    pub exists_in_db: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub number: String,
    #[serde(default)]
    pub points: f64,
    #[serde(default)]
    pub grade: f64,
    #[serde(default, rename = "type")]
    pub course_type: usize,
    #[serde(default)]
    pub binary: bool,
}

impl Default for Course {
    fn default() -> Self {
        Self {
            exists_in_db: false,
            name: String::new(),
            number: String::new(),
            points: 0.0,
            grade: 0.0,
            course_type: 0,
            binary: false,
        }
    }
}

impl Course {
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
            && (self.number.is_empty() || self.number.parse::<i32>().unwrap_or(0) == 0)
    }

    pub fn from_db_entry(entry: &CourseDBEntry, grade: Option<&str>) -> Self {
        let (parsed_grade, course_type) = match grade {
            Some(g) => {
                let parsed = g.parse::<f64>().unwrap_or(0.0);
                let ct = if g.contains("פטור") { EXEMPTION_INDEX } else { 0 };
                (parsed, ct)
            }
            None => (0.0, 0),
        };
        Self {
            exists_in_db: true,
            name: entry.name.clone(),
            number: entry.number.clone(),
            points: entry.points,
            grade: parsed_grade,
            course_type,
            binary: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseType {
    pub name: String,
    #[serde(default)]
    pub total_points: f64,
    #[serde(default)]
    pub points_left: f64,
    #[serde(default)]
    pub points_required: f64,
    #[serde(default)]
    pub points_done: f64,
    #[serde(default)]
    pub average: f64,
}

impl CourseType {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_points: 0.0,
            points_left: 0.0,
            points_required: 0.0,
            points_done: 0.0,
            average: 0.0,
        }
    }
}

pub fn default_course_types() -> Vec<CourseType> {
    vec![
        CourseType::new("חובה"),
        CourseType::new("פטור"),
        CourseType::new("מל\"ג"),
        CourseType::new("בחירה חופשית"),
        CourseType::new("רשימה א'"),
        CourseType::new("רשימה ב'"),
    ]
}

// --- Static course database (courses.json) ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CourseDBEntry {
    pub full_name: String,
    pub name: String,
    pub number: String,
    pub points: f64,
    #[serde(default)]
    pub prerequisites: Vec<Vec<String>>,
    #[serde(default)]
    pub linked: Vec<String>,
    #[serde(default)]
    pub identical: Vec<String>,
    #[serde(default)]
    pub overlapping: Vec<String>,
    #[serde(default)]
    pub inclusive: Vec<String>,
    #[serde(default)]
    pub including: Vec<String>,
    #[serde(default)]
    pub followed_by: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseDB {
    /// Content hash for cache invalidation. Auto-computed from course data.
    /// Replaces the old manual `version` number.
    #[serde(default)]
    pub content_hash: Option<String>,
    pub courses: Vec<CourseDBEntry>,
}

impl CourseDB {
    /// Compute a deterministic content hash from all course data.
    /// Any change to courses (added, removed, renamed, repointed) produces a different hash.
    pub fn compute_content_hash(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.courses.len().hash(&mut hasher);
        for course in &self.courses {
            course.number.hash(&mut hasher);
            course.name.hash(&mut hasher);
            course.points.to_bits().hash(&mut hasher);
        }
        format!("{:016x}", hasher.finish())
    }

    /// Deserialize from JSON with graceful fallback.
    /// Handles both new format (content_hash) and old format (version number).
    /// If deserialization fails entirely, returns None.
    pub fn from_json(json: &str) -> Option<Self> {
        // Try direct deserialization first (handles content_hash field)
        if let Ok(db) = serde_json::from_str::<CourseDB>(json) {
            return Some(db);
        }
        // Fallback: try parsing as a generic object with a "courses" array
        // This handles old format with "version": 12.0 or any unexpected field
        let val: serde_json::Value = serde_json::from_str(json).ok()?;
        let courses_val = val.get("courses")?;
        let courses: Vec<CourseDBEntry> = serde_json::from_value(courses_val.clone()).ok()?;
        Some(CourseDB {
            content_hash: None,
            courses,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_default() {
        let c = Course::default();
        assert!(c.is_empty());
        assert_eq!(c.grade, 0.0);
        assert_eq!(c.course_type, 0);
        assert!(!c.binary);
    }

    #[test]
    fn test_course_is_empty() {
        let mut c = Course::default();
        assert!(c.is_empty());
        c.name = "test".into();
        assert!(!c.is_empty());
    }

    #[test]
    fn test_default_course_types() {
        let types = default_course_types();
        assert_eq!(types.len(), 6);
        assert_eq!(types[0].name, "חובה");
        assert_eq!(types[1].name, "פטור");
    }

    #[test]
    fn test_from_json_new_format() {
        let json = r#"{"content_hash":"abc123","courses":[]}"#;
        let db = CourseDB::from_json(json).unwrap();
        assert_eq!(db.content_hash, Some("abc123".to_string()));
        assert!(db.courses.is_empty());
    }

    #[test]
    fn test_from_json_old_version_format() {
        let json = r#"{"version":12.0,"courses":[]}"#;
        let db = CourseDB::from_json(json).unwrap();
        // Old format has no content_hash — falls through to fallback parser
        assert!(db.courses.is_empty());
    }

    #[test]
    fn test_from_json_no_version_field() {
        let json = r#"{"courses":[]}"#;
        let db = CourseDB::from_json(json).unwrap();
        assert_eq!(db.content_hash, None);
    }

    #[test]
    fn test_from_json_garbage_returns_none() {
        assert!(CourseDB::from_json("not json at all").is_none());
    }

    #[test]
    fn test_content_hash_deterministic() {
        let db = CourseDB { content_hash: None, courses: vec![] };
        let h1 = db.compute_content_hash();
        let h2 = db.compute_content_hash();
        assert_eq!(h1, h2);
    }
}
