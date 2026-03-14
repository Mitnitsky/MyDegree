use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    println!("cargo:rerun-if-changed=courses.json");

    let data = std::fs::read_to_string("courses.json")
        .expect("Cannot read courses.json");

    // Parse just enough to compute the same hash the runtime would
    let val: serde_json::Value = serde_json::from_str(&data)
        .expect("courses.json is not valid JSON");
    let courses = val["courses"].as_array()
        .expect("courses.json missing 'courses' array");

    let mut hasher = DefaultHasher::new();
    courses.len().hash(&mut hasher);
    for c in courses {
        c["number"].as_str().unwrap_or("").hash(&mut hasher);
        c["name"].as_str().unwrap_or("").hash(&mut hasher);
        c["points"].as_f64().unwrap_or(0.0).to_bits().hash(&mut hasher);
    }
    let hash = format!("{:016x}", hasher.finish());

    println!("cargo:rustc-env=COURSES_HASH={}", hash);
}
