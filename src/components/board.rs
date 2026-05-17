use crate::state::{AppState, Orientation};
use chess_startpos_rs::chess::Piece;
use leptos::prelude::*;

/// Render a single rank of eight squares.
///
/// `pieces` should be exactly eight entries; shorter or longer input is
/// truncated or padded with empty squares so the component never panics.
/// Glyph case and square colors follow the global `orientation` signal in
/// `AppState`: `White` uses uppercase glyphs and renders square 0 dark
/// (matching a real board viewed from White's side); `Black` uses
/// lowercase glyphs and renders square 0 light.
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
                        view! {
                            <div class=class>
                                <span class="glyph">{piece.map(|p| piece_glyph(p, orient)).unwrap_or("")}</span>
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

fn piece_glyph(p: Piece, orient: Orientation) -> &'static str {
    match (orient, p) {
        (Orientation::White, Piece::King) => "♔",
        (Orientation::White, Piece::Queen) => "♕",
        (Orientation::White, Piece::Rook) => "♖",
        (Orientation::White, Piece::Bishop) => "♗",
        (Orientation::White, Piece::Knight) => "♘",
        (Orientation::Black, Piece::King) => "♚",
        (Orientation::Black, Piece::Queen) => "♛",
        (Orientation::Black, Piece::Rook) => "♜",
        (Orientation::Black, Piece::Bishop) => "♝",
        (Orientation::Black, Piece::Knight) => "♞",
    }
}
