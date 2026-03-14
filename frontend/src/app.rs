use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;
use crate::components::{Header, SemestersTabView, DegreeSummary, Footer, Toast, SearchCourseDialog};
use crate::components::histogram_viewer::{HistogramViewer, HistogramViewerProps};

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    el::div()
        .id("app")
        .attr("style", "font-family: Alef, Roboto, Helvetica, Arial, sans-serif; min-width: 965px !important;")
        .child((
            Header(),
            el::div().class("container-fluid").child((
                SemestersTabView(),
                DegreeSummary(),
                Footer(),
            )),
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
                                        HistogramViewer(HistogramViewerProps { course_number: num }),
                                    ),
                                )),
                        )
                })
            },
        ))
}
