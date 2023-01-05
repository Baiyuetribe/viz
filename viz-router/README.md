<p align="center">
  <img src="https://raw.githubusercontent.com/viz-rs/viz-rs.github.io/gh-pages/logo.svg" height="200" />
</p>

<h1 align="center">
  <a href="https://docs.rs/viz">Viz</a>
</h1>

<div align="center">
  <p><strong>Robust Routing for Viz</strong></p>
</div>

<div align="center">
  <!-- Safety -->
  <a href="/">
    <img src="https://img.shields.io/badge/-safety!-success?style=flat-square"
      alt="Safety!" /></a>
  <!-- Docs.rs docs -->
  <a href="https://docs.rs/viz-router">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="Docs.rs docs" /></a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/viz-router">
    <img src="https://img.shields.io/crates/v/viz-router.svg?style=flat-square"
    alt="Crates.io version" /></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/viz-router">
    <img src="https://img.shields.io/crates/d/viz-router.svg?style=flat-square"
      alt="Download" /></a>
</div>

## Example

```rust,no_run
use viz_core::{IntoHandler, IntoResponse, Response, Result, Request};
use viz_router::{get, Router};

async fn index() -> Result<impl IntoResponse> {
  Ok(())
}

async fn ws(_: Request) -> Result<Response> {
  Ok(())
}

let app = Router::new()
    .route("/", get(index.into_handler()))
    .route("/ws/:name", get(ws));
```

## License

This project is licensed under the [MIT license](LICENSE).

## Author

- [@fundon@fosstodon.org](https://fosstodon.org/@fundon)

- [@\_fundon](https://twitter.com/_fundon)
