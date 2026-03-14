use crate::course::Course;
use std::cmp::Ordering;

/// Custom alphabet for Hebrew + Latin sorting (matches the JS CharCompare).
const ALPHABET: &[char] = &[
    ' ', '-', ',', '\'',
    '1', '2', '3', '4', '5', '6', '7', '8', '9',
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'v', 'u', 'w', 'x', 'y', 'z',
    'א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט', 'י',
    'כ', 'ל', 'מ', 'ם', 'נ', 'ן', 'ס', 'ע', 'פ', 'ף',
    'צ', 'ץ', 'ק', 'ר', 'ש', 'ת',
];

fn char_index(c: char) -> Option<usize> {
    let upper = c.to_uppercase().next().unwrap_or(c);
    ALPHABET.iter().position(|&a| a == upper || a == c)
}

/// Compare two strings using the custom Hebrew/Latin alphabet.
/// Returns Ordering like the JS CharCompare.
pub fn char_compare(a: &str, b: &str) -> Ordering {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len = a_chars.len().min(b_chars.len());

    for i in 0..len {
        let ai = char_index(a_chars[i]).unwrap_or(usize::MAX);
        let bi = char_index(b_chars[i]).unwrap_or(usize::MAX);
        match ai.cmp(&bi) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal // matches JS behavior: returns 0 when one string ends
}

fn compare_by_numeric_field(a: &Course, b: &Course, field: &str) -> Ordering {
    let va = get_numeric_field(a, field);
    let vb = get_numeric_field(b, field);
    match (va, vb) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(Ordering::Equal),
    }
}

fn get_numeric_field(course: &Course, field: &str) -> Option<f64> {
    match field {
        "points" => Some(course.points),
        "grade" => Some(course.grade),
        "type" => Some(course.course_type as f64),
        "number" => course.number.parse::<f64>().ok(),
        _ => None,
    }
}

fn is_array_sorted(courses: &[Course], field: &str) -> bool {
    for i in 0..courses.len().saturating_sub(1) {
        if field == "name" {
            if char_compare(&courses[i].name, &courses[i + 1].name) == Ordering::Greater {
                return false;
            }
        } else {
            if courses[i + 1].name.is_empty() {
                continue;
            }
            if compare_by_numeric_field(&courses[i], &courses[i + 1], field) == Ordering::Greater {
                return false;
            }
        }
    }
    true
}

/// Sort courses by field with toggle (ascending ↔ descending). Empty rows go to the end.
pub fn sort_courses_by_field(courses: &mut Vec<Course>, field: &str) {
    let descending = is_array_sorted(courses, field);

    if field == "name" {
        courses.sort_by(|a, b| {
            let cmp = char_compare(&a.name, &b.name);
            if descending { cmp.reverse() } else { cmp }
        });
    } else {
        courses.sort_by(|a, b| {
            let cmp = compare_by_numeric_field(a, b, field);
            if descending { cmp.reverse() } else { cmp }
        });
    }

    // Move empty rows to the end (stable sort preserves relative order)
    courses.sort_by(|a, b| {
        let a_empty = a.number.is_empty() && a.name.is_empty();
        let b_empty = b.number.is_empty() && b.name.is_empty();
        match (a_empty, b_empty) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => Ordering::Equal,
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hebrew_sort_order() {
        assert_eq!(char_compare("אלגברה", "בדיקה"), Ordering::Less);
        assert_eq!(char_compare("תכנות", "אלגברה"), Ordering::Greater);
    }

    #[test]
    fn test_latin_sort() {
        assert_eq!(char_compare("abc", "abd"), Ordering::Less);
        assert_eq!(char_compare("xyz", "abc"), Ordering::Greater);
    }

    #[test]
    fn test_mixed_sort() {
        // Latin comes before Hebrew in our alphabet
        assert_eq!(char_compare("abc", "אבג"), Ordering::Less);
    }

    #[test]
    fn test_sort_toggle() {
        let mut courses = vec![
            Course { name: "בדיקה".into(), number: "1".into(), ..Default::default() },
            Course { name: "אלגברה".into(), number: "2".into(), ..Default::default() },
        ];
        // First sort: ascending
        sort_courses_by_field(&mut courses, "name");
        assert_eq!(courses[0].name, "אלגברה");
        assert_eq!(courses[1].name, "בדיקה");

        // Second sort (already sorted): descending
        sort_courses_by_field(&mut courses, "name");
        assert_eq!(courses[0].name, "בדיקה");
        assert_eq!(courses[1].name, "אלגברה");
    }

    #[test]
    fn test_empty_rows_at_end() {
        let mut courses = vec![
            Course::default(), // empty
            Course { name: "אלגברה".into(), number: "123".into(), ..Default::default() },
        ];
        sort_courses_by_field(&mut courses, "name");
        assert_eq!(courses[0].name, "אלגברה");
        assert!(courses[1].is_empty());
    }
}
