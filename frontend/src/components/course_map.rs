use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use crate::state::AppState;
use degree_core::{CourseStatus, course_status, CourseDBEntry, normalize_course_number};
use std::collections::HashSet;

#[component]
pub fn CourseMap() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let selected_faculty = RwSignal::new(String::new());
    let search_text = RwSignal::new(String::new());
    let selected_course_idx = RwSignal::<Option<usize>>::new(None);

    // Collect completed course numbers from user's semesters
    let completed_courses = move || {
        let user = state.user.get();
        let mut completed = HashSet::new();
        for sem in &user.semesters {
            for course in &sem.courses {
                if course.grade > 0.0 && !course.number.is_empty() {
                    completed.insert(normalize_course_number(&course.number));
                }
            }
        }
        completed
    };

    // Get faculties from the DB, sorted by course count
    let faculty_list = state.course_db.with_value(|db| {
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for c in &db.courses {
            if !c.faculty.is_empty() {
                *counts.entry(&c.faculty).or_default() += 1;
            }
        }
        let mut list: Vec<(String, usize)> = counts.into_iter()
            .map(|(name, count)| (name.to_string(), count))
            .collect();
        list.sort_by(|a, b| b.1.cmp(&a.1));
        list
    });

    let close = move |_| state.show_course_map.set(false);

    el::div().class("search-overlay")
        .on(ev::click, close)
        .child(
            el::div().class("search-dialog")
                .attr("style", "max-width: 900px; min-width: unset; max-height: 85vh;")
                .on(ev::click, move |e: web_sys::MouseEvent| e.stop_propagation())
                .child((
                    // Header
                    el::div().class("d-flex justify-content-between align-items-center").child((
                        el::h5().class("mb-0").child("מפת קורסים"),
                        el::button().class("btn btn-sm btn-outline-secondary")
                            .on(ev::click, close)
                            .child(el::i().class("fas fa-times")),
                    )),
                    // Body
                    el::div().attr("style", "display: flex; flex-direction: column; gap: 12px; overflow: hidden;").child((
                        // Search + faculty filter row
                        el::div().attr("style", "display: flex; gap: 8px; flex-wrap: wrap;").child((
                            el::input()
                                .class("form-control")
                                .attr("placeholder", "חיפוש קורס...")
                                .attr("style", "flex: 1; min-width: 200px;")
                                .prop("value", move || search_text.get())
                                .on(ev::input, move |e| {
                                    search_text.set(event_target_value(&e));
                                }),
                            el::select()
                                .class("form-select")
                                .attr("style", "width: auto; min-width: 180px;")
                                .prop("value", move || selected_faculty.get())
                                .on(ev::change, move |e| {
                                    selected_faculty.set(event_target_value(&e));
                                    selected_course_idx.set(None);
                                })
                                .child(
                                    std::iter::once(
                                        el::option().attr("value", "").child("כל הפקולטות").into_any()
                                    ).chain(
                                        faculty_list.iter().map(|(name, count)| {
                                            el::option()
                                                .attr("value", name.clone())
                                                .child(format!("{} ({})", name, count))
                                                .into_any()
                                        })
                                    ).collect::<Vec<_>>()
                                ),
                        )),
                        // Stats bar
                        move || {
                            let completed = completed_courses();
                            let faculty = selected_faculty.get();
                            let search = search_text.get().to_lowercase();

                            state.course_db.with_value(|db| {
                                let filtered: Vec<&CourseDBEntry> = db.courses.iter()
                                    .filter(|c| faculty.is_empty() || c.faculty == faculty)
                                    .filter(|c| {
                                        search.is_empty()
                                        || c.name.to_lowercase().contains(&search)
                                        || c.number.contains(&search)
                                    })
                                    .collect();

                                let done = filtered.iter().filter(|c| course_status(c, &completed) == CourseStatus::Done).count();
                                let overlap = filtered.iter().filter(|c| course_status(c, &completed) == CourseStatus::Overlap).count();
                                let available = filtered.iter().filter(|c| course_status(c, &completed) == CourseStatus::Available).count();
                                let blocked = filtered.iter().filter(|c| course_status(c, &completed) == CourseStatus::Blocked).count();

                                el::div().attr("style", "display: flex; gap: 16px; font-size: 0.85rem; color: var(--text-secondary); flex-wrap: wrap;").child((
                                    el::span().child(format!("סה״כ: {}", filtered.len())),
                                    el::span().attr("style", "color: #2ea043;").child(format!("✓ הושלם: {}", done)),
                                    el::span().attr("style", "color: #daaa3f;").child(format!("⊘ חופף: {}", overlap)),
                                    el::span().attr("style", "color: #539bf5;").child(format!("● זמין: {}", available)),
                                    el::span().attr("style", "color: var(--text-muted);").child(format!("○ חסום: {}", blocked)),
                                ))
                            })
                        },
                        // Course list + detail panel
                        el::div().attr("style", "display: flex; gap: 12px; overflow: hidden; flex: 1; min-height: 0;").child((
                            // Course list (scrollable)
                            move || {
                                let completed = completed_courses();
                                let faculty = selected_faculty.get();
                                let search = search_text.get().to_lowercase();
                                let selected = selected_course_idx.get();

                                state.course_db.with_value(|db| {
                                    let mut items: Vec<(usize, &CourseDBEntry, CourseStatus)> = db.courses.iter()
                                        .enumerate()
                                        .filter(|(_, c)| faculty.is_empty() || c.faculty == faculty)
                                        .filter(|(_, c)| {
                                            search.is_empty()
                                            || c.name.to_lowercase().contains(&search)
                                            || c.number.contains(&search)
                                        })
                                        .map(|(i, c)| (i, c, course_status(c, &completed)))
                                        .collect();

                                    // Sort: done first, then overlap, then available, then blocked
                                    items.sort_by_key(|(_, _, s)| match s {
                                        CourseStatus::Done => 0,
                                        CourseStatus::Overlap => 1,
                                        CourseStatus::Available => 2,
                                        CourseStatus::Blocked => 3,
                                    });

                                    el::div()
                                        .attr("style", "flex: 1; overflow-y: auto; border: 1px solid var(--border-color, #dee2e6); border-radius: 6px;")
                                        .child(
                                            items.into_iter().map(|(idx, course, status)| {
                                                let is_selected = selected == Some(idx);
                                                let (icon, color) = match status {
                                                    CourseStatus::Done => ("✓", "#2ea043"),
                                                    CourseStatus::Overlap => ("⊘", "#daaa3f"),
                                                    CourseStatus::Available => ("●", "#539bf5"),
                                                    CourseStatus::Blocked => ("○", "var(--text-muted, #999)"),
                                                };
                                                let bg = if is_selected { "var(--bg-input, #e9ecef)" } else { "transparent" };

                                                el::div()
                                                    .attr("style", format!(
                                                        "padding: 8px 12px; cursor: pointer; border-bottom: 1px solid var(--border-color, #eee); background: {}; display: flex; align-items: center; gap: 8px;",
                                                        bg
                                                    ))
                                                    .on(ev::click, move |_| selected_course_idx.set(Some(idx)))
                                                    .child((
                                                        el::span().attr("style", format!("color: {}; font-size: 0.9rem; min-width: 14px;", color)).child(icon),
                                                        el::div().attr("style", "flex: 1; min-width: 0;").child((
                                                            el::div().attr("style", "font-size: 0.85rem; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;")
                                                                .child(course.name.clone()),
                                                            el::div().attr("style", "font-size: 0.75rem; color: var(--text-secondary);")
                                                                .child(format!("{} · {} נ״ז", course.number, course.points)),
                                                        )),
                                                    ))
                                            }).collect::<Vec<_>>()
                                        )
                                })
                            },
                            // Detail panel
                            move || {
                                let completed = completed_courses();
                                selected_course_idx.get().map(|idx| {
                                    state.course_db.with_value(|db| {
                                        let course = &db.courses[idx];
                                        let status = course_status(course, &completed);
                                        let (status_text, status_color) = match status {
                                            CourseStatus::Done => ("הושלם", "#2ea043"),
                                            CourseStatus::Overlap => ("חופף לקורס שהושלם", "#daaa3f"),
                                            CourseStatus::Available => ("זמין ללקיחה", "#539bf5"),
                                            CourseStatus::Blocked => ("חסום (חסרים קורסי קדם)", "var(--text-muted)"),
                                        };
                                        let faculty_name = if course.faculty.is_empty() { "אחר" } else { &course.faculty };

                                        el::div()
                                            .attr("style", "width: 320px; overflow-y: auto; border: 1px solid var(--border-color, #dee2e6); border-radius: 6px; padding: 16px;")
                                            .child((
                                                // Course title
                                                el::h6().attr("style", "color: var(--text-primary); margin-bottom: 4px;")
                                                    .child(course.name.clone()),
                                                el::div().attr("style", "font-size: 0.8rem; color: var(--text-secondary); margin-bottom: 8px;")
                                                    .child(format!("{} · {} נ״ז · {}", course.number, course.points, faculty_name)),
                                                // Status badge
                                                el::div().attr("style", format!(
                                                    "display: inline-block; padding: 2px 8px; border-radius: 4px; font-size: 0.8rem; margin-bottom: 12px; color: #fff; background: {};",
                                                    status_color
                                                )).child(status_text),
                                                // Prerequisites
                                                (!course.prerequisites.is_empty() && course.prerequisites.iter().any(|g| !g.is_empty())).then(|| {
                                                    el::div().attr("style", "margin-bottom: 12px;").child((
                                                        el::div().attr("style", "font-size: 0.8rem; font-weight: bold; color: var(--text-primary); margin-bottom: 4px;")
                                                            .child("קורסי קדם:"),
                                                        course.prerequisites.iter().enumerate().filter(|(_, g)| !g.is_empty()).map(|(i, group)| {
                                                            let is_last = i == course.prerequisites.iter().filter(|g| !g.is_empty()).count() - 1;
                                                            el::div().child((
                                                                group.iter().map(|prereq| {
                                                                    let num = prereq.split(':').next().unwrap_or("").trim().to_string();
                                                                    let is_done = completed.contains(&num) || completed.contains(&normalize_course_number(&num));
                                                                    let color = if is_done { "#2ea043" } else { "var(--text-secondary)" };
                                                                    let prefix = if is_done { "✓ " } else { "○ " };
                                                                    el::div().attr("style", format!("font-size: 0.78rem; color: {}; padding: 1px 0;", color))
                                                                        .child(format!("{}{}", prefix, prereq))
                                                                }).collect::<Vec<_>>(),
                                                                if !is_last {
                                                                    Some(el::div().attr("style", "font-size: 0.7rem; color: var(--text-muted); text-align: center; margin: 2px 0;").child("— או —"))
                                                                } else {
                                                                    None
                                                                },
                                                            ))
                                                        }).collect::<Vec<_>>(),
                                                    ))
                                                }),
                                                // Followed by
                                                (!course.followed_by.is_empty()).then(|| {
                                                    el::div().attr("style", "margin-bottom: 12px;").child((
                                                        el::div().attr("style", "font-size: 0.8rem; font-weight: bold; color: var(--text-primary); margin-bottom: 4px;")
                                                            .child("קורסי המשך:"),
                                                        course.followed_by.iter().map(|fb| {
                                                            let num = fb.split(':').next().unwrap_or("").trim().to_string();
                                                            let is_done = completed.contains(&num) || completed.contains(&normalize_course_number(&num));
                                                            let color = if is_done { "#2ea043" } else { "var(--text-secondary)" };
                                                            let prefix = if is_done { "✓ " } else { "→ " };
                                                            el::div().attr("style", format!("font-size: 0.78rem; color: {}; padding: 1px 0;", color))
                                                                .child(format!("{}{}", prefix, fb))
                                                        }).collect::<Vec<_>>(),
                                                    ))
                                                }),
                                                // Linked courses
                                                (!course.linked.is_empty()).then(|| {
                                                    el::div().attr("style", "margin-bottom: 8px;").child((
                                                        el::div().attr("style", "font-size: 0.8rem; font-weight: bold; color: var(--text-primary); margin-bottom: 4px;")
                                                            .child("קורסים צמודים:"),
                                                        course.linked.iter().map(|l| {
                                                            el::div().attr("style", "font-size: 0.78rem; color: var(--text-secondary); padding: 1px 0;")
                                                                .child(format!("⟷ {}", l))
                                                        }).collect::<Vec<_>>(),
                                                    ))
                                                }),
                                                // View histogram button
                                                el::button()
                                                    .class("btn btn-sm btn-outline-primary mt-2")
                                                    .attr("style", "width: 100%;")
                                                    .on(ev::click, {
                                                        let number = course.number.clone();
                                                        move |_| {
                                                            state.show_course_map.set(false);
                                                            state.show_histogram_modal.set(Some(number.clone()));
                                                        }
                                                    })
                                                    .child("צפה בהיסטוגרמה"),
                                            ))
                                    })
                                })
                            },
                        )),
                    )),
                ))
        )
}
