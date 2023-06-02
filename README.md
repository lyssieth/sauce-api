# sauce-api

[![documentation](https://docs.rs/sauce-api/badge.svg)](https://docs.rs/sauce-api) [![crates.io](https://img.shields.io/crates/v/sauce-api)](https://crates.io/crates/sauce-api)

sauce-api is an API for finding the source image for low-quality or cropped images.  
Currently it only works with anime-styled images, but I hope to makeit capable of doing other kinds of images as well.

Asynchronous due to the usage of `reqwest`, and works best with Tokio.

## Supported Sources

- [IQDB](https://iqdb.org) (`iqdb` feature)
- [saucenao](https://saucenao.com) (`saucenao` feature)
- [fuzzysearch](https://fuzzysearch.net) (`fuzzysearch` feature)

If you wish to see more, please submit PRs or a request in an issue!

## Usage

### IQDB

```rust
use sauce_api::source::{Output, iqdb::Iqdb, Source};
use sauce_api::error::Error;

async fn find_source(url: &str) {
    let source = Iqdb::create(()).await.unwrap();
    let res: Result<Output, Error> = source.check(url).await; // Can take some time as IQDB is a bit slow.

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
use sauce_api::source::{Output, saucenao::SauceNao, Source};
use sauce_api::error::Error;

async fn find_source(url: &str, api_key: &str) {
    let source = SauceNao::create(api_key.to_string()).await.unwrap();
    let res: Result<Output, Error> = source.check(url).await;

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


### Fuzzysearch

```rust
use sauce_api::source::{Output, fuzzysearch::FuzzySearch, Source};
use sauce_api::error::Error;

async fn find_source(url: &str, api_key: &str) {
    let source = FuzzySearch::create(api_key.to_string()).await.unwrap();
    let res: Result<Output, Error> = source.check(url).await;

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

## Requirements

sauce-api by default uses the native TLS framework, see [this](https://github.com/seanmonstar/reqwest#requirements) for specific details.
You may opt-in to using rustls if you would like to by enabling the `rustls` feature like this:

```toml
sauce-api = { version = "1.0.0", features = ["rustls"] }
```
