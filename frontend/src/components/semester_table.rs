use leptos::prelude::*;
use leptos::html as el;
use leptos::ev;
use wasm_bindgen::JsCast;
use crate::state::AppState;

fn close_all_row_menus() {
    if let Some(win) = web_sys::window() {
        if let Ok(evt) = web_sys::CustomEvent::new("row-menu-close") {
            let _ = win.dispatch_event(&evt);
        }
    }
}

fn semester_table_header() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    el::thead().class("semester-thead").child(
        el::tr().attr("style", "font-family: Alef, serif;").child((
            el::th().attr("scope", "col").attr("style", "width: 30px;"),
            el::th().attr("scope", "col").child("קטגוריה"),
            el::th().attr("scope", "col")
                .attr("style", "cursor: pointer;")
                .attr("title", "לחץ למיון")
                .on(ev::click, move |_| state.sort_by_field("number"))
                .child("מספר קורס"),
            el::th().attr("scope", "col")
                .attr("style", "cursor: pointer;")
                .attr("title", "לחץ למיון")
                .on(ev::click, move |_| state.sort_by_field("name"))
                .child("שם קורס"),
            el::th().attr("scope", "col")
                .attr("style", "cursor: pointer;")
                .attr("title", "לחץ למיון")
                .on(ev::click, move |_| state.sort_by_field("points"))
                .child("נקודות"),
            el::th().attr("scope", "col")
                .attr("style", "cursor: pointer;")
                .attr("title", "לחץ למיון")
                .on(ev::click, move |_| state.sort_by_field("grade"))
                .child("ציון"),
            el::th().attr("scope", "col"),
        )),
    )
}

/// Helper: clear all drag preview transforms from course rows.
fn clear_drag_preview() {
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        if let Ok(rows) = doc.query_selector_all("tr[data-row-idx]") {
            for i in 0..rows.length() {
                if let Some(node) = rows.item(i) {
                    if let Some(el) = node.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = el.style().remove_property("transform");
                        let _ = el.class_list().remove_1("drag-shift-down");
                        let _ = el.class_list().remove_1("drag-shift-up");
                        let _ = el.class_list().remove_1("dragging");
                        let _ = el.class_list().remove_1("drag-fade-out");
                    }
                }
            }
        }
        // Also clear drop zone highlights
        if let Ok(zones) = doc.query_selector_all("tr.drop-zone-row") {
            for i in 0..zones.length() {
                if let Some(node) = zones.item(i) {
                    if let Some(el) = node.dyn_ref::<web_sys::HtmlElement>() {
                        let _ = el.class_list().remove_1("drag-over");
                    }
                }
            }
        }
    }
}

