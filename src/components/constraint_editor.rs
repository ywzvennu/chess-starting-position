use crate::state::{AppState, ChessConstraint, BOARD_SQUARES};
use chess_startpos_rs::chess::Piece;
use chess_startpos_rs::{Constraint, CountOp, SquareColor};
use leptos::prelude::*;

#[component]
pub fn ConstraintEditor() -> impl IntoView {
    let state = expect_context::<AppState>();
    let root = state.root_constraint;
    let alphabet = state.alphabet;

    let children = move || -> Vec<ChessConstraint> {
        root.with(|c| match c {
            Constraint::And(v) => v.clone(),
            other => vec![other.clone()],
        })
    };

    let with_children = move |mutate: Box<dyn FnOnce(&mut Vec<ChessConstraint>)>| {
        root.update(|c| {
            let mut v = match std::mem::replace(c, Constraint::And(Vec::new())) {
                Constraint::And(v) => v,
                other => vec![other],
            };
            mutate(&mut v);
            *c = Constraint::And(v);
        });
    };

    let add_count = move |_| {
        let first = alphabet.with(|a| a.first().copied()).unwrap_or(Piece::King);
        with_children(Box::new(move |v| {
            v.push(Constraint::Count {
                piece: first,
                op: CountOp::Eq,
                value: 1,
            });
        }));
    };
    let add_count_on_color = move |_| {
        let first = alphabet.with(|a| a.first().copied()).unwrap_or(Piece::King);
        with_children(Box::new(move |v| {
            v.push(Constraint::CountOnColor {
                piece: first,
                color: SquareColor::Light,
                op: CountOp::Eq,
                value: 1,
            });
        }));
    };
    let add_at = move |_| {
        let first = alphabet.with(|a| a.first().copied()).unwrap_or(Piece::King);
        with_children(Box::new(move |v| {
            v.push(Constraint::At {
                piece: first,
                square: 0,
            });
        }));
    };
    let add_not_at = move |_| {
        let first = alphabet.with(|a| a.first().copied()).unwrap_or(Piece::King);
        with_children(Box::new(move |v| {
            v.push(Constraint::NotAt {
                piece: first,
                square: 0,
            });
        }));
    };
    let add_order = move |_| {
        let first = alphabet.with(|a| a.first().copied()).unwrap_or(Piece::King);
        with_children(Box::new(move |v| {
            v.push(Constraint::Order(vec![(first, 0), (first, 1)]));
        }));
    };
    let add_relative = move |_| {
        let first = alphabet.with(|a| a.first().copied()).unwrap_or(Piece::King);
        with_children(Box::new(move |v| {
            v.push(Constraint::Relative {
                lhs: (first, 0),
                rhs: (first, 1),
                op: CountOp::Lt,
                offset: 0,
            });
        }));
    };

    view! {
        <fieldset class="constraint-editor">
            <legend>"Constraints (AND of leaves)"</legend>
            <p class="hint">
                "Each row is a leaf constraint combined under a top-level AND. Composite combinators land in a later change."
            </p>

            <div class="constraint-list">
                {move || {
                    let items = children();
                    if items.is_empty() {
                        view! {
                            <p class="empty">"No constraints — all arrangements over the alphabet are accepted."</p>
                        }
                        .into_any()
                    } else {
                        items
                            .into_iter()
                            .enumerate()
                            .map(|(idx, c)| {
                                view! {
                                    <ConstraintRow idx=idx initial=c/>
                                }
                            })
                            .collect_view()
                            .into_any()
                    }
                }}
            </div>

            <div class="add-row">
                <span class="add-label">"Add:"</span>
                <button type="button" on:click=add_count>"Count"</button>
                <button type="button" on:click=add_count_on_color>"CountOnColor"</button>
                <button type="button" on:click=add_at>"At"</button>
                <button type="button" on:click=add_not_at>"NotAt"</button>
                <button type="button" on:click=add_order>"Order"</button>
                <button type="button" on:click=add_relative>"Relative"</button>
            </div>
        </fieldset>
    }
}

