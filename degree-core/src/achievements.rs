use crate::degree::UserState;

pub struct Achievement {
    pub id: &'static str,
    pub emoji: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub hidden: bool, // When locked: hidden shows "🔒 הישג נסתר", visible shows name + "???"
    pub check: fn(&UserState) -> bool,
}

pub fn all_achievements() -> Vec<Achievement> {
    vec![
        // ── Milestones (visible when locked) ──
        Achievement {
            id: "first_step",
            emoji: "🎓",
            name: "צעד ראשון",
            description: "הוספת קורס ראשון",
            hidden: false,
            check: |s| s.semesters.iter().any(|sem| sem.courses.iter().any(|c| !c.name.is_empty())),
        },
        Achievement {
            id: "official_student",
            emoji: "📚",
            name: "סטודנט רשמי",
            description: "השלמת 10 נקודות",
            hidden: false,
            check: |s| s.degree_points_done >= 10.0,
        },
        Achievement {
            id: "halfway",
            emoji: "🏫",
            name: "באמצע הדרך",
            description: "השלמת 50% מהתואר",
            hidden: false,
            check: |s| s.degree_points > 0.0 && s.degree_points_done >= s.degree_points / 2.0,
        },
        Achievement {
            id: "almost_there",
            emoji: "🎊",
            name: "כמעט שם",
            description: "נותרו פחות מ-20 נקודות",
            hidden: false,
            check: |s| {
                s.degree_points > 0.0 && s.degree_points_left > 0.0 && s.degree_points_left <= 20.0
            },
        },
        Achievement {
            id: "finished",
            emoji: "🎉",
            name: "סיימתי!",
            description: "השלמת את כל נקודות התואר",
            hidden: false,
            check: |s| s.degree_points > 0.0 && s.degree_points_left <= 0.0,
        },
        // ── Grades (mostly hidden) ──
        Achievement {
            id: "perfect_100",
            emoji: "💯",
            name: "מאה!",
            description: "קיבלת 100 בקורס",
            hidden: true,
            check: |s| {
                s.semesters
                    .iter()
                    .flat_map(|sem| &sem.courses)
                    .any(|c| (c.grade - 100.0).abs() < f64::EPSILON)
            },
        },
        Achievement {
            id: "honors",
            emoji: "⭐",
            name: "מצטיין",
            description: "ממוצע כללי מעל 85",
            hidden: true,
            check: |s| s.degree_graded_points > 0.0 && s.degree_average >= 85.0,
        },
        Achievement {
            id: "deans_list",
            emoji: "🏅",
            name: "רשימת דיקן",
            description: "ממוצע סמסטריאלי מעל 90",
            hidden: true,
            check: |s| {
                s.semesters.iter().any(|sem| {
                    let has_graded = sem.courses.iter().any(|c| c.grade > 0.0 && !c.binary);
                    has_graded && sem.average >= 90.0
                })
            },
        },
        Achievement {
            id: "rising_star",
            emoji: "🌟",
            name: "כוכב עולה",
            description: "ממוצע כללי מעל 90",
            hidden: true,
            check: |s| s.degree_graded_points > 0.0 && s.degree_average >= 90.0,
        },
        Achievement {
            id: "grade_king",
            emoji: "👑",
            name: "שליט הציונים",
            description: "ממוצע כללי מעל 95",
            hidden: true,
            check: |s| s.degree_graded_points > 0.0 && s.degree_average >= 95.0,
        },
        Achievement {
            id: "no_fall",
            emoji: "💪",
            name: "לא נופל",
            description: "אין ציון מתחת ל-70 (מינימום 5 קורסים)",
            hidden: true,
            check: |s| {
                let graded: Vec<_> = s
                    .semesters
                    .iter()
                    .flat_map(|sem| &sem.courses)
                    .filter(|c| c.grade > 0.0 && !c.binary)
                    .collect();
                graded.len() >= 5 && graded.iter().all(|c| c.grade >= 70.0)
            },
        },
        Achievement {
            id: "sharpshooter",
            emoji: "🎯",
            name: "דייקן",
            description: "3 קורסים עם ציון מעל 90",
            hidden: true,
            check: |s| {
                let count = s
                    .semesters
                    .iter()
                    .flat_map(|sem| &sem.courses)
                    .filter(|c| c.grade >= 90.0)
                    .count();
                count >= 3
            },
        },
        // ── Semester related (hidden) ──
        Achievement {
            id: "studious",
            emoji: "📖",
            name: "שקדן",
            description: "סמסטר עם 25+ נקודות",
            hidden: true,
            check: |s| s.semesters.iter().any(|sem| sem.points >= 25.0),
        },
        Achievement {
            id: "summer_animal",
            emoji: "☀️",
            name: "חיית קיץ",
            description: "השלמת סמסטר קיץ",
            hidden: true,
            check: |s| {
                s.semesters
                    .iter()
                    .any(|sem| sem.is_summer() && sem.courses.iter().any(|c| !c.name.is_empty()))
            },
        },
        Achievement {
            id: "veteran",
            emoji: "🗓️",
            name: "ותיק",
            description: "6+ סמסטרים",
            hidden: true,
            check: |s| {
                let non_empty = s
                    .semesters
                    .iter()
                    .filter(|sem| sem.courses.iter().any(|c| !c.name.is_empty()))
                    .count();
                non_empty >= 6
            },
        },
        Achievement {
            id: "overloaded",
            emoji: "🏋️",
            name: "עמוס",
            description: "סמסטר עם 30+ נקודות",
            hidden: true,
            check: |s| s.semesters.iter().any(|sem| sem.points >= 30.0),
        },
        // ── Category related (hidden) ──
        Achievement {
            id: "category_done",
            emoji: "✅",
            name: "סגרתי קטגוריה",
            description: "השלמת את כל הנקודות בקטגוריה",
            hidden: true,
            check: |s| {
                s.course_types
                    .iter()
                    .any(|ct| ct.points_required > 0.0 && ct.points_done >= ct.points_required)
            },
        },
        Achievement {
            id: "mathematician",
            emoji: "📐",
            name: "מתמטיקאי",
            description: "20+ נקודות בקטגוריה אחת",
            hidden: true,
            check: |s| s.course_types.iter().any(|ct| ct.points_done >= 20.0),
        },
        // ── Special (hidden) ──
        Achievement {
            id: "binary_pass",
            emoji: "🔄",
            name: "עובר בינארי",
            description: "השלמת קורס עובר/נכשל",
            hidden: true,
            check: |s| {
                s.semesters
                    .iter()
                    .flat_map(|sem| &sem.courses)
                    .any(|c| c.binary && c.grade >= 55.0)
            },
        },
        Achievement {
            id: "night_owl",
            emoji: "🌙",
            name: "ינשוף לילה",
            description: "השתמשת באפליקציה בשעות הלילה",
            hidden: true,
            check: |_| false, // checked at runtime in frontend
        },
        Achievement {
            id: "english_exempt",
            emoji: "🔓",
            name: "פטור מאנגלית",
            description: "קיבלת פטור מאנגלית",
            hidden: true,
            check: |s| s.english_exemption,
        },
        Achievement {
            id: "hard_worker",
            emoji: "📊",
            name: "עובד קשה",
            description: "40+ קורסים",
            hidden: true,
            check: |s| {
                let count = s
                    .semesters
                    .iter()
                    .flat_map(|sem| &sem.courses)
                    .filter(|c| !c.name.is_empty())
                    .count();
                count >= 40
            },
        },
        Achievement {
            id: "dual_degree",
            emoji: "🎓",
            name: "תואר כפול",
            description: "מנהל שני תוארים",
            hidden: true,
            check: |_| false, // checked at runtime in frontend
        },
        Achievement {
            id: "calculator",
            emoji: "🧮",
            name: "חישובי",
            description: "10+ נקודות בציון מעל 90",
            hidden: true,
            check: |s| {
                let sum: f64 = s
                    .semesters
                    .iter()
                    .flat_map(|sem| &sem.courses)
                    .filter(|c| c.grade >= 90.0)
                    .map(|c| c.points)
                    .sum();
                sum >= 10.0
            },
        },
        Achievement {
            id: "collector",
            emoji: "🏆",
            name: "אספן הישגים",
            description: "פתחת 20 הישגים",
            hidden: true,
            check: |_| false, // checked at runtime in frontend
        },
    ]
}

