use crate::components::board::Board;
use crate::components::board_actions::BoardActions;
use crate::state::{build_problem, is_chess_960, AppState};
use chess_startpos_rs::chess::{self, Piece};
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn OutputPanel() -> impl IntoView {
    let state = expect_context::<AppState>();
    let alphabet = state.alphabet;
    let root_constraint = state.root_constraint;

    let problem = move || build_problem(alphabet.get(), root_constraint.get());
    let count = Memo::new(move |_| problem().count());

    let index = RwSignal::new(0u64);
    // `seed` is the value shown in the visible input. `internal_seed` is the
    // running PRNG state used to derive each sample; it advances on every
    // Sample click. Typing into the input syncs both.
    let seed = RwSignal::new(0u64);
    let internal_seed = StoredValue::new(0u64);
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

    let indexed_sp_id = Signal::derive(move || {
        if !is_chess_960(&alphabet.get(), &root_constraint.get()) {
            return None;
        }
        indexed_arrangement.with(|arr| {
            if arr.len() == 8 {
                chess::chess_960().sp_id_of(arr)
            } else {
                None
            }
        })
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

    // Clear the sample and reset the index when the problem itself changes
    // (alphabet or root constraint). The previously-drawn sample is no longer
    // guaranteed to satisfy the new problem; the user clicks Sample to refresh.
    Effect::new(move |prev: Option<()>| {
        alphabet.get();
        root_constraint.get();
        if prev.is_some() {
            sample.set(None);
            index.set(0);
        }
    });

    install_keyboard_shortcuts(
        index,
        count,
        seed,
        internal_seed,
        advance,
        sample,
        alphabet,
        root_constraint,
    );

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
                        // User-visible index is 1-based; internal index is 0-based.
                        let zero_based = v.saturating_sub(1);
                        let c = count.get();
                        let clamped = if c == 0 { 0 } else { zero_based.min(c - 1) };
                        index.set(clamped);
                    }
                };
                let on_seed_input = move |ev: leptos::ev::Event| {
                    let raw = event_target_value(&ev);
                    if let Ok(v) = raw.parse::<u64>() {
                        seed.set(v);
                        internal_seed.set_value(v);
                    }
                };
                let on_sample = move |_| {
                    let p = problem();
                    let c = p.count();
                    if c == 0 {
                        sample.set(None);
                        return;
                    }
                    let s = internal_seed.get_value();
                    let idx = mix_seed(s) % c;
                    if let Some(arr) = p.at(idx) {
                        sample.set(Some((idx, arr)));
                    }
                    let next = advance_seed(s);
                    internal_seed.set_value(next);
                    if advance.get() {
                        seed.set(next);
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
                                    min="1"
                                    aria-label="Arrangement index"
                                    prop:value=move || (index.get() + 1).to_string()
                                    prop:max=move || count.get().to_string()
                                    on:input=on_index_input
                                />
                            </label>
                            <button
                                type="button"
                                on:click=on_prev
                                prop:disabled=stepper_disabled
                                aria-label="Previous arrangement"
                                title="Previous (←)"
                            >"◀"</button>
                            <button
                                type="button"
                                on:click=on_next
                                prop:disabled=stepper_disabled
                                aria-label="Next arrangement"
                                title="Next (→)"
                            >"▶"</button>
                            <span class="of">
                                {move || format!("of {}", count.get())}
                            </span>
                        </div>
                        <Board pieces=indexed_arrangement/>
                        <BoardActions pieces=indexed_arrangement copied=copied_index/>
                        <p class="sample-meta">
                            {move || match indexed_sp_id.get() {
                                Some(sp) => format!("SP-ID {}", sp),
                                None => String::new(),
                            }}
                        </p>
                    </div>

                    <div class="output-block">
                        <h3 class="output-title">"Random sample"</h3>
                        <div class="output-controls">
                            <label>
                                <span>"Seed"</span>
                                <input
                                    type="number"
                                    min="0"
                                    aria-label="Sampling seed"
                                    prop:value=move || seed.get().to_string()
                                    on:input=on_seed_input
                                />
                            </label>
                            <button
                                type="button"
                                on:click=on_sample
                                title="Sample (S or space)"
                            >"Sample"</button>
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
                                    Some(sp) => format!("Index {} · SP-ID {}", idx + 1, sp),
                                    None => format!("Index {}", idx + 1),
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

/// Wire up a window-level `keydown` listener that drives the by-index
/// stepper and the Sample button without the user having to click. The
/// closure is installed once at mount and intentionally leaked — `OutputPanel`
/// lives for the lifetime of the app, so we don't need to track a handle for
/// teardown. Shortcuts are suppressed when focus is in a form control so
/// typing in the index/seed/constraint inputs continues to work normally.
#[allow(clippy::too_many_arguments)]
fn install_keyboard_shortcuts(
    index: RwSignal<u64>,
    count: Memo<u64>,
    seed: RwSignal<u64>,
    internal_seed: StoredValue<u64>,
    advance: RwSignal<bool>,
    sample: RwSignal<Option<(u64, Vec<Piece>)>>,
    alphabet: RwSignal<Vec<Piece>>,
    root_constraint: RwSignal<crate::state::ChessConstraint>,
) {
    let handler = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
        if is_typing_target(&ev) {
            return;
        }
        match ev.key().as_str() {
            "ArrowLeft" => {
                index.update(|i| {
                    if *i > 0 {
                        *i -= 1;
                    }
                });
                ev.prevent_default();
            }
            "ArrowRight" => {
                let c = count.get();
                index.update(|i| {
                    if c > 0 && *i + 1 < c {
                        *i += 1;
                    }
                });
                ev.prevent_default();
            }
            "s" | "S" | " " => {
                let p = build_problem(alphabet.get(), root_constraint.get());
                let c = p.count();
                if c == 0 {
                    sample.set(None);
                    ev.prevent_default();
                    return;
                }
                let s = internal_seed.get_value();
                let idx = mix_seed(s) % c;
                if let Some(arr) = p.at(idx) {
                    sample.set(Some((idx, arr)));
                }
                let next = advance_seed(s);
                internal_seed.set_value(next);
                if advance.get() {
                    seed.set(next);
                }
                ev.prevent_default();
            }
            _ => {}
        }
    }) as Box<dyn Fn(web_sys::KeyboardEvent)>);

    if let Some(win) = web_sys::window() {
        let _ = win.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
    }
    handler.forget();
}

