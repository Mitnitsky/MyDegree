use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use wasm_bindgen::JsCast;
use crate::state::AppState;

// ── Mobile Header ───────────────────────────────────────
#[component]
pub fn MobileHeader() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let show_menu = RwSignal::new(false);
    let show_auth_modal = RwSignal::new(false);

    let show_import_modal = RwSignal::new(false);
    let show_cf_modal = RwSignal::new(false);
    let show_category_modal = RwSignal::new(false);
    let show_clear_modal = RwSignal::new(false);
    let clear_input = RwSignal::new(String::new());
    let show_calc = RwSignal::new(false);
    let import_text = RwSignal::new(String::new());
    let cf_text = RwSignal::new(String::new());
    let new_category_name = RwSignal::new(String::new());

    let show_account_menu = RwSignal::new(false);

    // Close account dropdown on outside click
    {
        use wasm_bindgen::closure::Closure;
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            if !show_account_menu.get_untracked() {
                return;
            }
            if let Some(target) = e.target() {
                if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                    if el.closest(".mobile-account-menu").ok().flatten().is_none()
                        && el.closest(".mobile-header-btn").ok().flatten().is_none()
                    {
                        show_account_menu.set(false);
                    }
                }
            }
        });
        let doc = web_sys::window().unwrap().document().unwrap();
        doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }

    Effect::new(move |_| {
        if state.logged.get() {
            show_auth_modal.set(false);
        }
    });

    let on_import_json = move |_: web_sys::MouseEvent| {
        let text = import_text.get();
        if !text.is_empty() {
            state.import_json(&text);
            gloo_timers::callback::Timeout::new(0, move || show_import_modal.set(false)).forget();
            import_text.set(String::new());
            state.show_toast("קורסים יובאו בהצלחה");
        }
    };

    let on_clear = move |_: web_sys::MouseEvent| {
        clear_input.set(String::new());
        show_clear_modal.set(true);
        show_menu.set(false);
    };

    let on_import_cf = move |_: web_sys::MouseEvent| {
        let text = cf_text.get();
        if !text.is_empty() {
            state.import_cheesefork(&text);
            gloo_timers::callback::Timeout::new(0, move || show_cf_modal.set(false)).forget();
            cf_text.set(String::new());
            state.show_toast("קורסים יובאו מ-Cheesefork בהצלחה");
        }
    };

    let on_add_category = move |_: web_sys::MouseEvent| {
        let name = new_category_name.get();
        if !name.is_empty() {
            state.add_course_type(&name);
            new_category_name.set(String::new());
        }
    };

    el::div().class("mobile-only").child((
        // Header bar
        el::div().class("mobile-header").child((
            // Right: hamburger menu + dark mode toggle
            el::div().class("mobile-header-right").child((
                el::button().class("mobile-header-btn")
                    .on(ev::click, move |_| show_menu.update(|v| *v = !*v))
                    .child(el::i().class("fas fa-bars")),
                el::div().class("theme-toggle theme-toggle-mobile")
                    .child((
                        el::span().class("theme-toggle-icon theme-icon-sun")
                            .attr("title", "בהיר")
                            .on(ev::click, move |_| {
                                let _ = js_sys::eval("window.setThemeMode('light')");
                            })
                            .child(el::i().class("fas fa-sun")),
                        el::span().class("theme-toggle-icon theme-icon-auto")
                            .attr("title", "אוטומטי")
                            .on(ev::click, move |_| {
                                let _ = js_sys::eval("window.setThemeMode('auto')");
                            })
                            .child(el::i().class("fas fa-desktop")),
                        el::span().class("theme-toggle-icon theme-icon-moon")
                            .attr("title", "כהה")
                            .on(ev::click, move |_| {
                                let _ = js_sys::eval("window.setThemeMode('dark')");
                            })
                            .child(el::i().class("fas fa-moon")),
                    )),
            )),
            // Center: title
            el::span().class("mobile-header-title").child((
                "My Degree ",
                el::i().class("fas fa-graduation-cap"),
            )),
            // Left: account menu or login
            move || {
                if state.logged.get() {
                    el::div().attr("style", "position: relative;").child((
                        el::button().class("mobile-header-btn")
                            .attr("style", "color: #4ade80;")
                            .on(ev::click, move |_| show_account_menu.update(|v| *v = !*v))
                            .child(el::i().class("fas fa-user-circle")),
                        move || {
                            show_account_menu.get().then(|| {
                                el::div().class("mobile-account-menu").child((
                                    el::div().class("mobile-menu-header")
                                        .child(move || {
                                            let name = state.user_name.get();
                                            if name.is_empty() { "שלום".to_string() } else { format!("שלום {}", name) }
                                        }),
                                    el::a().attr("href", "#")
                                        .attr("style", "display: block; padding: 12px 16px; color: #dc3545; text-decoration: none;")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            show_account_menu.set(false);
                                            state.sign_out();
                                        })
                                        .child((el::i().class("fas fa-sign-out-alt").attr("style", "margin-left: 8px;"), "התנתק")),
                                ))
                            })
                        },
                    )).into_any()
                } else {
                    el::button().class("mobile-header-btn")
                        .on(ev::click, move |_| show_auth_modal.set(true))
                        .child(el::i().class("fas fa-user-circle"))
                        .into_any()
                }
            },
        )),

        // Slide-down menu with overlay
        move || {
            show_menu.get().then(|| {
                el::div()
                    .class("mobile-menu-overlay")
                    .on(ev::click, move |_| show_menu.set(false))
                    .child(
                        el::div()
                            .class("mobile-slide-menu")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                            .child((
                                menu_item("ייבוא מ-JSON", "fas fa-file-import", move |_: web_sys::MouseEvent| {
                                    show_import_modal.set(true);
                                    show_menu.set(false);
                                }),
                                menu_item("ייבוא מ-Cheesefork", "fas fa-utensils", move |_: web_sys::MouseEvent| {
                                    show_cf_modal.set(true);
                                    show_menu.set(false);
                                }),
                                menu_item("ייצוא עם ציונים", "fas fa-file-export", move |_: web_sys::MouseEvent| {
                                    let json = state.export_json(true);
                                    crate::components::header::trigger_download(&json, "courses_with_grades.json");
                                    show_menu.set(false);
                                }),
                                menu_item("ייצוא בלי ציונים", "fas fa-file-export", move |_: web_sys::MouseEvent| {
                                    let json = state.export_json(false);
                                    crate::components::header::trigger_download(&json, "courses.json");
                                    show_menu.set(false);
                                }),
                                menu_item("קטגוריות", "fas fa-graduation-cap", move |_: web_sys::MouseEvent| {
                                    show_category_modal.set(true);
                                    show_menu.set(false);
                                }),
                                menu_item("תכנון ממוצע", "", move |_: web_sys::MouseEvent| {
                                    show_calc.set(true);
                                    show_menu.set(false);
                                }),
                                menu_item("מפת קורסים", "", move |_: web_sys::MouseEvent| {
                                    state.show_course_map.set(true);
                                    show_menu.set(false);
                                }),
                                menu_item("הישגים", "fas fa-trophy", move |_: web_sys::MouseEvent| {
                                    state.show_achievements.set(true);
                                    show_menu.set(false);
                                }),
                                el::div().attr("style", "border-top: 1px solid var(--border-color, #dee2e6); margin: 4px 16px;"),
                                el::a()
                                    .attr("href", "#")
                                    .attr("style", "display: block; padding: 12px 20px; color: #dc3545; text-decoration: none; font-size: 0.95rem;")
                                    .on(ev::click, on_clear)
                                    .child((el::i().class("fas fa-trash").attr("style", "margin-left: 8px;"), "מחק הכל")),
                            ))
                    )
            })
        },

        // Auth modal (reused)
        move || {
            show_auth_modal.get().then(|| {
                let start_ui = wasm_bindgen::closure::Closure::once_into_js(move || {
                    crate::firebase::start_auth_ui("mobile-firebaseui-auth");
                });
                let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                    start_ui.unchecked_ref(), 100,
                );
                let dismiss_overlay = move || {
                    gloo_timers::callback::Timeout::new(0, move || show_auth_modal.set(false)).forget();
                };
                let dismiss_x = dismiss_overlay.clone();
                el::div().class("search-overlay")
                    .on(ev::click, move |_| dismiss_overlay())
                    .child(
                        el::div().class("search-dialog")
                            .attr("style", "max-width: 420px;")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                            .child((
                                el::div().class("d-flex justify-content-between align-items-center").child((
                                    el::h5().class("mb-0").child("כניסה"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |e: web_sys::MouseEvent| { e.stop_propagation(); dismiss_x(); })
                                        .child(el::i().class("fas fa-times")),
                                )),
                                el::div().child(el::div().id("mobile-firebaseui-auth")),
                            )),
                    )
            })
        },

        // Import JSON modal
        move || {
            show_import_modal.get().then(|| {
                let dismiss = move || {
                    gloo_timers::callback::Timeout::new(0, move || show_import_modal.set(false)).forget();
                };
                let dismiss2 = dismiss.clone();
                let dismiss3 = dismiss.clone();
                el::div().class("search-overlay")
                    .on(ev::click, move |_| dismiss())
                    .child(
                        el::div().class("search-dialog")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                            .child((
                                el::div().class("d-flex justify-content-between align-items-center").child((
                                    el::h5().class("mb-0").child("ייבוא קורסים מ-JSON"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |e: web_sys::MouseEvent| { e.stop_propagation(); dismiss2(); })
                                        .child(el::i().class("fas fa-times")),
                                )),
                                el::div().child((
                                    el::textarea()
                                        .class("form-control mb-3")
                                        .attr("rows", "8")
                                        .attr("placeholder", "הדבק JSON כאן...")
                                        .prop("value", move || import_text.get())
                                        .on(ev::input, move |e| import_text.set(event_target_value(&e))),
                                    el::div().class("d-flex justify-content-end gap-2").child((
                                        el::button().class("btn btn-secondary")
                                            .on(ev::click, move |_| dismiss3())
                                            .child("ביטול"),
                                        el::button().class("btn btn-primary")
                                            .on(ev::click, on_import_json)
                                            .child("ייבוא"),
                                    )),
                                )),
                            )),
                    )
            })
        },

        // Cheesefork modal
        move || {
            show_cf_modal.get().then(|| {
                let dismiss = move || {
                    gloo_timers::callback::Timeout::new(0, move || show_cf_modal.set(false)).forget();
                };
                let dismiss2 = dismiss.clone();
                el::div().class("search-overlay")
                    .on(ev::click, move |_| dismiss())
                    .child(
                        el::div().class("search-dialog")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                            .child((
                                el::div().class("d-flex justify-content-between align-items-center").child((
                                    el::h5().class("mb-0").child("ייבוא מ-Cheesefork"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |e: web_sys::MouseEvent| { e.stop_propagation(); dismiss2(); })
                                        .child(el::i().class("fas fa-times")),
                                )),
                                el::div().child((
                                    el::textarea()
                                        .class("form-control mb-3")
                                        .attr("rows", "5")
                                        .attr("placeholder", "יש להעתיק את התוכן לכאן")
                                        .prop("value", move || cf_text.get())
                                        .on(ev::input, move |e| cf_text.set(event_target_value(&e))),
                                    el::div().class("d-flex justify-content-center").child(
                                        el::button().class("btn btn-outline-primary")
                                            .on(ev::click, on_import_cf)
                                            .child("יבוא קורסים"),
                                    ),
                                )),
                            )),
                    )
            })
        },

        // Category modal
        move || {
            show_category_modal.get().then(|| {
                let dismiss = move || {
                    gloo_timers::callback::Timeout::new(0, move || show_category_modal.set(false)).forget();
                };
                let dismiss2 = dismiss.clone();
                el::div().class("search-overlay")
                    .on(ev::click, move |_| dismiss())
                    .child(
                        el::div().class("search-dialog")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                            .child((
                                el::div().class("d-flex justify-content-between align-items-center").child((
                                    el::h5().class("mb-0").child("ניהול קטגוריות"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |e: web_sys::MouseEvent| { e.stop_propagation(); dismiss2(); })
                                        .child(el::i().class("fas fa-times")),
                                )),
                                el::div().child((
                                    el::ul().class("list-group mb-3").child(
                                        move || {
                                            state.course_types().into_iter().enumerate().map(|(i, ct)| {
                                                let name = ct.name.clone();
                                                let permanent = i < 2;
                                                el::li()
                                                    .class(if permanent {
                                                        "list-group-item d-flex justify-content-between align-items-center text-muted"
                                                    } else {
                                                        "list-group-item d-flex justify-content-between align-items-center"
                                                    })
                                                    .child((
                                                        name,
                                                        if permanent {
                                                            None
                                                        } else {
                                                            Some(el::button().class("btn btn-sm btn-outline-danger")
                                                                .on(ev::click, move |_| state.delete_course_type(i))
                                                                .child(el::i().class("fas fa-times")))
                                                        },
                                                    ))
                                            }).collect::<Vec<_>>()
                                        },
                                    ),
                                    el::div().class("input-group").child((
                                        el::input()
                                            .attr("type", "text")
                                            .class("form-control")
                                            .attr("placeholder", "שם קטגוריה חדשה")
                                            .prop("value", move || new_category_name.get())
                                            .on(ev::input, move |e| new_category_name.set(event_target_value(&e))),
                                        el::button().class("btn btn-outline-primary")
                                            .on(ev::click, on_add_category)
                                            .child("הוסף"),
                                    )),
                                )),
                            )),
                    )
            })
        },

        // Clear all confirmation modal
        move || {
            show_clear_modal.get().then(|| {
                let dismiss = move || {
                    gloo_timers::callback::Timeout::new(0, move || show_clear_modal.set(false)).forget();
                };
                let dismiss2 = dismiss.clone();
                let dismiss3 = dismiss.clone();
                el::div().class("search-overlay")
                    .on(ev::click, move |_| dismiss())
                    .child(
                        el::div().class("search-dialog")
                            .attr("style", "max-width: 380px; min-width: unset;")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                            .child(
                                el::div().attr("style", "padding: 20px;").child((
                                    el::div().class("d-flex justify-content-between align-items-center mb-3").child((
                                        el::h5().class("mb-0").child("מחיקת כל הנתונים"),
                                        el::button().class("btn btn-sm btn-outline-secondary")
                                            .on(ev::click, move |e: web_sys::MouseEvent| { e.stop_propagation(); dismiss2(); })
                                            .child(el::i().class("fas fa-times")),
                                    )),
                                    el::p().attr("style", "color: var(--text-secondary); margin-bottom: 12px;")
                                        .child("הקלד REMOVE כדי לאשר מחיקה"),
                                    el::input()
                                        .class("form-control mb-3")
                                        .attr("placeholder", "REMOVE")
                                        .attr("dir", "ltr")
                                        .prop("value", move || clear_input.get())
                                        .on(ev::input, move |e| clear_input.set(event_target_value(&e))),
                                    el::button()
                                        .class(move || if clear_input.get().trim() == "REMOVE" { "btn btn-danger w-100" } else { "btn btn-secondary w-100" })
                                        .prop("disabled", move || clear_input.get().trim() != "REMOVE")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.stop_propagation();
                                            if clear_input.get().trim() == "REMOVE" {
                                                state.clear_user_data();
                                                dismiss3();
                                            }
                                        })
                                        .child("מחק הכל"),
                                )),
                            ),
                    )
            })
        },
        move || crate::components::degree_summary::grade_calc_modal(state, show_calc),
    ))
}

