/// Round `value` to the decimal place specified by `exp`.
/// e.g. `math_round_10(84.285714, -1)` → `84.3`
pub fn math_round_10(value: f64, exp: i32) -> f64 {
    if exp == 0 {
        return value.round();
    }
    if value.is_nan() {
        return f64::NAN;
    }
    let factor = 10.0_f64.powi(-exp);
    (value * factor).round() / factor
}

/// Download helper — in WASM this will trigger a browser download.
/// For the core crate we just provide the JSON serialization.
pub fn export_semesters_json(semesters: &[crate::Semester], with_grades: bool) -> String {
    let mut copy: Vec<crate::Semester> = semesters.to_vec();
    if !with_grades {
        for sem in &mut copy {
            for course in &mut sem.courses {
                course.grade = 0.0;
            }
            sem.calculate_average();
            sem.calculate_points();
        }
    }
    serde_json::to_string_pretty(&copy).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_one_decimal() {
        assert!((math_round_10(84.285714, -1) - 84.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_round_integer() {
        assert!((math_round_10(84.0, -1) - 84.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_round_half_up() {
        assert!((math_round_10(84.25, -1) - 84.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_round_exp_zero() {
        assert!((math_round_10(84.5, 0) - 85.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_round_nan() {
        assert!(math_round_10(f64::NAN, -1).is_nan());
    }
}