/// Spawn a ghost element at the cursor that fades out, mimicking the drag image disappearing.
fn spawn_fade_ghost(e: &web_sys::DragEvent) {
    let doc = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };
    // Read course name from the source row
    let src_idx = get_drag_src_idx();
    let course_name = src_idx.and_then(|idx| {
        let sel = format!("tr[data-row-idx='{}'] td:nth-child(4) input", idx);
        doc.query_selector(&sel).ok().flatten()
            .and_then(|el| el.dyn_ref::<web_sys::HtmlInputElement>().map(|i| i.value()))
    }).unwrap_or_default();
    let label = if course_name.is_empty() { "שורה".to_string() } else { course_name };

    let ghost = match doc.create_element("div") {
        Ok(el) => el,
        Err(_) => return,
    };
    let gh: &web_sys::HtmlElement = ghost.unchecked_ref();
    let is_dark = doc.document_element()
        .and_then(|el| el.get_attribute("data-theme"))
        .map(|t| t == "dark").unwrap_or(false);
    let bg = if is_dark { "#2d333b" } else { "#fff" };
    let border_col = if is_dark { "#539bf5" } else { "#0d6efd" };
    let text_col = if is_dark { "#adbac7" } else { "#212529" };
    gh.set_inner_html(&format!(
        "<span style='margin:0 8px;color:{}'><i class='fas fa-grip-lines' style='color:#adb5bd;margin-left:8px'></i>{}</span>",
        text_col, label
    ));
    let x = e.client_x() - 60;
    let y = e.client_y() - 18;
    let style = format!(
        "position:fixed;left:{}px;top:{}px;z-index:9999;\
         background:{};border:2px solid {};border-radius:6px;\
         padding:6px 16px;font-family:Alef,sans-serif;font-size:14px;\
         box-shadow:0 4px 12px rgba(0,0,0,0.15);direction:rtl;white-space:nowrap;\
         pointer-events:none;opacity:1;transform:scale(1) translateY(0);\
         transition:opacity 0.4s cubic-bezier(.36,.07,.19,.97),\
                    transform 0.4s cubic-bezier(.36,.07,.19,.97);",
        x, y, bg, border_col
    );
    let _ = gh.set_attribute("style", &style);
    let _ = doc.body().unwrap().append_child(gh);

    // Telegram-style: shrink, float up, fade out
    let ghost_ref = ghost.clone();
    let fade = wasm_bindgen::closure::Closure::once_into_js(move || {
        if let Some(el) = ghost_ref.dyn_ref::<web_sys::HtmlElement>() {
            let _ = el.style().set_property("opacity", "0");
            let _ = el.style().set_property("transform", "scale(0.3) translateY(-30px)");
        }
    });
    let _ = web_sys::window().unwrap().request_animation_frame(fade.unchecked_ref());

    // Remove after animation
    let ghost_ref2 = ghost.clone();
    let remove = wasm_bindgen::closure::Closure::once_into_js(move || {
        let _ = ghost_ref2.remove();
    });
    let _ = web_sys::window().unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(remove.unchecked_ref(), 400);
}

/// Helper: show preview is now handled purely by drop zone expansion.
fn show_drag_preview(_src_idx: usize, _drop_pos: usize) {}

/// Read the dragged source index from a JS window property.
fn get_drag_src_idx() -> Option<usize> {
    let window = web_sys::window()?;
    let val = js_sys::Reflect::get(&window, &"_dragSrcIdx".into()).ok()?;
    val.as_f64().map(|v| v as usize)
}

fn set_drag_src_idx(idx: Option<usize>) {
    if let Some(window) = web_sys::window() {
        match idx {
            Some(i) => { let _ = js_sys::Reflect::set(&window, &"_dragSrcIdx".into(), &(i as f64).into()); }
            None => { let _ = js_sys::Reflect::set(&window, &"_dragSrcIdx".into(), &wasm_bindgen::JsValue::NULL); }
        }
    }
}