fn is_typing_target(ev: &web_sys::KeyboardEvent) -> bool {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        .map(|el| {
            matches!(
                el.tag_name().to_uppercase().as_str(),
                "INPUT" | "SELECT" | "TEXTAREA"
            )
        })
        .unwrap_or(false)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_seed_is_deterministic() {
        assert_eq!(mix_seed(42), mix_seed(42));
        assert_eq!(mix_seed(0), mix_seed(0));
    }

    #[test]
    fn mix_seed_differs_for_adjacent_seeds() {
        // Three adjacent inputs should not collide.
        let outputs: Vec<u64> = (0..3).map(mix_seed).collect();
        assert_ne!(outputs[0], outputs[1]);
        assert_ne!(outputs[1], outputs[2]);
        assert_ne!(outputs[0], outputs[2]);
    }

    #[test]
    fn mix_seed_avoids_trivial_linearity() {
        // For a trivial `seed % count` mapping the differences would be 1.
        // SplitMix64 should not produce a constant difference across adjacent
        // inputs.
        let a = mix_seed(0);
        let b = mix_seed(1);
        let c = mix_seed(2);
        let d1 = b.wrapping_sub(a);
        let d2 = c.wrapping_sub(b);
        assert_ne!(d1, d2, "mix_seed should not be a linear function of input");
    }

    #[test]
    fn advance_seed_changes_value() {
        assert_ne!(advance_seed(0), 0);
        let s = 1234567;
        assert_ne!(advance_seed(s), s);
    }

    #[test]
    fn advance_seed_is_deterministic() {
        assert_eq!(advance_seed(42), advance_seed(42));
    }
}
