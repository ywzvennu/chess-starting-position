use chess_startpos_rs::chess::Piece;
use leptos::prelude::*;

/// Render a single back rank of eight alternating-color squares.
///
/// `pieces` should be exactly eight entries; any shorter or longer slice
/// is truncated or padded with empty squares so the component never panics.
#[component]
pub fn Board(#[prop(into)] pieces: Signal<Vec<Piece>>) -> impl IntoView {
    view! {
        <div class="board" role="img" aria-label="Chess back rank">
            {move || {
                let pieces = pieces.get();
                (0..8usize)
                    .map(|file| {
                        let piece = pieces.get(file).copied();
                        let dark = file % 2 == 1;
                        let class = if dark { "square dark" } else { "square light" };
                        view! {
                            <div class=class>
                                <span class="glyph">{piece.map(piece_glyph).unwrap_or("")}</span>
                            </div>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

fn piece_glyph(p: Piece) -> &'static str {
    match p {
        Piece::King => "♚",
        Piece::Queen => "♛",
        Piece::Rook => "♜",
        Piece::Bishop => "♝",
        Piece::Knight => "♞",
    }
}
