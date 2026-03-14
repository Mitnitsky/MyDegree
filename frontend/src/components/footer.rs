use leptos::prelude::*;
use leptos::html as el;
use crate::state::AppState;

#[component]
pub fn Footer() -> impl IntoView {
    el::nav()
        .class("navbar fixed-bottom navbar-dark bg-dark app-footer")
        .child(
            el::div()
                .class("mx-auto")
                .attr("style", "color: lightgray;")
                .child((
                    "© Created by: ",
                    el::a()
                        .attr("href", "https://github.com/Mitnitsky/")
                        .attr("target", "_blank")
                        .attr("style", "color: lightgray;")
                        .child("Vladimir Mitnitsky"),
                )),
        )
}

#[component]
pub fn Toast() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    move || {
        state.toast_message.get().map(|msg| {
            el::div().class("toast-notification").child(msg)
        })
    }
}
