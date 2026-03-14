use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;
use crate::components::SearchCourseDialog;
use crate::components::histogram_viewer::{HistogramViewer, HistogramViewerProps};

fn semester_table_header() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    el::thead().attr("style", "background-color: rgb(233, 236, 239);").child(
        el::tr().attr("style", "font-family: Alef, serif;").child((
            el::th().attr("scope", "col").child("קטגוריה"),
            el::th().attr("scope", "col")
                .attr("style", "cursor: pointer;")
                .attr("title", "לחץ למיון")
                .on(ev::click, move |_| state.sort_by_field("number"))
                .child("מספר קורס"),
            el::th().attr("scope", "col")
                .attr("style", "cursor: pointer;")
                .attr("title", "לחץ למיון")
                .on(ev::click, move |_| state.sort_by_field("name"))
                .child("שם קורס"),
            el::th().attr("scope", "col")
                .attr("style", "cursor: pointer;")
                .attr("title", "לחץ למיון")
                .on(ev::click, move |_| state.sort_by_field("points"))
                .child("נקודות"),
            el::th().attr("scope", "col")
                .attr("style", "cursor: pointer;")
                .attr("title", "לחץ למיון")
                .on(ev::click, move |_| state.sort_by_field("grade"))
                .child("ציון"),
            el::th().attr("scope", "col"),
        )),
    )
}

fn semester_table_row(index: usize) -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let show_menu = RwSignal::new(false);
    let show_histogram_modal = RwSignal::new(false);

    let course = Memo::new(move |_| {
        state.user.with(|u| {
            let sem_idx = u.active_semester;
            u.semesters.get(sem_idx).and_then(|s| s.courses.get(index).cloned())
        })
    });

    let course_types = Memo::new(move |_| state.course_types());

    let type_class = move || {
        course.with(|c| {
            c.as_ref().map(|c| format!("course-type-{}", c.course_type.min(5))).unwrap_or_default()
        })
    };

    let row = el::tr().class(type_class).child((
        // Category selector
        el::td().attr("style", "width: 15%;").child(
            el::select()
                .class("form-select form-select-sm")
                .on(ev::change, move |e| {
                    let val = event_target_value(&e);
                    state.update_course_field(index, "type", &val);
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
        ),
        // Course number
        el::td().attr("style", "width: 100px;").child(
            el::input()
                .attr("type", "text")
                .class("form-control form-control-sm text-center")
                .attr("style", "direction: ltr;")
                .prop("value", move || {
                    course.with(|c| c.as_ref().map(|c| c.number.clone()).unwrap_or_default())
                })
                .on(ev::change, move |e| {
                    state.update_course_field(index, "number", &event_target_value(&e));
                }),
        ),
        // Course name
        el::td().child(
            el::input()
                .attr("type", "text")
                .class("form-control form-control-sm")
                .prop("value", move || {
                    course.with(|c| c.as_ref().map(|c| c.name.clone()).unwrap_or_default())
                })
                .on(ev::change, move |e| {
                    state.update_course_field(index, "name", &event_target_value(&e));
                }),
        ),
        // Points
        el::td().attr("style", "width: 80px;").child(
            el::input()
                .attr("type", "number")
                .class("form-control form-control-sm text-center")
                .attr("style", "direction: ltr;")
                .attr("step", "0.5")
                .attr("min", "0")
                .attr("max", "20")
                .prop("value", move || {
                    course.with(|c| c.as_ref().map(|c| c.points.to_string()).unwrap_or_default())
                })
                .on(ev::change, move |e| {
                    state.update_course_field(index, "points", &event_target_value(&e));
                }),
        ),
        // Grade
        el::td().attr("style", "width: 80px;").child(
            el::input()
                .attr("type", "number")
                .class("form-control form-control-sm text-center")
                .attr("style", "direction: ltr;")
                .attr("min", "0")
                .attr("max", "100")
                .prop("value", move || {
                    course.with(|c| c.as_ref().map(|c| (c.grade as i64).to_string()).unwrap_or_default())
                })
                .on(ev::change, move |e| {
                    state.update_course_field(index, "grade", &event_target_value(&e));
                }),
        ),
        // Actions dropdown
        el::td().class("text-center").attr("style", "width: 6%; vertical-align: middle;").child(
            el::div().class("dropdown").attr("style", "position: relative;").child((
                el::button()
                    .class("btn btn-outline-secondary")
                    .attr("style", "padding: 4px 10px;")
                    .on(ev::click, move |_| show_menu.update(|v| *v = !*v))
                    .child(el::i().class("fas fa-ellipsis-v")),
                move || {
                    show_menu.get().then(|| {
                        el::ul()
                            .class("dropdown-menu show")
                            .attr("style", "text-align: right; position: absolute; left: 0;")
                            .child((
                                // Histogram
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            show_histogram_modal.set(true);
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-chart-bar").attr("style", "color: dodgerblue; margin-left: 5px;"), " הצג היסטוגרמות")),
                                ),
                                // Binary toggle
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            let is_binary = course.with(|c| c.as_ref().map(|c| c.binary).unwrap_or(false));
                                            state.update_course_field(index, "binary", if !is_binary { "true" } else { "false" });
                                            show_menu.set(false);
                                        })
                                        .child(move || {
                                            let is_binary = course.with(|c| c.as_ref().map(|c| c.binary).unwrap_or(false));
                                            if is_binary {
                                                (el::i().class("fas fa-ban").attr("style", "color: red; margin-left: 5px;").into_any(), " בטל עובר בינארי".into_any())
                                            } else {
                                                (el::i().class("fas fa-check").attr("style", "color: green; margin-left: 5px;").into_any(), " סמן עובר בינארי".into_any())
                                            }
                                        }),
                                ),
                                el::li().child(el::hr().class("dropdown-divider")),
                                // Clear row
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
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
                                        .child((el::i().class("fas fa-broom").attr("style", "color: burlywood; margin-left: 5px;"), " נקה שורה")),
                                ),
                                // Delete row
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            state.remove_course(index);
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-trash").attr("style", "color: darkred; margin-left: 10px;"), " הסר שורה")),
                                ),
                                el::li().child(el::hr().class("dropdown-divider")),
                                // Move up
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            state.move_course(index, "up");
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-arrow-up").attr("style", "color: black; margin-left: 10px;"), " העלה")),
                                ),
                                // Move down
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            state.move_course(index, "down");
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-arrow-down").attr("style", "color: black; margin-left: 10px;"), " הורד")),
                                ),
                            ))
                    })
                },
            )),
        ),
    ));

    // Return both the row and the histogram modal
    (
        row,
        move || {
            show_histogram_modal.get().then(|| {
                let num = course.with(|c| c.as_ref().map(|c| c.number.clone()).unwrap_or_default());
                if num.is_empty() {
                    return None;
                }
                Some(el::div()
                    .class("search-overlay")
                    .on(ev::click, move |_| show_histogram_modal.set(false))
                    .child(
                        el::div()
                            .class("search-dialog")
                            .attr("style", "max-width: 900px;")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                            .child((
                                el::div().class("d-flex justify-content-between align-items-center mb-3").child((
                                    el::h5().class("mb-0").child("היסטוגרמות"),
                                    el::button().class("btn btn-sm btn-outline-secondary")
                                        .on(ev::click, move |_| show_histogram_modal.set(false))
                                        .child(el::i().class("fas fa-times")),
                                )),
                                HistogramViewer(HistogramViewerProps { course_number: num }),
                            )),
                    ))
            })
        },
    )
}

