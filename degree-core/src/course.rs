use serde::{Deserialize, Serialize, Deserializer};

pub const EXEMPTION_INDEX: usize = 1;

/// Deserialize an f64 that may be stored as a string (e.g. "33.0") or a number.
pub fn f64_from_any<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    let val = serde_json::Value::deserialize(deserializer)?;
    match &val {
        serde_json::Value::Number(n) => n.as_f64().ok_or_else(|| serde::de::Error::custom("invalid number")),
        serde_json::Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
        serde_json::Value::Null => Ok(0.0),
        _ => Err(serde::de::Error::custom("expected number or string")),
    }
}

/// Deserialize a usize that may be stored as a string (e.g. "2") or a number.
pub fn usize_from_any<'de, D: Deserializer<'de>>(deserializer: D) -> Result<usize, D::Error> {
    let val = serde_json::Value::deserialize(deserializer)?;
    match &val {
        serde_json::Value::Number(n) => n.as_u64().map(|u| u as usize).ok_or_else(|| serde::de::Error::custom("invalid usize")),
        serde_json::Value::String(s) => s.parse::<usize>().map_err(serde::de::Error::custom),
        serde_json::Value::Null => Ok(0),
        _ => Err(serde::de::Error::custom("expected number or string")),
    }
}

/// Deserialize a bool that may be stored as a string (e.g. "true") or a bool.
pub fn bool_from_any<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let val = serde_json::Value::deserialize(deserializer)?;
    match &val {
        serde_json::Value::Bool(b) => Ok(*b),
        serde_json::Value::String(s) => match s.as_str() {
            "true" | "1" => Ok(true),
            _ => Ok(false),
        },
        serde_json::Value::Number(n) => Ok(n.as_i64().unwrap_or(0) != 0),
        serde_json::Value::Null => Ok(false),
        _ => Err(serde::de::Error::custom("expected bool or string")),
    }
}

/// Deserialize a String that may be stored as a number (e.g. `1` instead of `"1"`).
pub fn string_from_any<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let val = serde_json::Value::deserialize(deserializer)?;
    match &val {
        serde_json::Value::String(s) => Ok(s.clone()),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        serde_json::Value::Bool(b) => Ok(b.to_string()),
        serde_json::Value::Null => Ok(String::new()),
        _ => Err(serde::de::Error::custom("expected string or number")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Course {
    #[serde(default, rename = "existsInDB", deserialize_with = "bool_from_any")]
    pub exists_in_db: bool,
    #[serde(default, deserialize_with = "string_from_any")]
    pub name: String,
    #[serde(default, deserialize_with = "string_from_any")]
    pub number: String,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub points: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub grade: f64,
    #[serde(default, rename = "type", deserialize_with = "usize_from_any")]
    pub course_type: usize,
    #[serde(default, deserialize_with = "bool_from_any")]
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
    #[serde(default, deserialize_with = "string_from_any")]
    pub name: String,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub total_points: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub points_left: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub points_required: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
    pub points_done: f64,
    #[serde(default, deserialize_with = "f64_from_any")]
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
    pub courses: Vec<CourseDBEntry>,
}

impl CourseDB {
    /// Deserialize from JSON. Extra fields (e.g. legacy "version") are ignored by serde.
    pub fn from_json(json: &str) -> Option<Self> {
        serde_json::from_str::<CourseDB>(json).ok()
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
    fn test_from_json_with_extra_fields() {
        // Extra fields like "content_hash" or "version" are ignored by serde
        let json = r#"{"content_hash":"abc123","courses":[]}"#;
        let db = CourseDB::from_json(json).unwrap();
        assert!(db.courses.is_empty());
    }

    #[test]
    fn test_from_json_minimal() {
        let json = r#"{"courses":[]}"#;
        let db = CourseDB::from_json(json).unwrap();
        assert!(db.courses.is_empty());
    }

    #[test]
    fn test_from_json_garbage_returns_none() {
        assert!(CourseDB::from_json("not json at all").is_none());
    }
}