#[component]
fn ConstraintRow(idx: usize, initial: ChessConstraint) -> impl IntoView {
    let state = expect_context::<AppState>();
    let root = state.root_constraint;
    let alphabet = state.alphabet;

    let replace_at = move |new: ChessConstraint| {
        root.update(|c| {
            let mut v = match std::mem::replace(c, Constraint::And(Vec::new())) {
                Constraint::And(v) => v,
                other => vec![other],
            };
            if let Some(slot) = v.get_mut(idx) {
                *slot = new;
            }
            *c = Constraint::And(v);
        });
    };
    let remove = move |_| {
        root.update(|c| {
            let mut v = match std::mem::replace(c, Constraint::And(Vec::new())) {
                Constraint::And(v) => v,
                other => vec![other],
            };
            if idx < v.len() {
                v.remove(idx);
            }
            *c = Constraint::And(v);
        });
    };

    let label = leaf_label(&initial);
    let body = render_body(initial, alphabet.into(), replace_at);

    view! {
        <div class="constraint-row">
            <div class="row-head">
                <span class="row-kind">{label}</span>
                <button type="button" class="row-remove" on:click=remove aria-label="Remove constraint">"×"</button>
            </div>
            {body}
        </div>
    }
}

fn leaf_label(c: &ChessConstraint) -> &'static str {
    match c {
        Constraint::Count { .. } => "Count",
        Constraint::CountOnColor { .. } => "CountOnColor",
        Constraint::At { .. } => "At",
        Constraint::NotAt { .. } => "NotAt",
        Constraint::Order(_) => "Order",
        Constraint::Relative { .. } => "Relative",
        Constraint::And(_) => "And",
        Constraint::Or(_) => "Or",
        Constraint::Not(_) => "Not",
        _ => "Unknown",
    }
}

