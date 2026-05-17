use crate::components::board::Board;
use crate::state::{build_problem, AppState};
use leptos::prelude::*;

#[component]
pub fn OutputPanel() -> impl IntoView {
    let state = expect_context::<AppState>();
    let alphabet = state.alphabet;
    let root_constraint = state.root_constraint;

    let problem = move || build_problem(alphabet.get(), root_constraint.get());
    let count = Memo::new(move |_| problem().count());

    let index = RwSignal::new(0u64);
    let seed = RwSignal::new(0u64);

    // Clamp index whenever the count drops below it.
    Effect::new(move |_| {
        let c = count.get();
        index.update(|i| {
            if c == 0 {
                *i = 0;
            } else if *i >= c {
                *i = c - 1;
            }
        });
    });

    let indexed_arrangement = Signal::derive(move || {
        if count.get() == 0 {
            Vec::new()
        } else {
            problem().at(index.get()).unwrap_or_default()
        }
    });

    let sampled_arrangement = Signal::derive(move || {
        if count.get() == 0 {
            Vec::new()
        } else {
            problem().sample(seed.get()).unwrap_or_default()
        }
    });

    view! {
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
                }
                .into_any()
            } else {
                let stepper_disabled = move || count.get() <= 1;
                let on_prev = move |_| {
                    index.update(|i| {
                        if *i > 0 {
                            *i -= 1;
                        }
                    });
                };
                let on_next = move |_| {
                    let c = count.get();
                    index.update(|i| {
                        if c > 0 && *i + 1 < c {
                            *i += 1;
                        }
                    });
                };
                let on_index_input = move |ev: leptos::ev::Event| {
                    let raw = event_target_value(&ev);
                    if let Ok(v) = raw.parse::<u64>() {
                        let c = count.get();
                        let clamped = if c == 0 { 0 } else { v.min(c - 1) };
                        index.set(clamped);
                    }
                };
                let on_seed_input = move |ev: leptos::ev::Event| {
                    let raw = event_target_value(&ev);
                    if let Ok(v) = raw.parse::<u64>() {
                        seed.set(v);
                    }
                };
                let on_random_seed = move |_| {
                    seed.update(|s| *s = next_seed(*s));
                };

                view! {
                    <div class="output-block">
                        <div class="output-controls">
                            <label>
                                <span>"Index"</span>
                                <input
                                    type="number"
                                    min="0"
                                    prop:value=move || index.get().to_string()
                                    prop:max=move || {
                                        let c = count.get();
                                        if c == 0 { "0".to_string() } else { (c - 1).to_string() }
                                    }
                                    on:input=on_index_input
                                />
                            </label>
                            <button
                                type="button"
                                on:click=on_prev
                                prop:disabled=stepper_disabled
                                aria-label="Previous arrangement"
                            >
                                "◀"
                            </button>
                            <button
                                type="button"
                                on:click=on_next
                                prop:disabled=stepper_disabled
                                aria-label="Next arrangement"
                            >
                                "▶"
                            </button>
                            <span class="of">
                                {move || format!("of {}", count.get())}
                            </span>
                        </div>
                        <Board pieces=indexed_arrangement/>
                    </div>

                    <div class="output-block">
                        <div class="output-controls">
                            <label>
                                <span>"Seed"</span>
                                <input
                                    type="number"
                                    min="0"
                                    prop:value=move || seed.get().to_string()
                                    on:input=on_seed_input
                                />
                            </label>
                            <button
                                type="button"
                                on:click=on_random_seed
                            >
                                "Random seed"
                            </button>
                        </div>
                        <Board pieces=sampled_arrangement/>
                    </div>
                }
                .into_any()
            }
        }}
    }
}

fn next_seed(prev: u64) -> u64 {
    let mut x = prev.wrapping_add(0x9E3779B97F4A7C15);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}
