use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;
use crate::components::{Header, SemestersTabView, DegreeSummary, Footer, Toast, SearchCourseDialog, MobileFooter};
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
                el::div().class("container-fluid").child((
                    SemestersTabView(),
                    DegreeSummary(),
                    Footer(),
                )),
            )),
            // Mobile layout (hidden on desktop via CSS)
            el::div().class("mobile-only mobile-layout").child((
                MobileHeader(),
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
                state.show_histogram_modal.get().map(|num| {
                    el::div()
                        .class("search-overlay")
                        .on(ev::click, move |_| state.show_histogram_modal.set(None))
                        .child(
                            el::div()
                                .class("search-dialog")
                                .attr("style", "max-width: 900px;")
                                .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                                .child((
                                    el::div().class("d-flex justify-content-between align-items-center").child((
                                        el::h5().class("mb-0").child("היסטוגרמות"),
                                        el::button().class("btn btn-sm btn-outline-secondary")
                                            .on(ev::click, move |_| state.show_histogram_modal.set(None))
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
