mod app;
mod state;
mod firebase;
mod components;
pub mod histogram;

use app::App;

fn main() {
    // Custom panic hook that forwards to Sentry via JS
    std::panic::set_hook(Box::new(|info| {
        let msg = info.to_string();
        // Also log to console
        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&msg));
        // Send to Sentry
        let js_code = format!(
            "if(typeof Sentry!=='undefined')Sentry.captureMessage('WASM Panic: '+{})",
            serde_json::to_string(&msg).unwrap_or_else(|_| format!("\"{}\"", msg.replace('"', "\\\""))),
        );
        let _ = js_sys::eval(&js_code);
    }));
    let _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount::mount_to_body(App);
}
