use crate::state::AppState;
use leptos::prelude::*;

#[component]
pub fn ConstraintView() -> impl IntoView {
    let state = expect_context::<AppState>();
    let pretty = move || state.root_constraint.with(|c| format!("{:#?}", c));

    view! {
        <fieldset class="constraint-view">
            <legend>"Root constraint"</legend>
            <p class="hint">
                "Read-only preview. An interactive editor lands in a later change."
            </p>
            <pre>{pretty}</pre>
        </fieldset>
    }
}