/// FLIP animation: fade out source, record positions, apply change, animate rows.
fn animate_drop_and_apply(state: AppState, src_sem: usize, src_idx: usize, dst_sem: usize, drop_pos: usize) {
    // No-op: dropping at same position or adjacent
    if src_sem == dst_sem && (drop_pos == src_idx || drop_pos == src_idx + 1) {
        clear_drag_preview();
        set_drag_src_idx(None);
        return;
    }

    // Set flag so dragend doesn't interfere
    if let Some(window) = web_sys::window() {
        let _ = js_sys::Reflect::set(&window, &"_dragFading".into(), &true.into());
    }

    // Step 1: Fade out the dragged row
    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
        let selector = format!("tr[data-row-idx='{}']", src_idx);
        if let Ok(Some(el)) = doc.query_selector(&selector) {
            if let Some(html_el) = el.dyn_ref::<web_sys::HtmlElement>() {
                let _ = html_el.class_list().remove_1("dragging");
                let _ = html_el.class_list().add_1("drag-fade-out");
            }
        }
    }

    // Step 2: After fade, clear visuals → measure → apply → FLIP animate
    let fade_cb = wasm_bindgen::closure::Closure::once_into_js(move || {
        // Clear fade flag
        if let Some(window) = web_sys::window() {
            let _ = js_sys::Reflect::set(&window, &"_dragFading".into(), &false.into());
        }
        clear_drag_preview();
        set_drag_src_idx(None);

        // Force reflow after clearing
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Ok(Some(el)) = doc.query_selector("tbody") {
                if let Some(h) = el.dyn_ref::<web_sys::HtmlElement>() { let _ = h.offset_height(); }
            }
        }

        // FIRST: record old positions
        let mut old_tops: Vec<f64> = Vec::new();
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Ok(rows) = doc.query_selector_all("tr.course-row") {
                for i in 0..rows.length() {
                    if let Some(node) = rows.item(i) {
                        if let Some(el) = node.dyn_ref::<web_sys::HtmlElement>() {
                            old_tops.push(el.get_bounding_client_rect().top());
                        }
                    }
                }
            }
        }

        let count = old_tops.len();
        if count == 0 {
            state.move_course_drag(src_sem, src_idx, dst_sem, drop_pos);
            return;
        }

        // Compute new→old index mapping
        let effective = if src_sem == dst_sem && src_idx < drop_pos { drop_pos - 1 } else { drop_pos };
        let mut new_to_old: Vec<usize> = (0..count).collect();
        if src_idx < count {
            let removed = new_to_old.remove(src_idx);
            let insert_at = effective.min(new_to_old.len());
            new_to_old.insert(insert_at, removed);
        }

        // LAST: apply data change — Leptos re-renders
        state.move_course_drag(src_sem, src_idx, dst_sem, drop_pos);

        // INVERT + PLAY: double rAF to ensure DOM is flushed
        let raf1 = wasm_bindgen::closure::Closure::once_into_js(move || {
            let raf2 = wasm_bindgen::closure::Closure::once_into_js(move || {
                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                    if let Ok(rows) = doc.query_selector_all("tr.course-row") {
                        // INVERT: offset each row to its old position
                        for i in 0..rows.length() {
                            let idx = i as usize;
                            if let Some(node) = rows.item(i) {
                                if let Some(el) = node.dyn_ref::<web_sys::HtmlElement>() {
                                    if idx < new_to_old.len() {
                                        let old_idx = new_to_old[idx];
                                        if old_idx < old_tops.len() {
                                            let new_top = el.get_bounding_client_rect().top();
                                            let delta = old_tops[old_idx] - new_top;
                                            if delta.abs() > 1.0 {
                                                let _ = el.style().set_property("transform",
                                                    &format!("translateY({}px)", delta));
                                                let _ = el.style().set_property("transition", "none");
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Force reflow
                        if let Ok(Some(el)) = doc.query_selector("tbody") {
                            if let Some(h) = el.dyn_ref::<web_sys::HtmlElement>() { let _ = h.offset_height(); }
                        }

                        // PLAY: animate to final positions
                        for i in 0..rows.length() {
                            if let Some(node) = rows.item(i) {
                                if let Some(el) = node.dyn_ref::<web_sys::HtmlElement>() {
                                    let _ = el.style().set_property("transition", "transform 0.75s ease");
                                    let _ = el.style().set_property("transform", "none");
                                }
                            }
                        }

                        // Clean up after animation
                        let cleanup = wasm_bindgen::closure::Closure::once_into_js(move || {
                            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                                if let Ok(rows) = doc.query_selector_all("tr.course-row") {
                                    for i in 0..rows.length() {
                                        if let Some(node) = rows.item(i) {
                                            if let Some(el) = node.dyn_ref::<web_sys::HtmlElement>() {
                                                let _ = el.style().remove_property("transform");
                                                let _ = el.style().remove_property("transition");
                                            }
                                        }
                                    }
                                }
                            }
                        });
                        let _ = web_sys::window().unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                cleanup.unchecked_ref(), 800);
                    }
                }
            });
            let _ = web_sys::window().unwrap().request_animation_frame(raf2.unchecked_ref());
        });
        let _ = web_sys::window().unwrap().request_animation_frame(raf1.unchecked_ref());
    });
    let _ = web_sys::window().unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            fade_cb.unchecked_ref(), 250);
}