fn menu_item(
    label: &'static str,
    icon_class: &'static str,
    handler: impl Fn(web_sys::MouseEvent) + 'static,
) -> impl IntoView {
    el::a()
        .attr("href", "#")
        .attr("style", "display: block; padding: 12px 20px; color: var(--text-primary); text-decoration: none; font-size: 0.95rem;")
        .on(ev::click, handler)
        .child(
            if icon_class.is_empty() {
                leptos::either::Either::Left(label)
            } else {
                leptos::either::Either::Right((
                    el::i().class(icon_class).attr("style", "margin-left: 8px; width: 1.2em;"),
                    label,
                ))
            }
        )
}

// ── Mobile Semester Tabs ────────────────────────────────
#[component]
pub fn MobileSemesterTabs() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    let on_new_tab = move |_: web_sys::MouseEvent| {
        state.add_semester();
        let count = state.user.with(|u| u.semesters.len());
        state.set_active_semester(count - 1);
    };

    el::div().class("mobile-only mobile-tabs").child((
        move || {
            let semesters = state.semesters();
            if semesters.is_empty() {
                return vec![];
            }
            let active = state.active_semester_index();
            semesters.into_iter().enumerate().map(|(i, sem)| {
                let label = format!("סמסטר {}", sem.name);
                let is_active = i == active;
                let is_summer = sem.is_summer();
                let cls = format!(
                    "mobile-tab{}{}",
                    if is_active { " active" } else { "" },
                    if is_summer && !is_active { " summer" } else { "" },
                );
                el::button()
                    .class(cls)
                    .on(ev::click, move |_| state.set_active_semester(i))
                    .child(label)
            }).collect::<Vec<_>>()
        },
        move || {
            let has_semesters = state.user.with(|u| !u.semesters.is_empty());
            has_semesters.then(|| {
                el::button().class("mobile-tab-add")
                    .on(ev::click, on_new_tab)
                    .child("+")
            })
        },
    ))
}

