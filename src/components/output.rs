use crate::components::board::Board;
use crate::components::board_actions::BoardActions;
use crate::state::{build_problem, is_chess_960, AppState, Orientation};
use chess_startpos_rs::chess::{self, Piece};
use leptos::prelude::*;

#[component]
pub fn OutputPanel() -> impl IntoView {
    let state = expect_context::<AppState>();
    let alphabet = state.alphabet;
    let root_constraint = state.root_constraint;
    let orientation = state.orientation;

    let problem = move || build_problem(alphabet.get(), root_constraint.get());
    let count = Memo::new(move |_| problem().count());

    let index = RwSignal::new(0u64);
    let seed = RwSignal::new(0u64);
    let advance = RwSignal::new(false);
    let copied_index = RwSignal::new(false);
    let copied_sample = RwSignal::new(false);

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

    // Sample is button-driven only; the displayed arrangement does not
    // change in response to alphabet, constraint, or seed input changes —
    // the user must click Sample to refresh it.
    let initial_sample: Option<(u64, Vec<Piece>)> = {
        let p = build_problem(alphabet.get_untracked(), root_constraint.get_untracked());
        let c = p.count();
        if c == 0 {
            None
        } else {
            let idx = mix_seed(0) % c;
            p.at(idx).map(|arr| (idx, arr))
        }
    };
    let sample: RwSignal<Option<(u64, Vec<Piece>)>> = RwSignal::new(initial_sample);

    let sample_arrangement = Signal::derive(move || {
        sample.with(|s| s.as_ref().map(|(_, a)| a.clone()).unwrap_or_default())
    });

    let sample_sp_id = Signal::derive(move || {
        if !is_chess_960(&alphabet.get(), &root_constraint.get()) {
            return None;
        }
        sample.with(|s| {
            s.as_ref()
                .and_then(|(_, arr)| chess::chess_960().sp_id_of(arr))
        })
    });

    view! {
        <div class="results-toolbar">
            <dl class="stats">
                <dt>"Count"</dt>
                <dd>{move || count.get()}</dd>
            </dl>
            <fieldset class="orientation-toggle">
                <legend>"Orientation"</legend>
                <label>
                    <input
                        type="radio"
                        name="orientation"
                        prop:checked=move || matches!(orientation.get(), Orientation::White)
                        on:change=move |_| orientation.set(Orientation::White)
                    />
                    <span>"White"</span>
                </label>
                <label>
                    <input
                        type="radio"
                        name="orientation"
                        prop:checked=move || matches!(orientation.get(), Orientation::Black)
                        on:change=move |_| orientation.set(Orientation::Black)
                    />
                    <span>"Black"</span>
                </label>
            </fieldset>
        </div>

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
                let on_sample = move |_| {
                    let p = problem();
                    let c = p.count();
                    if c == 0 {
                        sample.set(None);
                        return;
                    }
                    let idx = mix_seed(seed.get()) % c;
                    if let Some(arr) = p.at(idx) {
                        sample.set(Some((idx, arr)));
                    }
                    if advance.get() {
                        seed.update(|s| *s = advance_seed(*s));
                    }
                };

                view! {
                    <div class="output-block">
                        <h3 class="output-title">"By index"</h3>
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
                            >"◀"</button>
                            <button
                                type="button"
                                on:click=on_next
                                prop:disabled=stepper_disabled
                                aria-label="Next arrangement"
                            >"▶"</button>
                            <span class="of">
                                {move || format!("of {}", count.get())}
                            </span>
                        </div>
                        <Board pieces=indexed_arrangement/>
                        <BoardActions pieces=indexed_arrangement copied=copied_index/>
                    </div>

                    <div class="output-block">
                        <h3 class="output-title">"Random sample"</h3>
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
                            <button type="button" on:click=on_sample>"Sample"</button>
                            <label class="advance-toggle" title="Advance the seed via xorshift after each sample">
                                <input
                                    type="checkbox"
                                    prop:checked=move || advance.get()
                                    on:change=move |ev| {
                                        let checked = event_target_checked(&ev);
                                        advance.set(checked);
                                    }
                                />
                                <span>"Advance seed each click"</span>
                            </label>
                        </div>
                        <Board pieces=sample_arrangement/>
                        <BoardActions pieces=sample_arrangement copied=copied_sample/>
                        <p class="sample-meta">
                            {move || match sample.get() {
                                Some((idx, _)) => match sample_sp_id.get() {
                                    Some(sp) => format!("Index {} · SP-ID {}", idx, sp),
                                    None => format!("Index {}", idx),
                                },
                                None => String::new(),
                            }}
                        </p>
                    </div>
                }
                .into_any()
            }
        }}
    }
}

fn advance_seed(prev: u64) -> u64 {
    let mut x = prev.wrapping_add(0x9E3779B97F4A7C15);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

// SplitMix64 — uncorrelated index for nearby seeds.
fn mix_seed(seed: u64) -> u64 {
    let mut x = seed.wrapping_add(0x9E3779B97F4A7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}
