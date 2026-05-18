# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - Initial release

### Added
- Two-pane responsive layout with header, intro, configuration pane, results pane, and footer.
- **Presets** for **Standard**, **Shuffle**, **Chess-2880**, and **Chess-960** — each loads a known alphabet + constraint tree.
- **Piece alphabet** selector toggling which of `{King, Queen, Rook, Bishop, Knight}` are part of the active problem.
- **Constraint tree editor** with the six leaf primitives (`Count`, `CountOnColor`, `At`, `NotAt`, `Order`, `Relative`) and the `And` / `Or` / `Not` combinators, including arbitrary nesting.
- **By-index browser** — numeric input plus prev / next stepper to walk the lex-order arrangements.
- **Random sample** — deliberate Sample button driven by an internal PRNG seeded from the visible seed field. **Advance seed each click** option mirrors the advancing state into the visible input.
- **FEN tools** under each board: **Copy FEN** with 1 s "Copied" confirmation; **Lichess editor** link; collapsible **FEN** disclosure.
- **Chess-960 SP-ID** rendered inline under each board when the active problem matches the canonical Chess-960 problem.
- **White / Black** board orientation toggle; glyph case and the underlying square color pattern flip together.
- **Light / Dark / System** theme dropdown with `localStorage` persistence.
- **URL state sharing** — the active alphabet and constraint tree are encoded in the location hash so a URL fully restores the problem on another machine.
- **CI**: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo check`, `cargo test` on every PR.
- **Deploy**: GitHub Actions workflow that publishes to GitHub Pages on push to `main`.
- **Unit tests** for `fen_for_arrangement`, `lichess_editor_url`, `is_chess_960`, `mix_seed`, `advance_seed`.
- Contributor onboarding: `CONTRIBUTING.md`, bug / feature issue templates, PR template.
- Architecture overview at `docs/architecture.md`.

[Unreleased]: https://github.com/ywzvennu/chess-starting-position/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ywzvennu/chess-starting-position/releases/tag/v0.1.0
