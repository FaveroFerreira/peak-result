# peak result

This is a simple `trait` that extends the standard Result enum to allow you to run some code in case of error or success.

This crate exists because I find it annoying that it's needed to use `map_err` or `map` so that I can log the results from my functions. However, maybe people find this issue annoying too, and wish to extend upon this basic functionality that `peak-error` provides.

## Usage

### Quickstart

```toml
[dependencies]
peak-result = "1.0.1"
```

```rust
use peak_result::Peak;

async fn main() {
    // using `peak_err`
    let result = some_function_that_returns_result()
        .peak_err(|e| tracing::error!("function failed: {e:?}"));

    // using `peak_ok`
    let result = some_function_that_returns_result()
        .peak_ok(|it| tracing::info!("function succeeded: {it:?}"));
}
```

## Contribuiting

Take a look at our [contributing guide](https://github.com/FaveroFerreira/peak-result/blob/main/CONTRIBUTING.md) if you wish to contribute.

## License

This project is licensed under the MIT license.

-   MIT license
    ([LICENSE](https://github.com/FaveroFerreira/peak-result/blob/main/LICENSE) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `peak-result` by you, shall be licensed as MIT, without any additional terms or conditions.
