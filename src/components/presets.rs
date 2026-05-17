use crate::state::{AppState, ChessConstraint};
use chess_startpos_rs::chess::{self, Piece};
use chess_startpos_rs::Problem;
use leptos::prelude::*;

#[component]
pub fn PresetButtons() -> impl IntoView {
    let state = expect_context::<AppState>();

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
                    on:click=move |_| {
                        let p: Problem<Piece> = chess::standard();
                        apply(p.pieces.clone(), p.constraint.clone());
                    }
                >
                    "Standard"
                </button>
                <button
                    type="button"
                    on:click=move |_| {
                        let p: Problem<Piece> = chess::shuffle();
                        apply(p.pieces.clone(), p.constraint.clone());
                    }
                >
                    "Shuffle"
                </button>
                <button
                    type="button"
                    on:click=move |_| {
                        let p: Problem<Piece> = chess::chess_2880();
                        apply(p.pieces.clone(), p.constraint.clone());
                    }
                >
                    "Chess-2880"
                </button>
                <button
                    type="button"
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