// ── Mobile Semester Summary Card ────────────────────────
#[component]
pub fn MobileSemesterSummary() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let show_actions = RwSignal::new(false);

    let average = Memo::new(move |_| {
        state.user.with(|u| u.semesters.get(u.active_semester).map(|s| s.average).unwrap_or(0.0))
    });
    let points = Memo::new(move |_| {
        state.user.with(|u| u.semesters.get(u.active_semester).map(|s| s.points).unwrap_or(0.0))
    });

    let on_remove = move |_: web_sys::MouseEvent| {
        if web_sys::window()
            .and_then(|w| w.confirm_with_message("למחוק סמסטר זה?").ok())
            .unwrap_or(false)
        {
            state.remove_semester();
            let count = state.user.with(|u| u.semesters.len());
            if count > 0 { state.set_active_semester(count - 1); }
        }
        show_actions.set(false);
    };

    let on_toggle_type = move |_: web_sys::MouseEvent| {
        state.toggle_semester_type();
        show_actions.set(false);
    };

    el::div().class("mobile-only").child(
        move || {
            let semesters = state.semesters();
            if semesters.is_empty() {
                el::div()
                    .attr("style", "text-align: center; padding: 32px 16px; color: var(--text-secondary);")
                    .child((
                        el::p().child("עוד לא נוספו סמסטרים"),
                        el::button().class("btn btn-outline-secondary")
                            .on(ev::click, move |_: web_sys::MouseEvent| {
                                state.add_semester();
                                state.set_active_semester(0);
                            })
                            .child("הוסף סמסטר"),
                    )).into_any()
            } else {
                let active = state.active_semester_index();
                let is_summer = semesters.get(active).map(|s| s.is_summer()).unwrap_or(false);
                let toggle_text = if is_summer { "הפוך לסמסטר רגיל" } else { "הפוך לסמסטר קיץ" };

                el::div().class("mobile-sem-summary").child((
                    el::div().class("mobile-sem-summary-title")
                        .child("סיכום סמסטר"),
                    el::div().class("mobile-sem-summary-metrics").child((
                        el::div().class("mobile-sem-metric").child((
                            el::div().class("mobile-sem-metric-value").child(
                                move || format!("{:.1}", average.get())
                            ),
                            el::div().class("mobile-sem-metric-label").child("ממוצע"),
                        )),
                        el::div().class("mobile-sem-metric").child((
                            el::div().class("mobile-sem-metric-value").child(
                                move || format!("{:.1}", points.get())
                            ),
                            el::div().class("mobile-sem-metric-label").child("נקודות"),
                        )),
                    )),
                    el::button().class("mobile-sem-actions-btn")
                        .on(ev::click, move |_| show_actions.update(|v| *v = !*v))
                        .child(el::i().class("fas fa-ellipsis-v")),
                    move || {
                        show_actions.get().then(|| {
                            el::div().class("mobile-sem-actions-menu").child((
                                el::a().attr("href", "#")
                                    .on(ev::click, on_remove)
                                    .attr("style", "color: #dc3545;")
                                    .child((el::i().class("fas fa-trash").attr("style", "margin-left: 8px;"), "מחק סמסטר")),
                                el::a().attr("href", "#")
                                    .on(ev::click, on_toggle_type)
                                    .child((el::i().class("fas fa-sun").attr("style", "color: #ffc107; margin-left: 8px;"), toggle_text)),
                            ))
                        })
                    },
                )).into_any()
            }
        }
    )
}

