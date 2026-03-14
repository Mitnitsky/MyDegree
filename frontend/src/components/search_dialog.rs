use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;
use crate::components::histogram_viewer::{HistogramViewer, HistogramViewerProps};

#[component]
pub fn SearchCourseDialog() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let search_text = RwSignal::new(String::new());
    let selected_index = RwSignal::new(Option::<usize>::None);
    let show_list = RwSignal::new(true);

    // true when showing fuzzy (edit-distance) results instead of exact matches
    let is_fuzzy = RwSignal::new(false);
    let fuzzy_query = RwSignal::new(String::new());

    let filtered_courses = Memo::new(move |_| {
        let query = search_text.get();
        if query.len() < 2 {
            is_fuzzy.set(false);
            return vec![];
        }
        state.course_db.with_value(|db| {
            let query_lower = query.to_lowercase();
            let is_numeric = query.chars().all(|c| c.is_ascii_digit());

            // Exact substring matches first
            let mut exact: Vec<(usize, String, f64)> = db.courses
                .iter()
                .enumerate()
                .filter(|(_, c)| {
                    c.name.contains(&query)
                        || c.number.contains(&query)
                        || c.full_name.to_lowercase().contains(&query_lower)
                })
                .map(|(i, c)| (i, c.full_name.clone(), c.points))
                .collect();

            if !exact.is_empty() || !is_numeric || query.len() < 3 {
                exact.truncate(50);
                is_fuzzy.set(false);
                return exact;
            }

            // Fuzzy fallback: numeric queries with edit distance ≤ 3 on course number
            let mut fuzzy: Vec<(usize, String, f64, u32)> = db.courses
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    let dist = edit_distance_bounded(&query, &c.number, 3)?;
                    Some((i, c.full_name.clone(), c.points, dist))
                })
                .collect();
            fuzzy.sort_by_key(|&(_, _, _, d)| d);
            fuzzy.truncate(50);
            is_fuzzy.set(!fuzzy.is_empty());
            fuzzy_query.set(query);
            fuzzy.into_iter().map(|(i, name, pts, _)| (i, name, pts)).collect()
        })
    });

    let selected_course_details = Memo::new(move |_| {
        selected_index.get().and_then(|idx| {
            state.course_db.with_value(|db| db.courses.get(idx).cloned())
        })
    });

    let on_close = move |_: web_sys::MouseEvent| {
        state.show_search_modal.set(false);
    };

    el::div().class("search-overlay")
        .on(ev::click, on_close)
        .child(
            el::div().class("search-dialog")
                .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                .child((
                    // Header (sticky)
                    el::div().class("d-flex justify-content-between align-items-center").child((
                        el::h5().class("mb-0").child("חיפוש קורסים"),
                        el::button().class("btn btn-sm btn-outline-secondary")
                            .on(ev::click, move |_| state.show_search_modal.set(false))
                            .child(el::i().class("fas fa-times")),
                    )),
                    // Scrollable body
                    el::div().child((

                    // Search input
                    el::div().class("position-relative mb-3").child((
                        el::input()
                            .attr("type", "text")
                            .class("form-control")
                            .attr("placeholder", "חפש לפי שם או מספר קורס...")
                            .attr("autofocus", "")
                            .prop("value", move || search_text.get())
                            .on(ev::input, move |e| {
                                search_text.set(event_target_value(&e));
                                selected_index.set(None);
                                show_list.set(true);
                            }),
                        // Results list
                        move || {
                            let courses = filtered_courses.get();
                            let visible = show_list.get();
                            let fuzzy = is_fuzzy.get();
                            let fq = fuzzy_query.get();
                            (!courses.is_empty() && visible).then(|| {
                                el::div().class("autocomplete-list").child((
                                    fuzzy.then(|| {
                                        el::div()
                                            .attr("style", "padding: 6px 10px; color: #856404; background: #fff3cd; border-bottom: 1px solid #eee; font-size: 0.9em;")
                                            .child("קורס לא נמצא. האם התכוונת ל?")
                                    }),
                                    courses.into_iter().map(|(idx, full_name, points)| {
                                        let display_name = if fuzzy {
                                            // Extract the course number from full_name (before ':')
                                            let number = full_name.split(':').next().unwrap_or("").trim();
                                            let rest = full_name.split_once(':').map(|(_, r)| r).unwrap_or("");
                                            let highlighted = diff_highlight(&fq, number);
                                            format!("{}:{}", highlighted, rest)
                                        } else {
                                            full_name.clone()
                                        };
                                        el::div().class("autocomplete-item")
                                            .attr("style", "display: flex; justify-content: space-between; align-items: center; padding: 6px 10px; cursor: pointer;")
                                            .on(ev::mousedown, move |e: web_sys::MouseEvent| {
                                                e.prevent_default();
                                                selected_index.set(Some(idx));
                                                show_list.set(false);
                                            })
                                            .child((
                                                el::span().inner_html(display_name),
                                                el::span().class("badge bg-secondary")
                                                    .attr("style", "margin-right: 8px; white-space: nowrap;")
                                                    .child(format!("{} נק'", points)),
                                            ))
                                    }).collect::<Vec<_>>(),
                                ))
                            })
                        },
                    )),

                    // Selected course details
                    move || {
                        selected_course_details.get().map(|course| {
                            let full_name = course.full_name.clone();
                            let points = course.points;
                            let prereqs = course.prerequisites.clone();
                            let linked = course.linked.clone();
                            let overlapping = course.overlapping.clone();
                            let identical = course.identical.clone();
                            let inclusive = course.inclusive.clone();
                            let including = course.including.clone();
                            let followed_by = course.followed_by.clone();

                            let show_prereq = RwSignal::new(false);
                            let show_followed = RwSignal::new(false);
                            let show_extra = RwSignal::new(false);
                            let show_histogram = RwSignal::new(false);
                            let course_number_for_hist = course.number.clone();

                            let has_prereqs = !prereqs.is_empty() && (prereqs.len() > 1 || !prereqs[0].is_empty());
                            let has_linked = !linked.is_empty();
                            let has_prereq_section = has_prereqs || has_linked;
                            let has_followed = !followed_by.is_empty();
                            let has_extra = !overlapping.is_empty() || !identical.is_empty() || !inclusive.is_empty() || !including.is_empty();

                            el::div().attr("style", "text-align: right; color: black; margin-top: 7px; min-height: 300px;").child((
                                // Course header card
                                el::div().class("card mb-3").child((
                                    el::div().class("card-header text-white text-center")
                                        .attr("style", "background-color: #343a40;")
                                        .child(full_name),
                                    el::div().class("card-body").child((
                                        // Points
                                        el::div().class("card mb-2").child((
                                            el::div().class("card-header").child(el::strong().child("נקודות")),
                                            el::div().class("card-body text-center")
                                                .attr("style", "padding: 5px 0 10px;")
                                                .child(points.to_string()),
                                        )),

                                        // Add course button
                                        el::div().class("d-flex justify-content-center mb-2").child(
                                            el::button().class("btn btn-primary")
                                                .on(ev::click, move |_| {
                                                    if let Some(idx) = selected_index.get() {
                                                        state.add_course_from_db(idx);
                                                        state.show_toast("הקורס נוסף בהצלחה");
                                                    }
                                                })
                                                .child("הוסף קורס"),
                                        ),

                                        // Toggle buttons
                                        el::div().class("d-flex flex-wrap justify-content-center gap-1 mb-2").child((
                                            Some(el::button()
                                                .class(move || if show_histogram.get() { "btn btn-secondary btn-sm" } else { "btn btn-outline-secondary btn-sm" })
                                                .attr("style", "margin: 5px;")
                                                .on(ev::click, move |_| show_histogram.update(|v| *v = !*v))
                                                .child(move || if show_histogram.get() { "היסטוגרמות ↑" } else { "היסטוגרמות ↓" })
                                            ),
                                            has_prereq_section.then(|| {
                                                el::button()
                                                    .class(move || if show_prereq.get() { "btn btn-secondary btn-sm" } else { "btn btn-outline-secondary btn-sm" })
                                                    .attr("style", "margin: 5px;")
                                                    .on(ev::click, move |_| show_prereq.update(|v| *v = !*v))
                                                    .child(move || if show_prereq.get() { "קורסי קדם/צמודים ↑" } else { "קורסי קדם/צמודים ↓" })
                                            }),
                                            has_followed.then(|| {
                                                el::button()
                                                    .class(move || if show_followed.get() { "btn btn-secondary btn-sm" } else { "btn btn-outline-secondary btn-sm" })
                                                    .attr("style", "margin: 5px;")
                                                    .on(ev::click, move |_| show_followed.update(|v| *v = !*v))
                                                    .child(move || if show_followed.get() { "קורסי המשך ↑" } else { "קורסי המשך ↓" })
                                            }),
                                            has_extra.then(|| {
                                                el::button()
                                                    .class(move || if show_extra.get() { "btn btn-secondary btn-sm" } else { "btn btn-outline-secondary btn-sm" })
                                                    .attr("style", "margin: 5px;")
                                                    .on(ev::click, move |_| show_extra.update(|v| *v = !*v))
                                                    .child(move || if show_extra.get() { "מידע נוסף ↑" } else { "מידע נוסף ↓" })
                                            }),
                                        )),

                                        // Histogram section
                                        {
                                            let num = course_number_for_hist.clone();
                                            move || show_histogram.get().then(|| {
                                                HistogramViewer(HistogramViewerProps { course_number: num.clone() })
                                            })
                                        },

                                        // Prerequisites / Linked section
                                        {
                                            let prereqs2 = prereqs.clone();
                                            let linked2 = linked.clone();
                                            move || show_prereq.get().then(|| {
                                                el::div().child((
                                                    if has_prereqs {
                                                        Some(relation_card("קורסי קדם", &prereqs2, true, selected_index, search_text, show_list, state))
                                                    } else { None },
                                                    if has_linked {
                                                        Some(relation_card("קורסים צמודים", &linked2.iter().map(|s| vec![s.clone()]).collect::<Vec<_>>(), false, selected_index, search_text, show_list, state))
                                                    } else { None },
                                                ))
                                            })
                                        },

                                        // Followed by section
                                        {
                                            let followed2 = followed_by.clone();
                                            move || show_followed.get().then(|| {
                                                relation_card("קורסי המשך:", &followed2.iter().map(|s| vec![s.clone()]).collect::<Vec<_>>(), false, selected_index, search_text, show_list, state)
                                            })
                                        },

                                        // Extra info section
                                        {
                                            let overlapping2 = overlapping.clone();
                                            let identical2 = identical.clone();
                                            let inclusive2 = inclusive.clone();
                                            let including2 = including.clone();
                                            move || show_extra.get().then(|| {
                                                el::div().child((
                                                    if !overlapping2.is_empty() {
                                                        Some(relation_card("קורסים ללא זיכוי נוסף", &overlapping2.iter().map(|s| vec![s.clone()]).collect::<Vec<_>>(), false, selected_index, search_text, show_list, state))
                                                    } else { None },
                                                    if !identical2.is_empty() {
                                                        Some(relation_card("קורסים זהים", &identical2.iter().map(|s| vec![s.clone()]).collect::<Vec<_>>(), false, selected_index, search_text, show_list, state))
                                                    } else { None },
                                                    if !inclusive2.is_empty() {
                                                        Some(relation_card("קורסים כלולים", &inclusive2.iter().map(|s| vec![s.clone()]).collect::<Vec<_>>(), false, selected_index, search_text, show_list, state))
                                                    } else { None },
                                                    if !including2.is_empty() {
                                                        Some(relation_card("קורסים מכילים", &including2.iter().map(|s| vec![s.clone()]).collect::<Vec<_>>(), false, selected_index, search_text, show_list, state))
                                                    } else { None },
                                                ))
                                            })
                                        },
                                    )),
                                )),
                            ))
                        })
                    },
                    )),
                )),
        )
}

