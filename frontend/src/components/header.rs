use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use wasm_bindgen::JsCast;
use crate::state::AppState;

#[component]
pub fn Header() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let show_import_modal = RwSignal::new(false);
    let show_cf_modal = RwSignal::new(false);
    let import_text = RwSignal::new(String::new());
    let cf_text = RwSignal::new(String::new());
    let new_category_name = RwSignal::new(String::new());
    let show_category_modal = RwSignal::new(false);
    let show_clear_modal = RwSignal::new(false);
    let clear_input = RwSignal::new(String::new());
    let show_calc = RwSignal::new(false);
    let show_account_dd = RwSignal::new(false);
    let show_io_dd = RwSignal::new(false);

    // Close dropdowns on any outside click
    {
        use wasm_bindgen::closure::Closure;
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |e: web_sys::MouseEvent| {
            // Only act when a dropdown is actually open
            if !show_account_dd.get_untracked() && !show_io_dd.get_untracked() {
                return;
            }
            if let Some(target) = e.target() {
                if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                    if el.closest(".dropdown").ok().flatten().is_none() {
                        show_account_dd.set(false);
                        show_io_dd.set(false);
                    }
                }
            }
        });
        let doc = web_sys::window().unwrap().document().unwrap();
        doc.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).ok();
        cb.forget();
    }


    let on_import_json = move |_: web_sys::MouseEvent| {
        let text = import_text.get();
        if !text.is_empty() {
            state.import_json(&text);
            gloo_timers::callback::Timeout::new(0, move || show_import_modal.set(false)).forget();
            import_text.set(String::new());
            state.show_toast("קורסים יובאו בהצלחה");
        }
    };

    let on_export_with_grades = move |_: web_sys::MouseEvent| {
        let json = state.export_json(true);
        trigger_download(&json, "courses_with_grades.json");
    };

    let on_export_without_grades = move |_: web_sys::MouseEvent| {
        let json = state.export_json(false);
        trigger_download(&json, "courses.json");
    };

    let on_clear = move |_: web_sys::MouseEvent| {
        clear_input.set(String::new());
        show_clear_modal.set(true);
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

    let show_auth_modal = RwSignal::new(false);

    // Auto-close auth modal when user logs in
    Effect::new(move |_| {
        if state.logged.get() {
            show_auth_modal.set(false);
        }
    });

    el::div().child((
        // Navbar
        el::nav()
            .class("navbar navbar-expand-lg navbar-dark bg-dark app-header")
            .child(
                el::div().class("container-fluid").child((
                    // Auth buttons (rightmost in RTL)
                    el::div().class("d-flex align-items-center").attr("style", "padding: 0 12px;").child(
                        move || {
                            if state.logged.get() {
                                let name = state.user_name.get();
                                let display = if name.is_empty() { "שלום".to_string() } else { format!("שלום {}", name) };
                                // Bootstrap dropdown with account icon + name
                                el::div().class("nav-item dropdown").child((
                                    el::a()
                                        .class("nav-link dropdown-toggle")
                                        .attr("href", "#")
                                        .attr("role", "button")
                                        .attr("aria-expanded", "false")
                                        .attr("style", "color: lightgray;")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.stop_propagation();
                                            show_io_dd.set(false);
                                            show_account_dd.update(|v| *v = !*v);
                                        })
                                        .child((
                                            el::i().class("fas fa-user-circle").attr("style", "margin-left: 5px;"),
                                            display,
                                        )),
                                    el::ul().class(move || if show_account_dd.get() { "dropdown-menu show" } else { "dropdown-menu" })
                                        .attr("style", "text-align: right;")
                                        .child(
                                            el::li().child(
                                                el::a().class("dropdown-item").attr("href", "#")
                                                    .on(ev::click, move |_| { state.sign_out(); show_account_dd.set(false); })
                                                    .child((
                                                        el::i().class("fas fa-sign-out-alt").attr("style", "margin-left: 5px;"),
                                                        "יציאה",
                                                    )),
                                            ),
                                        ),
                                )).into_any()
                            } else {
                                el::a().class("nav-link")
                                    .attr("href", "#")
                                    .attr("style", "color: lightgray;")
                                    .on(ev::click, move |_| show_auth_modal.set(true))
                                    .child((
                                        el::i().class("fas fa-sign-in-alt fa-flip-horizontal").attr("style", "margin-left: 5px;"),
                                        "כניסה",
                                    )).into_any()
                            }
                        },
                    ),
                    // Divider
                    el::div().class("nav-divider"),
                    el::div().class("navbar-nav me-auto").child((
                        // Import/Export dropdown
                        el::div().class("nav-item dropdown").child((
                            el::a()
                                .class("nav-link dropdown-toggle")
                                .attr("href", "#")
                                .attr("role", "button")
                                .attr("aria-expanded", "false")
                                .on(ev::click, move |e: web_sys::MouseEvent| {
                                    e.stop_propagation();
                                    show_account_dd.set(false);
                                    show_io_dd.update(|v| *v = !*v);
                                })
                                .child("ייבוא / ייצוא"),
                            el::ul()
                                .class(move || if show_io_dd.get() { "dropdown-menu show" } else { "dropdown-menu" })
                                .attr("style", "text-align: right;")
                                .child((
                                    el::li().child(
                                        el::a().class("dropdown-item").attr("href", "#")
                                            .on(ev::click, move |_| { show_import_modal.set(true); show_io_dd.set(false); })
                                            .child((el::i().class("fas fa-file-import me-2"), "ייבוא מ-JSON")),
                                    ),
                                    el::li().child(
                                        el::a().class("dropdown-item").attr("href", "#")
                                            .on(ev::click, move |_| { show_cf_modal.set(true); show_io_dd.set(false); })
                                            .child((el::i().class("fas fa-utensils me-2"), "ייבוא מ-Cheesefork")),
                                    ),
                                    el::li().child(el::hr().class("dropdown-divider")),
                                    el::li().child(
                                        el::a().class("dropdown-item").attr("href", "#")
                                            .on(ev::click, move |e: web_sys::MouseEvent| { (on_export_with_grades)(e); show_io_dd.set(false); })
                                            .child((el::i().class("fas fa-file-export me-2"), "ייצוא עם ציונים")),
                                    ),
                                    el::li().child(
                                        el::a().class("dropdown-item").attr("href", "#")
                                            .on(ev::click, move |e: web_sys::MouseEvent| { (on_export_without_grades)(e); show_io_dd.set(false); })
                                            .child((el::i().class("fas fa-file-export me-2"), "ייצוא בלי ציונים")),
                                    ),
                                    el::li().child(el::hr().class("dropdown-divider")),
                                    el::li().child(
                                        el::a().class("dropdown-item text-danger").attr("href", "#")
                                            .on(ev::click, move |e: web_sys::MouseEvent| { (on_clear)(e); show_io_dd.set(false); })
                                            .child((el::i().class("fas fa-trash me-2"), "מחק הכל")),
                                    ),
                                )),
                        )),
                        // Divider
                        el::div().class("nav-divider"),
                        // Categories link
                        el::div().class("nav-item").child(
                            el::a().class("nav-link").attr("href", "#")
                                .on(ev::click, move |_| show_category_modal.set(true))
                                .child("קטגוריות"),
                        ),
                        // Divider
                        el::div().class("nav-divider"),
                        // Average planner link
                        el::div().class("nav-item").child(
                            el::a().class("nav-link").attr("href", "#")
                                .on(ev::click, move |_| show_calc.set(true))
                                .child("תכנון ממוצע"),
                        ),
                        // Divider
                        el::div().class("nav-divider"),
                        // Course map link
                        el::div().class("nav-item").child(
                            el::a().class("nav-link").attr("href", "#")
                                .on(ev::click, move |_| state.show_course_map.set(true))
                                .child("מפת קורסים"),
                        ),
                        // Divider
                        el::div().class("nav-divider"),
                        // Dark mode toggle (pill with sun/moon)
                        el::div().class("nav-item d-flex align-items-center").attr("style", "margin-right: 8px;").child(
                            el::div().class("theme-toggle")
                                .attr("title", "מצב כהה / בהיר")
                                .on(ev::click, move |_| {
                                    let _ = js_sys::eval("window.toggleDarkMode()");
                                })
                                .child((
                                    el::span().class("theme-toggle-icon theme-icon-sun").child(
                                        el::i().class("fas fa-sun"),
                                    ),
                                    el::span().class("theme-toggle-icon theme-icon-moon").child(
                                        el::i().class("fas fa-moon"),
                                    ),
                                )),
                        ),
                    )),
                    el::a().class("navbar-brand")
                        .attr("href", "#")
                        .attr("style", "padding: 8px 10px; margin: 0;")
                        .child((
                            "My Degree ",
                            el::i().class("fas fa-graduation-cap ms-2"),
                        )),
                )),
            ),

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
                                        .on(ev::click, move |_| dismiss2())
                                        .child(el::i().class("fas fa-times")),
                                )),
                                el::div().child((
                                    el::textarea()
                                        .class("form-control mb-3")
                                        .attr("rows", "10")
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

        // Cheesefork import modal
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
                            .attr("style", "max-width: 900px; min-width: 700px;")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                            .child((
                                el::div().class("d-flex justify-content-between align-items-center").child((
                                    el::h5().class("mb-0").child("ייבוא מ-Cheesefork"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |_| dismiss2())
                                        .child(el::i().class("fas fa-times")),
                                )),
                                el::div().child((
                                    // Instructions inline toggle
                                    {
                                        let show_pop = RwSignal::new(false);
                                        (
                                            el::div().class("d-flex justify-content-center mb-2").child(
                                                el::button().class("btn btn-outline-primary btn-sm")
                                                    .on(ev::click, move |_| show_pop.update(|v| *v = !*v))
                                                    .child("הוראות"),
                                            ),
                                            move || {
                                                show_pop.get().then(|| {
                                                    el::div().class("card mb-3").child(
                                                        el::div().class("card-body").attr("style", "text-align: right;").child((
                                                            el::h6().child("הוראות"),
                                                            el::p().child((
                                                                "יש לסמן את הקורסים ב-",
                                                                el::a().attr("href", "https://cheesefork.cf/").attr("target", "_blank").child("Cheesefork"),
                                                                " ולהעתיק אותם לתיבת הטקסט בחלון זה",
                                                            )),
                                                            el::img()
                                                                .class("img-fluid rounded")
                                                                .attr("src", "images/import_from_cf.png")
                                                                .attr("alt", "הוראות ייבוא"),
                                                        )),
                                                    )
                                                })
                                            },
                                        )
                                    },
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

        // Category management modal
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
                                // Header (sticky)
                                el::div().class("d-flex justify-content-between align-items-center").child((
                                    el::h5().class("mb-0").child("ניהול קטגוריות"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |_| dismiss2())
                                        .child(el::i().class("fas fa-times")),
                                )),
                                // Scrollable body
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
                                            .on(ev::input, move |e| new_category_name.set(event_target_value(&e)))
                                            .on(ev::keydown, move |e: web_sys::KeyboardEvent| {
                                                if e.key() == "Enter" {
                                                    let name = new_category_name.get();
                                                    if !name.is_empty() {
                                                        state.add_course_type(&name);
                                                        new_category_name.set(String::new());
                                                    }
                                                }
                                            }),
                                        el::button().class("btn btn-outline-primary")
                                            .on(ev::click, on_add_category)
                                            .child("הוסף"),
                                    )),
                                )),
                            )),
                    )
            })
        },

        // Auth modal
        move || {
            show_auth_modal.get().then(|| {
                // Trigger auth widget after DOM is rendered
                let start_ui = wasm_bindgen::closure::Closure::once_into_js(move || {
                    crate::firebase::start_auth_ui("firebaseui-auth-container");
                });
                let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                    start_ui.unchecked_ref(),
                    100,
                );
                let dismiss_overlay = move || {
                    gloo_timers::callback::Timeout::new(0, move || show_auth_modal.set(false)).forget();
                };
                let dismiss_x = dismiss_overlay.clone();

                el::div().class("search-overlay")
                    .on(ev::click, move |_| dismiss_overlay())
                    .child(
                        el::div().class("search-dialog")
                            .attr("style", "max-width: 420px; min-width: 320px;")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                            .child((
                                el::div().class("d-flex justify-content-between align-items-center").child((
                                    el::h5().class("mb-0").child("כניסה"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |_| dismiss_x())
                                        .child(el::i().class("fas fa-times")),
                                )),
                                el::div().child(
                                    el::div().id("firebaseui-auth-container"),
                                ),
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
                            .child((
                                // Header
                                el::div().class("d-flex justify-content-between align-items-center").child((
                                    el::h5().class("mb-0").child("מחיקת כל הנתונים"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |_| dismiss2())
                                        .child(el::i().class("fas fa-times")),
                                )),
                                // Body
                                el::div().child((
                                    el::p().class("text-muted").attr("style", "margin-bottom: 12px;")
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
                                        .on(ev::click, move |_| {
                                            if clear_input.get().trim() == "REMOVE" {
                                                state.clear_user_data();
                                                dismiss3();
                                            }
                                        })
                                        .child("מחק הכל"),
                                )),
                            )),
                    )
            })
        },
        move || crate::components::degree_summary::grade_calc_modal(state, show_calc),
    ))
}

pub fn trigger_download(content: &str, filename: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Ok(elem) = document.create_element("a") {
                let href = format!(
                    "data:application/json;charset=utf-8,{}",
                    js_sys::encode_uri_component(content)
                );
                let _ = elem.set_attribute("href", &href);
                let _ = elem.set_attribute("download", filename);
                let _ = elem.set_attribute("style", "display:none");
                if let Some(body) = document.body() {
                    let _ = body.append_child(&elem);
                    if let Some(html_elem) = elem.dyn_ref::<web_sys::HtmlElement>() {
                        html_elem.click();
                    }
                    let _ = body.remove_child(&elem);
                }
            }
        }
    }
}
