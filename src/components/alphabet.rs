use crate::state::{AppState, Orientation, ALL_PIECES};
use chess_startpos_rs::chess::Piece;
use leptos::prelude::*;

#[component]
pub fn AlphabetSelector() -> impl IntoView {
    let state = expect_context::<AppState>();
    let alphabet = state.alphabet;
    let orientation = state.orientation;

    view! {
        <fieldset class="alphabet">
            <legend>"Piece alphabet"</legend>
            <p class="hint">
                "Pieces available to the problem. Multiplicity is governed by count constraints, not this list."
            </p>
            <div class="alphabet-grid">
                {ALL_PIECES.iter().copied().map(|p| {
                    let checked = move || alphabet.with(|a| a.contains(&p));
                    let on_change = move |_| {
                        alphabet.update(|a| {
                            if let Some(idx) = a.iter().position(|q| *q == p) {
                                a.remove(idx);
                            } else {
                                a.push(p);
                                a.sort();
                            }
                        });
                    };
                    let glyph = move || piece_glyph(p, orientation.get());
                    let swatch_class = move || match orientation.get() {
                        Orientation::White => "glyph swatch-dark",
                        Orientation::Black => "glyph swatch-light",
                    };
                    view! {
                        <label class="alphabet-pill">
                            <input
                                type="checkbox"
                                prop:checked=checked
                                on:change=on_change
                            />
                            <span class=swatch_class>{glyph}</span>
                            <span>{piece_label(p)}</span>
                        </label>
                    }
                }).collect_view()}
            </div>
        </fieldset>
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

fn piece_label(p: Piece) -> &'static str {
    match p {
        Piece::King => "King",
        Piece::Queen => "Queen",
        Piece::Rook => "Rook",
        Piece::Bishop => "Bishop",
        Piece::Knight => "Knight",
    }
}