// ── Mobile Course Card ──────────────────────────────────

/// Close any open mobile card menus by dispatching a custom event
fn close_all_mobile_menus() {
    if let Some(win) = web_sys::window() {
        if let Ok(evt) = web_sys::CustomEvent::new("mobile-menu-close") {
            let _ = win.dispatch_event(&evt);
        }
    }
}

fn mobile_course_card(index: usize, count: usize) -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let show_menu = RwSignal::new(false);

    // Listen for global close event to close this menu
    Effect::new(move |_| {
        if let Some(win) = web_sys::window() {
            let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                show_menu.set(false);
            });
            let _ = win.add_event_listener_with_callback("mobile-menu-close", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    let course = Memo::new(move |_| {
        state.user.with(|u| {
            let sem_idx = u.active_semester;
            u.semesters.get(sem_idx).and_then(|s| s.courses.get(index).cloned())
        })
    });

    let course_types = Memo::new(move |_| state.course_types());

    let type_class = move || {
        course.with(|c| {
            c.as_ref().map(|c| format!("mobile-course-card course-type-{}", c.course_type.min(5)))
                .unwrap_or_else(|| "mobile-course-card".to_string())
        })
    };

    el::div()
        .class(type_class)
        .child((
            // Row 1: Course name (floating label)
            el::div().class("mobile-card-row").child(
                el::div().class("mobile-float-field").child((
                    el::input()
                        .attr("type", "text")
                        .class("form-control mobile-float-input")
                        .attr("placeholder", " ")
                        .prop("value", move || {
                            course.with(|c| c.as_ref().map(|c| c.name.clone()).unwrap_or_default())
                        })
                        .on(ev::change, move |e| {
                            state.update_course_field(index, "name", &event_target_value(&e));
                        }),
                    el::label().class("mobile-float-label").child("שם קורס"),
                )),
            ),
            // Row 2: Category + Course number
            el::div().class("mobile-card-row").child((
                el::div().class("mobile-float-field mobile-card-category").child((
                    el::select()
                        .class("form-select mobile-float-input")
                        .on(ev::change, move |e| {
                            state.update_course_field(index, "type", &event_target_value(&e));
                        })
                        .child(move || {
                            let current_type = course.with(|c| c.as_ref().map(|c| c.course_type).unwrap_or(0));
                            course_types.get_untracked().into_iter().enumerate().map(|(i, ct)| {
                                let opt = el::option().attr("value", i.to_string());
                                if i == current_type {
                                    opt.attr("selected", "").child(ct.name).into_any()
                                } else {
                                    opt.child(ct.name).into_any()
                                }
                            }).collect::<Vec<_>>()
                        }),
                    el::label().class("mobile-float-label mobile-float-label-select").child("קטגוריה"),
                )),
                el::div().class("mobile-float-field mobile-card-number").child((
                    el::input()
                        .attr("type", "text")
                        .attr("inputmode", "numeric")
                        .attr("pattern", "[0-9]*")
                        .class("form-control mobile-float-input text-center")
                        .attr("placeholder", " ")
                        .attr("style", "direction: ltr;")
                        .prop("value", move || {
                            course.with(|c| c.as_ref().map(|c| c.number.clone()).unwrap_or_default())
                        })
                        .on(ev::input, move |e| {
                            let val: String = event_target_value(&e).chars().filter(|c| c.is_ascii_digit()).collect();
                            if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                input.set_value(&val);
                            }
                        })
                        .on(ev::change, move |e| {
                            let val: String = event_target_value(&e).chars().filter(|c| c.is_ascii_digit()).collect();
                            state.update_course_field(index, "number", &val);
                        }),
                    el::label().class("mobile-float-label").child("מספר קורס"),
                )),
            )),
            // Row 3: Actions menu + Grade + Points
            el::div().class("mobile-card-row").child((
                el::div().class("mobile-card-menu-wrap").child((
                    el::button().class("mobile-card-actions-btn")
                        .on(ev::click, move |_| {
                            let was_open = show_menu.get_untracked();
                            close_all_mobile_menus();
                            if !was_open { show_menu.set(true); }
                        })
                        .child(el::i().class("fas fa-ellipsis-v")),
                    move || {
                        show_menu.get().then(|| {
                            el::div().class("mobile-card-menu")
                                .child((
                                    el::a().attr("href", "#")
                                        .attr("style", if index == 0 { "pointer-events: none; opacity: 0.35;" } else { "" })
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            if index > 0 {
                                                state.move_course(index, "up");
                                                show_menu.set(false);
                                            }
                                        })
                                        .child((el::i().class("fas fa-arrow-up").attr("style", "color: var(--text-secondary); margin-left: 8px;"), "הזז למעלה")),
                                    el::a().attr("href", "#")
                                        .attr("style", if index >= count - 1 { "pointer-events: none; opacity: 0.35;" } else { "" })
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            if index < count - 1 {
                                                state.move_course(index, "down");
                                                show_menu.set(false);
                                            }
                                        })
                                        .child((el::i().class("fas fa-arrow-down").attr("style", "color: var(--text-secondary); margin-left: 8px;"), "הזז למטה")),
                                    el::div().attr("style", "border-top: 1px solid #dee2e6; margin: 2px 12px;"),
                                    el::a().attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            let num = course.with(|c| c.as_ref().map(|c| c.number.clone()).unwrap_or_default());
                                            if !num.is_empty() { state.show_histogram_modal.set(Some(num)); }
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-chart-bar").attr("style", "color: dodgerblue; margin-left: 8px;"), "היסטוגרמות")),
                                    el::a().attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            let is_binary = course.with(|c| c.as_ref().map(|c| c.binary).unwrap_or(false));
                                            state.update_course_field(index, "binary", if !is_binary { "true" } else { "false" });
                                            show_menu.set(false);
                                        })
                                        .child(move || {
                                            let is_binary = course.with(|c| c.as_ref().map(|c| c.binary).unwrap_or(false));
                                            if is_binary {
                                                (el::i().class("fas fa-ban").attr("style", "color: red; margin-left: 8px;").into_any(), "בטל עובר בינארי".into_any())
                                            } else {
                                                (el::i().class("fas fa-check").attr("style", "color: green; margin-left: 8px;").into_any(), "סמן עובר בינארי".into_any())
                                            }
                                        }),
                                    el::a().attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            state.update_course_field(index, "name", "");
                                            state.update_course_field(index, "number", "");
                                            state.update_course_field(index, "points", "0");
                                            state.update_course_field(index, "grade", "0");
                                            state.update_course_field(index, "type", "0");
                                            state.update_course_field(index, "binary", "false");
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-broom").attr("style", "color: burlywood; margin-left: 8px;"), "נקה שורה")),
                                    el::a().attr("href", "#")
                                        .attr("style", "color: #dc3545;")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            state.remove_course(index);
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-trash").attr("style", "color: darkred; margin-left: 8px;"), "הסר שורה")),
                                ))
                        })
                    },
                )),
                el::div().class("mobile-float-field mobile-card-grade").child((
                    move || {
                        let is_binary = course.with(|c| c.as_ref().map(|c| c.binary).unwrap_or(false));
                        if is_binary {
                            el::input()
                                .attr("type", "text")
                                .class("form-control mobile-float-input text-center mobile-binary-pass")
                                .attr("readonly", "")
                                .attr("placeholder", " ")
                                .prop("value", "✔")
                                .into_any()
                        } else {
                            el::input()
                                .attr("type", "number")
                                .class("form-control mobile-float-input text-center")
                                .attr("placeholder", " ")
                                .attr("style", "direction: ltr;")
                                .attr("min", "0").attr("max", "100")
                                .prop("value", move || {
                                    course.with(|c| c.as_ref().map(|c| {
                                        if c.grade == 0.0 { String::new() } else { (c.grade as i64).to_string() }
                                    }).unwrap_or_default())
                                })
                                .on(ev::change, move |e| {
                                    state.update_course_field(index, "grade", &event_target_value(&e));
                                })
                                .into_any()
                        }
                    },
                    el::label().class("mobile-float-label").child("ציון"),
                )),
                el::div().class("mobile-float-field mobile-card-points").child((
                    el::input()
                        .attr("type", "number")
                        .class("form-control mobile-float-input text-center")
                        .attr("placeholder", " ")
                        .attr("style", "direction: ltr;")
                        .attr("step", "0.5").attr("min", "0").attr("max", "20")
                        .prop("value", move || {
                            course.with(|c| c.as_ref().map(|c| {
                                if c.points == 0.0 { String::new() } else { c.points.to_string() }
                            }).unwrap_or_default())
                        })
                        .on(ev::change, move |e| {
                            state.update_course_field(index, "points", &event_target_value(&e));
                        }),
                    el::label().class("mobile-float-label").child("נקודות"),
                )),
            )),
        ))
}