/// Thin invisible row that acts as a drop target between course rows.
fn drop_zone_row(position: usize) -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    el::tr()
        .class("drop-zone-row")
        .attr("data-drop-pos", position.to_string())
        .on(ev::dragover, move |e: web_sys::DragEvent| {
            // Skip adjacent zones (no-op drop positions)
            if let Some(src_idx) = get_drag_src_idx() {
                if position == src_idx || position == src_idx + 1 {
                    return;
                }
            }
            e.prevent_default();
            if let Some(target) = e.current_target() {
                if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = el.class_list().add_1("drag-over");
                }
            }
        })
        .on(ev::dragleave, move |e: web_sys::DragEvent| {
            if let Some(target) = e.current_target() {
                if let Some(el) = target.dyn_ref::<web_sys::HtmlElement>() {
                    let _ = el.class_list().remove_1("drag-over");
                }
            }
        })
        .on(ev::drop, move |e: web_sys::DragEvent| {
            e.prevent_default();
            // Create a fading ghost at cursor position
            spawn_fade_ghost(&e);
            if let Some(dt) = e.data_transfer() {
                if let Ok(data) = dt.get_data("text/plain") {
                    let parts: Vec<&str> = data.split(',').collect();
                    if parts.len() == 2 {
                        if let (Ok(src_sem), Ok(src_idx)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                            let dst_sem = state.user.with_untracked(|u| u.active_semester);
                            animate_drop_and_apply(state, src_sem, src_idx, dst_sem, position);
                        }
                    }
                }
            }
        })
        .child(
            el::td()
                .attr("colspan", "7")
                .attr("style", "padding: 0; height: 6px;"),
        )
}

