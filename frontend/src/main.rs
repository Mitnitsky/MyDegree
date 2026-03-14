mod app;
mod state;
mod firebase;
mod components;
pub mod histogram;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount::mount_to_body(App);
}