/// Renders a card with a dark header and a list-group of course names.
/// `groups` is Vec<Vec<String>> — for prerequisites each inner vec is an OR-group,
/// for other relations each inner vec has one item.
fn relation_card(
    title: &str,
    groups: &[Vec<String>],
    show_or: bool,
    selected_index: RwSignal<Option<usize>>,
    search_text: RwSignal<String>,
    show_list: RwSignal<bool>,
    state: AppState,
) -> impl IntoView {
    let mut items: Vec<AnyView> = Vec::new();
    for (i, group) in groups.iter().enumerate() {
        for course in group {
            if !course.is_empty() {
                let c = course.clone();
                let c2 = course.clone();
                items.push(
                    el::a().class("list-group-item list-group-item-action")
                        .attr("href", "#")
                        .attr("style", "cursor: pointer;")
                        .on(ev::click, move |e: web_sys::MouseEvent| {
                            e.prevent_default();
                            // Find this course in the DB by name
                            let name = c2.clone();
                            let found = state.course_db.with_value(|db| {
                                db.courses.iter().enumerate()
                                    .find(|(_, c)| c.full_name == name || c.name == name)
                                    .map(|(idx, _)| idx)
                            });
                            if let Some(idx) = found {
                                selected_index.set(Some(idx));
                                search_text.set(name);
                                show_list.set(false);
                            }
                        })
                        .child(c)
                        .into_any()
                );
            }
        }
        if show_or && i < groups.len() - 1 && !items.is_empty() {
            items.push(
                el::p().attr("style", "margin-bottom: 2px; text-align: center;").child("או-").into_any()
            );
        }
    }

    el::div().class("card mb-2").child((
        el::div().class("card-header text-white")
            .attr("style", "background-color: #343a40;")
            .child(title.to_string()),
        el::div().class("list-group list-group-flush")
            .attr("style", "margin-bottom: 7px;")
            .child(items),
    ))
}

