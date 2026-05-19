use crate::pieces::piece_svg;
use crate::state::{AppState, Orientation};
use chess_startpos_rs::chess::Piece;
use leptos::prelude::*;

/// Render a single rank of eight squares.
///
/// `pieces` should be exactly eight entries; shorter or longer input is
/// truncated or padded with empty squares so the component never panics.
/// Glyph orientation follows the global `orientation` signal in `AppState`.
#[component]
pub fn Board(#[prop(into)] pieces: Signal<Vec<Piece>>) -> impl IntoView {
    let state = expect_context::<AppState>();
    let orientation = state.orientation;

    view! {
        <div class="board" role="img" aria-label="Chess back rank">
            {move || {
                let pieces = pieces.get();
                let orient = orientation.get();
                (0..8usize)
                    .map(|file| {
                        let piece = pieces.get(file).copied();
                        let dark_square = match orient {
                            Orientation::White => file % 2 == 0,
                            Orientation::Black => file % 2 == 1,
                        };
                        let class = if dark_square { "square dark" } else { "square light" };
                        let svg = piece.map(|p| piece_svg(p, orient)).unwrap_or("");
                        view! {
                            <div class=class>
                                <span class="glyph" inner_html=svg></span>
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}
