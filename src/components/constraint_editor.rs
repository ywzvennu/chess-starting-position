use crate::state::{AppState, ChessConstraint, BOARD_SQUARES};
use chess_startpos_rs::chess::Piece;
use chess_startpos_rs::{Constraint, CountOp, SquareColor};
use leptos::prelude::*;
use std::rc::Rc;

type Replacer = Rc<dyn Fn(ChessConstraint)>;
type Remover = Rc<dyn Fn()>;

#[component]
pub fn ConstraintEditor() -> impl IntoView {
    let state = expect_context::<AppState>();
    let root = state.root_constraint;
    let alphabet = state.alphabet;

    view! {
        <fieldset class="constraint-editor">
            <legend>"Constraints"</legend>
            <p class="hint">
                "Build a tree of constraints with And / Or / Not combinators and the six leaf primitives."
            </p>
            {move || {
                let current = root.get();
                let replace: Replacer = Rc::new(move |new| root.set(new));
                render_node(current, replace, None, alphabet.into())
            }}
        </fieldset>
    }
}

fn render_node(
    node: ChessConstraint,
    replace: Replacer,
    remove: Option<Remover>,
    alphabet: Signal<Vec<Piece>>,
) -> AnyView {
    match node {
        Constraint::And(children) => {
            render_combinator("And", children, replace, remove, alphabet)
        }
        Constraint::Or(children) => {
            render_combinator("Or", children, replace, remove, alphabet)
        }
        Constraint::Not(child) => render_not(*child, replace, remove, alphabet),
        leaf => render_leaf(leaf, replace, remove, alphabet),
    }
}

