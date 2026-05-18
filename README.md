# chess-starting-position

A browser-based explorer for chess back-rank arrangements under composable constraints.

Pick which piece kinds are available, build a constraint tree (or load a preset such as Chess-960), then count, index, sample, copy a FEN, or jump straight to the Lichess board editor — all client-side in the browser. The combinatorics are handled by the [`chess-startpos-rs`](https://github.com/ywzvennu/chess-startpos-rs) Rust crate compiled to WebAssembly.

## Features

- **Presets** — load **Standard**, **Shuffle**, **Chess-2880**, or **Chess-960** as starting templates and edit freely.
- **Piece alphabet** — toggle which of `{King, Queen, Rook, Bishop, Knight}` are part of the problem.
- **Constraint tree editor** — interactive editor over the six leaf primitives (`Count`, `CountOnColor`, `At`, `NotAt`, `Order`, `Relative`) with `And` / `Or` / `Not` combinators, including arbitrary nesting.
- **Live count** of arrangements satisfying the current alphabet + constraint tree.
- **By-index browser** — jump to a specific lex-order arrangement with a stepper.
- **Seeded sampling** — click **Sample** to draw a fresh arrangement; the seed acts as an initial PRNG state that advances on every click. Optional **Advance seed each click** mirrors the advancing state in the visible input.
- **FEN tools** — copy a canonical FEN derived from the arrangement, or open it directly in the [Lichess board editor](https://lichess.org/editor) for further play.
- **Chess-960 SP-ID** — when the active problem matches Chess-960, the FIDE / Stockfish / Lichess SP-ID is shown alongside each arrangement.
- **Light / Dark / System theme** with `localStorage` persistence.
- **White / Black board orientation** toggle that flips glyph case and the underlying square color pattern.

## Stack

- **[Leptos](https://leptos.dev)** (CSR mode) for the UI.
- **[Trunk](https://trunkrs.dev)** for the build toolchain.
- **[`chess-startpos-rs`](https://crates.io/crates/chess-startpos-rs)** as a regular cargo dependency (no FFI wrapper).
- Single WebAssembly bundle, no JS framework, no backend.

## Quick start

Prerequisites:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

Run the dev server:

```sh
trunk serve
```

Open `http://127.0.0.1:8080`.

Production build (output in `dist/`):

```sh
trunk build --release
```

## Continuous integration

Every pull request runs `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo check`, and `cargo test` via [`.github/workflows/ci.yml`](.github/workflows/ci.yml). The production trunk build is also smoked on PRs by [`.github/workflows/deploy.yml`](.github/workflows/deploy.yml).

## Deployment

`deploy.yml` publishes the static site to GitHub Pages via `actions/deploy-pages` on every push to `main`. One-time setup: **Settings → Pages → Source → GitHub Actions**.

The build runs with `--public-url "/chess-starting-position/"` so assets resolve under the project subpath.

## Acknowledgements

This UI is a thin shell around [`chess-startpos-rs`](https://github.com/ywzvennu/chess-startpos-rs) — the library does the actual enumeration, indexing, and sampling.

- Crate: [`chess-startpos-rs` on crates.io](https://crates.io/crates/chess-startpos-rs)
- Docs: [docs.rs/chess-startpos-rs](https://docs.rs/chess-startpos-rs)
- Source: [github.com/ywzvennu/chess-startpos-rs](https://github.com/ywzvennu/chess-startpos-rs)

## License

MIT. See [`LICENSE`](LICENSE).