#[component]
pub fn SemesterTable() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    el::div().child((
        el::div().class("row").child(
            el::table().class("table table-sm table-borderless").attr("style", "margin-right: 5px;").child((
                semester_table_header(),
                el::tbody().child(
                    move || {
                        let user = state.user.get();
                        let sem_idx = user.active_semester;
                        let count = user.semesters.get(sem_idx).map(|s| s.courses.len()).unwrap_or(0);
                        (0..count).map(|i| semester_table_row(i)).collect::<Vec<_>>()
                    },
                ),
            )),
        ),
        el::div().class("d-flex justify-content-center").child(
            el::div().class("d-flex gap-2 mx-1").child((
                el::button().class("btn btn-info")
                    .on(ev::click, move |_| state.add_empty_course())
                    .child("הוספת שורה"),
                el::button().class("btn btn-primary")
                    .on(ev::click, move |_| state.show_search_modal.set(true))
                    .child("חיפוש קורסים"),
            )),
        ),
        move || {
            state.show_search_modal.get().then(SearchCourseDialog)
        },
    ))
}

#[component]
pub fn SemesterSummary() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    let average = Memo::new(move |_| {
        state.user.with(|u| u.semesters.get(u.active_semester).map(|s| s.average).unwrap_or(0.0))
    });
    let points = Memo::new(move |_| {
        state.user.with(|u| u.semesters.get(u.active_semester).map(|s| s.points).unwrap_or(0.0))
    });

    el::div().class("container").attr("style", "max-width: 300px;").child(
        el::div().class("card").child((
            el::div().class("card-header text-center")
                .attr("style", "padding: 3px; background-color: #e9ecef !important; color: #495057 !important;")
                .child(el::p().attr("style", "color: #495057; margin-bottom: 0; font-weight: bold;").child("סיכום סמסטר")),
            el::div().class("card-body").child((
                el::div().class("row mb-2").child((
                    el::div().class("col-sm-3 align-self-center").attr("style", "margin: 4px;").child(
                        el::label().child("ממוצע:"),
                    ),
                    el::div().class("col-sm-8").child(
                        el::input()
                            .attr("type", "number")
                            .class("form-control text-center")
                            .attr("readonly", "")
                            .attr("style", "direction: ltr; cursor: default; margin: 0 12px;")
                            .prop("value", move || format!("{:.1}", average.get())),
                    ),
                )),
                el::div().class("row").child((
                    el::div().class("col-sm-3 align-self-center").attr("style", "margin: 4px;").child(
                        el::label().child("נקודות:"),
                    ),
                    el::div().class("col-sm-8").child(
                        el::input()
                            .attr("type", "number")
                            .class("form-control text-center")
                            .attr("readonly", "")
                            .attr("style", "direction: ltr; cursor: default; margin: 0 12px;")
                            .prop("value", move || format!("{:.1}", points.get())),
                    ),
                )),
            )),
        )),
    )
}
