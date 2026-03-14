use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use degree_core::course::{CourseDB, CourseType};
use degree_core::degree::UserState;
use degree_core::semester::Semester;
use crate::firebase;

/// Loaded once at startup from the embedded courses.json.
static COURSES_JSON: &str = include_str!("../courses.json");

#[derive(Clone, Copy)]
pub struct AppState {
    pub user: RwSignal<UserState>,
    pub logged: RwSignal<bool>,
    pub user_name: RwSignal<String>,
    pub uid: RwSignal<Option<String>>,
    pub course_db: StoredValue<CourseDB>,
    pub show_search_modal: RwSignal<bool>,
    pub show_histogram_modal: RwSignal<Option<String>>,
    pub toast_message: RwSignal<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        // Parse bundled courses.json with graceful fallback
        let course_db = CourseDB::from_json(COURSES_JSON)
            .expect("courses.json is completely unparseable — cannot start");
        let bundled_hash = course_db.compute_content_hash();

        // Check if cached session data is stale (course DB changed since last visit)
        let session_stale = Self::is_session_stale(&bundled_hash);

        // Try to load saved session from localStorage (skip if DB changed)
        let user = if session_stale {
            Self::save_content_hash(&bundled_hash);
            UserState::default()
        } else {
            Self::load_from_storage().unwrap_or_default()
        };

        let state = Self {
            user: RwSignal::new(user),
            logged: RwSignal::new(false),
            user_name: RwSignal::new(String::new()),
            uid: RwSignal::new(None),
            course_db: StoredValue::new(course_db),
            show_search_modal: RwSignal::new(false),
            show_histogram_modal: RwSignal::new(None),
            toast_message: RwSignal::new(None),
        };

        // Persist content hash on first run
        if !session_stale {
            Self::save_content_hash(&bundled_hash);
        }

