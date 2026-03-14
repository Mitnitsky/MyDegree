use leptos::prelude::*;
use leptos::html as el;
use crate::state::AppState;
use crate::components::{Header, SemestersTabView, DegreeSummary, Footer, Toast};

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
        ))
}
