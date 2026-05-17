use crate::components::board::Board;
use chess_startpos_rs::chess;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let chess960_count = chess::chess_960().count();
    let standard_position = chess::chess_960()
        .sp_id(518)
        .expect("SP-ID 518 is the standard FIDE position");
    let standard_signal = RwSignal::new(standard_position);

    view! {
        <header class="app-header">
            <h1>"chess-starting-position"</h1>
            <a
                class="upstream-link"
                href="https://github.com/ywzvennu/chess-startpos-rs"
                target="_blank"
                rel="noopener noreferrer"
            >
                "chess-startpos-rs ↗"
            </a>
        </header>

        <div class="layout">
            <section class="pane config-pane" aria-label="Configuration">
                <h2>"Configuration"</h2>
                <p class="placeholder">
                    "Piece alphabet, presets, and constraints will appear here."
                </p>
            </section>

            <section class="pane results-pane" aria-label="Results">
                <h2>"Results"</h2>
                <dl class="stats">
                    <dt>"chess_960 count"</dt>
                    <dd>{chess960_count}</dd>
                    <dt>"Sample"</dt>
                    <dd>"SP-ID 518 (standard)"</dd>
                </dl>
                <Board pieces=standard_signal/>
            </section>
        </div>
    }
}