fn render_body(
    initial: ChessConstraint,
    alphabet: Signal<Vec<Piece>>,
    replace: impl Fn(ChessConstraint) + Copy + 'static,
) -> AnyView {
    match initial {
        Constraint::Count { piece, op, value } => view! {
            <div class="row-body">
                <PieceSelect
                    value=piece
                    alphabet=alphabet
                    on_change=Box::new(move |p| replace(Constraint::Count { piece: p, op, value }))
                />
                <OpSelect
                    value=op
                    on_change=Box::new(move |o| replace(Constraint::Count { piece, op: o, value }))
                />
                <NumberInput
                    value=value as i64
                    min=0
                    on_change=Box::new(move |v| {
                        replace(Constraint::Count { piece, op, value: v.max(0) as usize })
                    })
                />
            </div>
        }
        .into_any(),
        Constraint::CountOnColor {
            piece,
            color,
            op,
            value,
        } => view! {
            <div class="row-body">
                <PieceSelect
                    value=piece
                    alphabet=alphabet
                    on_change=Box::new(move |p| replace(Constraint::CountOnColor { piece: p, color, op, value }))
                />
                <ColorSelect
                    value=color
                    on_change=Box::new(move |c| replace(Constraint::CountOnColor { piece, color: c, op, value }))
                />
                <OpSelect
                    value=op
                    on_change=Box::new(move |o| replace(Constraint::CountOnColor { piece, color, op: o, value }))
                />
                <NumberInput
                    value=value as i64
                    min=0
                    on_change=Box::new(move |v| {
                        replace(Constraint::CountOnColor { piece, color, op, value: v.max(0) as usize })
                    })
                />
            </div>
        }
        .into_any(),
        Constraint::At { piece, square } => view! {
            <div class="row-body">
                <PieceSelect
                    value=piece
                    alphabet=alphabet
                    on_change=Box::new(move |p| replace(Constraint::At { piece: p, square }))
                />
                <SquareSelect
                    value=square
                    on_change=Box::new(move |s| replace(Constraint::At { piece, square: s }))
                />
            </div>
        }
        .into_any(),
        Constraint::NotAt { piece, square } => view! {
            <div class="row-body">
                <PieceSelect
                    value=piece
                    alphabet=alphabet
                    on_change=Box::new(move |p| replace(Constraint::NotAt { piece: p, square }))
                />
                <SquareSelect
                    value=square
                    on_change=Box::new(move |s| replace(Constraint::NotAt { piece, square: s }))
                />
            </div>
        }
        .into_any(),
        Constraint::Order(items) => {
            let items_for_view = items.clone();
            let len = items.len();
            view! {
                <div class="row-body order">
                    {items_for_view
                        .into_iter()
                        .enumerate()
                        .map(|(i, (piece, k))| {
                            let outer = items.clone();
                            let replace_pi = move |new_piece: Piece| {
                                let mut next = outer.clone();
                                if let Some(slot) = next.get_mut(i) { slot.0 = new_piece; }
                                replace(Constraint::Order(next));
                            };
                            let outer2 = items.clone();
                            let replace_k = move |new_k: i64| {
                                let mut next = outer2.clone();
                                if let Some(slot) = next.get_mut(i) { slot.1 = new_k.max(0) as usize; }
                                replace(Constraint::Order(next));
                            };
                            let outer3 = items.clone();
                            let remove_item = move |_| {
                                let mut next = outer3.clone();
                                if i < next.len() { next.remove(i); }
                                replace(Constraint::Order(next));
                            };
                            let k_display = k as i64;
                            view! {
                                <span class="order-item">
                                    <PieceSelect
                                        value=piece
                                        alphabet=alphabet
                                        on_change=Box::new(replace_pi)
                                    />
                                    <span class="hash">"#"</span>
                                    <NumberInput
                                        value=k_display
                                        min=0
                                        on_change=Box::new(replace_k)
                                    />
                                    <button type="button" class="row-remove small" on:click=remove_item aria-label="Remove indexed piece">"×"</button>
                                </span>
                            }
                        })
                        .collect_view()}
                    <button
                        type="button"
                        class="add-item"
                        on:click={
                            let items = items.clone();
                            move |_| {
                                let mut next = items.clone();
                                let last_piece = next.last().map(|(p, _)| *p)
                                    .or_else(|| alphabet.with(|a| a.first().copied()))
                                    .unwrap_or(Piece::King);
                                next.push((last_piece, len));
                                replace(Constraint::Order(next));
                            }
                        }
                    >"+ piece"</button>
                </div>
            }
            .into_any()
        }
        Constraint::Relative { lhs, rhs, op, offset } => view! {
            <div class="row-body relative">
                <span class="rel-side">
                    <PieceSelect
                        value=lhs.0
                        alphabet=alphabet
                        on_change=Box::new(move |p| replace(Constraint::Relative { lhs: (p, lhs.1), rhs, op, offset }))
                    />
                    <span class="hash">"#"</span>
                    <NumberInput
                        value=lhs.1 as i64
                        min=0
                        on_change=Box::new(move |v| {
                            replace(Constraint::Relative { lhs: (lhs.0, v.max(0) as usize), rhs, op, offset })
                        })
                    />
                </span>
                <span class="rel-op">
                    <OpSelect
                        value=op
                        on_change=Box::new(move |o| replace(Constraint::Relative { lhs, rhs, op: o, offset }))
                    />
                </span>
                <span class="rel-side">
                    <PieceSelect
                        value=rhs.0
                        alphabet=alphabet
                        on_change=Box::new(move |p| replace(Constraint::Relative { lhs, rhs: (p, rhs.1), op, offset }))
                    />
                    <span class="hash">"#"</span>
                    <NumberInput
                        value=rhs.1 as i64
                        min=0
                        on_change=Box::new(move |v| {
                            replace(Constraint::Relative { lhs, rhs: (rhs.0, v.max(0) as usize), op, offset })
                        })
                    />
                </span>
                <span class="rel-offset">
                    <span>"offset"</span>
                    <NumberInput
                        value=offset as i64
                        min=-7
                        on_change=Box::new(move |v| {
                            replace(Constraint::Relative { lhs, rhs, op, offset: v as i32 })
                        })
                    />
                </span>
            </div>
        }
        .into_any(),
        other => view! {
            <div class="row-body">
                <pre class="composite-pre">{format!("{:#?}", other)}</pre>
            </div>
        }
        .into_any(),
    }
}

#[component]
fn PieceSelect(
    value: Piece,
    alphabet: Signal<Vec<Piece>>,
    on_change: Box<dyn Fn(Piece)>,
) -> impl IntoView {
    let in_alphabet = alphabet.with(|a| a.contains(&value));
    let class = if in_alphabet { "piece-select" } else { "piece-select invalid" };

    view! {
        <select
            class=class
            title=move || if in_alphabet { String::new() } else { "Piece not in current alphabet".to_string() }
            on:change=move |ev| {
                let raw = event_target_value(&ev);
                if let Some(p) = parse_piece(&raw) {
                    on_change(p);
                }
            }
        >
            {move || {
                let current = value;
                let union: Vec<Piece> = {
                    let mut u = alphabet.get();
                    if !u.contains(&current) { u.push(current); }
                    u
                };
                union.into_iter().map(|p| {
                    let selected = p == current;
                    view! {
                        <option value=piece_key(p) selected=selected>
                            {piece_display(p)}
                        </option>
                    }
                }).collect_view()
            }}
        </select>
    }
}