/// Evaluate which achievements are unlocked for a given UserState.
/// Returns a Vec of bools parallel to all_achievements().
pub fn evaluate_achievements(state: &UserState) -> Vec<bool> {
    all_achievements().iter().map(|a| (a.check)(state)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_achievements_count() {
        assert_eq!(all_achievements().len(), 25);
    }

    #[test]
    fn test_unique_ids() {
        let achievements = all_achievements();
        let mut ids: Vec<&str> = achievements.iter().map(|a| a.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 25);
    }

    #[test]
    fn test_default_state_unlocks_nothing() {
        let state = UserState::default();
        let results = evaluate_achievements(&state);
        // default state should have no achievements
        assert!(results.iter().all(|&v| !v));
    }

    #[test]
    fn test_first_step() {
        let mut state = UserState::default();
        state.semesters.push(crate::semester::Semester::new("1", 0));
        state.semesters[0].courses.push(crate::course::Course {
            name: "מתמטיקה".into(),
            ..Default::default()
        });
        let results = evaluate_achievements(&state);
        assert!(results[0]); // first_step
    }

    #[test]
    fn test_official_student() {
        let mut state = UserState::default();
        state.degree_points_done = 10.0;
        let results = evaluate_achievements(&state);
        assert!(results[1]); // official_student
    }

    #[test]
    fn test_halfway() {
        let mut state = UserState::default();
        state.degree_points = 120.0;
        state.degree_points_done = 60.0;
        let results = evaluate_achievements(&state);
        assert!(results[2]); // halfway
    }

    #[test]
    fn test_perfect_100() {
        let mut state = UserState::default();
        state.semesters.push(crate::semester::Semester::new("1", 0));
        state.semesters[0].courses.push(crate::course::Course {
            name: "מתמטיקה".into(),
            grade: 100.0,
            ..Default::default()
        });
        let results = evaluate_achievements(&state);
        assert!(results[5]); // perfect_100
    }

    #[test]
    fn test_no_fall_needs_5_graded() {
        let mut state = UserState::default();
        state.semesters.push(crate::semester::Semester::new("1", 0));
        // Only 3 graded courses — not enough
        for _ in 0..3 {
            state.semesters[0].courses.push(crate::course::Course {
                name: "X".into(),
                grade: 80.0,
                points: 3.0,
                ..Default::default()
            });
        }
        let results = evaluate_achievements(&state);
        assert!(!results[10]); // no_fall requires >= 5

        // Add 2 more
        for _ in 0..2 {
            state.semesters[0].courses.push(crate::course::Course {
                name: "Y".into(),
                grade: 75.0,
                points: 3.0,
                ..Default::default()
            });
        }
        let results = evaluate_achievements(&state);
        assert!(results[10]); // now it's unlocked
    }

    #[test]
    fn test_english_exemption() {
        let mut state = UserState::default();
        state.english_exemption = true;
        let results = evaluate_achievements(&state);
        assert!(results[20]); // english_exempt
    }

    #[test]
    fn test_binary_pass() {
        let mut state = UserState::default();
        state.semesters.push(crate::semester::Semester::new("1", 0));
        state.semesters[0].courses.push(crate::course::Course {
            name: "ספורט".into(),
            binary: true,
            grade: 60.0,
            ..Default::default()
        });
        let results = evaluate_achievements(&state);
        assert!(results[18]); // binary_pass
    }

    #[test]
    fn test_category_done() {
        let mut state = UserState::default();
        state.course_types[0].points_required = 30.0;
        state.course_types[0].points_done = 30.0;
        let results = evaluate_achievements(&state);
        assert!(results[16]); // category_done
    }
}
