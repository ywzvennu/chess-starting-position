use crate::components::alphabet::AlphabetSelector;
use crate::components::constraint_editor::ConstraintEditor;
use crate::components::orientation::OrientationToggle;
use crate::components::output::OutputPanel;
use crate::components::presets::PresetButtons;
use crate::state::{clear_url_state, is_default_state, read_url_state, write_url_state, AppState};
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

    // Mirror live state into the URL hash on every change, including the
    // initial mount. The default state (full alphabet + empty constraint)
    // carries no information, so clear the hash instead of writing a verbose
    // serialisation; a fresh page therefore has a clean URL.
    Effect::new(move |_| {
        let alphabet = state.alphabet.get();
        let root = state.root_constraint.get();
        if is_default_state(&alphabet, &root) {
            clear_url_state();
        } else {
            write_url_state(&alphabet, &root);
        }
    });

    view! {
        <main class="app-main">
        <header class="app-header">
            <h1>"Chess Starting Position Explorer"</h1>
            <div class="header-controls">
                <OrientationToggle/>
                <ThemeToggle/>
            </div>
        </header>

        <section class="intro" aria-label="About">
            <p>"Explore the space of chess starting positions under composable constraints."</p>
            <p>"Pick the piece kinds available to the problem, build a constraint tree or load a preset such as Chess-960, then count, browse, and sample."</p>
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
                class="footer-link"
                href="https://github.com/ywzvennu/chess-starting-position"
                target="_blank"
                rel="noopener noreferrer"
            >
                <svg
                    class="footer-icon"
                    viewBox="0 0 16 16"
                    width="16"
                    height="16"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
                </svg>
                <span>"chess-starting-position"</span>
            </a>
            <a
                class="footer-link"
                href="https://github.com/ywzvennu/chess-startpos-rs"
                target="_blank"
                rel="noopener noreferrer"
            >
                <svg
                    class="footer-icon"
                    viewBox="0 0 16 16"
                    width="16"
                    height="16"
                    fill="currentColor"
                    aria-hidden="true"
                >
                    <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
                </svg>
                <span>"chess-startpos-rs"</span>
            </a>
            <a
                class="footer-link footer-version"
                href=format!(
                    "https://github.com/ywzvennu/chess-starting-position/releases/tag/v{}",
                    env!("CARGO_PKG_VERSION"),
                )
                target="_blank"
                rel="noopener noreferrer"
                title="View this release on GitHub"
            >
                <svg
                    class="footer-icon"
                    viewBox="0 0 24 24"
                    width="14"
                    height="14"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    aria-hidden="true"
                >
                    <path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z"></path>
                    <circle cx="7.5" cy="7.5" r=".5" fill="currentColor"></circle>
                </svg>
                <span>{format!("v{}", env!("CARGO_PKG_VERSION"))}</span>
            </a>
        </footer>
        </main>
    }
}
