use crate::components::alphabet::AlphabetSelector;
use crate::components::board::Board;
use crate::components::constraint_view::ConstraintView;
use crate::components::presets::PresetButtons;
use crate::state::{build_problem, AppState};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    let alphabet = state.alphabet;
    let root_constraint = state.root_constraint;

    let count = Memo::new(move |_| {
        build_problem(alphabet.get(), root_constraint.get()).count()
    });

    let arrangement = Signal::derive(move || {
        let problem = build_problem(alphabet.get(), root_constraint.get());
        problem.at(0).unwrap_or_default()
    });

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
                <PresetButtons/>
                <AlphabetSelector/>
                <ConstraintView/>
            </section>

            <section class="pane results-pane" aria-label="Results">
                <h2>"Results"</h2>
                <dl class="stats">
                    <dt>"Count"</dt>
                    <dd>{move || count.get()}</dd>
                </dl>
                {move || {
                    if count.get() == 0 {
                        view! {
                            <p class="empty">
                                "No arrangements satisfy the current alphabet and constraints."
                            </p>
                        }.into_any()
                    } else {
                        view! {
                            <Board pieces=arrangement/>
                            <p class="hint">"Showing arrangement at index 0."</p>
                        }.into_any()
                    }
                }}
            </section>
        </div>
    }
}
