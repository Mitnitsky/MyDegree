use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use degree_core::course::{CourseDB, CourseType};
use degree_core::degree::{UserState, ProfilesData, Profile};
use degree_core::semester::Semester;
use crate::firebase;

/// Hash of courses.json, computed at build time by build.rs.
const COURSES_HASH: &str = env!("COURSES_HASH");

#[derive(Clone, Copy)]
pub struct AppState {
    pub user: RwSignal<UserState>,
    pub profiles: RwSignal<ProfilesData>,
    pub active_profile: RwSignal<usize>,
    pub logged: RwSignal<bool>,
    pub user_name: RwSignal<String>,
    pub uid: RwSignal<Option<String>>,
    pub course_db: StoredValue<CourseDB>,
    pub show_search_modal: RwSignal<bool>,
    pub show_histogram_modal: RwSignal<Option<String>>,
    pub show_course_map: RwSignal<bool>,
    pub toast_message: RwSignal<Option<String>>,
    pub data_warnings: RwSignal<Vec<String>>,
    /// Guard: true while loading from Firestore — prevents auto-save from overwriting cloud data
    loading_from_cloud: RwSignal<bool>,
}

impl AppState {
    /// Load the course DB (from localStorage cache or network), then build the full state.
    pub async fn load() -> Self {
        let course_db = Self::load_course_db().await;
        let profiles_data = Self::load_profiles_from_storage().unwrap_or_default();
        let active = profiles_data.active.min(profiles_data.profiles.len().saturating_sub(1));
        let user = profiles_data.profiles.get(active)
            .map(|p| p.data.clone())
            .unwrap_or_default();

        let state = Self {
            user: RwSignal::new(user),
            profiles: RwSignal::new(profiles_data),
            active_profile: RwSignal::new(active),
            logged: RwSignal::new(false),
            user_name: RwSignal::new(String::new()),
            uid: RwSignal::new(None),
            course_db: StoredValue::new(course_db),
            show_search_modal: RwSignal::new(false),
            show_histogram_modal: RwSignal::new(None),
            show_course_map: RwSignal::new(false),
            toast_message: RwSignal::new(None),
            data_warnings: RwSignal::new(Vec::new()),
            loading_from_cloud: RwSignal::new(false),
        };

        // Auto-save: localStorage is immediate, Firestore is debounced (3s) with hash dedup
        let user_signal = state.user;
        let profiles_signal = state.profiles;
        let active_signal = state.active_profile;
        let uid_signal = state.uid;
        let logged_signal = state.logged;
        let loading_guard = state.loading_from_cloud;
        let pending_timeout: std::rc::Rc<std::cell::Cell<Option<gloo_timers::callback::Timeout>>> =
            std::rc::Rc::new(std::cell::Cell::new(None));
        let last_written_hash: std::rc::Rc<std::cell::RefCell<String>> =
            std::rc::Rc::new(std::cell::RefCell::new(String::new()));

        Effect::new(move |_| {
            let user = user_signal.get();

            // Sync user back into profiles
            profiles_signal.update(|p| {
                let idx = active_signal.get_untracked();
                if let Some(profile) = p.profiles.get_mut(idx) {
                    profile.data = user.clone();
                }
                p.active = idx;
            });

            let profiles = profiles_signal.get_untracked();

            // Always save full profiles to localStorage immediately
            if let Ok(json) = serde_json::to_string(&profiles) {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(storage)) = window.local_storage() {
                        let _ = storage.set_item("saved_session_data", &json);
                    }
                }
            }

            // Skip Firestore while loading from cloud
            if loading_guard.get_untracked() {
                return;
            }

            // Debounced Firestore write — drop previous pending timeout (cancels it)
            pending_timeout.set(None);

            if logged_signal.get_untracked() {
                if let Some(uid) = uid_signal.get_untracked() {
                    let uid_clone = uid.clone();
                    let hash_ref = last_written_hash.clone();
                    let timeout = gloo_timers::callback::Timeout::new(3_000, move || {
                        let profiles_now = profiles_signal.get_untracked();
                        if let Ok(json) = serde_json::to_string(&profiles_now) {
                            if *hash_ref.borrow() == json {
                                return;
                            }
                            let _ = firebase::firestore_set(&uid_clone, &json);
                            *hash_ref.borrow_mut() = json;
                        }
                    });
                    pending_timeout.set(Some(timeout));
                }
            }
        });

        // Initialize Firebase auth listener
        state.init_auth();

        state
    }

    /// Try localStorage cache first, fall back to network fetch.
    async fn load_course_db() -> CourseDB {
        // Check if we have a cached copy with matching hash
        if let Some(cached) = Self::load_cached_courses() {
            return cached;
        }

        // Fetch from network
        let json = Self::fetch_courses_json().await
            .expect("Failed to fetch courses.json");
        let db = CourseDB::from_json(&json)
            .expect("courses.json is unparseable");

        // Cache in localStorage
        Self::cache_courses(&json);

        db
    }

    fn load_cached_courses() -> Option<CourseDB> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        let stored_hash = storage.get_item("courses_hash").ok()??;
        if stored_hash != COURSES_HASH {
            return None;
        }
        let json = storage.get_item("courses_json").ok()??;
        CourseDB::from_json(&json)
    }

    fn cache_courses(json: &str) {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("courses_hash", COURSES_HASH);
                let _ = storage.set_item("courses_json", json);
            }
        }
    }

    async fn fetch_courses_json() -> Option<String> {
        let resp = gloo_net::http::Request::get("/courses.json")
            .send().await.ok()?;
        resp.text().await.ok()
    }

    fn load_profiles_from_storage() -> Option<ProfilesData> {
        let window = web_sys::window()?;
        let storage = window.local_storage().ok()??;
        let json = storage.get_item("saved_session_data").ok()??;
        match ProfilesData::from_json(&json) {
            Some(profiles) => Some(profiles),
            None => {
                web_sys::console::warn_1(
                    &"Failed to deserialize saved session, starting fresh".into(),
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
                        "grade" => course.grade = value.parse::<f64>().unwrap_or(0.0).clamp(0.0, 100.0),
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
        let profiles_signal = self.profiles;
        let active_signal = self.active_profile;
        let course_db = self.course_db;
        let toast = self.toast_message;
        let warn_signal = self.data_warnings;
        let loading_guard = self.loading_from_cloud;

        let cb = Closure::new(move |json: Option<String>| {
            match json {
                Some(json_str) => {
                    // Parse auth user info
                    if let Ok(auth_user) = serde_json::from_str::<firebase::AuthUser>(&json_str) {
                        let uid = auth_user.uid.clone();
                        logged.set(true);
                        user_name.set(auth_user.display_name);

                        // Set loading guard BEFORE setting uid to prevent auto-save race
                        loading_guard.set(true);
                        uid_signal.set(Some(uid.clone()));

                        // Store auth flag in localStorage
                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("authenticated", "true");
                            }
                        }

                        // Capture uid and local state before async to avoid stale reads
                        let uid_for_async = uid.clone();
                        let local_has_data = !user_signal.get_untracked().semesters.is_empty();

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
                                        let (sanitized, warnings) = ProfilesData::sanitize_json(&json);
                                        if !warnings.is_empty() {
                                            web_sys::console::warn_1(
                                                &format!("Sanitized user data: {:?}", warnings).into()
                                            );
                                        }
                                        match ProfilesData::from_json(&sanitized) {
                                            Some(mut cloud_profiles) => {
                                                let active = cloud_profiles.active.min(cloud_profiles.profiles.len().saturating_sub(1));
                                                cloud_profiles.active = active;
                                                // Recalculate all profiles
                                                for profile in &mut cloud_profiles.profiles {
                                                    course_db.with_value(|db| profile.data.recalculate(db));
                                                }
                                                let active_user = cloud_profiles.profiles[active].data.clone();
                                                profiles_signal.set(cloud_profiles);
                                                active_signal.set(active);
                                                user_signal.set(active_user);
                                                if !warnings.is_empty() {
                                                    if let Ok(clean_json) = serde_json::to_string(&profiles_signal.get_untracked()) {
                                                        let _ = firebase::firestore_set(&uid_for_async, &clean_json);
                                                    }
                                                    gloo_timers::callback::Timeout::new(500, move || {
                                                        warn_signal.set(warnings);
                                                    }).forget();
                                                }
                                            }
                                            None => {
                                                web_sys::console::warn_1(
                                                    &"Failed to parse Firestore data".into()
                                                );
                                                if local_has_data {
                                                    if let Ok(json) = serde_json::to_string(&profiles_signal.get_untracked()) {
                                                        let _ = firebase::firestore_set(&uid_for_async, &json);
                                                    }
                                                    toast.set(Some("⚠️ נתוני ענן פגומים — נשמרים מחדש מהמכשיר".into()));
                                                } else {
                                                    toast.set(Some("⚠️ נתוני ענן פגומים — לא ניתן לשחזר".into()));
                                                }
                                            }
                                        }
                                    } else {
                                        // No document yet — only upload if local has real data
                                        if local_has_data {
                                            if let Ok(json) = serde_json::to_string(&profiles_signal.get_untracked()) {
                                                let _ = firebase::firestore_set(&uid_for_async, &json);
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    web_sys::console::error_1(
                                        &format!("Firestore read failed: {:?}", e).into()
                                    );
                                    if local_has_data {
                                        if let Ok(json) = serde_json::to_string(&profiles_signal.get_untracked()) {
                                            let _ = firebase::firestore_set(&uid_for_async, &json);
                                        }
                                        toast.set(Some("⚠️ טעינה מהענן נכשלה — הנתונים המקומיים נשמרו".into()));
                                    } else {
                                        toast.set(Some("⚠️ טעינה מהענן נכשלה — נסה שוב מאוחר יותר".into()));
                                    }
                                }
                            }
                            // Release the guard after effects have processed
                            // (Leptos batches effects, so a microtask delay is needed)
                            gloo_timers::callback::Timeout::new(100, move || {
                                loading_guard.set(false);
                            }).forget();
                        });
                    }
                }
                None => {
                    // Signed out
                    logged.set(false);
                    user_name.set(String::new());
                    uid_signal.set(None);
                    loading_guard.set(false);
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
        self.profiles.set(ProfilesData::default());
        self.active_profile.set(0);
        self.user.set(UserState::default());
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("authenticated", "false");
                let _ = storage.remove_item("saved_session_data");
            }
        }
        self.show_toast("התנתקת בהצלחה");
    }

    // --- Profile management ---

    pub fn switch_profile(&self, index: usize) {
        // Save current user back to profiles
        self.profiles.update(|p| {
            let cur = self.active_profile.get_untracked();
            if let Some(profile) = p.profiles.get_mut(cur) {
                profile.data = self.user.get_untracked();
            }
        });
        // Switch
        let profiles = self.profiles.get_untracked();
        if let Some(profile) = profiles.profiles.get(index) {
            self.active_profile.set(index);
            self.user.set(profile.data.clone());
            self.recalculate();
        }
    }

    pub fn add_profile(&self, name: String) {
        // Save current user first
        self.profiles.update(|p| {
            let cur = self.active_profile.get_untracked();
            if let Some(profile) = p.profiles.get_mut(cur) {
                profile.data = self.user.get_untracked();
            }
            p.profiles.push(Profile {
                name,
                data: UserState::default(),
            });
        });
        let new_idx = self.profiles.get_untracked().profiles.len() - 1;
        self.active_profile.set(new_idx);
        self.user.set(UserState::default());
    }

    pub fn rename_profile(&self, index: usize, name: String) {
        self.profiles.update(|p| {
            if let Some(profile) = p.profiles.get_mut(index) {
                profile.name = name;
            }
        });
    }

    pub fn delete_profile(&self, index: usize) {
        let count = self.profiles.get_untracked().profiles.len();
        if count <= 1 { return; } // Can't delete last profile

        self.profiles.update(|p| {
            p.profiles.remove(index);
        });

        let active = self.active_profile.get_untracked();
        let new_active = if active >= index && active > 0 { active - 1 } else { active.min(count - 2) };
        self.active_profile.set(new_active);

        let profiles = self.profiles.get_untracked();
        if let Some(profile) = profiles.profiles.get(new_active) {
            self.user.set(profile.data.clone());
            self.recalculate();
        }
    }

    pub fn profile_names(&self) -> Vec<String> {
        self.profiles.with(|p| p.profiles.iter().map(|pr| pr.name.clone()).collect())
    }
}
