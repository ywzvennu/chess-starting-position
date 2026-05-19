use crate::state::{AppState, ChessConstraint};
use chess_startpos_rs::chess::{self, Piece};
use chess_startpos_rs::Problem;
use leptos::prelude::*;

const STANDARD: &str = "standard";
const SHUFFLE: &str = "shuffle";
const CHESS_2880: &str = "chess_2880";
const CHESS_960: &str = "chess_960";

fn active_preset(alphabet: &[Piece], root: &ChessConstraint) -> Option<&'static str> {
    let s = chess::standard();
    if alphabet == s.pieces.as_slice() && root == &s.constraint {
        return Some(STANDARD);
    }
    let sh = chess::shuffle();
    if alphabet == sh.pieces.as_slice() && root == &sh.constraint {
        return Some(SHUFFLE);
    }
    let c2880 = chess::chess_2880();
    if alphabet == c2880.pieces.as_slice() && root == &c2880.constraint {
        return Some(CHESS_2880);
    }
    let c960 = chess::chess_960().into_problem();
    if alphabet == c960.pieces.as_slice() && root == &c960.constraint {
        return Some(CHESS_960);
    }
    None
}

#[component]
pub fn PresetButtons() -> impl IntoView {
    let state = expect_context::<AppState>();
    let alphabet = state.alphabet;
    let root_constraint = state.root_constraint;

    let active = Memo::new(move |_| {
        let a = alphabet.get();
        let r = root_constraint.get();
        active_preset(&a, &r)
    });

    let apply = move |pieces: Vec<Piece>, constraint: ChessConstraint| {
        state.alphabet.set(pieces);
        state.root_constraint.set(constraint);
    };

    view! {
        <fieldset class="presets">
            <legend>"Presets"</legend>
            <div class="preset-grid">
                <button
                    type="button"
                    class:active=move || active.get() == Some(STANDARD)
                    on:click=move |_| {
                        let p: Problem<Piece> = chess::standard();
                        apply(p.pieces.clone(), p.constraint.clone());
                    }
                >
                    "Standard"
                </button>
                <button
                    type="button"
                    class:active=move || active.get() == Some(SHUFFLE)
                    on:click=move |_| {
                        let p: Problem<Piece> = chess::shuffle();
                        apply(p.pieces.clone(), p.constraint.clone());
                    }
                >
                    "Shuffle"
                </button>
                <button
                    type="button"
                    class:active=move || active.get() == Some(CHESS_2880)
                    on:click=move |_| {
                        let p: Problem<Piece> = chess::chess_2880();
                        apply(p.pieces.clone(), p.constraint.clone());
                    }
                >
                    "Chess-2880"
                </button>
                <button
                    type="button"
                    class:active=move || active.get() == Some(CHESS_960)
                    on:click=move |_| {
                        let p: Problem<Piece> = chess::chess_960().into_problem();
                        apply(p.pieces.clone(), p.constraint.clone());
                    }
                >
                    "Chess-960"
                </button>
            </div>
        </fieldset>
    }
}