#[component]
pub fn MobileCourseList() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    el::div().class("mobile-only").child((
        move || {
            let user = state.user.get();
            let sem_idx = user.active_semester;
            let count = user.semesters.get(sem_idx).map(|s| s.courses.len()).unwrap_or(0);
            (0..count).map(|i| mobile_course_card(i, count)).collect::<Vec<_>>()
        },
        // Action buttons (only when semesters exist)
        move || {
            let has_semesters = state.user.with(|u| !u.semesters.is_empty());
            has_semesters.then(|| {
                el::div().class("mobile-action-buttons").child((
                    el::button().class("btn btn-primary")
                        .on(ev::click, move |_| state.show_search_modal.set(true))
                        .child((el::i().class("fas fa-search").attr("style", "margin-left: 6px;"), "חיפוש קורסים")),
                    el::button().class("btn btn-outline-secondary")
                        .on(ev::click, move |_| state.add_empty_course())
                        .child((el::i().class("fas fa-plus").attr("style", "margin-left: 6px;"), "הוספת קורס")),
                ))
            })
        },
    ))
}

// ── Mobile Degree Summary Bottom Sheet ──────────────────
#[component]
pub fn MobileDegreeSummary() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let show_sheet = RwSignal::new(false);

    let degree_points = Memo::new(move |_| state.user.with(|u| u.degree_points));
    let degree_average = Memo::new(move |_| state.user.with(|u| u.degree_average));
    let degree_points_done = Memo::new(move |_| state.user.with(|u| u.degree_points_done));
    let degree_points_left = Memo::new(move |_| state.user.with(|u| u.degree_points_left));
    let degree_points_to_choose = Memo::new(move |_| state.user.with(|u| u.degree_points_to_choose));
    let english_exemption = Memo::new(move |_| state.user.with(|u| u.english_exemption));

    el::div().class("mobile-only").child((
        // Trigger button at bottom
        el::button().class("mobile-degree-trigger")
            .on(ev::click, move |_| show_sheet.set(true))
            .child("הראה סיכום תואר ↑"),

        // Bottom sheet
        move || {
            show_sheet.get().then(|| {
                (
                    el::div().class("mobile-bottom-sheet-overlay")
                        .on(ev::click, move |_| show_sheet.set(false)),
                    el::div().class("mobile-bottom-sheet")
                        .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                        .child((
                            el::div().class("mobile-bottom-sheet-handle")
                                .on(ev::click, move |_| show_sheet.set(false)),
                            el::div().class("d-flex justify-content-between align-items-center")
                                .attr("style", "margin-bottom: 12px;")
                                .child((
                                    el::span(),
                                    el::h5().attr("style", "margin: 0; font-weight: bold;").child("סיכום תואר"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |_| show_sheet.set(false))
                                        .child(el::i().class("fas fa-times")),
                                )),
                            // Progress ring
                            mobile_progress_ring(degree_points_done, degree_points),
                            // Summary fields
                            mobile_summary_row("נקודות תואר", move || degree_points.get().to_string(), true, Some(move |val: f64| state.set_degree_points(val))),
                            mobile_summary_row_readonly("ממוצע תואר", move || format!("{:.1}", degree_average.get())),
                            mobile_gpa_sparkline(state, degree_average),
                            mobile_summary_row_readonly("נקודות בוצעו", move || format!("{:.1}", degree_points_done.get())),
                            mobile_summary_row_readonly("נקודות נותרו", move || format!("{:.1}", degree_points_left.get())),
                            mobile_summary_row_readonly("נותרו לשבץ", move || format!("{:.1}", degree_points_to_choose.get())),

                            el::hr(),
                            el::h6().attr("style", "text-align: center; margin-bottom: 12px; font-weight: bold;")
                                .child("ניתוח סוגי קורסים"),
                            // Column headers
                            el::div().class("d-flex align-items-center gap-2 mb-2").attr("style", "font-size: 0.8rem; color: var(--accent-blue, #0d6efd); font-weight: 600;").child((
                                el::span().attr("style", "min-width: 80px;").child(""),
                                el::span().attr("style", "flex: 1; text-align: center;").child("נותרו"),
                                el::span().attr("style", "flex: 1; text-align: center;").child("מתוך"),
                            )),
                            // Course types
                            move || {
                                state.course_types().into_iter().enumerate()
                                    .filter(|(_, ct)| ct.name != "פטור" || ct.total_points > 0.0)
                                    .map(|(i, ct)| {
                                        let name = ct.name.clone();
                                        let is_ptor = name.contains("פטור");
                                        let dot_class = format!("ct-dot ct-dot-{}", i.min(5));
                                        el::div().class("d-flex align-items-center gap-2 mb-2").child((
                                            el::span().attr("style", "min-width: 80px; font-size: 0.85rem; color: var(--text-primary); display: flex; align-items: center; gap: 4px;")
                                                .child((el::span().class(dot_class), name)),
                                            if is_ptor {
                                                el::input()
                                                    .attr("type", "number")
                                                    .class("form-control form-control-sm text-center disabled-input")
                                                    .attr("readonly", "").attr("disabled", "").attr("dir", "ltr")
                                                    .prop("value", move || {
                                                        state.course_types().get(i).map(|ct| ct.total_points.to_string()).unwrap_or_default()
                                                    })
                                                    .into_any()
                                            } else {
                                                el::input()
                                                    .attr("type", "number")
                                                    .class("form-control form-control-sm text-center disabled-input")
                                                    .attr("readonly", "").attr("disabled", "").attr("dir", "ltr")
                                                    .prop("value", move || {
                                                        state.course_types().get(i).map(|ct| format!("{:.1}", ct.points_left)).unwrap_or_default()
                                                    })
                                                    .into_any()
                                            },
                                            if is_ptor {
                                                None
                                            } else {
                                                Some(el::input()
                                                    .attr("type", "number")
                                                    .class("form-control form-control-sm text-center degree-input-field")
                                                    .attr("dir", "ltr").attr("min", "0").attr("step", "0.5")
                                                    .prop("value", move || {
                                                        state.course_types().get(i).map(|ct| ct.points_required.to_string()).unwrap_or_default()
                                                    })
                                                    .on(ev::change, move |e| {
                                                        let val: f64 = event_target_value(&e).parse().unwrap_or(0.0);
                                                        state.set_course_type_required(i, val);
                                                    }))
                                            },
                                        ))
                                    }).collect::<Vec<_>>()
                            },
                            // English exemption
                            el::div().class("form-check mt-3").child((
                                el::input()
                                    .attr("type", "checkbox")
                                    .class("form-check-input")
                                    .attr("id", "mobile-english-exemption")
                                    .prop("checked", move || english_exemption.get())
                                    .on(ev::change, move |e| {
                                        let checked = e.target()
                                            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                                            .map(|el| el.checked())
                                            .unwrap_or(false);
                                        state.set_english_exemption(checked);
                                    }),
                                el::label()
                                    .class("form-check-label")
                                    .attr("for", "mobile-english-exemption")
                                    .child("פטור מאנגלית"),
                            )),

                            // Bottom spacer so content doesn't stick to edge
                            el::div().attr("style", "height: 16px;"),
                        )),
                )
            })
        },
    ))
}

