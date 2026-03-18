use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
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
                                .child("🏆 הישגים"),
                            el::button()
                                .class("btn btn-sm btn-outline-secondary")
                                .on(ev::click, move |_| dismiss_x())
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

                        let cards: Vec<_> = achievements
                            .iter()
                            .zip(unlocked.iter())
                            .map(|(ach, &is_unlocked)| achievement_card(ach, is_unlocked))
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
) -> impl IntoView {
    let emoji = if unlocked { ach.emoji } else { "🔒" };
    let desc = if unlocked {
        ach.description.to_string()
    } else {
        "???".to_string()
    };

    let opacity = if unlocked { "1" } else { "0.45" };
    let bg = if unlocked {
        "var(--bg-card, #fff)"
    } else {
        "var(--bg-secondary, #f6f8fa)"
    };

    el::div()
        .attr(
            "style",
            format!(
                "border: 1px solid var(--border-color, #d0d7de); border-radius: 10px; padding: 14px 10px; text-align: center; opacity: {}; background: {}; transition: transform 0.15s, box-shadow 0.15s;",
                opacity, bg
            ),
        )
        .child((
            el::div()
                .attr("style", "font-size: 2rem; margin-bottom: 4px;")
                .child(emoji.to_string()),
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
                .child(desc),
        ))
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
