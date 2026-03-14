use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;

#[component]
pub fn DegreeSummary() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let collapsed = RwSignal::new(true);

    el::div()
        .class("justify-content-md-center text-center text-muted")
        .attr("style", "margin-bottom: 60px; margin-right: 5px;")
        .child((
            el::button()
                .class("btn btn-outline-dark")
                .attr("style", "margin: 5px;")
                .on(ev::click, move |_| collapsed.update(|v| *v = !*v))
                .child(move || if collapsed.get() { "הראה סיכום תואר ↓" } else { "הסתר סיכום תואר ↑" }),
            move || {
                (!collapsed.get()).then(|| {
                    el::div().class("d-flex gap-3").child(
                        el::div().class("container justify-content-center mt-1").child(
                            el::div().class("row justify-content-center").child((
                                // Left card: degree summary
                                el::div().class("col").attr("style", "max-width: 590px; min-width: 480px;").child(
                                    degree_summary_card(state),
                                ),
                                // Right card: course type analysis
                                el::div().class("col").attr("style", "max-width: 590px; min-width: 480px;").child(
                                    course_types_card(state),
                                ),
                            )),
                        ),
                    )
                })
            },
        ))
}

fn degree_summary_card(state: AppState) -> impl IntoView {
    let degree_points = Memo::new(move |_| state.user.with(|u| u.degree_points));
    let degree_average = Memo::new(move |_| state.user.with(|u| u.degree_average));
    let degree_points_done = Memo::new(move |_| state.user.with(|u| u.degree_points_done));
    let degree_points_left = Memo::new(move |_| state.user.with(|u| u.degree_points_left));
    let degree_points_to_choose = Memo::new(move |_| state.user.with(|u| u.degree_points_to_choose));

    el::div().class("card shadow bg-white rounded h-100").child((
        el::div()
            .class("card-header summary-card-header text-white")
            .attr("style", "background-color: #343a40; font-weight: bold;")
            .child("סיכום תואר"),
        el::div().class("card-body").child(
            el::div().attr("style", "height: 100%; margin-top: 46px;").child((
                input_group_row(
                    "נקודות תואר",
                    move || degree_points.get().to_string(),
                    true,
                    Some(move |val: f64| state.set_degree_points(val)),
                ),
                input_group_row_readonly("ממוצע תואר", move || format!("{:.1}", degree_average.get())),
                input_group_row_readonly("נקודות בוצעו", move || format!("{:.1}", degree_points_done.get())),
                input_group_row_readonly("נקודות נותרו", move || format!("{:.1}", degree_points_left.get())),
                input_group_row_readonly("נותרו לשבץ", move || format!("{:.1}", degree_points_to_choose.get())),
            )),
        ),
    ))
}

