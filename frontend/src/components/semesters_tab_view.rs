use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;
use crate::components::semester_table::{SemesterTable, SemesterSummary};

#[component]
pub fn SemestersTabView() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    let on_new_tab = move |_: web_sys::MouseEvent| {
        state.add_semester();
        let count = state.user.with(|u| u.semesters.len());
        state.set_active_semester(count - 1);
    };

    let on_remove = move |_: web_sys::MouseEvent| {
        if web_sys::window()
            .and_then(|w| w.confirm_with_message("למחוק סמסטר זה?").ok())
            .unwrap_or(false)
        {
            state.remove_semester();
            let count = state.user.with(|u| u.semesters.len());
            if count > 0 {
                state.set_active_semester(count - 1);
            }
        }
    };

    let on_toggle_type = move |_: web_sys::MouseEvent| {
        state.toggle_semester_type();
    };

    el::div()
        .class("card shadow bg-white rounded")
        .attr("style", "margin: 10px 20px;")
        .child((
            // Tab headers
            el::div()
                .class("card-header")
                .attr("style", "min-height: 3.5rem; padding: 8px;")
                .child(
                    el::ul().class("nav nav-pills").attr("role", "tablist").child((
                        move || {
                            let semesters = state.semesters();
                            let active = state.active_semester_index();
                            semesters.into_iter().enumerate().map(|(i, sem)| {
                                let label = format!("סמסטר {}", sem.name);
                                let is_active = i == active;
                                let is_summer = sem.is_summer();
                                let tab_class = format!(
                                    "nav-link{}{}",
                                    if is_active { " active bg-primary text-white" } else { "" },
                                    if is_summer && !is_active { " summer" } else { "" },
                                );
                                el::li().class("nav-item").attr("role", "presentation").child(
                                    el::button()
                                        .class(tab_class)
                                        .on(ev::click, move |_| state.set_active_semester(i))
                                        .child(label),
                                )
                            }).collect::<Vec<_>>()
                        },
                        el::li().class("nav-item").child(
                            el::button().class("nav-link")
                                .on(ev::click, on_new_tab)
                                .child(el::b().child("+")),
                        ),
                    )),
                ),

            // Tab content
            el::div().class("card-body").child(
                move || {
                    let semesters = state.semesters();
                    let active = state.active_semester_index();
                    if semesters.is_empty() {
                        el::div()
                            .class("container text-center text-muted alert alert-secondary")
                            .child((
                                el::h2().child("עוד לא נוספו סמסטרים"),
                                el::br(),
                                el::button().class("btn btn-outline-secondary")
                                    .on(ev::click, move |_: web_sys::MouseEvent| {
                                        state.add_semester();
                                        let count = state.user.with(|u| u.semesters.len());
                                        state.set_active_semester(count - 1);
                                    })
                                    .child("הוסף סמסטר"),
                            ))
                            .into_any()
                    } else if active < semesters.len() {
                        let is_summer = semesters[active].is_summer();
                        let toggle_text = if is_summer { "הפוך לסמסטר רגיל" } else { "הפוך לסמסטר קיץ" };
                        el::div().class("semester-fade-in").child((
                            el::div().class("row justify-content-md-center").child((
                                el::div().class("col-xl-10").attr("style", "margin-bottom: 10px;").child(
                                    SemesterTable(),
                                ),
                                el::div().class("col-xl-2").attr("style", "padding: 0;").child(
                                    SemesterSummary(),
                                ),
                            )),
                            el::div().class("row").child((
                                el::div().class("col-xl-10"),
                                el::div().class("col-xl-2").child(
                                    el::div().class("d-flex gap-2 mx-1 mt-2").child((
                                        el::button().class("btn btn-sm sem-btn sem-btn-delete")
                                            .on(ev::click, on_remove)
                                            .child("מחק סמסטר"),
                                        el::button().class("btn btn-sm sem-btn sem-btn-summer")
                                            .on(ev::click, on_toggle_type)
                                            .child(toggle_text),
                                    )),
                                ),
                            )),
                        )).into_any()
                    } else {
                        el::div().into_any()
                    }
                },
            ),
        ))
}
