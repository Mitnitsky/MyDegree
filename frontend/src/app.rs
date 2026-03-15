use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;
use crate::components::{Header, SemestersTabView, DegreeSummary, Footer, Toast, SearchCourseDialog};
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
            MobileHeader(),
            MobileSemesterTabs(),
            MobileSemesterSummary(),
            MobileCourseList(),
            MobileDegreeSummary(),
            // Shared elements
            Toast(),
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
