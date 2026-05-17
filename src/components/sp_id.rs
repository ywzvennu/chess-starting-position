use crate::components::board::Board;
use crate::components::board_actions::BoardActions;
use crate::state::{is_chess_960, AppState};
use chess_startpos_rs::chess;
use leptos::prelude::*;

#[component]
pub fn SpIdRoundTrip() -> impl IntoView {
    let state = expect_context::<AppState>();
    let alphabet = state.alphabet;
    let root_constraint = state.root_constraint;

    let active = Memo::new(move |_| {
        is_chess_960(&alphabet.get(), &root_constraint.get())
    });

    let sp_input = RwSignal::new(518u32);
    let copied_sp = RwSignal::new(false);

    let arrangement = Signal::derive(move || {
        if !active.get() {
            return Vec::new();
        }
        chess::chess_960().sp_id(sp_input.get()).unwrap_or_default()
    });

    let round_trip = Signal::derive(move || {
        if !active.get() {
            return None;
        }
        let arr = chess::chess_960().sp_id(sp_input.get())?;
        chess::chess_960().sp_id_of(&arr)
    });

    view! {
        {move || {
            if !active.get() {
                ().into_any()
            } else {
                let on_input = move |ev: leptos::ev::Event| {
                    let raw = event_target_value(&ev);
                    if let Ok(v) = raw.parse::<u32>() {
                        sp_input.set(v.min(959));
                    }
                };
                view! {
                    <fieldset class="sp-id">
                        <legend>"Chess-960 SP-ID"</legend>
                        <p class="hint">
                            "FIDE/Stockfish/Lichess Chess960 numbering. SP-ID 518 is the standard starting position."
                        </p>
                        <div class="sp-controls">
                            <label>
                                <span>"SP-ID"</span>
                                <input
                                    type="number"
                                    min="0"
                                    max="959"
                                    prop:value=move || sp_input.get().to_string()
                                    on:input=on_input
                                />
                            </label>
                            <span class="round-trip">
                                "round-trip "
                                {move || match round_trip.get() {
                                    Some(id) => format!("→ {}", id),
                                    None => String::new(),
                                }}
                            </span>
                        </div>
                        <Board pieces=arrangement/>
                        <BoardActions pieces=arrangement copied=copied_sp/>
                    </fieldset>
                }
                .into_any()
            }
        }}
    }
}
