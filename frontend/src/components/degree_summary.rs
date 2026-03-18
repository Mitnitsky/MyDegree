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
            el::div()
                .class("summary-collapse-wrapper")
                .class(("summary-collapsed", move || collapsed.get()))
                .child(
                    el::div().class("d-flex gap-3").child(
                        el::div().class("container justify-content-center mt-1").child(
                            el::div().class("row justify-content-center").child((
                                el::div().class("col").attr("style", "max-width: 590px; min-width: 480px;").child(
                                    degree_summary_card(state),
                                ),
                                el::div().class("col").attr("style", "max-width: 590px; min-width: 480px;").child(
                                    course_types_card(state),
                                ),
                            )),
                        ),
                    ),
                ),
        ))
}

fn degree_summary_card(state: AppState) -> impl IntoView {
    let degree_points = Memo::new(move |_| state.user.with(|u| u.degree_points));
    let degree_average = Memo::new(move |_| state.user.with(|u| u.degree_average));
    let degree_points_done = Memo::new(move |_| state.user.with(|u| u.degree_points_done));
    let degree_points_left = Memo::new(move |_| state.user.with(|u| u.degree_points_left));
    let degree_points_to_choose = Memo::new(move |_| state.user.with(|u| u.degree_points_to_choose));

    el::div().class("card shadow rounded h-100 summary-card").child((
        el::div()
            .class("card-header summary-card-header text-white")
            .child("סיכום תואר"),
        el::div().class("card-body").child(
            el::div().attr("style", "height: 100%;").child((
                progress_ring(degree_points_done, degree_points),
                input_group_row(
                    "נקודות תואר",
                    move || degree_points.get().to_string(),
                    true,
                    Some(move |val: f64| state.set_degree_points(val)),
                ),
                input_group_row_readonly("ממוצע תואר", move || format!("{:.1}", degree_average.get())),
                gpa_sparkline(state, degree_average),
                input_group_row_readonly("נקודות בוצעו", move || format!("{:.1}", degree_points_done.get())),
                input_group_row_readonly("נקודות נותרו", move || format!("{:.1}", degree_points_left.get())),
                input_group_row_readonly("נותרו לשבץ", move || format!("{:.1}", degree_points_to_choose.get())),
            )),
        ),
    ))
}