fn mobile_summary_row(
    label: &'static str,
    value_fn: impl Fn() -> String + Send + Sync + 'static,
    editable: bool,
    on_change: Option<impl Fn(f64) + Send + Sync + 'static>,
) -> impl IntoView {
    let input = if editable {
        let base = el::input()
            .attr("type", "number")
            .class("form-control form-control-sm text-center degree-input-field")
            .attr("dir", "ltr").attr("step", "0.5").attr("min", "0")
            .prop("value", value_fn);
        if let Some(cb) = on_change {
            base.on(ev::change, move |e| {
                let val: f64 = event_target_value(&e).parse().unwrap_or(0.0);
                cb(val);
            }).into_any()
        } else {
            base.into_any()
        }
    } else {
        el::input()
            .attr("type", "number")
            .class("form-control form-control-sm text-center disabled-input")
            .attr("readonly", "").attr("disabled", "").attr("dir", "ltr")
            .prop("value", value_fn)
            .into_any()
    };

    el::div().class("d-flex align-items-center gap-2 mb-2").child((
        el::span().attr("style", "min-width: 100px; font-size: 0.85rem; color: var(--text-primary);").child(label),
        input,
    ))
}

fn mobile_summary_row_readonly(
    label: &'static str,
    value_fn: impl Fn() -> String + Send + Sync + 'static,
) -> impl IntoView {
    mobile_summary_row(label, value_fn, false, None::<fn(f64)>)
}

