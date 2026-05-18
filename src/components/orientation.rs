use crate::state::{AppState, Orientation};
use leptos::prelude::*;

#[component]
pub fn OrientationToggle() -> impl IntoView {
    let state = expect_context::<AppState>();
    let orientation = state.orientation;

    view! {
        <div class="orientation-toggle" role="group" aria-label="Board orientation">
            <button
                type="button"
                class:selected=move || matches!(orientation.get(), Orientation::White)
                aria-pressed=move || {
                    if matches!(orientation.get(), Orientation::White) {
                        "true"
                    } else {
                        "false"
                    }
                }
                on:click=move |_| orientation.set(Orientation::White)
            >
                "White"
            </button>
            <button
                type="button"
                class:selected=move || matches!(orientation.get(), Orientation::Black)
                aria-pressed=move || {
                    if matches!(orientation.get(), Orientation::Black) {
                        "true"
                    } else {
                        "false"
                    }
                }
                on:click=move |_| orientation.set(Orientation::Black)
            >
                "Black"
            </button>
        </div>
    }
}
