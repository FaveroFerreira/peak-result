# Changelog

## 1.0.1 - 2023-10-25

### Changed
 
-   Add `#[inline(always)]` to Result impl to improve performance.
-   Add benches to test performance.

## 1.0.0 - 2022-10-31

### Changed

-   Modified method names from `if_err` and `if_ok` to `peak_err` and `peak_ok` respectively

### Added

-   Added tests for `std::result::Result<T, E>` support for `Peak`.

## 0.0.1 - 2022-10-30

### Added

-   Initial `std::result::Result<T, E>` support for `Peak`.