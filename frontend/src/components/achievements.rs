use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use std::collections::HashMap;
use crate::state::AppState;
use degree_core::achievements::{all_achievements, evaluate_achievements};

#[component]
pub fn Achievements() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    let dismiss = move || {
        gloo_timers::callback::Timeout::new(0, move || {
            state.show_achievements.set(false);
        })
        .forget();
    };
    let dismiss_overlay = dismiss.clone();
    let dismiss_x = dismiss.clone();

    el::div()
        .class("search-overlay")
        .on(ev::click, move |_| dismiss_overlay())
        .child(
            el::div()
                .class("search-dialog")
                .attr(
                    "style",
                    "max-width: 700px; min-width: unset; max-height: 85vh; overflow-y: auto;",
                )
                .on(ev::click, move |e: web_sys::MouseEvent| {
                    e.stop_propagation()
                })
                .child((
                    // Header
                    el::div()
                        .class("d-flex justify-content-between align-items-center")
                        .child((
                            el::h5()
                                .class("mb-0")
                                .attr("style", "color: var(--text-primary);")
                                .child((
                                    el::i().class("fas fa-trophy").attr("style", "margin-left: 6px;"),
                                    " הישגים",
                                )),
                            el::button()
                                .class("btn btn-sm btn-outline-secondary")
                                .on(ev::click, move |e: web_sys::MouseEvent| {
                                    e.stop_propagation();
                                    dismiss_x();
                                })
                                .child(el::i().class("fas fa-times")),
                        )),
                    // Body
                    el::div().child(move || {
                        let user = state.user.get();
                        let achievements = all_achievements();
                        let mut unlocked = evaluate_achievements(&user);

                        // Special: night_owl (#19, index 19)
                        let hour = js_sys::Date::new_0().get_hours();
                        if (1..=5).contains(&hour) {
                            unlocked[19] = true;
                        }

                        // Special: dual_degree (#22, index 22)
                        let dual = state.profiles.with(|p| p.profiles.len() >= 2);
                        if dual {
                            unlocked[22] = true;
                        }

                        // Special: collector (#24, index 24) — count other 24
                        let other_count = unlocked[..24].iter().filter(|&&v| v).count();
                        if other_count >= 20 {
                            unlocked[24] = true;
                        }

                        let total_unlocked = unlocked.iter().filter(|&&v| v).count();

                        // Check confetti
                        let should_confetti = check_and_update_seen_count(total_unlocked);

                        // Load/update timestamps
                        let mut timestamps = load_timestamps();
                        let today = today_str();
                        for (i, &is_u) in unlocked.iter().enumerate() {
                            let id = achievements[i].id;
                            if is_u && !timestamps.contains_key(id) {
                                timestamps.insert(id.to_string(), today.clone());
                            }
                        }
                        save_timestamps(&timestamps);

                        // Build (index, unlocked) pairs and sort: unlocked first (most recent→oldest), then locked
                        let mut indices: Vec<(usize, bool)> = (0..achievements.len())
                            .map(|i| (i, unlocked[i]))
                            .collect();
                        indices.sort_by(|a, b| {
                            match (a.1, b.1) {
                                (true, false) => std::cmp::Ordering::Less,
                                (false, true) => std::cmp::Ordering::Greater,
                                (true, true) => {
                                    // Both unlocked: most recent first (reverse chronological)
                                    let ta = timestamps.get(achievements[a.0].id).cloned().unwrap_or_default();
                                    let tb = timestamps.get(achievements[b.0].id).cloned().unwrap_or_default();
                                    // dd-mm-yyyy → yyyymmdd for sorting
                                    let ka = date_sort_key(&ta);
                                    let kb = date_sort_key(&tb);
                                    kb.cmp(&ka)
                                }
                                (false, false) => a.0.cmp(&b.0),
                            }
                        });

                        let cards: Vec<_> = indices
                            .iter()
                            .map(|&(i, is_u)| {
                                let ts = if is_u {
                                    timestamps.get(achievements[i].id).cloned()
                                } else {
                                    None
                                };
                                achievement_card(&achievements[i], is_u, ts)
                            })
                            .collect();

                        (
                            // Count display
                            el::div()
                                .attr(
                                    "style",
                                    "text-align: center; padding: 8px 0 16px; font-size: 1.1rem; color: var(--text-primary); font-weight: 600;",
                                )
                                .child(format!("{}/25 הישגים", total_unlocked)),
                            // Grid
                            el::div()
                                .attr(
                                    "style",
                                    "display: grid; grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); gap: 12px; padding: 0 4px 16px;",
                                )
                                .child(cards),
                            // Confetti container
                            should_confetti.then(confetti_animation),
                        )
                    }),
                )),
        )
}