fn semester_table_row(index: usize) -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let show_menu = RwSignal::new(false);

    // Close this menu when another row's menu opens
    Effect::new(move |_| {
        if let Some(win) = web_sys::window() {
            let cb = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                show_menu.set(false);
            });
            let _ = win.add_event_listener_with_callback("row-menu-close", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    });

    let course = Memo::new(move |_| {
        state.user.with(|u| {
            let sem_idx = u.active_semester;
            u.semesters.get(sem_idx).and_then(|s| s.courses.get(index).cloned())
        })
    });

    let course_types = Memo::new(move |_| state.course_types());

    let type_class = move || {
        course.with(|c| {
            c.as_ref().map(|c| format!("course-type-{}", c.course_type.min(5))).unwrap_or_default()
        })
    };

    let row = el::tr()
        .class(type_class)
        .class("course-row")
        .attr("data-row-idx", index.to_string())
        .child((
        // Drag handle
        el::td()
            .attr("draggable", "true")
            .attr("style", "width: 30px; cursor: grab; vertical-align: middle; text-align: center; padding-left: 2px;")
            .on(ev::dragstart, move |e: web_sys::DragEvent| {
                let sem_idx = state.user.with_untracked(|u| u.active_semester);
                let dt = match e.data_transfer() {
                    Some(d) => d,
                    None => return,
                };
                let _ = dt.set_effect_allowed("move");
                let _ = dt.set_data("text/plain", &format!("{},{}", sem_idx, index));
                set_drag_src_idx(Some(index));
                // Create a styled drag image clone
                if let Some(target) = e.target() {
                    if let Some(td) = target.dyn_ref::<web_sys::HtmlElement>() {
                        if let Some(tr) = td.closest("tr").ok().flatten() {
                            if let Some(tr_el) = tr.dyn_ref::<web_sys::HtmlElement>() {
                                let _ = tr_el.class_list().add_1("dragging");
                                // Create drag ghost: a styled div with course name
                                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                                    let ghost = doc.create_element("div").unwrap();
                                    let ghost_el: &web_sys::HtmlElement = ghost.unchecked_ref();
                                    let course_name = course.with_untracked(|c| {
                                        c.as_ref().map(|c| {
                                            if c.name.is_empty() { format!("שורה {}", index) } else { c.name.clone() }
                                        }).unwrap_or_else(|| format!("שורה {}", index))
                                    });
                                    let is_dark = doc.document_element()
                                        .and_then(|el| el.get_attribute("data-theme"))
                                        .map(|t| t == "dark").unwrap_or(false);
                                    let bg = if is_dark { "#2d333b" } else { "#fff" };
                                    let border_col = if is_dark { "#539bf5" } else { "#0d6efd" };
                                    let text_col = if is_dark { "#adbac7" } else { "#212529" };
                                    ghost_el.set_inner_html(&format!(
                                        "<span style='margin: 0 8px; color:{};'><i class='fas fa-grip-lines' style='color:#adb5bd;margin-left:8px;'></i>{}</span>",
                                        text_col, course_name
                                    ));
                                    let _ = ghost_el.style().set_property("position", "absolute");
                                    let _ = ghost_el.style().set_property("top", "-1000px");
                                    let _ = ghost_el.style().set_property("background", bg);
                                    let _ = ghost_el.style().set_property("border", &format!("2px solid {}", border_col));
                                    let _ = ghost_el.style().set_property("border-radius", "6px");
                                    let _ = ghost_el.style().set_property("padding", "6px 16px");
                                    let _ = ghost_el.style().set_property("font-family", "Alef, sans-serif");
                                    let _ = ghost_el.style().set_property("font-size", "14px");
                                    let _ = ghost_el.style().set_property("box-shadow", "0 4px 12px rgba(0,0,0,0.15)");
                                    let _ = ghost_el.style().set_property("direction", "rtl");
                                    let _ = ghost_el.style().set_property("white-space", "nowrap");
                                    let _ = doc.body().unwrap().append_child(ghost_el);
                                    let _ = dt.set_drag_image(ghost_el, 60, 18);
                                    // Remove ghost after browser captures it
                                    let ghost_clone = ghost.clone();
                                    let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
                                        let _ = ghost_clone.remove();
                                    });
                                    let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                                        cb.unchecked_ref(), 0
                                    );
                                }
                            }
                        }
                    }
                }
            })
            .on(ev::dragend, move |_: web_sys::DragEvent| {
                // Don't clear if animate_drop_and_apply is running (fade in progress)
                let fading = web_sys::window()
                    .and_then(|w| js_sys::Reflect::get(&w, &"_dragFading".into()).ok())
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                if !fading {
                    clear_drag_preview();
                    set_drag_src_idx(None);
                }
            })
            .child(
                el::i().class("fas fa-grip-lines").attr("style", "color: #adb5bd;"),
            ),
        // Category selector
        el::td().attr("style", "width: 15%;").child(
            el::select()
                .class("form-select form-select-sm")
                .attr("style", "text-align: center; text-align-last: center;")
                .on(ev::change, move |e| {
                    let val = event_target_value(&e);
                    state.update_course_field(index, "type", &val);
                })
                .child(move || {
                    let current_type = course.with(|c| c.as_ref().map(|c| c.course_type).unwrap_or(0));
                    course_types.get_untracked().into_iter().enumerate().map(|(i, ct)| {
                        let opt = el::option().attr("value", i.to_string());
                        if i == current_type {
                            opt.attr("selected", "").child(ct.name).into_any()
                        } else {
                            opt.child(ct.name).into_any()
                        }
                    }).collect::<Vec<_>>()
                }),
        ),
        // Course number
        el::td().attr("style", "width: 100px;").child(
            el::input()
                .attr("type", "text")
                .attr("inputmode", "numeric")
                .attr("pattern", "[0-9]*")
                .class("form-control form-control-sm text-center")
                .attr("style", "direction: ltr;")
                .prop("value", move || {
                    course.with(|c| c.as_ref().map(|c| c.number.clone()).unwrap_or_default())
                })
                .on(ev::input, move |e| {
                    let val: String = event_target_value(&e).chars().filter(|c| c.is_ascii_digit()).collect();
                    if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                        input.set_value(&val);
                    }
                })
                .on(ev::change, move |e| {
                    let val: String = event_target_value(&e).chars().filter(|c| c.is_ascii_digit()).collect();
                    state.update_course_field(index, "number", &val);
                }),
        ),
        // Course name
        el::td().child(
            el::input()
                .attr("type", "text")
                .class("form-control form-control-sm text-center")
                .prop("value", move || {
                    course.with(|c| c.as_ref().map(|c| c.name.clone()).unwrap_or_default())
                })
                .on(ev::change, move |e| {
                    state.update_course_field(index, "name", &event_target_value(&e));
                }),
        ),
        // Points
        el::td().attr("style", "width: 80px;").child(
            el::input()
                .attr("type", "number")
                .class("form-control form-control-sm text-center")
                .attr("style", "direction: ltr;")
                .attr("step", "0.5")
                .attr("min", "0")
                .attr("max", "20")
                .prop("value", move || {
                    course.with(|c| c.as_ref().map(|c| c.points.to_string()).unwrap_or_default())
                })
                .on(ev::change, move |e| {
                    state.update_course_field(index, "points", &event_target_value(&e));
                }),
        ),
        // Grade
        el::td().attr("style", "width: 80px;").child(
            move || {
                let is_binary = course.with(|c| c.as_ref().map(|c| c.binary).unwrap_or(false));
                if is_binary {
                    el::input()
                        .attr("type", "text")
                        .class("form-control form-control-sm text-center")
                        .attr("style", "color: green; cursor: default;")
                        .attr("readonly", "")
                        .prop("value", "✔")
                        .attr("title", "עובר בינארי")
                        .into_any()
                } else {
                    el::input()
                        .attr("type", "number")
                        .class("form-control form-control-sm text-center")
                        .attr("style", "direction: ltr;")
                        .attr("min", "0")
                        .attr("max", "100")
                        .prop("value", move || {
                            course.with(|c| c.as_ref().map(|c| (c.grade as i64).to_string()).unwrap_or_default())
                        })
                        .on(ev::change, move |e| {
                            state.update_course_field(index, "grade", &event_target_value(&e));
                        })
                        .into_any()
                }
            },
        ),
        // Actions dropdown
        el::td().class("text-center").attr("style", "width: 6%; vertical-align: middle;").child(
            el::div().class("dropdown").attr("style", "position: relative;").child((
                el::button()
                    .class("btn btn-outline-secondary")
                    .attr("style", "padding: 4px 10px;")
                    .on(ev::click, move |_| {
                        let was_open = show_menu.get_untracked();
                        close_all_row_menus();
                        if !was_open { show_menu.set(true); }
                    })
                    .child(el::i().class("fas fa-ellipsis-v")),
                move || {
                    show_menu.get().then(|| {
                        el::ul()
                            .class("dropdown-menu show")
                            .attr("style", "text-align: right; position: absolute; left: 0;")
                            .child((
                                // Histogram
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            let num = course.with(|c| c.as_ref().map(|c| c.number.clone()).unwrap_or_default());
                                            if !num.is_empty() {
                                                state.show_histogram_modal.set(Some(num));
                                            }
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-chart-bar").attr("style", "color: dodgerblue; margin-left: 5px;"), " הצג היסטוגרמות")),
                                ),
                                // Binary toggle
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            let is_binary = course.with(|c| c.as_ref().map(|c| c.binary).unwrap_or(false));
                                            state.update_course_field(index, "binary", if !is_binary { "true" } else { "false" });
                                            show_menu.set(false);
                                        })
                                        .child(move || {
                                            let is_binary = course.with(|c| c.as_ref().map(|c| c.binary).unwrap_or(false));
                                            if is_binary {
                                                (el::i().class("fas fa-ban").attr("style", "color: red; margin-left: 5px;").into_any(), " בטל עובר בינארי".into_any())
                                            } else {
                                                (el::i().class("fas fa-check").attr("style", "color: green; margin-left: 5px;").into_any(), " סמן עובר בינארי".into_any())
                                            }
                                        }),
                                ),
                                el::li().child(el::hr().class("dropdown-divider")),
                                // Clear row
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            state.update_course_field(index, "name", "");
                                            state.update_course_field(index, "number", "");
                                            state.update_course_field(index, "points", "0");
                                            state.update_course_field(index, "grade", "0");
                                            state.update_course_field(index, "type", "0");
                                            state.update_course_field(index, "binary", "false");
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-broom").attr("style", "color: burlywood; margin-left: 5px;"), " נקה שורה")),
                                ),
                                // Delete row
                                el::li().child(
                                    el::a().class("dropdown-item").attr("href", "#")
                                        .on(ev::click, move |e: web_sys::MouseEvent| {
                                            e.prevent_default();
                                            state.remove_course(index);
                                            show_menu.set(false);
                                        })
                                        .child((el::i().class("fas fa-trash").attr("style", "color: darkred; margin-left: 10px;"), " הסר שורה")),
                                ),
                            ))
                    })
                },
            )),
        ),
    ));

    row
}

