use chess_startpos_rs::chess;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let chess960_count = chess::chess_960().count();
    view! {
        <h1>"chess-starting-position"</h1>
        <p>"chess_960 arrangement count: " <strong>{chess960_count}</strong></p>
    }
}
