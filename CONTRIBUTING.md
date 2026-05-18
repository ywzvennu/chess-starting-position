# Contributing

Thanks for considering a contribution. This project is small and the conventions are deliberately light, but please skim the rules below before opening a pull request.

## Development setup

Prerequisites:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

Run the dev server:

```sh
trunk serve
```

The site is served at `http://127.0.0.1:8080` with live reload.

To run the same checks CI runs locally:

```sh
cargo fmt --all -- --check
cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings
cargo check --target wasm32-unknown-unknown --all-targets
cargo test
```

All of the above must pass before a PR can land.

## Branch naming

Branches use a conventional type prefix and a kebab-case slug:

- `feat/<slug>` — user-visible features
- `fix/<slug>` — bug fixes
- `chore/<slug>` — repo plumbing, refactors with no user-visible change
- `docs/<slug>` — documentation only
- `ci/<slug>` — workflow / build infrastructure
- `refactor/<slug>` — internal restructuring with no behavior change
- `polish/<slug>` — UX / styling refinements
- `test/<slug>` — test-only changes

## Commits and pull requests

- Commit subject lines mirror the branch type: `feat: add board renderer`, `fix(copy): …`, `chore: …`, etc.
- The PR title mirrors the lead commit; the body should contain the **purpose** and **manual verification steps** the reviewer should run.
- Keep PRs focused. One issue per PR, one PR per issue. If your work fans out into multiple concerns, split it.
- Every PR must build and exercise the new feature end-to-end on its own (no "wired up in a follow-up" placeholders).
- Use `Closes #<n>` in the PR body to auto-link the issue.
- **Do not add `Co-Authored-By` trailers** for AI tools or otherwise misattribute authorship in commit messages, PR bodies, or comments.

## Code style

- Run `cargo fmt --all` before committing.
- Keep new code free of `clippy --all-targets -- -D warnings` violations.
- Add a unit test for any new pure helper if the test is cheap to write (see existing tests in `src/state.rs` and `src/components/output.rs`).
- Default to no comments. Only add a comment when the *why* is non-obvious (a workaround, an invariant, a subtle constraint).

## Issues

Use the templates under `.github/ISSUE_TEMPLATE/`:

- **Bug report** — what you saw, what you expected, how to reproduce.
- **Feature request** — what, why, a sketch if you have one.

## License

By contributing you agree your changes are licensed under the project's MIT license.