fn render_combinator(
    kind: &'static str,
    children: Vec<ChessConstraint>,
    replace: Replacer,
    remove: Option<Remover>,
    alphabet: Signal<Vec<Piece>>,
) -> AnyView {
    let len = children.len();
    let is_root = remove.is_none();

    // Per-child render entries: (index, child, child_replace, child_remove)
    let children_for_render: Vec<_> = children
        .iter()
        .cloned()
        .enumerate()
        .map({
            let outer = children.clone();
            let replace = Rc::clone(&replace);
            let kind = kind;
            move |(i, child)| {
                // Replacer for this child: rebuilds the parent combinator with the child slot updated.
                let child_replace: Replacer = {
                    let outer = outer.clone();
                    let replace = Rc::clone(&replace);
                    Rc::new(move |new_child| {
                        let mut next = outer.clone();
                        if let Some(slot) = next.get_mut(i) {
                            *slot = new_child;
                        }
                        replace(rebuild_combinator(kind, next));
                    })
                };
                // Remover for this child: rebuilds the parent combinator without the child slot.
                let child_remove: Remover = {
                    let outer = outer.clone();
                    let replace = Rc::clone(&replace);
                    Rc::new(move || {
                        let mut next = outer.clone();
                        if i < next.len() {
                            next.remove(i);
                        }
                        replace(rebuild_combinator(kind, next));
                    })
                };
                (i, child, child_replace, child_remove)
            }
        })
        .collect();

    // Combinator type change (And ↔ Or, or wrap with Not)
    let on_combinator_change = {
        let children = children.clone();
        let replace = Rc::clone(&replace);
        move |new_kind: &str| match new_kind {
            "and" => replace(Constraint::And(children.clone())),
            "or" => replace(Constraint::Or(children.clone())),
            "not" => {
                let inner = if children.len() == 1 {
                    children[0].clone()
                } else {
                    Constraint::And(children.clone())
                };
                replace(Constraint::Not(Box::new(inner)));
            }
            _ => {}
        }
    };

    let first_piece = alphabet.with(|a| a.first().copied()).unwrap_or(Piece::King);

    view! {
        <div class={if is_root { "node combinator root" } else { "node combinator" }}>
            <div class="node-head">
                <CombinatorKindSelect
                    value=kind
                    on_change=Box::new(move |k| on_combinator_change(k))
                />
                <span class="node-count">{format!("{} child{}", len, if len == 1 { "" } else { "ren" })}</span>
                <div class="node-actions">
                    <AddLeafMenu
                        first_piece=first_piece
                        on_add=Rc::new({
                            let replace = Rc::clone(&replace);
                            let children = children.clone();
                            let kind = kind;
                            move |c: ChessConstraint| {
                                let mut next = children.clone();
                                next.push(c);
                                replace(rebuild_combinator(kind, next));
                            }
                        })
                    />
                    {remove.clone().map(|r| {
                        view! {
                            <button
                                type="button"
                                class="row-remove"
                                on:click=move |_| r()
                                aria-label="Remove subtree"
                            >"×"</button>
                        }
                    })}
                </div>
            </div>
            <div class="node-children">
                {if children_for_render.is_empty() {
                    view! { <p class="empty-children">"No children — add a leaf or nested combinator."</p> }.into_any()
                } else {
                    children_for_render
                        .into_iter()
                        .map(|(_i, child, child_replace, child_remove)| {
                            view! {
                                <div class="node-child">
                                    {render_node(child, child_replace, Some(child_remove), alphabet)}
                                </div>
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </div>
        </div>
    }
    .into_any()
}

fn render_not(
    child: ChessConstraint,
    replace: Replacer,
    remove: Option<Remover>,
    alphabet: Signal<Vec<Piece>>,
) -> AnyView {
    let is_root = remove.is_none();

    // Replacer for the inner child: rebuilds Not(new_inner).
    let inner_replace: Replacer = {
        let replace = Rc::clone(&replace);
        Rc::new(move |new_inner| {
            replace(Constraint::Not(Box::new(new_inner)));
        })
    };

    // Change combinator: unwrap Not -> And/Or with the current child as the lone element.
    let child_for_swap = child.clone();
    let on_combinator_change = {
        let replace = Rc::clone(&replace);
        move |new_kind: &str| match new_kind {
            "and" => replace(Constraint::And(vec![child_for_swap.clone()])),
            "or" => replace(Constraint::Or(vec![child_for_swap.clone()])),
            "not" => { /* already Not */ }
            _ => {}
        }
    };

    view! {
        <div class={if is_root { "node combinator root not-node" } else { "node combinator not-node" }}>
            <div class="node-head">
                <CombinatorKindSelect
                    value="Not"
                    on_change=Box::new(move |k| on_combinator_change(k))
                />
                <div class="node-actions">
                    {remove.clone().map(|r| {
                        view! {
                            <button
                                type="button"
                                class="row-remove"
                                on:click=move |_| r()
                                aria-label="Remove subtree"
                            >"×"</button>
                        }
                    })}
                </div>
            </div>
            <div class="node-children">
                <div class="node-child">
                    // Not's child is not independently removable — pass None.
                    {render_node(child, inner_replace, None, alphabet)}
                </div>
            </div>
        </div>
    }
    .into_any()
}

fn render_leaf(
    node: ChessConstraint,
    replace: Replacer,
    remove: Option<Remover>,
    alphabet: Signal<Vec<Piece>>,
) -> AnyView {
    let label = leaf_label(&node);
    let body = render_leaf_body(node.clone(), alphabet, Rc::clone(&replace));

    // Wrap-in-Not shortcut
    let wrap_in_not = {
        let node = node.clone();
        let replace = Rc::clone(&replace);
        move |_| replace(Constraint::Not(Box::new(node.clone())))
    };

    view! {
        <div class="node leaf">
            <div class="node-head">
                <span class="row-kind">{label}</span>
                <div class="node-actions">
                    <button type="button" class="wrap-btn" on:click=wrap_in_not title="Wrap in Not">"¬"</button>
                    {remove.clone().map(|r| {
                        view! {
                            <button
                                type="button"
                                class="row-remove"
                                on:click=move |_| r()
                                aria-label="Remove leaf"
                            >"×"</button>
                        }
                    })}
                </div>
            </div>
            {body}
        </div>
    }
    .into_any()
}

fn rebuild_combinator(kind: &str, children: Vec<ChessConstraint>) -> ChessConstraint {
    match kind {
        "And" => Constraint::And(children),
        "Or" => Constraint::Or(children),
        _ => Constraint::And(children),
    }
}

#[component]
fn CombinatorKindSelect(
    value: &'static str,
    on_change: Box<dyn Fn(&str)>,
) -> impl IntoView {
    view! {
        <select
            class="combinator-select"
            on:change=move |ev| {
                let raw = event_target_value(&ev);
                on_change(&raw);
            }
        >
            <option value="and" selected={value == "And"}>"And (∧)"</option>
            <option value="or" selected={value == "Or"}>"Or (∨)"</option>
            <option value="not" selected={value == "Not"}>"Not (¬)"</option>
        </select>
    }
}

#[component]
fn AddLeafMenu(first_piece: Piece, on_add: Rc<dyn Fn(ChessConstraint)>) -> impl IntoView {
    let mk = move |kind: &str| -> Option<ChessConstraint> {
        Some(match kind {
            "count" => Constraint::Count { piece: first_piece, op: CountOp::Eq, value: 1 },
            "ccolor" => Constraint::CountOnColor { piece: first_piece, color: SquareColor::Light, op: CountOp::Eq, value: 1 },
            "at" => Constraint::At { piece: first_piece, square: 0 },
            "notat" => Constraint::NotAt { piece: first_piece, square: 0 },
            "order" => Constraint::Order(vec![(first_piece, 0), (first_piece, 1)]),
            "relative" => Constraint::Relative { lhs: (first_piece, 0), rhs: (first_piece, 1), op: CountOp::Lt, offset: 0 },
            "and" => Constraint::And(Vec::new()),
            "or" => Constraint::Or(Vec::new()),
            "not" => Constraint::Not(Box::new(Constraint::And(Vec::new()))),
            _ => return None,
        })
    };

    view! {
        <select
            class="add-leaf"
            on:change=move |ev| {
                let raw = event_target_value(&ev);
                if let Some(c) = mk(&raw) {
                    on_add(c);
                }
            }
        >
            <option value="">"+ add…"</option>
            <option value="count">"Count"</option>
            <option value="ccolor">"CountOnColor"</option>
            <option value="at">"At"</option>
            <option value="notat">"NotAt"</option>
            <option value="order">"Order"</option>
            <option value="relative">"Relative"</option>
            <option value="and">"And"</option>
            <option value="or">"Or"</option>
            <option value="not">"Not"</option>
        </select>
    }
}

fn render_leaf_body(
    node: ChessConstraint,
    alphabet: Signal<Vec<Piece>>,
    replace: Replacer,
) -> AnyView {
    let r = Rc::clone(&replace);

    match node {
        Constraint::Count { piece, op, value } => {
            let r1 = Rc::clone(&r);
            let r2 = Rc::clone(&r);
            let r3 = Rc::clone(&r);
            view! {
                <div class="row-body">
                    <PieceSelect value=piece alphabet=alphabet
                        on_change=Box::new(move |p| r1(Constraint::Count { piece: p, op, value }))/>
                    <OpSelect value=op
                        on_change=Box::new(move |o| r2(Constraint::Count { piece, op: o, value }))/>
                    <NumberInput value=value as i64 min=0
                        on_change=Box::new(move |v| r3(Constraint::Count { piece, op, value: v.max(0) as usize }))/>
                </div>
            }.into_any()
        }
        Constraint::CountOnColor { piece, color, op, value } => {
            let r1 = Rc::clone(&r);
            let r2 = Rc::clone(&r);
            let r3 = Rc::clone(&r);
            let r4 = Rc::clone(&r);
            view! {
                <div class="row-body">
                    <PieceSelect value=piece alphabet=alphabet
                        on_change=Box::new(move |p| r1(Constraint::CountOnColor { piece: p, color, op, value }))/>
                    <ColorSelect value=color
                        on_change=Box::new(move |c| r2(Constraint::CountOnColor { piece, color: c, op, value }))/>
                    <OpSelect value=op
                        on_change=Box::new(move |o| r3(Constraint::CountOnColor { piece, color, op: o, value }))/>
                    <NumberInput value=value as i64 min=0
                        on_change=Box::new(move |v| r4(Constraint::CountOnColor { piece, color, op, value: v.max(0) as usize }))/>
                </div>
            }.into_any()
        }
        Constraint::At { piece, square } => {
            let r1 = Rc::clone(&r);
            let r2 = Rc::clone(&r);
            view! {
                <div class="row-body">
                    <PieceSelect value=piece alphabet=alphabet
                        on_change=Box::new(move |p| r1(Constraint::At { piece: p, square }))/>
                    <SquareSelect value=square
                        on_change=Box::new(move |s| r2(Constraint::At { piece, square: s }))/>
                </div>
            }.into_any()
        }
        Constraint::NotAt { piece, square } => {
            let r1 = Rc::clone(&r);
            let r2 = Rc::clone(&r);
            view! {
                <div class="row-body">
                    <PieceSelect value=piece alphabet=alphabet
                        on_change=Box::new(move |p| r1(Constraint::NotAt { piece: p, square }))/>
                    <SquareSelect value=square
                        on_change=Box::new(move |s| r2(Constraint::NotAt { piece, square: s }))/>
                </div>
            }.into_any()
        }
        Constraint::Order(items) => {
            let len = items.len();
            let rows = items.clone();
            let add_r = Rc::clone(&r);
            let items_for_add = items.clone();
            view! {
                <div class="row-body order">
                    {rows.into_iter().enumerate().map(|(i, (piece, k))| {
                        let items_p = items.clone();
                        let items_k = items.clone();
                        let items_rm = items.clone();
                        let rp = Rc::clone(&r);
                        let rk = Rc::clone(&r);
                        let rrm = Rc::clone(&r);
                        let k_display = k as i64;
                        view! {
                            <span class="order-item">
                                <PieceSelect value=piece alphabet=alphabet
                                    on_change=Box::new(move |np| {
                                        let mut next = items_p.clone();
                                        if let Some(s) = next.get_mut(i) { s.0 = np; }
                                        rp(Constraint::Order(next));
                                    })/>
                                <span class="hash">"#"</span>
                                <NumberInput value=k_display min=0
                                    on_change=Box::new(move |nk| {
                                        let mut next = items_k.clone();
                                        if let Some(s) = next.get_mut(i) { s.1 = nk.max(0) as usize; }
                                        rk(Constraint::Order(next));
                                    })/>
                                <button type="button" class="row-remove small"
                                    on:click=move |_| {
                                        let mut next = items_rm.clone();
                                        if i < next.len() { next.remove(i); }
                                        rrm(Constraint::Order(next));
                                    }
                                    aria-label="Remove indexed piece">"×"</button>
                            </span>
                        }
                    }).collect_view()}
                    <button type="button" class="add-item"
                        on:click=move |_| {
                            let mut next = items_for_add.clone();
                            let last_piece = next.last().map(|(p, _)| *p)
                                .or_else(|| alphabet.with(|a| a.first().copied()))
                                .unwrap_or(Piece::King);
                            next.push((last_piece, len));
                            add_r(Constraint::Order(next));
                        }>"+ piece"</button>
                </div>
            }.into_any()
        }
        Constraint::Relative { lhs, rhs, op, offset } => {
            let r1 = Rc::clone(&r);
            let r2 = Rc::clone(&r);
            let r3 = Rc::clone(&r);
            let r4 = Rc::clone(&r);
            let r5 = Rc::clone(&r);
            let r6 = Rc::clone(&r);
            view! {
                <div class="row-body relative">
                    <span class="rel-side">
                        <PieceSelect value=lhs.0 alphabet=alphabet
                            on_change=Box::new(move |p| r1(Constraint::Relative { lhs: (p, lhs.1), rhs, op, offset }))/>
                        <span class="hash">"#"</span>
                        <NumberInput value=lhs.1 as i64 min=0
                            on_change=Box::new(move |v| r2(Constraint::Relative { lhs: (lhs.0, v.max(0) as usize), rhs, op, offset }))/>
                    </span>
                    <span class="rel-op">
                        <OpSelect value=op
                            on_change=Box::new(move |o| r3(Constraint::Relative { lhs, rhs, op: o, offset }))/>
                    </span>
                    <span class="rel-side">
                        <PieceSelect value=rhs.0 alphabet=alphabet
                            on_change=Box::new(move |p| r4(Constraint::Relative { lhs, rhs: (p, rhs.1), op, offset }))/>
                        <span class="hash">"#"</span>
                        <NumberInput value=rhs.1 as i64 min=0
                            on_change=Box::new(move |v| r5(Constraint::Relative { lhs, rhs: (rhs.0, v.max(0) as usize), op, offset }))/>
                    </span>
                    <span class="rel-offset">
                        <span>"offset"</span>
                        <NumberInput value=offset as i64 min=-7
                            on_change=Box::new(move |v| r6(Constraint::Relative { lhs, rhs, op, offset: v as i32 }))/>
                    </span>
                </div>
            }.into_any()
        }
        other => view! {
            <div class="row-body"><pre class="composite-pre">{format!("{:#?}", other)}</pre></div>
        }.into_any(),
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
        _ => "Unknown",
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
    ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][s % BOARD_SQUARES].to_string()
}
