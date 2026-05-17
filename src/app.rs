use crate::components::alphabet::AlphabetSelector;
use crate::components::constraint_editor::ConstraintEditor;
use crate::components::output::OutputPanel;
use crate::components::presets::PresetButtons;
use crate::components::sp_id::SpIdRoundTrip;
use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

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
                <ConstraintEditor/>
            </section>

            <section class="pane results-pane" aria-label="Results">
                <h2>"Results"</h2>
                <OutputPanel/>
                <SpIdRoundTrip/>
            </section>
        </div>
    }
}