fn achievement_card(
    ach: &degree_core::achievements::Achievement,
    unlocked: bool,
    timestamp: Option<String>,
) -> impl IntoView {
    if unlocked {
        // Unlocked: show everything + date
        el::div()
            .attr(
                "style",
                "border: 1px solid var(--border-color, #d0d7de); border-radius: 10px; padding: 14px 10px; text-align: center; background: var(--bg-card, #fff); transition: transform 0.15s, box-shadow 0.15s;",
            )
            .child((
                el::div()
                    .attr("style", "font-size: 2rem; margin-bottom: 4px;")
                    .child(ach.emoji.to_string()),
                el::div()
                    .attr(
                        "style",
                        "font-weight: 600; font-size: 0.85rem; color: var(--text-primary); margin-bottom: 2px;",
                    )
                    .child(ach.name.to_string()),
                el::div()
                    .attr(
                        "style",
                        "font-size: 0.75rem; color: var(--text-secondary);",
                    )
                    .child(ach.description.to_string()),
                el::div()
                    .attr(
                        "style",
                        "font-size: 0.65rem; color: var(--text-secondary); margin-top: 6px; opacity: 0.7; display: flex; align-items: center; justify-content: center; gap: 4px;",
                    )
                    .child(timestamp.map(|t| {
                        (
                            el::i()
                                .class("fas fa-calendar-alt")
                                .attr("style", "font-size: 0.6rem;"),
                            t,
                        )
                    })),
            ))
            .into_any()
    } else if ach.hidden {
        // Hidden + locked: mysterious card
        el::div()
            .attr(
                "style",
                "border: 1px solid var(--border-color, #d0d7de); border-radius: 10px; padding: 14px 10px; text-align: center; opacity: 0.4; background: var(--bg-secondary, #f6f8fa); transition: transform 0.15s;",
            )
            .child((
                el::div()
                    .attr("style", "font-size: 2rem; margin-bottom: 4px;")
                    .child("🔒"),
                el::div()
                    .attr(
                        "style",
                        "font-weight: 600; font-size: 0.85rem; color: var(--text-secondary);",
                    )
                    .child("הישג נסתר"),
                el::div()
                    .attr(
                        "style",
                        "font-size: 0.75rem; color: var(--text-secondary);",
                    )
                    .child("???"),
            ))
            .into_any()
    } else {
        // Visible + locked: show name but not description
        el::div()
            .attr(
                "style",
                "border: 1px solid var(--border-color, #d0d7de); border-radius: 10px; padding: 14px 10px; text-align: center; opacity: 0.55; background: var(--bg-secondary, #f6f8fa); transition: transform 0.15s;",
            )
            .child((
                el::div()
                    .attr("style", "font-size: 2rem; margin-bottom: 4px;")
                    .child("🔒"),
                el::div()
                    .attr(
                        "style",
                        "font-weight: 600; font-size: 0.85rem; color: var(--text-primary); margin-bottom: 2px;",
                    )
                    .child(ach.name.to_string()),
                el::div()
                    .attr(
                        "style",
                        "font-size: 0.75rem; color: var(--text-secondary);",
                    )
                    .child("???"),
            ))
            .into_any()
    }
}

fn check_and_update_seen_count(current: usize) -> bool {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().ok().flatten();
    let prev = storage
        .as_ref()
        .and_then(|s| s.get_item("achievements_seen_count").ok().flatten())
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    if let Some(s) = &storage {
        let _ = s.set_item("achievements_seen_count", &current.to_string());
    }

    current > prev && prev > 0
}

fn confetti_animation() -> impl IntoView {
    let colors = [
        "#f44336", "#e91e63", "#9c27b0", "#673ab7", "#3f51b5", "#2196f3", "#00bcd4", "#4caf50",
        "#ffeb3b", "#ff9800", "#ff5722",
    ];

    let pieces: Vec<_> = (0..30)
        .map(|i| {
            let color = colors[i % colors.len()];
            let left = (i as f64 * 3.33) % 100.0;
            let delay = (i as f64 * 0.1) % 1.5;
            let size = 6 + (i % 4) * 2;

            el::div().attr(
                "style",
                format!(
                    "position: fixed; top: -10px; left: {left}%; width: {size}px; height: {size}px; \
                     background: {color}; border-radius: 50%; pointer-events: none; z-index: 10000; \
                     animation: confetti-fall 2.5s ease-in {delay}s forwards;",
                ),
            )
        })
        .collect();

    // Inject keyframes once
    let style_el = el::style().child(
        "@keyframes confetti-fall { \
            0% { transform: translateY(0) rotate(0deg); opacity: 1; } \
            100% { transform: translateY(100vh) rotate(720deg); opacity: 0; } \
        }",
    );

    (style_el, pieces)
}

fn today_str() -> String {
    let d = js_sys::Date::new_0();
    let day = d.get_date();
    let month = d.get_month() + 1; // 0-indexed
    let year = d.get_full_year();
    format!("{:02}-{:02}-{}", day, month, year)
}

fn date_sort_key(date_str: &str) -> String {
    // dd-mm-yyyy → yyyymmdd
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 3 {
        format!("{}{}{}", parts[2], parts[1], parts[0])
    } else {
        String::new()
    }
}

fn load_timestamps() -> HashMap<String, String> {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().ok().flatten();
    storage
        .as_ref()
        .and_then(|s| s.get_item("achievement_timestamps").ok().flatten())
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

fn save_timestamps(ts: &HashMap<String, String>) {
    let window = web_sys::window().unwrap();
    if let Some(storage) = window.local_storage().ok().flatten() {
        if let Ok(json) = serde_json::to_string(ts) {
            let _ = storage.set_item("achievement_timestamps", &json);
        }
    }
}
