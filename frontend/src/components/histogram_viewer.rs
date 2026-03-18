use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::histogram::{self, HistogramData, ExamEntry};

/// Reusable histogram viewer component.
/// Call with a course number to fetch and display histograms.
#[component]
pub fn HistogramViewer(course_number: String, #[prop(optional)] hide_header: bool) -> impl IntoView {
    let loading = RwSignal::new(true);
    let data = RwSignal::new(Option::<HistogramData>::None);
    let selected_entry = RwSignal::new(Option::<ExamEntry>::None);
    let image_url = RwSignal::new(Option::<String>::None);
    let show_image_modal = RwSignal::new(false);

    // Fetch histogram data
    let num = course_number.clone();
    leptos::task::spawn_local(async move {
        let result = histogram::fetch_histogram(&num).await;
        if let Some(ref hist) = result {
            // Auto-select first entry
            if let Some(first_sem) = hist.semesters.first() {
                if let Some(first_entry) = first_sem.entries.first() {
                    selected_entry.set(Some(first_entry.clone()));
                    image_url.set(Some(histogram::build_image_url(
                        &hist.resolved_number,
                        &first_entry.semester_number,
                        &first_entry.entry_name,
                    )));
                }
            }
        }
        data.set(result);
        loading.set(false);
    });

    let header_style = if hide_header { "display: none;" } else { "background-color: #343a40;" };

    el::div().class("card mb-2").child((
        el::div()
            .class("card-header text-white")
            .attr("style", header_style)
            .child(el::strong().child("היסטוגרמות")),
        el::div().class("card-body").child((
            // Loading spinner
            move || {
                loading.get().then(|| {
                    el::div()
                        .class("d-flex justify-content-center my-3")
                        .child(
                            el::div()
                                .class("spinner-border text-primary")
                                .attr("role", "status")
                                .child(el::span().class("visually-hidden").child("טוען...")),
                        )
                })
            },
            // No data message
            move || {
                let is_loading = loading.get();
                let has_data = data.with(|d| d.as_ref().map(|h| !h.semesters.is_empty()).unwrap_or(false));
                (!is_loading && !has_data).then(|| {
                    el::div().class("text-center my-2").child(
                        el::strong().child("אין היסטוגרמות זמינות"),
                    )
                })
            },
            // Semester/exam selector + stats + image
            move || {
                let d = data.get();
                let is_loading = loading.get();
                if is_loading { return None; }
                d.and_then(|hist| {
                    if hist.semesters.is_empty() { return None; }
                    let resolved = hist.resolved_number.clone();

                    // Build flat list of all entries for the dropdown
                    let all_entries: Vec<(String, ExamEntry)> = hist.semesters.iter().flat_map(|sem| {
                        sem.entries.iter().map(|e| {
                            let label = format!("{} - {}", sem.label, e.display_name);
                            (label, e.clone())
                        })
                    }).collect();

                    let all_entries_for_change = all_entries.clone();

                    Some(el::div().child((
                        // Dropdown
                        el::select()
                            .class("form-select mb-2")
                            .on(ev::change, {
                                let resolved2 = resolved.clone();
                                move |e| {
                                    let idx: usize = event_target_value(&e).parse().unwrap_or(0);
                                    if let Some((_, entry)) = all_entries_for_change.get(idx) {
                                        image_url.set(Some(histogram::build_image_url(
                                            &resolved2,
                                            &entry.semester_number,
                                            &entry.entry_name,
                                        )));
                                        selected_entry.set(Some(entry.clone()));
                                    }
                                }
                            })
                            .child(
                                all_entries.iter().enumerate().map(|(i, (label, _))| {
                                    el::option().attr("value", i.to_string()).child(label.clone())
                                }).collect::<Vec<_>>()
                            ),
                        // Lecturer name
                        move || {
                            selected_entry.get().and_then(|entry| {
                                entry.staff.map(|s| {
                                    el::p().class("text-center mb-2").child(
                                        el::strong().child(s),
                                    )
                                })
                            })
                        },
                        // Stats grid
                        move || {
                            selected_entry.get().map(|entry| {
                                let s = &entry.stats;
                                let fmt = |v: &Option<serde_json::Value>| {
                                    v.as_ref().map(|v| match v {
                                        serde_json::Value::Number(n) => n.to_string(),
                                        serde_json::Value::String(s) => s.clone(),
                                        other => other.to_string(),
                                    }).unwrap_or("-".to_string())
                                };
                                let stat = |label: &str, value: String| {
                                    el::div().class("hist-stat-item").child((
                                        el::div().class("hist-stat-value").child(value),
                                        el::div().class("hist-stat-label").child(label.to_string()),
                                    ))
                                };
                                el::div().class("hist-stats-grid").child((
                                    stat("ממוצע", fmt(&s.average)),
                                    stat("חציון", fmt(&s.median)),
                                    stat("סטודנטים", fmt(&s.students)),
                                    stat("אחוז עוברים", s.pass_percent.clone().unwrap_or("-".into())),
                                    stat("נכשל/עובר", s.pass_fail.clone().unwrap_or("-".into())),
                                    stat("מינימלי", fmt(&s.min)),
                                    stat("מקסימלי", fmt(&s.max)),
                                ))
                            })
                        },
                        // Histogram image
                        move || {
                            image_url.get().map(|url| {
                                el::img()
                                    .class("img-fluid rounded mb-2")
                                    .attr("src", url)
                                    .attr("style", "cursor: zoom-in; width: 100%;")
                                    .on(ev::click, move |_| show_image_modal.set(true))
                            })
                        },
                        // Full-size image modal
                        move || {
                            show_image_modal.get().then(|| {
                                let url = image_url.get().unwrap_or_default();
                                el::div()
                                    .class("search-overlay")
                                    .attr("style", "z-index: 2000;")
                                    .on(ev::click, move |_| {
                                        gloo_timers::callback::Timeout::new(0, move || show_image_modal.set(false)).forget();
                                    })
                                    .child(
                                        el::div()
                                            .attr("style", "max-width: 90%; max-height: 90%; margin: auto; margin-top: 5vh;")
                                            .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                                            .child(
                                                el::img()
                                                    .class("img-fluid")
                                                    .attr("src", url)
                                                    .attr("style", "width: 100%;"),
                                            ),
                                    )
                            })
                        },
                    )))
                })
            },
        )),
    ))
}