#[component]
fn OpSelect(value: CountOp, on_change: Box<dyn Fn(CountOp)>) -> impl IntoView {
    let ops = [
        CountOp::Eq,
        CountOp::NotEq,
        CountOp::Le,
        CountOp::Lt,
        CountOp::Ge,
        CountOp::Gt,
    ];
    view! {
        <select
            on:change=move |ev| {
                let raw = event_target_value(&ev);
                if let Some(o) = parse_op(&raw) {
                    on_change(o);
                }
            }
        >
            {ops.into_iter().map(|o| {
                view! {
                    <option value=op_key(o) selected={o == value}>
                        {op_display(o)}
                    </option>
                }
            }).collect_view()}
        </select>
    }
}

#[component]
fn ColorSelect(value: SquareColor, on_change: Box<dyn Fn(SquareColor)>) -> impl IntoView {
    view! {
        <select
            on:change=move |ev| {
                let raw = event_target_value(&ev);
                let color = if raw == "light" { SquareColor::Light } else { SquareColor::Dark };
                on_change(color);
            }
        >
            <option value="light" selected={matches!(value, SquareColor::Light)}>"Light"</option>
            <option value="dark" selected={matches!(value, SquareColor::Dark)}>"Dark"</option>
        </select>
    }
}

#[component]
fn SquareSelect(value: usize, on_change: Box<dyn Fn(usize)>) -> impl IntoView {
    view! {
        <select
            on:change=move |ev| {
                let raw = event_target_value(&ev);
                if let Ok(s) = raw.parse::<usize>() {
                    on_change(s);
                }
            }
        >
            {(0..BOARD_SQUARES).map(|s| {
                view! {
                    <option value=s.to_string() selected={s == value}>
                        {square_label(s)}
                    </option>
                }
            }).collect_view()}
        </select>
    }
}

#[component]
fn NumberInput(value: i64, min: i64, on_change: Box<dyn Fn(i64)>) -> impl IntoView {
    view! {
        <input
            type="number"
            min=min.to_string()
            prop:value=value.to_string()
            on:input=move |ev| {
                let raw = event_target_value(&ev);
                if let Ok(v) = raw.parse::<i64>() {
                    on_change(v);
                }
            }
        />
    }
}

fn piece_key(p: Piece) -> &'static str {
    match p {
        Piece::King => "K",
        Piece::Queen => "Q",
        Piece::Rook => "R",
        Piece::Bishop => "B",
        Piece::Knight => "N",
    }
}

fn parse_piece(s: &str) -> Option<Piece> {
    match s {
        "K" => Some(Piece::King),
        "Q" => Some(Piece::Queen),
        "R" => Some(Piece::Rook),
        "B" => Some(Piece::Bishop),
        "N" => Some(Piece::Knight),
        _ => None,
    }
}

fn piece_display(p: Piece) -> &'static str {
    match p {
        Piece::King => "♚ K",
        Piece::Queen => "♛ Q",
        Piece::Rook => "♜ R",
        Piece::Bishop => "♝ B",
        Piece::Knight => "♞ N",
    }
}

fn op_key(o: CountOp) -> &'static str {
    match o {
        CountOp::Eq => "eq",
        CountOp::NotEq => "ne",
        CountOp::Le => "le",
        CountOp::Lt => "lt",
        CountOp::Ge => "ge",
        CountOp::Gt => "gt",
        _ => "eq",
    }
}

fn parse_op(s: &str) -> Option<CountOp> {
    Some(match s {
        "eq" => CountOp::Eq,
        "ne" => CountOp::NotEq,
        "le" => CountOp::Le,
        "lt" => CountOp::Lt,
        "ge" => CountOp::Ge,
        "gt" => CountOp::Gt,
        _ => return None,
    })
}

fn op_display(o: CountOp) -> &'static str {
    match o {
        CountOp::Eq => "=",
        CountOp::NotEq => "≠",
        CountOp::Le => "≤",
        CountOp::Lt => "<",
        CountOp::Ge => "≥",
        CountOp::Gt => ">",
        _ => "?",
    }
}

fn square_label(s: usize) -> String {
    let file = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][s % BOARD_SQUARES];
    format!("{} ({})", s, file)
}