fn course_types_card(state: AppState) -> impl IntoView {
    let english_exemption = Memo::new(move |_| state.user.with(|u| u.english_exemption));

    el::div().class("card shadow bg-white rounded h-100").child((
        el::div()
            .class("card-header summary-card-header text-white")
            .attr("style", "background-color: #343a40; font-weight: bold;")
            .child("ניתוח סוגי קורסים"),
        el::div().class("card-body").child((
            // Column headers — spacer + two headers matching the row layout
            el::div().class("input-group mb-2").child((
                el::span()
                    .class("input-group-text category-label")
                    .attr("style", "visibility: hidden;")
                    .child("\u{00a0}"),
                el::input()
                    .attr("type", "text")
                    .class("form-control")
                    .attr("disabled", "")
                    .attr("readonly", "")
                    .attr("value", "נותרו")
                    .attr("style", "background-color: aliceblue; text-align: center;"),
                el::input()
                    .attr("type", "text")
                    .class("form-control")
                    .attr("disabled", "")
                    .attr("readonly", "")
                    .attr("value", "מתוך")
                    .attr("style", "background-color: aliceblue; text-align: center;"),
            )),
            // Course type rows
            move || {
                state.course_types().into_iter().enumerate()
                    .filter(|(_, ct)| ct.name != "פטור" || ct.total_points > 0.0)
                    .map(|(i, ct)| {
                        let name = ct.name.clone();
                        let is_ptor = name.contains("פטור");
                        el::div().class("input-group mb-2").child((
                            // Category name label
                            el::span()
                                .class("input-group-text category-label")
                                .child(name),
                            // Points left (or total for פטור)
                            if is_ptor {
                                el::input()
                                    .attr("type", "number")
                                    .class("form-control degree-summary disabled-input")
                                    .attr("dir", "ltr")
                                    .attr("readonly", "")
                                    .attr("disabled", "")
                                    .prop("value", move || {
                                        state.course_types().get(i).map(|ct| ct.total_points.to_string()).unwrap_or_default()
                                    })
                                    .into_any()
                            } else {
                                el::input()
                                    .attr("type", "number")
                                    .class("form-control degree-summary disabled-input")
                                    .attr("dir", "ltr")
                                    .attr("readonly", "")
                                    .attr("disabled", "")
                                    .prop("value", move || {
                                        state.course_types().get(i).map(|ct| format!("{:.1}", ct.points_left)).unwrap_or_default()
                                    })
                                    .into_any()
                            },
                            // Points required (editable, not for פטור)
                            if is_ptor {
                                None
                            } else {
                                Some(el::input()
                                    .attr("type", "number")
                                    .class("form-control degree-summary degree-summary-number degree-input-field")
                                    .attr("dir", "ltr")
                                    .attr("min", "0")
                                    .attr("step", "0.5")
                                    .prop("value", move || {
                                        state.course_types().get(i).map(|ct| ct.points_required.to_string()).unwrap_or_default()
                                    })
                                    .on(ev::change, move |e| {
                                        let val: f64 = event_target_value(&e).parse().unwrap_or(0.0);
                                        state.set_course_type_required(i, val);
                                    }))
                            },
                        ))
                    }).collect::<Vec<_>>()
            },
            // English exemption checkbox
            el::div().class("input-group mb-2").child(
                el::div().class("form-check").child((
                    el::input()
                        .attr("type", "checkbox")
                        .class("form-check-input")
                        .attr("id", "english-exemption")
                        .prop("checked", move || english_exemption.get())
                        .on(ev::change, move |e| {
                            use wasm_bindgen::JsCast;
                            let checked = e.target()
                                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                                .map(|el| el.checked())
                                .unwrap_or(false);
                            state.set_english_exemption(checked);
                        }),
                    el::label()
                        .class("form-check-label")
                        .attr("for", "english-exemption")
                        .child("פטור מאנגלית"),
                )),
            ),
        )),
    ))
}

fn input_group_row(
    label: &'static str,
    value_fn: impl Fn() -> String + Send + Sync + 'static,
    editable: bool,
    on_change: Option<impl Fn(f64) + Send + Sync + 'static>,
) -> impl IntoView {
    let base = el::input()
        .attr("type", "number")
        .attr("dir", "ltr")
        .attr("step", "0.5")
        .attr("min", "0")
        .prop("value", value_fn);

    let input = if editable {
        let el = base.class("form-control degree-summary degree-summary-number degree-input-field");
        if let Some(cb) = on_change {
            el.on(ev::change, move |e| {
                let val: f64 = event_target_value(&e).parse().unwrap_or(0.0);
                cb(val);
            }).into_any()
        } else {
            el.into_any()
        }
    } else {
        base.class("form-control degree-summary degree-summary-number disabled-input")
            .attr("readonly", "")
            .attr("disabled", "")
            .into_any()
    };

    el::div().class("input-group mb-2").child((
        el::span()
            .class("input-group-text category-label")
            .child(label),
        input,
    ))
}

fn input_group_row_readonly(
    label: &'static str,
    value_fn: impl Fn() -> String + Send + Sync + 'static,
) -> impl IntoView {
    input_group_row(label, value_fn, false, None::<fn(f64)>)
}
