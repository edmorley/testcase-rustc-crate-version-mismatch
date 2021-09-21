# Testcase for Rust compiler suboptimal error messages when crate versions are mismatched

This is a variation of:
https://github.com/rust-lang/rust/issues/22750

## Steps to reproduce

1. Install Rust stable
2. Clone this repo
3. `cd a && cargo check`

## Expected

A Rust compiler error message that mentions that the trait mismatch could be due to the crate versions differing.
For example like the error message added in: https://github.com/rust-lang/rust/pull/66561

## Actual

```
$ cargo check
    Checking c v0.1.0 (/Users/emorley/src/testcase-rustc-crate-version-mismatch/c-v0.1)
    Checking c v0.2.0 (/Users/emorley/src/testcase-rustc-crate-version-mismatch/c-v0.2)
    Checking b v0.1.0 (/Users/emorley/src/testcase-rustc-crate-version-mismatch/b)
    Checking a v0.1.0 (/Users/emorley/src/testcase-rustc-crate-version-mismatch/a)
error[E0277]: the trait bound `CustomErrorHandler: ErrorHandler` is not satisfied
 --> src/main.rs:5:17
  |
5 |     cnb_runtime(CustomErrorHandler {});
  |                 ^^^^^^^^^^^^^^^^^^^^^ the trait `ErrorHandler` is not implemented for `CustomErrorHandler`
  |
note: required by a bound in `cnb_runtime`
 --> /Users/emorley/src/testcase-rustc-crate-version-mismatch/c-v0.2/src/lib.rs:3:41
  |
3 | pub fn cnb_runtime(_error_handler: impl ErrorHandler) {}
  |                                         ^^^^^^^^^^^^ required by this bound in `cnb_runtime`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `a` due to previous error
```