/// Levenshtein edit distance with early cutoff.
/// Returns `Some(distance)` if distance ≤ max_dist, otherwise `None`.
fn edit_distance_bounded(a: &str, b: &str, max_dist: u32) -> Option<u32> {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());

    if m.abs_diff(n) > max_dist as usize {
        return None;
    }

    let mut prev = vec![0u32; n + 1];
    let mut curr = vec![0u32; n + 1];
    for j in 0..=n {
        prev[j] = j as u32;
    }

    for i in 1..=m {
        curr[0] = i as u32;
        let mut row_min = curr[0];
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1)
                .min(curr[j - 1] + 1)
                .min(prev[j - 1] + cost);
            row_min = row_min.min(curr[j]);
        }
        if row_min > max_dist {
            return None;
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    let d = prev[n];
    if d <= max_dist { Some(d) } else { None }
}

/// Produce HTML highlighting the diff between query and target strings.
/// Uses Levenshtein backtrace: additions (lightgreen), removals (red), changes (yellow).
fn diff_highlight(query: &str, target: &str) -> String {
    let a: Vec<char> = query.chars().collect();
    let b: Vec<char> = target.chars().collect();
    let (m, n) = (a.len(), b.len());

    // Build full DP matrix
    let mut dp = vec![vec![0u32; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i as u32; }
    for j in 0..=n { dp[0][j] = j as u32; }
    for i in 1..=m {
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    // Backtrace to get edit operations
    #[derive(Clone)]
    enum Op { Match(char), Insert(char), Delete, Replace(char) }
    let mut ops = Vec::new();
    let (mut i, mut j) = (m, n);
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && a[i - 1] == b[j - 1] {
            ops.push(Op::Match(b[j - 1]));
            i -= 1; j -= 1;
        } else if i > 0 && j > 0 && dp[i][j] == dp[i - 1][j - 1] + 1 {
            // Substitution
            ops.push(Op::Replace(b[j - 1]));
            i -= 1; j -= 1;
        } else if j > 0 && dp[i][j] == dp[i][j - 1] + 1 {
            // Insertion in target
            ops.push(Op::Insert(b[j - 1]));
            j -= 1;
        } else {
            // Deletion from query
            ops.push(Op::Delete);
            i -= 1;
        }
    }
    ops.reverse();

    // Render HTML
    let mut html = String::new();
    for op in &ops {
        match op {
            Op::Match(c) => html.push(*c),
            Op::Insert(c) => {
                html.push_str("<span style='background:#90ee90'>");
                html.push(*c);
                html.push_str("</span>");
            }
            Op::Delete => {
                html.push_str("<span style='background:#f8d7da;text-decoration:line-through'>·</span>");
            }
            Op::Replace(c) => {
                html.push_str("<span style='background:#fff3cd'>");
                html.push(*c);
                html.push_str("</span>");
            }
        }
    }
    html
}
