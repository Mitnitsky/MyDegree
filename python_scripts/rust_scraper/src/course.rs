use serde::{Deserialize, Serialize};

/// Course in the target JSON format (matches courses.json).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub full_name: String,
    pub name: String,
    pub number: String,
    pub points: f64,
    /// Each inner Vec is an AND-group; the outer Vec is OR between groups.
    /// e.g. `[["A","B"],["C"]]` means `(A AND B) OR (C)`.
    pub prerequisites: Vec<Vec<String>>,
    pub linked: Vec<String>,
    pub identical: Vec<String>,
    pub overlapping: Vec<String>,
    pub inclusive: Vec<String>,
    pub including: Vec<String>,
    pub followed_by: Vec<String>,
}

impl Course {
    pub fn new() -> Self {
        Self {
            full_name: String::new(),
            name: String::new(),
            number: String::new(),
            points: 0.0,
            prerequisites: vec![vec![]],
            linked: Vec::new(),
            identical: Vec::new(),
            overlapping: Vec::new(),
            inclusive: Vec::new(),
            including: Vec::new(),
            followed_by: Vec::new(),
        }
    }
}

/// Top-level wrapper matching `{"courses": [...]}`.
#[derive(Debug, Serialize, Deserialize)]
pub struct CoursesJson {
    pub courses: Vec<Course>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_aliases: Option<std::collections::HashMap<String, String>>,
}