#[component]
pub fn SemesterTable() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    el::div().child((
        el::div().class("semester-table-wrap").child(
            el::table().class("table table-sm semester-table").child((
                semester_table_header(),
                el::tbody().child(
                    move || {
                        let user = state.user.get();
                        let sem_idx = user.active_semester;
                        let count = user.semesters.get(sem_idx).map(|s| s.courses.len()).unwrap_or(0);
                        let mut rows: Vec<leptos::tachys::view::any_view::AnyView> = Vec::with_capacity(count * 2 + 1);
                        // Drop zone before first row
                        rows.push(drop_zone_row(0).into_any());
                        for i in 0..count {
                            rows.push(semester_table_row(i).into_any());
                            // Drop zone after each row
                            rows.push(drop_zone_row(i + 1).into_any());
                        }
                        rows
                    },
                ),
            )),
        ),
        el::div().class("d-flex justify-content-center").child(
            el::div().class("d-flex gap-2 mx-1").child((
                el::button().class("btn sem-btn sem-btn-add")
                    .on(ev::click, move |_| state.add_empty_course())
                    .child("הוספת שורה"),
                el::button().class("btn sem-btn sem-btn-search")
                    .on(ev::click, move |_| state.show_search_modal.set(true))
                    .child("חיפוש קורסים"),
            )),
        ),
    ))
}

