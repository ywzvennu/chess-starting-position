mod app;
mod components;
mod pieces;
mod state;
mod theme;

use app::App;
use leptos::prelude::*;

fn main() {
    // Replace `console_error_panic_hook::set_once` so we can also show a
    // user-visible overlay alongside the console log.
    std::panic::set_hook(Box::new(|info| {
        console_error_panic_hook::hook(info);
        show_panic_overlay();
    }));

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

/// Insert a fixed-position overlay describing that an unrecoverable error
/// occurred and offering a reload. Called from the panic hook so a WASM
/// panic doesn't just leave a blank page. No-op if an overlay is already
/// present, which guards against a panic inside the hook itself looping.
fn show_panic_overlay() {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    if doc.get_element_by_id("panic-overlay").is_some() {
        return;
    }
    let Ok(div) = doc.create_element("div") else {
        return;
    };
    let _ = div.set_attribute("id", "panic-overlay");
    let _ = div.set_attribute("role", "alert");
    div.set_inner_html(
        r#"<div class="panic-card">
            <strong>Something went wrong.</strong>
            <p>An unexpected error occurred. Please reload the page to continue.</p>
            <button type="button" onclick="location.reload()">Reload</button>
          </div>"#,
    );
    if let Some(body) = doc.body() {
        let _ = body.append_child(&div);
    }
}
