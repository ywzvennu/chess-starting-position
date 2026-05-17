use crate::state::{fen_for_arrangement, lichess_editor_url};
use chess_startpos_rs::chess::Piece;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn BoardActions(
    #[prop(into)] pieces: Signal<Vec<Piece>>,
    copied: RwSignal<bool>,
) -> impl IntoView {
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
    let Some(win) = web_sys::window() else { return; };

    // Modern path: fire-and-forget. Works on secure contexts.
    let _ = win.navigator().clipboard().write_text(text);

    // Synchronous fallback. Runs inside the same user-gesture activation
    // and writes via the legacy execCommand path, so the copy succeeds
    // even when the async clipboard API is blocked or delayed.
    let Some(doc) = win.document() else { return; };
    let Some(body) = doc.body() else { return; };
    let Ok(element) = doc.create_element("textarea") else { return; };
    let Ok(textarea) = element.dyn_into::<web_sys::HtmlTextAreaElement>() else { return; };

    textarea.set_value(text);
    let _ = textarea.set_attribute(
        "style",
        "position:fixed;top:0;left:0;opacity:0;pointer-events:none;",
    );

    if body.append_child(&textarea).is_ok() {
        textarea.focus().ok();
        textarea.select();
        if let Ok(html_doc) = doc.dyn_into::<web_sys::HtmlDocument>() {
            let _ = html_doc.exec_command("copy");
        }
        let _ = body.remove_child(&textarea);
    }
}
