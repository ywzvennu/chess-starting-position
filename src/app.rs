use crate::components::alphabet::AlphabetSelector;
use crate::components::constraint_editor::ConstraintEditor;
use crate::components::output::OutputPanel;
use crate::components::presets::PresetButtons;
use crate::state::{read_url_state, write_url_state, AppState};
use crate::theme::ThemeToggle;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();

    // Hydrate from the URL hash before any reactive subscribers wire up so the
    // initial paint already shows the shared state.
    if let Some((alphabet, root)) = read_url_state() {
        state.alphabet.set(alphabet);
        state.root_constraint.set(root);
    }

    provide_context(state);

    // Mirror live state into the URL hash on every change after mount.
    Effect::new(move |prev: Option<()>| {
        let alphabet = state.alphabet.get();
        let root = state.root_constraint.get();
        if prev.is_some() {
            write_url_state(&alphabet, &root);
        }
    });

    view! {
        <header class="app-header">
            <h1>"Chess Starting Position Explorer"</h1>
            <ThemeToggle/>
        </header>

        <section class="intro" aria-label="About">
            <p>
                "Explore the space of chess starting positions under composable constraints. "
                "Pick the piece kinds available to the problem, build a constraint tree (or "
                "load a preset such as Chess-960), then count, browse, and sample, with "
                "FIDE SP-ID lookup for Chess-960."
            </p>
        </section>

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
            </section>
        </div>

        <footer class="app-footer">
            <a
                href="https://github.com/ywzvennu/chess-starting-position"
                target="_blank"
                rel="noopener noreferrer"
            >
                "chess-starting-position (this app) ↗"
            </a>
            <span class="footer-sep">"·"</span>
            <a
                href="https://github.com/ywzvennu/chess-startpos-rs"
                target="_blank"
                rel="noopener noreferrer"
            >
                "chess-startpos-rs (library) ↗"
            </a>
            <span class="footer-sep">"·"</span>
            <a
                href="https://crates.io/crates/chess-startpos-rs"
                target="_blank"
                rel="noopener noreferrer"
            >
                "crates.io ↗"
            </a>
        </footer>
    }
}
