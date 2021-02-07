# sauce-api

[![documentation](https://docs.rs/sauce-api/badge.svg)](https://docs.rs/sauce-api) [![crates.io](https://img.shields.io/crates/v/sauce-api)](https://crates.io/crates/sauce-api)

A simple-to-use async API for finding the source of an image.

Best used with Tokio, but async-std should work too.

## Supported Sources

- [IQDB](https://iqdb.org)
- [saucenao](https://saucenao.com)
- [Yandex](https://yandex.com)

If you wish to see more, please submit PRs or a request in an issue!

## Usage

### IQDB

```rust
use sauce_api::prelude::*;

async fn find_source(url: String) {
    let source = IQDB;
    let res: Result<SauceResult, String> = source.check_sauce(url).await; // Can take some time as IQDB is a bit slow.

    match res {
        Ok(result) => {
            println!("Found results! {:?}", result);
        }
        Err(e) => {
            eprintln!("Unable to find results: {}", e);
        }
    }
}
```

### SauceNao

```rust
use sauce_api::prelude::*;

async fn find_source(url: String) {
    let mut source = SauceNao::new();
    source.set_api_key("an_api_key".to_string());
    let res: Result<SauceResult, String> = source.check_sauce(url).await;

    match res {
        Ok(result) => {
            println!("Found results! {:?}", result);
        }
        Err(e) => {
            eprintln!("Unable to find results: {}", e);
        }
    }
}
```

### Yandex

```rust
use sauce_api::prelude::*;

async fn find_source(url: String) {
    let mut source = Yandex;
    let res: Result<SauceResult, String> = source.check_sauce(url).await;

    match res {
        Ok(result) => {
            println!("Found results! {:?}", result);
        }
        Err(e) => {
            println!("Unable to find results: {}", e);
        }
    }
}
```

## Requirements

sauce-api by default uses the native TLS framework, see [this](https://github.com/seanmonstar/reqwest#requirements) for specific details.
You may opt-in to using rustls if you would like to by enabling the `rustls` feature like this:

```toml
sauce-api = { version = "0.6.0", features = ["rustls"] }
```