fn mobile_progress_ring(
    done: Memo<f64>,
    total: Memo<f64>,
) -> impl IntoView {
    let radius = 46.0_f64;
    let circumference = 2.0 * std::f64::consts::PI * radius;

    el::div()
        .class("progress-ring-wrap")
        .attr("style", "display: flex; flex-direction: column; align-items: center; margin-bottom: 12px;")
        .child(move || {
            let d = done.get();
            let t = total.get();
            let pct = if t > 0.0 { (d / t * 100.0).min(100.0) } else { 0.0 };
            let offset = circumference - (pct / 100.0) * circumference;

            let is_dark = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|doc| doc.document_element())
                .and_then(|el| el.get_attribute("data-theme"))
                .map(|t| t == "dark").unwrap_or(false);

            let color = if is_dark {
                if pct >= 100.0 { "#57ab5a" } else if pct >= 66.0 { "#539bf5" } else if pct >= 33.0 { "#c69026" } else { "#e5534b" }
            } else {
                if pct >= 100.0 { "#28a745" } else if pct >= 66.0 { "#0d6efd" } else if pct >= 33.0 { "#fd7e14" } else { "#dc3545" }
            };
            let track = if is_dark { "#373e47" } else { "#e9ecef" };

            let svg = format!(
                "<svg width='110' height='110' viewBox='0 0 110 110'>\
                <circle cx='55' cy='55' r='{radius}' fill='none' stroke='{track}' stroke-width='8'/>\
                <circle cx='55' cy='55' r='{radius}' fill='none' stroke='{color}' stroke-width='8' \
                stroke-linecap='round' stroke-dasharray='{circumference}' stroke-dashoffset='{offset}' \
                transform='rotate(-90 55 55)' style='transition: stroke-dashoffset 0.8s ease, stroke 0.5s ease;'/>\
                <text x='55' y='50' text-anchor='middle' font-size='20' font-weight='bold' class='ring-text-main'>{pct:.0}%</text>\
                <text x='55' y='68' text-anchor='middle' font-size='10' class='ring-text-sub'>{d:.1} / {t:.0} נ״ז</text>\
                </svg>",
            );

            el::div().inner_html(svg)
        })
}