#[component]
pub fn SemesterSummary() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();

    let average = Memo::new(move |_| {
        state.user.with(|u| u.semesters.get(u.active_semester).map(|s| s.average).unwrap_or(0.0))
    });
    let points = Memo::new(move |_| {
        state.user.with(|u| u.semesters.get(u.active_semester).map(|s| s.points).unwrap_or(0.0))
    });

    el::div().class("container").attr("style", "max-width: 300px;").child(
        el::div().class("card semester-summary-card").child((
            el::div().class("card-header text-center semester-summary-header")
                .child(el::p().attr("style", "margin-bottom: 0; font-weight: bold;").child("סיכום סמסטר")),
            el::div().class("card-body").child((
                el::div().class("row mb-2").child((
                    el::div().class("col-sm-3 align-self-center").attr("style", "margin: 4px;").child(
                        el::label().child("ממוצע:"),
                    ),
                    el::div().class("col-sm-8").child(
                        el::input()
                            .attr("type", "number")
                            .class("form-control text-center")
                            .attr("readonly", "")
                            .attr("style", "direction: ltr; cursor: default; margin: 0 12px;")
                            .prop("value", move || format!("{:.1}", average.get())),
                    ),
                )),
                el::div().class("row").child((
                    el::div().class("col-sm-3 align-self-center").attr("style", "margin: 4px;").child(
                        el::label().child("נקודות:"),
                    ),
                    el::div().class("col-sm-8").child(
                        el::input()
                            .attr("type", "number")
                            .class("form-control text-center")
                            .attr("readonly", "")
                            .attr("style", "direction: ltr; cursor: default; margin: 0 12px;")
                            .prop("value", move || format!("{:.1}", points.get())),
                    ),
                )),
            )),
        )),
    )
}
