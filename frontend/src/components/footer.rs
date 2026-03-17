use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;

#[component]
pub fn Footer() -> impl IntoView {
    let show_about = RwSignal::new(false);

    el::div().child((
        el::nav()
            .class("navbar fixed-bottom navbar-dark bg-dark app-footer")
            .child(
                el::div()
                    .class("mx-auto")
                    .attr("style", "color: lightgray; cursor: pointer;")
                    .on(ev::click, move |_| show_about.set(true))
                    .child((
                        "© Created by: ",
                        el::a()
                            .attr("href", "#")
                            .attr("style", "color: lightgray;")
                            .on(ev::click, move |e: web_sys::MouseEvent| e.prevent_default())
                            .child("Vladimir Mitnitsky"),
                    )),
            ),
        move || show_about.get().then(|| AboutModal(AboutModalProps { on_close: move || show_about.set(false) })),
    ))
}

#[component]
pub fn MobileFooter() -> impl IntoView {
    let show_about = RwSignal::new(false);

    el::div().class("mobile-only mobile-footer-credit").child((
        el::div()
            .attr("style", "cursor: pointer;")
            .on(ev::click, move |_| show_about.set(true))
            .child((
                "© Created by: ",
                el::a()
                    .attr("href", "#")
                    .attr("style", "color: inherit;")
                    .on(ev::click, move |e: web_sys::MouseEvent| e.prevent_default())
                    .child("Vladimir Mitnitsky"),
            )),
        move || show_about.get().then(|| AboutModal(AboutModalProps { on_close: move || show_about.set(false) })),
    ))
}

#[component]
fn AboutModal(on_close: impl Fn() + 'static + Copy) -> impl IntoView {
    let feedback_text = RwSignal::new(String::new());
    let feedback_name = RwSignal::new(String::new());
    let feedback_email = RwSignal::new(String::new());
    let feedback_sent = RwSignal::new(false);
    let feedback_sending = RwSignal::new(false);

    let on_send_feedback = move |_: web_sys::MouseEvent| {
        let msg = feedback_text.get();
        let name = feedback_name.get();
        let email = feedback_email.get();
        if msg.trim().is_empty() { return; }
        feedback_sending.set(true);
        let name_clone = name.clone();
        let msg_clone = msg.clone();
        let email_clone = email.clone();
        let js_code = format!(
            "emailjs.send('service_bzn5iqf','template_99kn4p1',{{name:'{}',message:'{}',email:'{}'}}).then(function(){{}})",
            name_clone.replace('\'', "\\'").replace('\n', "\\n"),
            msg_clone.replace('\'', "\\'").replace('\n', "\\n"),
            email_clone.replace('\'', "\\'"),
        );
        let _ = js_sys::eval(&js_code);
        feedback_sending.set(false);
        feedback_sent.set(true);
        feedback_text.set(String::new());
        feedback_name.set(String::new());
        feedback_email.set(String::new());
    };

    el::div().class("search-overlay")
        .on(ev::click, move |_| on_close())
        .child(
            el::div().class("search-dialog")
                .attr("style", "max-width: 600px; min-width: unset; overflow: hidden;")
                .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                .child((
                    // Header with title and close button
                    el::div().attr("style", "display: flex; justify-content: space-between; align-items: center; padding: 12px 16px;").child((
                        el::h5().class("mb-0").child("אודות MyDegree"),
                        el::button().class("btn btn-sm btn-outline-secondary")
                            .on(ev::click, move |_| on_close())
                            .child(el::i().class("fas fa-times")),
                    )),
                    // Modal content
                    el::div().class("about-modal-content").child((
                        // Logo side
                        el::div().class("about-logo-side").child(
                            el::img().attr("src", "images/logo_transparent.png").attr("alt", "MyDegree"),
                        ),
                        // Info side
                        el::div().class("about-info-side").child((
                            el::h5().child("Contact Info"),
                            el::a().class("about-link")
                                .attr("href", "https://github.com/Mitnitsky/")
                                .attr("target", "_blank")
                                .child((el::i().class("fab-github"), " GitHub")),
                            el::a().class("about-link")
                                .attr("href", "https://linkedin.com/in/vladimir-mitnitsky")
                                .attr("target", "_blank")
                                .child((el::i().class("fab-linkedin"), " LinkedIn")),
                            // Feedback section
                            el::div().class("about-feedback").child((
                                el::h6().child("Send Feedback"),
                                move || {
                                    if feedback_sent.get() {
                                        el::div()
                                            .attr("style", "color: green; text-align: center; padding: 12px;")
                                            .child((el::i().class("fas fa-check").attr("style", "margin-right: 6px;"), "Feedback sent! Thank you."))
                                            .into_any()
                                    } else {
                                        el::div().child((
                                            el::input()
                                                .class("form-control mb-2")
                                                .attr("placeholder", "Your name (optional)")
                                                .attr("dir", "ltr")
                                                .prop("value", move || feedback_name.get())
                                                .on(ev::input, move |e| feedback_name.set(event_target_value(&e))),
                                            el::input()
                                                .class("form-control mb-2")
                                                .attr("type", "email")
                                                .attr("placeholder", "Your email (optional, for reply)")
                                                .attr("dir", "ltr")
                                                .prop("value", move || feedback_email.get())
                                                .on(ev::input, move |e| feedback_email.set(event_target_value(&e))),
                                            el::textarea()
                                                .class("form-control mb-2")
                                                .attr("placeholder", "Your feedback...")
                                                .attr("rows", "3")
                                                .attr("dir", "ltr")
                                                .prop("value", move || feedback_text.get())
                                                .on(ev::input, move |e| feedback_text.set(event_target_value(&e))),
                                            el::button()
                                                .class(move || if feedback_text.get().trim().is_empty() { "btn btn-secondary btn-sm" } else { "btn btn-primary btn-sm" })
                                                .prop("disabled", move || feedback_text.get().trim().is_empty() || feedback_sending.get())
                                                .on(ev::click, on_send_feedback)
                                                .child((el::i().class("fas fa-paper-plane").attr("style", "margin-right: 6px;"), "Send")),
                                        )).into_any()
                                    }
                                },
                            )),
                        )),
                    )),
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
