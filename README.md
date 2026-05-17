# chess-starting-position

A web UI for the [`chess-startpos-rs`](https://github.com/ywzvennu/chess-startpos-rs) Rust library, letting you assemble piece alphabets and constraint trees in the browser to count, index, and sample chess back-rank arrangements.

The application is a single-page WebAssembly app built with [Leptos](https://leptos.dev) and [Trunk](https://trunkrs.dev), deployed as a static site to GitHub Pages.

## Prerequisites

- Rust (stable) with the `wasm32-unknown-unknown` target:
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- [Trunk](https://trunkrs.dev/#install):
  ```sh
  cargo install --locked trunk
  ```

## Local development

```sh
trunk serve
```

Open `http://127.0.0.1:8080`.

## Production build

```sh
trunk build --release
```

The static site is emitted into `dist/`.

## Deployment

A GitHub Actions workflow at [`.github/workflows/deploy.yml`](.github/workflows/deploy.yml) builds on every push to `main` and publishes to GitHub Pages via `actions/deploy-pages`.

**One-time repository setting:** in the GitHub UI, go to **Settings → Pages** and set **Source** to **GitHub Actions**. No `gh-pages` branch is created or used.

The published URL is `https://<owner>.github.io/chess-starting-position/`. The workflow passes `--public-url "/chess-starting-position/"` to Trunk so all asset paths resolve under that subpath.

## License

MIT. See [`LICENSE`](LICENSE).
