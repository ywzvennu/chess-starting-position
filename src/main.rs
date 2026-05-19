mod app;
mod components;
mod state;
mod theme;

use app::App;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
    remove_loading_placeholder();
}

/// Drop the static `#loading` element placed in `index.html` once Leptos has
/// mounted the App. Both happen synchronously in `main`, so the browser does
/// not paint between mount and removal — no flicker.
fn remove_loading_placeholder() {
    if let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("loading"))
    {
        el.remove();
    }
}
