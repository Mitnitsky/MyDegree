use leptos::prelude::*;
use leptos::prelude::event_target_value;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;
use crate::components::{Header, SemestersTabView, DegreeSummary, Footer, Toast, SearchCourseDialog, MobileFooter, CourseMap, Achievements};
use crate::components::{MobileHeader, MobileSemesterTabs, MobileSemesterSummary, MobileCourseList, MobileDegreeSummary};
use crate::components::histogram_viewer::{HistogramViewer, HistogramViewerProps};

#[component]
pub fn App() -> impl IntoView {
    let state_resource = LocalResource::new(|| AppState::load());

    move || {
        match state_resource.get() {
            None => {
                // Loading spinner
                el::div()
                    .class("d-flex justify-content-center align-items-center")
                    .attr("style", "height: 100vh; font-family: Alef, Roboto, Helvetica, Arial, sans-serif;")
                    .child(
                        el::div().class("text-center").child((
                            el::div().class("spinner-border text-primary mb-3")
                                .attr("role", "status")
                                .child(el::span().class("visually-hidden").child("טוען...")),
                            el::div().child("טוען נתוני קורסים..."),
                        ))
                    )
                    .into_any()
            }
            Some(state) => {
                let state: AppState = *state;
                provide_context(state);
                app_content().into_any()
            }
        }
    }
}

fn app_content() -> impl IntoView {
    el::div()
        .id("app")
        .attr("style", "font-family: Alef, Roboto, Helvetica, Arial, sans-serif;")
        .child((
            // Desktop layout (hidden on mobile via CSS)
            el::div().class("desktop-only").child((
                Header(),
                profile_tabs(),
                el::div().class("container-fluid").child((
                    SemestersTabView(),
                    DegreeSummary(),
                    Footer(),
                )),
            )),
            // Mobile layout (hidden on desktop via CSS)
            el::div().class("mobile-only mobile-layout").child((
                MobileHeader(),
                profile_tabs(),
                MobileSemesterTabs(),
                MobileSemesterSummary(),
                MobileCourseList(),
                MobileDegreeSummary(),
                MobileFooter(),
            )),
            // Shared elements
            Toast(),
            // Data sanitization warnings modal
            move || {
                let state = use_context::<AppState>().unwrap();
                let warnings = state.data_warnings.get();
                let dismiss = move || {
                    // Defer DOM removal to next tick so click event finishes bubbling
                    gloo_timers::callback::Timeout::new(0, move || {
                        state.data_warnings.set(Vec::new());
                    }).forget();
                };
                (!warnings.is_empty()).then(|| {
                    let dismiss_overlay = dismiss.clone();
                    let dismiss_x = dismiss.clone();
                    let dismiss_btn = dismiss.clone();
                    el::div()
                        .class("search-overlay")
                        .attr("style", "z-index: 2000;")
                        .on(ev::click, move |_| dismiss_overlay())
                        .child(
                            el::div()
                                .class("search-dialog")
                                .attr("style", "max-width: 600px; min-width: unset;")
                                .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                                .child((
                                    // First child → gets padding: 16px 24px from .search-dialog > :first-child
                                    el::div().class("d-flex justify-content-between align-items-center").child((
                                        el::h5().class("mb-0").attr("style", "color: var(--text-primary); display: flex; align-items: center;").child((
                                            el::i().class("fas fa-wrench").attr("style", "color: #f0ad4e; margin-left: 8px; font-size: 1rem;"),
                                            "תיקון נתונים אוטומטי",
                                        )),
                                        el::button().class("btn btn-sm btn-outline-secondary")
                                            .on(ev::click, move |e: web_sys::MouseEvent| { e.stop_propagation(); dismiss_x(); })
                                            .child(el::i().class("fas fa-times")),
                                    )),
                                    // Second child → gets padding: 24px from .search-dialog > :nth-child(2)
                                    el::div().child((
                                        el::p().attr("style", "color: var(--text-secondary); font-size: 0.9rem; margin-bottom: 12px;")
                                            .child("נמצאו שדות עם ערכים לא תקינים שאופסו לברירת מחדל (0). אנא בדוק את הקורסים הבאים ועדכן ידנית:"),
                                        el::ul()
                                            .attr("style", "max-height: 300px; overflow-y: auto; border: 1px solid var(--border-color, #dee2e6); border-radius: 6px; padding: 12px 24px 12px 32px; background: var(--bg-input, #f8f9fa); margin: 0;")
                                            .child(
                                                warnings.into_iter().map(|w| {
                                                    el::li()
                                                        .attr("style", "padding: 3px 0; font-size: 0.85rem; color: var(--text-primary);")
                                                        .child(w)
                                                }).collect::<Vec<_>>()
                                            ),
                                        el::div().class("d-flex justify-content-center").attr("style", "padding-top: 16px; padding-bottom: 10px;").child(
                                            el::button()
                                                .class("btn btn-primary")
                                                .on(ev::click, move |e: web_sys::MouseEvent| { e.stop_propagation(); dismiss_btn(); })
                                                .child("הבנתי"),
                                        ),
                                    )),
                                ))
                        )
                })
            },
            move || {
                let state = use_context::<AppState>().unwrap();
                state.show_search_modal.get().then(SearchCourseDialog)
            },
            move || {
                let state = use_context::<AppState>().unwrap();
                state.show_course_map.get().then(CourseMap)
            },
            move || {
                let state = use_context::<AppState>().unwrap();
                state.show_achievements.get().then(Achievements)
            },
            move || {
                let state = use_context::<AppState>().unwrap();
                state.show_histogram_modal.get().map(|num| {
                    let dismiss = move || {
                        gloo_timers::callback::Timeout::new(0, move || state.show_histogram_modal.set(None)).forget();
                    };
                    let dismiss2 = dismiss.clone();
                    el::div()
                        .class("search-overlay")
                        .on(ev::click, move |_| dismiss())
                        .child(
                            el::div()
                                .class("search-dialog")
                                .attr("style", "max-width: 900px;")
                                .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                                .child((
                                    el::div().class("d-flex justify-content-between align-items-center").child((
                                        el::h5().class("mb-0").child("היסטוגרמות"),
                                        el::button().class("btn btn-sm btn-outline-secondary")
                                            .on(ev::click, move |e: web_sys::MouseEvent| { e.stop_propagation(); dismiss2(); })
                                            .child(el::i().class("fas fa-times")),
                                    )),
                                    el::div().child(
                                        HistogramViewer(HistogramViewerProps { course_number: num, hide_header: true }),
                                    ),
                                )),
                        )
                })
            },
        ))
}

