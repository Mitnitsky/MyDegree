use leptos::prelude::*;
use degree_core::course::{CourseDB, CourseType};
use degree_core::degree::UserState;
use degree_core::semester::Semester;

/// Loaded once at startup from the embedded courses.json.
static COURSES_JSON: &str = include_str!("../../src/data/courses.json");

#[derive(Clone, Copy)]
pub struct AppState {
    pub user: RwSignal<UserState>,
    #[allow(dead_code)]
    pub logged: RwSignal<bool>,
    #[allow(dead_code)]
    pub user_name: RwSignal<String>,
    pub course_db: StoredValue<CourseDB>,
    pub show_search_modal: RwSignal<bool>,
    pub toast_message: RwSignal<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        let course_db: CourseDB =
            serde_json::from_str(COURSES_JSON).expect("Failed to parse courses.json");

        // Try to load saved session from localStorage
        let user = Self::load_from_storage().unwrap_or_default();

        let state = Self {
            user: RwSignal::new(user),
            logged: RwSignal::new(false),
            user_name: RwSignal::new(String::new()),
            course_db: StoredValue::new(course_db),
            show_search_modal: RwSignal::new(false),
            toast_message: RwSignal::new(None),
        };

        // Auto-save to localStorage on every change
        let user_signal = state.user;
        Effect::new(move |_| {
            let user = user_signal.get();
            if let Ok(json) = serde_json::to_string(&user) {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item("saved_session_data", &json);
                        let _ = storage.set_item("authenticated", "false");
                    }
                }
            }
        });

        state
    }

    fn load_from_storage() -> Option<UserState> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        let json = storage.get_item("saved_session_data").ok()??;
        serde_json::from_str(&json).ok()
    }

    pub fn recalculate(&self) {
        self.user.update(|u| {
            self.course_db.with_value(|db| {
                u.recalculate(db);
            });
        });
    }

    pub fn add_semester(&self) {
        self.user.update(|u| {
            u.add_semester(1);
        });
        self.recalculate();
    }

    pub fn remove_semester(&self) {
        self.user.update(|u| {
            u.remove_semester();
        });
        self.recalculate();
    }

    pub fn set_active_semester(&self, index: usize) {
        self.user.update(|u| {
            u.active_semester = index;
        });
    }

    pub fn active_semester_index(&self) -> usize {
        self.user.with(|u| u.active_semester)
    }

    pub fn semesters(&self) -> Vec<Semester> {
        self.user.with(|u| u.semesters.clone())
    }

    pub fn course_types(&self) -> Vec<CourseType> {
        self.user.with(|u| u.course_types.clone())
    }

    pub fn add_empty_course(&self) {
        self.user.update(|u| {
            if let Some(sem) = u.active_semester_mut() {
                sem.add_empty_course();
            }
        });
    }

    pub fn remove_course(&self, index: usize) {
        self.user.update(|u| {
            if let Some(sem) = u.active_semester_mut() {
                sem.remove_course(index);
            }
        });
        self.recalculate();
    }

    pub fn move_course(&self, index: usize, direction: &str) {
        self.user.update(|u| {
            u.move_course(index, direction);
        });
        self.recalculate();
    }

    pub fn sort_by_field(&self, field: &str) {
        let field = field.to_string();
        self.user.update(|u| {
            let idx = u.active_semester;
            if let Some(sem) = u.semesters.get_mut(idx) {
                degree_core::sorting::sort_courses_by_field(&mut sem.courses, &field);
            }
        });
        self.recalculate();
    }

    pub fn toggle_semester_type(&self) {
        self.user.update(|u| {
            let idx = u.active_semester;
            u.toggle_semester_type(idx);
        });
    }

    pub fn add_course_from_db(&self, entry_index: usize) {
        self.course_db.with_value(|db| {
            if let Some(entry) = db.courses.get(entry_index) {
                let entry = entry.clone();
                self.user.update(|u| {
                    if let Some(sem) = u.active_semester_mut() {
                        sem.add_existing_course(&entry, None);
                    }
                });
            }
        });
        self.recalculate();
    }

    pub fn update_course_field(&self, index: usize, field: &str, value: &str) {
        self.user.update(|u| {
            if let Some(sem) = u.active_semester_mut() {
                if let Some(course) = sem.courses.get_mut(index) {
                    match field {
                        "name" => course.name = value.to_string(),
                        "number" => course.number = value.to_string(),
                        "points" => course.points = value.parse().unwrap_or(0.0),
                        "grade" => course.grade = value.parse().unwrap_or(0.0),
                        "type" => course.course_type = value.parse().unwrap_or(0),
                        "binary" => course.binary = value == "true",
                        _ => {}
                    }
                }
            }
        });
        self.recalculate();
    }

    pub fn set_english_exemption(&self, val: bool) {
        self.user.update(|u| {
            u.english_exemption = val;
        });
        self.recalculate();
    }

    pub fn set_degree_points(&self, val: f64) {
        self.user.update(|u| {
            u.degree_points = val;
        });
        self.recalculate();
    }

    pub fn set_course_type_required(&self, index: usize, val: f64) {
        self.user.update(|u| {
            if let Some(ct) = u.course_types.get_mut(index) {
                ct.points_required = val;
            }
        });
        self.recalculate();
    }

    pub fn add_course_type(&self, name: &str) {
        self.user.update(|u| {
            u.add_course_type(name);
        });
        self.recalculate();
    }

    pub fn delete_course_type(&self, index: usize) {
        self.user.update(|u| {
            u.delete_course_type(index);
        });
        self.recalculate();
    }

    pub fn clear_user_data(&self) {
        self.user.update(|u| {
            u.clear();
        });
    }

    pub fn show_toast(&self, msg: &str) {
        let message = msg.to_string();
        self.toast_message.set(Some(message));
        let signal = self.toast_message;
        set_timeout(
            move || {
                signal.set(None);
            },
            std::time::Duration::from_secs(3),
        );
    }

    pub fn export_json(&self, with_grades: bool) -> String {
        self.user.with(|u| {
            degree_core::utils::export_semesters_json(&u.semesters, with_grades)
        })
    }

    pub fn import_json(&self, data: &str) {
        if let Ok(semesters) = serde_json::from_str::<Vec<Semester>>(data) {
            self.user.update(|u| {
                u.semesters = semesters;
            });
            self.recalculate();
        }
    }

    pub fn import_cheesefork(&self, data: &str) {
        let mut courses = Vec::new();
        for line in data.lines() {
            let parts: Vec<&str> = line.trim().splitn(2, '-').collect();
            if parts.len() >= 2 {
                let course_number = parts[0].trim();
                if course_number.chars().all(|c| c.is_ascii_digit()) && !course_number.is_empty() {
                    // Find this course in the DB
                    self.course_db.with_value(|db| {
                        if let Some(entry) = db.courses.iter().find(|c| c.number == course_number) {
                            courses.push(entry.clone());
                        }
                    });
                }
            }
        }
        if !courses.is_empty() {
            self.user.update(|u| {
                u.add_semester(0);
                let sem_idx = u.semesters.len() - 1;
                if let Some(sem) = u.semesters.get_mut(sem_idx) {
                    for entry in &courses {
                        sem.add_existing_course(entry, None);
                    }
                }
                u.active_semester = sem_idx;
            });
            self.recalculate();
        }
    }
}
