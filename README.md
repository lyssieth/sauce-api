# sauce-api

A simple-to-use async API for finding the source of an image.

Best used with Tokio, but async-std should work too.

## Supported Sources

- [IQDB](https://iqdb.org)
- [saucenao](https://saucenao.com)

If you wish to see more, please submit PRs or a request in an issue!

## Usage

### IQDB

```rust
use sauce_api::prelude::*;

async fn find_source(url: &str) {
    let res: Result<SauceResult, String> = IQDB::check_sauce(url).await; // Can take some time as IQDB is a bit slow.

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

// NOTE: Requires that `SAUCENAO_API_KEY` is set in environment variables.
//       Am looking for a neat way around that.
async fn find_source(url: &str) {
    let res: Result<SauceResult, String> = SauceNao::check_sauce(url).await;

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
