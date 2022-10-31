# peak result

This is a simple `trait` that extends the standard Result enum to allow you to run some code in case of error or success.

## Usage

### Quickstart

```toml
[dependencies]
peak-result = "0.1.1"
```

```rust
use peak_result::Peak;

async fn main() {
    // using `if_err`
    let result = some_function_that_returns_result()
        .if_err(|e| tracing::error!("function failed: {e:?}"));

    // using `if_ok`
    let result = some_function_that_returns_result()
        .if_ok(|it| tracing::info!("function succeeded: {it:?}"));
}
```

## Contribuiting

Take a look at our [contributing guide](https://github.com/FaveroFerreira/peak-result/blob/main/CONTRIBUTING.md) if you wish to contribute.

## License

This project is licensed under the MIT license.

-   MIT license
    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `peak-result` by you, shall be licensed as MIT, without any additional terms or conditions.
