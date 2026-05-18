# Architecture

A short tour of the code so a new contributor can find their way around in a few minutes.

## Top-level layout

```
src/
├── main.rs                   # entry: console_error_panic_hook + mount_to_body(App)
├── app.rs                    # the root <App>: header, intro, two panes, footer
├── state.rs                  # shared AppState, ChessProblem helpers, FEN / Lichess helpers, is_chess_960
├── theme.rs                  # <ThemeToggle> + Theme enum + localStorage persistence
└── components/
    ├── mod.rs
    ├── board.rs              # <Board>: single rank of 8 squares
    ├── board_actions.rs      # <BoardActions>: Copy FEN, Lichess link, FEN disclosure
    ├── alphabet.rs           # <AlphabetSelector>: K/Q/R/B/N pills
    ├── presets.rs            # <PresetButtons>: Standard / Shuffle / 2880 / 960
    ├── constraint_editor.rs  # <ConstraintEditor>: recursive And/Or/Not + 6 leaves
    └── output.rs             # <OutputPanel>: count, by-index browser, random sample
```

## Shared state

`AppState` (in `state.rs`) lives at the top of `App` and is shared via Leptos context:

```rust
pub struct AppState {
    pub alphabet: RwSignal<Vec<Piece>>,
    pub root_constraint: RwSignal<ChessConstraint>,
    pub orientation: RwSignal<Orientation>,
}
```

Three signals drive everything below them. The derived `Problem` is rebuilt on every read via the free function `build_problem(alphabet, root)`, never stored. `count`, `indexed_arrangement`, and the random `sample` slot are computed in `OutputPanel` from those signals (plus a local `index` and `seed`).

The Chess-960 detector `is_chess_960(&alphabet, &root)` is used in two places — the SP-ID meta lines under each board, and the FEN/Lichess actions — so it lives in `state.rs` as a free function.

## Constraint tree

`constraint_editor.rs` renders the root `Constraint<Piece, SquareColor>` recursively. Two closure types are threaded through every level:

```rust
type Replacer = Rc<dyn Fn(ChessConstraint)>;  // "set the node at this position to X"
type Remover  = Rc<dyn Fn()>;                 // "delete this subtree from its parent"
```

`render_node` dispatches on the variant and forwards both closures down to `render_combinator`, `render_not`, or `render_leaf`. For every child of an `And` / `Or` combinator the parent builds:

- a child `Replacer` that rebuilds the parent's children Vec with the child slot replaced;
- a child `Remover` that rebuilds the same Vec with the slot removed.

`Not` always has a `Replacer` (it can be reshaped) but its sole child has no `Remover` (deleting the child would break the `Not` invariant). The root has no `Remover` either.

This pattern keeps the recursive editor self-contained: no global tree-walk path indices and no string keys.

## Build and deploy

- `trunk serve` runs the dev server on `127.0.0.1:8080`.
- `trunk build --release` produces the static site in `dist/` with `--public-url "/chess-starting-position/"` when the deploy workflow runs.

Two GitHub Actions workflows:

- `.github/workflows/ci.yml` — fmt + clippy + check + test on every PR.
- `.github/workflows/deploy.yml` — build on PR (smoke); build + deploy via `actions/deploy-pages` on push to `main`.

The release profile (`opt-level = "z"`, `lto = true`, `codegen-units = 1`) tunes for size.

## External dependency

The combinatorics core is the [`chess-startpos-rs`](https://crates.io/crates/chess-startpos-rs) crate from crates.io with its `serde` feature. Nothing in this repo reimplements enumeration, constraint evaluation, or the Chess-960 SP-ID bijection — those are upstream.