fn profile_tabs() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let editing = RwSignal::<Option<usize>>::new(None);
    let edit_name = RwSignal::new(String::new());
    let delete_confirm = RwSignal::<Option<(usize, String)>>::new(None); // (index, name)
    let delete_input = RwSignal::new(String::new());

    (move || {
        let profiles = state.profiles.get();
        let active = state.active_profile.get();
        let count = profiles.profiles.len();

        // Single profile — show compact "add degree" button
        if count <= 1 && editing.get().is_none() {
            return el::div()
                .attr("style", "display: flex; justify-content: center; padding: 4px 12px; background: var(--bg-secondary, #f6f8fa); border-bottom: 1px solid var(--border-color, #d0d7de);")
                .child(
                    el::button()
                        .attr("style", "padding: 3px 12px; border-radius: 6px; border: 1px dashed var(--border-color, #d0d7de); background: transparent; color: var(--text-secondary); cursor: pointer; font-size: 0.8rem;")
                        .on(ev::click, move |_| {
                            gloo_timers::callback::Timeout::new(0, move || {
                                state.add_profile("תואר 2".to_string());
                                state.rename_profile(0, "תואר 1".to_string());
                            }).forget();
                        })
                        .child("+ הוסף תואר נוסף"),
                )
                .into_any();
        }

        el::div()
            .attr("style", "display: flex; align-items: center; gap: 6px; padding: 4px 12px; background: var(--bg-secondary, #f6f8fa); border-bottom: 1px solid var(--border-color, #d0d7de); overflow-x: auto; font-size: 0.85rem;")
            .child((
                profiles.profiles.iter().enumerate().map(|(i, profile)| {
                    let name = profile.name.clone();
                    let is_active = i == active;
                    let is_editing = editing.get() == Some(i);

                    if is_editing {
                        el::input()
                            .class("form-control form-control-sm")
                            .attr("style", "width: 140px; font-size: 0.8rem;")
                            .prop("value", move || edit_name.get())
                            .on(ev::input, move |e| edit_name.set(event_target_value(&e)))
                            .on(ev::keydown, move |e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" {
                                    let val = edit_name.get_untracked();
                                    if !val.trim().is_empty() {
                                        state.rename_profile(i, val.trim().to_string());
                                    }
                                    gloo_timers::callback::Timeout::new(0, move || editing.set(None)).forget();
                                } else if e.key() == "Escape" {
                                    gloo_timers::callback::Timeout::new(0, move || editing.set(None)).forget();
                                }
                            })
                            .on(ev::blur, move |_| {
                                let val = edit_name.get_untracked();
                                if !val.trim().is_empty() {
                                    state.rename_profile(i, val.trim().to_string());
                                }
                                gloo_timers::callback::Timeout::new(0, move || editing.set(None)).forget();
                            })
                            .into_any()
                    } else {
                        let name_for_edit = name.clone();
                        let name_display = name.clone();
                        el::div()
                            .attr("style", format!(
                                "display: flex; align-items: center; gap: 4px; padding: 4px 10px; border-radius: 6px; cursor: pointer; white-space: nowrap; transition: background 0.15s; {}",
                                if is_active { "background: var(--accent-blue, #0d6efd); color: white; font-weight: 600;" }
                                else { "background: var(--bg-primary, #fff); color: var(--text-primary); border: 1px solid var(--border-color, #d0d7de);" }
                            ))
                            .on(ev::click, move |_| {
                                if is_active {
                                    // Tap active tab to rename (works on mobile too)
                                    edit_name.set(name_for_edit.clone());
                                    editing.set(Some(i));
                                } else {
                                    gloo_timers::callback::Timeout::new(0, move || state.switch_profile(i)).forget();
                                }
                            })
                            .child((
                                name_display,
                                // Delete button (only if > 1 profile, and it's active)
                                (count > 1 && is_active).then(|| {
                                    let del_name = name.clone();
                                    el::span()
                                        .attr("style", format!(
                                            "margin-right: 4px; font-size: 0.7rem; opacity: 0.7; cursor: pointer; {}",
                                            if is_active { "color: white;" } else { "color: var(--text-muted);" }
                                        ))
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.stop_propagation();
                                            delete_input.set(String::new());
                                            delete_confirm.set(Some((i, del_name.clone())));
                                        })
                                        .child("✕")
                                }),
                            ))
                            .into_any()
                    }
                }).collect::<Vec<_>>(),
                // Add profile button
                el::button()
                    .attr("style", "padding: 4px 8px; border-radius: 6px; border: 1px dashed var(--border-color, #d0d7de); background: transparent; color: var(--text-secondary); cursor: pointer; font-size: 0.8rem; white-space: nowrap;")
                    .on(ev::click, move |_| {
                        gloo_timers::callback::Timeout::new(0, move || {
                            let count = state.profiles.get_untracked().profiles.len();
                            let name = format!("תואר {}", count + 1);
                            state.add_profile(name);
                        }).forget();
                    })
                    .child("+ תואר חדש"),
            ))
            .into_any()
    },
    // Delete confirmation modal
    move || {
        delete_confirm.get().map(|(idx, name)| {
            let expected = format!("מחק - {}", name);
            let expected_clone = expected.clone();
            let dismiss_overlay = move || {
                gloo_timers::callback::Timeout::new(0, move || delete_confirm.set(None)).forget();
            };
            let dismiss_x = dismiss_overlay.clone();
            el::div().class("search-overlay")
                .attr("style", "z-index: 2000;")
                .on(ev::click, move |_| dismiss_overlay())
                .child(
                    el::div().class("search-dialog")
                        .attr("style", "max-width: 400px; min-width: unset; overflow: hidden;")
                        .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                        .child((
                            el::div().class("d-flex justify-content-between align-items-center").child((
                                el::h5().class("mb-0").attr("style", "color: var(--text-primary);").child("מחיקת תואר"),
                                el::button().class("btn btn-sm btn-outline-secondary")
                                    .on(ev::click, move |e: web_sys::MouseEvent| { e.stop_propagation(); dismiss_x(); })
                                    .child(el::i().class("fas fa-times")),
                            )),
                            el::div().child((
                                el::p().attr("style", "color: var(--text-secondary); margin-bottom: 12px; font-size: 0.9rem;")
                                    .child(format!("הקלד {} כדי לאשר מחיקה", &expected)),
                                el::input()
                                    .class("form-control mb-3")
                                    .attr("placeholder", expected.clone())
                                    .attr("dir", "rtl")
                                    .prop("value", move || delete_input.get())
                                    .on(ev::input, move |e| delete_input.set(event_target_value(&e))),
                                el::button()
                                    .class(move || {
                                        let expected_inner = expected_clone.clone();
                                        if delete_input.get().trim() == expected_inner { "btn btn-danger w-100" } else { "btn btn-secondary w-100" }
                                    })
                                    .prop("disabled", move || {
                                        let expected_inner = format!("מחק - {}", delete_confirm.get().map(|(_, n)| n).unwrap_or_default());
                                        delete_input.get().trim() != expected_inner
                                    })
                                    .on(ev::click, move |_| {
                                        let expected_inner = format!("מחק - {}", delete_confirm.get_untracked().map(|(_, n)| n).unwrap_or_default());
                                        if delete_input.get().trim() == expected_inner {
                                            gloo_timers::callback::Timeout::new(0, move || {
                                                state.delete_profile(idx);
                                                delete_confirm.set(None);
                                            }).forget();
                                        }
                                    })
                                    .child("מחק תואר"),
                            )),
                        )),
                )
        })
    })
}