        // Auto-save to localStorage on every change
        let user_signal = state.user;
        let uid_signal = state.uid;
        let logged_signal = state.logged;
        Effect::new(move |_| {
            let user = user_signal.get();
            if let Ok(json) = serde_json::to_string(&user) {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item("saved_session_data", &json);
                    }
                }
                // Also save to Firestore if authenticated
                if logged_signal.get_untracked() {
                    if let Some(uid) = uid_signal.get_untracked() {
                        let _ = firebase::firestore_set(&uid, &json);
                    }
                }
            }
        });

        // Initialize Firebase auth listener
        state.init_auth();

        state
    }

    /// Check if bundled course DB hash differs from the one stored in localStorage.
    fn is_session_stale(bundled_hash: &str) -> bool {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return false,
        };
        let storage = match window.local_storage() {
            Ok(Some(s)) => s,
            _ => return false,
        };
        match storage.get_item("course_db_hash") {
            Ok(Some(stored)) => stored != bundled_hash,
            // No hash stored yet — first visit, not stale
            _ => false,
        }
    }

    fn save_content_hash(hash: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("course_db_hash", hash);
            }
        }
    }

    fn load_from_storage() -> Option<UserState> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        let json = storage.get_item("saved_session_data").ok()??;
        match serde_json::from_str(&json) {
            Ok(user) => Some(user),
            Err(e) => {
                // Deserialization failed — data is corrupted or schema changed.
                // Clear it and start fresh.
                web_sys::console::warn_1(
                    &format!("Failed to deserialize saved session, starting fresh: {}", e).into(),
                );
                let _ = storage.remove_item("saved_session_data");
                None
            }
        }
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
        // Restart fade-in animation after Leptos re-renders
        let restart = wasm_bindgen::closure::Closure::once_into_js(move || {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Ok(Some(el)) = doc.query_selector(".semester-fade-in") {
                    if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = html_el.class_list().remove_1("semester-fade-in");
                        let _ = html_el.offset_height();
                        let _ = html_el.class_list().add_1("semester-fade-in");
                    }
                }
            }
        });
        let _ = web_sys::window().unwrap().request_animation_frame(restart.unchecked_ref());
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

    /// Move a course via drag-and-drop, possibly between semesters.
    pub fn move_course_drag(&self, src_sem: usize, src_idx: usize, dst_sem: usize, dst_idx: usize) {
        self.user.update(|u| {
            let course = {
                let sem = match u.semesters.get_mut(src_sem) {
                    Some(s) => s,
                    None => return,
                };
                if src_idx >= sem.courses.len() {
                    return;
                }
                sem.courses.remove(src_idx)
            };
            if let Some(dst) = u.semesters.get_mut(dst_sem) {
                // After removing from source, adjust target index if same semester
                let insert_at = if src_sem == dst_sem && src_idx < dst_idx {
                    (dst_idx - 1).min(dst.courses.len())
                } else {
                    dst_idx.min(dst.courses.len())
                };
                dst.courses.insert(insert_at, course);
            }
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

    /// Initialize Firebase auth state listener.
    /// On sign-in: loads user data from Firestore.
    /// On sign-out: clears auth state.
    fn init_auth(&self) {
        let logged = self.logged;
        let user_name = self.user_name;
        let uid_signal = self.uid;
        let user_signal = self.user;
        let course_db = self.course_db;
        let toast = self.toast_message;

        let cb = Closure::new(move |json: Option<String>| {
            match json {
                Some(json_str) => {
                    // Parse auth user info
                    if let Ok(auth_user) = serde_json::from_str::<firebase::AuthUser>(&json_str) {
                        let uid = auth_user.uid.clone();
                        logged.set(true);
                        user_name.set(auth_user.display_name);
                        uid_signal.set(Some(uid.clone()));

                        // Store auth flag in localStorage
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("authenticated", "true");
                            }
                        }

                        // Load user data from Firestore
                        let promise = firebase::firestore_get(&uid);
                        let future = wasm_bindgen_futures::JsFuture::from(promise);
                        leptos::task::spawn_local(async move {
                            match future.await {
                                Ok(val) => {
                                    let json_opt = val.as_string().or_else(|| {
                                        if !val.is_null() && !val.is_undefined() {
                                            js_sys::JSON::stringify(&val).ok().and_then(|s| s.as_string())
                                        } else {
                                            None
                                        }
                                    });
                                    if let Some(json) = json_opt {
                                        match serde_json::from_str::<UserState>(&json) {
                                            Ok(cloud_user) => {
                                                user_signal.set(cloud_user);
                                                user_signal.update(|u| {
                                                    course_db.with_value(|db| u.recalculate(db));
                                                });
                                            }
                                            Err(e) => {
                                                web_sys::console::warn_1(
                                                    &format!("Failed to parse Firestore data: {}", e).into()
                                                );
                                            }
                                        }
                                    } else {
                                        // No document yet — upload current local state
                                        if let Ok(json) = serde_json::to_string(&user_signal.get_untracked()) {
                                            let _ = firebase::firestore_set(&uid_signal.get_untracked().unwrap_or_default(), &json);
                                        }
                                    }
                                }
                                Err(e) => {
                                    web_sys::console::warn_1(
                                        &format!("Firestore read failed: {:?}", e).into()
                                    );
                                }
                            }
                        });
                    }
                }
                None => {
                    // Signed out
                    logged.set(false);
                    user_name.set(String::new());
                    uid_signal.set(None);
                    if let Some(window) = web_sys::window() {
                        if let Ok(Some(storage)) = window.local_storage() {
                            let _ = storage.set_item("authenticated", "false");
                        }
                    }
                }
            }
        });

        firebase::on_auth_change(&cb);
        // Leak the closure so it lives for the duration of the app
        cb.forget();
    }

    pub fn sign_out(&self) {
        firebase::sign_out_user();
        self.logged.set(false);
        self.user_name.set(String::new());
        self.uid.set(None);
        self.user.set(UserState::default());
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("authenticated", "false");
                let _ = storage.remove_item("saved_session_data");
            }
        }
        self.show_toast("התנתקת בהצלחה");
    }
}