fn mobile_gpa_sparkline(state: AppState, cumulative_avg: Memo<f64>) -> impl IntoView {
    el::div()
        .attr("style", "display: flex; justify-content: center; margin-bottom: 8px;")
        .child(move || {
            let averages: Vec<f64> = state.user.with(|u| {
                u.semesters.iter().map(|s| s.average).filter(|&a| a > 0.0).collect()
            });
            if averages.len() < 2 { return el::div().into_any(); }

            let cum = cumulative_avg.get();
            let w = 280.0_f64;
            let h = 50.0_f64;
            let pad = 8.0_f64;
            let min_v = averages.iter().cloned().fold(f64::MAX, f64::min).min(cum) - 5.0;
            let max_v = averages.iter().cloned().fold(f64::MIN, f64::max).max(cum) + 5.0;
            let range = (max_v - min_v).max(1.0);
            let n = averages.len();

            let points: Vec<String> = averages.iter().enumerate().map(|(i, &a)| {
                let x = pad + (i as f64 / (n - 1) as f64) * (w - 2.0 * pad);
                let y = pad + (1.0 - (a - min_v) / range) * (h - 2.0 * pad);
                format!("{x:.1},{y:.1}")
            }).collect();

            let polyline_pts = points.join(" ");
            let last = averages.last().unwrap_or(&0.0);
            let first = averages.first().unwrap_or(&0.0);
            let color = if last >= first { "#28a745" } else { "#dc3545" };
            let cum_y = pad + (1.0 - (cum - min_v) / range) * (h - 2.0 * pad);
            let first_x = pad;
            let last_x = pad + (w - 2.0 * pad);

            let dots: String = averages.iter().enumerate().map(|(i, &a)| {
                let x = pad + (i as f64 / (n - 1) as f64) * (w - 2.0 * pad);
                let y = pad + (1.0 - (a - min_v) / range) * (h - 2.0 * pad);
                format!("<circle cx='{x:.1}' cy='{y:.1}' r='3' fill='{color}'><title>סמסטר {}: {a:.1}</title></circle>", i + 1)
            }).collect();

            let area_pts = format!("{first_x:.1},{h:.1} {polyline_pts} {last_x:.1},{h:.1}");

            let svg = format!(
                "<svg width='100%' height='{h:.0}' viewBox='0 0 {w:.0} {h:.0}' preserveAspectRatio='xMidYMid meet'>\
                <defs><linearGradient id='m-spark-fill' x1='0' y1='0' x2='0' y2='1'>\
                <stop offset='0%' stop-color='{color}' stop-opacity='0.2'/>\
                <stop offset='100%' stop-color='{color}' stop-opacity='0.02'/>\
                </linearGradient></defs>\
                <polygon points='{area_pts}' fill='url(#m-spark-fill)'/>\
                <line x1='{pad:.1}' y1='{cum_y:.1}' x2='{last_x:.1}' y2='{cum_y:.1}' \
                stroke='#6c757d' stroke-width='1' stroke-dasharray='4,3' opacity='0.6'/>\
                <polyline points='{polyline_pts}' fill='none' stroke='{color}' stroke-width='2' stroke-linejoin='round' stroke-linecap='round'/>\
                {dots}</svg>",
            );

            el::div().inner_html(svg).into_any()
        })
}