fn course_types_card(state: AppState) -> impl IntoView {
    let english_exemption = Memo::new(move |_| state.user.with(|u| u.english_exemption));

    el::div().class("card shadow rounded h-100 summary-card").child((
        el::div()
            .class("card-header summary-card-header text-white")
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
                    .attr("style", "text-align: center;")
                    .class("form-control ct-column-header"),
                el::input()
                    .attr("type", "text")
                    .class("form-control")
                    .attr("disabled", "")
                    .attr("readonly", "")
                    .attr("value", "מתוך")
                    .attr("style", "text-align: center;")
                    .class("form-control ct-column-header"),
            )),
            // Course type rows
            move || {
                state.course_types().into_iter().enumerate()
                    .filter(|(_, ct)| ct.name != "פטור" || ct.total_points > 0.0)
                    .map(|(i, ct)| {
                        let name = ct.name.clone();
                        let is_ptor = name.contains("פטור");
                        let dot_class = format!("ct-dot ct-dot-{}", i.min(5));
                        el::div().class("input-group mb-2").child((
                            // Category name label with color dot
                            el::span()
                                .class("input-group-text category-label")
                                .child((el::span().class(dot_class), name)),
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

fn progress_ring(
    done: Memo<f64>,
    total: Memo<f64>,
) -> impl IntoView {
    let radius = 54.0_f64;
    let circumference = 2.0 * std::f64::consts::PI * radius;

    el::div()
        .class("progress-ring-wrap")
        .attr("style", "display: flex; flex-direction: column; align-items: center; margin-bottom: 16px;")
        .child(move || {
            let d = done.get();
            let t = total.get();
            let pct = if t > 0.0 { (d / t * 100.0).min(100.0) } else { 0.0 };
            let offset = circumference - (pct / 100.0) * circumference;

            let is_dark = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|doc| doc.document_element())
                .and_then(|el| el.get_attribute("data-theme"))
                .map(|t| t == "dark").unwrap_or(false);

            let color = if is_dark {
                if pct >= 100.0 { "#57ab5a" } else if pct >= 66.0 { "#539bf5" } else if pct >= 33.0 { "#c69026" } else { "#e5534b" }
            } else {
                if pct >= 100.0 { "#28a745" } else if pct >= 66.0 { "#0d6efd" } else if pct >= 33.0 { "#fd7e14" } else { "#dc3545" }
            };
            let track = if is_dark { "#373e47" } else { "#e9ecef" };

            let svg = format!(
                "<svg width='130' height='130' viewBox='0 0 130 130'>\
                <circle cx='65' cy='65' r='{radius}' fill='none' stroke='{track}' stroke-width='10'/>\
                <circle cx='65' cy='65' r='{radius}' fill='none' stroke='{color}' stroke-width='10' \
                stroke-linecap='round' stroke-dasharray='{circumference}' stroke-dashoffset='{offset}' \
                transform='rotate(-90 65 65)' style='transition: stroke-dashoffset 0.8s ease, stroke 0.5s ease;'/>\
                <text x='65' y='60' text-anchor='middle' font-size='24' font-weight='bold' class='ring-text-main'>{pct:.0}%</text>\
                <text x='65' y='80' text-anchor='middle' font-size='11' class='ring-text-sub'>{d:.1} / {t:.0} נ״ז</text>\
                </svg>",
            );

            el::div().inner_html(svg)
        })
}

fn gpa_sparkline(state: AppState, cumulative_avg: Memo<f64>) -> impl IntoView {
    el::div()
        .attr("style", "display: flex; justify-content: center; margin-bottom: 12px;")
        .child(move || {
            let averages: Vec<f64> = state.user.with(|u| {
                u.semesters.iter().map(|s| s.average).filter(|&a| a > 0.0).collect()
            });
            if averages.len() < 2 { return el::div().into_any(); }

            let cum = cumulative_avg.get();
            let w = 220.0_f64;
            let h = 50.0_f64;
            let pad = 8.0_f64;
            let min_v = averages.iter().cloned().fold(f64::MAX, f64::min).min(cum) - 5.0;
            let max_v = averages.iter().cloned().fold(f64::MIN, f64::max).max(cum) + 5.0;
            let range = (max_v - min_v).max(1.0);
            let n = averages.len();

            let points: Vec<String> = averages.iter().enumerate().map(|(i, &a)| {
                let x = pad + (i as f64 / (n - 1) as f64) * (w - 2.0 * pad);
                let y = pad + (1.0 - (a - min_v) / range) * (h - 2.0 * pad);
                format!("{x:.1},{y:.1}")
            }).collect();

            let polyline_pts = points.join(" ");
            let last = averages.last().unwrap_or(&0.0);
            let first = averages.first().unwrap_or(&0.0);
            let color = if last >= first { "#28a745" } else { "#dc3545" };

            // Cumulative average dashed line
            let cum_y = pad + (1.0 - (cum - min_v) / range) * (h - 2.0 * pad);

            // Dots
            let dots: String = averages.iter().enumerate().map(|(i, &a)| {
                let x = pad + (i as f64 / (n - 1) as f64) * (w - 2.0 * pad);
                let y = pad + (1.0 - (a - min_v) / range) * (h - 2.0 * pad);
                format!("<circle cx='{x:.1}' cy='{y:.1}' r='3' fill='{color}'><title>סמסטר {}: {a:.1}</title></circle>", i + 1)
            }).collect();

            // Gradient fill area
            let first_x = pad;
            let last_x = pad + (w - 2.0 * pad);
            let area_pts = format!("{first_x:.1},{h:.1} {polyline_pts} {last_x:.1},{h:.1}");

            let svg = format!(
                "<svg width='{w:.0}' height='{h:.0}' viewBox='0 0 {w:.0} {h:.0}'>\
                <defs><linearGradient id='spark-fill' x1='0' y1='0' x2='0' y2='1'>\
                <stop offset='0%' stop-color='{color}' stop-opacity='0.2'/>\
                <stop offset='100%' stop-color='{color}' stop-opacity='0.02'/>\
                </linearGradient></defs>\
                <polygon points='{area_pts}' fill='url(#spark-fill)'/>\
                <line x1='{pad:.1}' y1='{cum_y:.1}' x2='{last_x:.1}' y2='{cum_y:.1}' \
                stroke='#6c757d' stroke-width='1' stroke-dasharray='4,3' opacity='0.6'/>\
                <polyline points='{polyline_pts}' fill='none' stroke='{color}' stroke-width='2' stroke-linejoin='round' stroke-linecap='round'/>\
                {dots}</svg>",
            );

            el::div().inner_html(svg).into_any()
        })
}

pub fn grade_calc_modal(state: AppState, show: RwSignal<bool>) -> impl IntoView {
    let target_gpa = RwSignal::new(String::from("85"));
    let future_points = RwSignal::new(String::from("20"));

    move || {
        show.get().then(|| {
            let cur_avg = state.user.with(|u| u.degree_average);
            let graded_pts = state.user.with(|u| u.degree_graded_points);

            let dismiss = move || {
                gloo_timers::callback::Timeout::new(0, move || show.set(false)).forget();
            };
            let dismiss2 = dismiss.clone();

            el::div().class("search-overlay")
                .on(ev::click, move |_| dismiss())
                .child(
                    el::div().class("search-dialog")
                        .attr("style", "max-width: 440px;")
                        .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                        .child((
                            // Header
                            el::div().class("d-flex justify-content-between align-items-center").child((
                                el::h5().class("mb-0").child(
                                    "תכנון ממוצע",
                                ),
                                el::button().class("btn btn-sm btn-outline-secondary")
                                    .on(ev::click, move |_| dismiss2())
                                    .child(el::i().class("fas fa-times")),
                            )),
                            // Body
                            el::div().child((
                                // Current stats (readonly)
                                el::div().class("mb-3").child((
                                    el::label().class("form-label fw-bold").child("מצב נוכחי"),
                                    el::div().class("d-flex gap-3").child((
                                        el::div().class("flex-fill").child((
                                            el::small().class("text-muted").child("ממוצע נוכחי"),
                                            el::div().class("form-control grade-calc-readonly")
                                                .child(format!("{:.1}", cur_avg)),
                                        )),
                                        el::div().class("flex-fill").child((
                                            el::small().class("text-muted").child("נ״ז עם ציון"),
                                            el::div().class("form-control grade-calc-readonly")
                                                .child(format!("{:.1}", graded_pts)),
                                        )),
                                    )),
                                )),
                                // Target GPA input
                                el::div().class("mb-3").child((
                                    el::label().class("form-label fw-bold")
                                        .child("ממוצע מטרה"),
                                    el::input()
                                        .class("form-control")
                                        .attr("type", "number")
                                        .attr("min", "0")
                                        .attr("max", "100")
                                        .attr("step", "0.1")
                                        .prop("value", move || target_gpa.get())
                                        .on(ev::input, move |e| {
                                            target_gpa.set(event_target_value(&e));
                                        }),
                                    // Slider
                                    el::input()
                                        .class("form-range mt-1")
                                        .attr("type", "range")
                                        .attr("min", "55")
                                        .attr("max", "100")
                                        .attr("step", "0.5")
                                        .prop("value", move || target_gpa.get())
                                        .on(ev::input, move |e| {
                                            target_gpa.set(event_target_value(&e));
                                        }),
                                )),
                                // Future points input
                                el::div().class("mb-3").child((
                                    el::label().class("form-label fw-bold")
                                        .child("נקודות עתידיות (נ״ז)"),
                                    el::input()
                                        .class("form-control")
                                        .attr("type", "number")
                                        .attr("min", "0.5")
                                        .attr("max", "200")
                                        .attr("step", "0.5")
                                        .prop("value", move || future_points.get())
                                        .on(ev::input, move |e| {
                                            future_points.set(event_target_value(&e));
                                        }),
                                )),
                                // Result
                                move || {
                                    let target: f64 = target_gpa.get().parse().unwrap_or(0.0);
                                    let future: f64 = future_points.get().parse().unwrap_or(0.0);
                                    let cur_avg = state.user.with(|u| u.degree_average);
                                    let graded_pts = state.user.with(|u| u.degree_graded_points);

                                    if future <= 0.0 {
                                        return el::div().class("alert grade-calc-result mt-3 text-center")
                                            .child("הזן נקודות עתידיות")
                                            .into_any();
                                    }

                                    let current_sum = cur_avg * graded_pts;
                                    let total_pts = graded_pts + future;
                                    let needed_sum = target * total_pts;
                                    let required = (needed_sum - current_sum) / future;

                                    let (msg, cls) = if required > 100.0 {
                                        ("לא ניתן להשיג — נדרש ממוצע מעל 100".to_string(), "alert-danger")
                                    } else if required < 0.0 {
                                        ("המטרה כבר הושגה! 🎉".to_string(), "alert-success")
                                    } else {
                                        (format!("ממוצע נדרש בקורסים עתידיים: {:.1}", required), 
                                         if required <= 70.0 { "alert-success" } 
                                         else if required <= 85.0 { "alert-info" } 
                                         else if required <= 95.0 { "alert-warning" } 
                                         else { "alert-danger" })
                                    };

                                    el::div()
                                        .class(format!("alert {} grade-calc-result mt-3 text-center fw-bold", cls))
                                        .child(msg)
                                        .into_any()
                                },
                            )),
                        )),
                )
        })
    }
}
