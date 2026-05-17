use crate::state::{fen_for_arrangement, lichess_editor_url};
use chess_startpos_rs::chess::Piece;
use leptos::prelude::*;

#[component]
pub fn BoardActions(#[prop(into)] pieces: Signal<Vec<Piece>>) -> impl IntoView {
    let fen = Signal::derive(move || {
        let p = pieces.get();
        if p.len() == 8 {
            Some(fen_for_arrangement(&p))
        } else {
            None
        }
    });

    let lichess_href = move || {
        fen.with(|f| f.as_ref().map(|s| lichess_editor_url(s)))
            .unwrap_or_else(|| "https://lichess.org/editor".to_string())
    };

    let copied = RwSignal::new(false);
    let on_copy = move |_| {
        let Some(text) = fen.get() else { return; };
        copy_to_clipboard(&text);
        copied.set(true);
    };

    view! {
        <div class="board-actions">
            <button
                type="button"
                class="action-btn"
                on:click=on_copy
                title="Copy FEN to clipboard"
            >
                {move || if copied.get() { "Copied" } else { "Copy FEN" }}
            </button>
            <a
                class="action-btn"
                href=lichess_href
                target="_blank"
                rel="noopener noreferrer"
                title="Open this position in the Lichess board editor"
            >
                "Lichess editor ↗"
            </a>
            <code class="fen-preview">
                {move || fen.get().unwrap_or_default()}
            </code>
        </div>
    }
}

fn copy_to_clipboard(text: &str) {
    if let Some(win) = web_sys::window() {
        let clipboard = win.navigator().clipboard();
        let _ = clipboard.write_text(text);
    }
}
