# Uniswap SDK Core Rust

[![Unit Tests](https://github.com/malik672/uniswap-sdk-core-rust/workflows/Rust%20Tests/badge.svg)](https://github.com/malik672/uniswap-sdk-core-rust/actions?query=workflow%3A%22Rust+Tests%22)
[![Lint](https://github.com/malik672/uniswap-sdk-core-rust/workflows/Rust%20Linting/badge.svg)](https://github.com/malik672/uniswap-sdk-core-rust/actions?query=workflow%3A%22Rust%20Linting%22)
[![crates.io](https://img.shields.io/crates/v/uniswap-sdk-core.svg)](https://crates.io/crates/uniswap-sdk-core)

**A Custom Uniswap SDK Core in Rust provides essential functionality for interacting with the Uniswap decentralized
exchange.**

> **Warning**
>
>   This is a custom Uniswap library

## Quickstart

Add this to your Cargo.toml

```
[dependencies]
uniswap-sdk-core = "0.17.0";
```

And this to your code:

```
use uniswap_sdk_core::prelude::*;
```

## Examples

The code below shows an example of how you can validate an address

```
// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.
use uniswap_sdk_core::prelude::*;

fn main() {
        let valid_address: &str = "0x1234567890123456789012345678901234567890";
        assert!(check_valid_ethereum_address(valid_address).is_ok());
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contribution

Contributions are welcome! If you find a bug or have suggestions for improvements, feel free to open an issue or submit
a pull request on the [GitHub repository](https://github.com/malik672/uniswap-sdk-core-rust).

## Acknowledgments

The Uniswap SDK Core in Rust is inspired by the original [Uniswap SDK](https://github.com/Uniswap/sdk-core) and aims to
provide similar functionality in the Rust programming language.

## Used by

- [Uniswap V3 SDK Rust](https://github.com/shuhuiluo/uniswap-v3-sdk-rs): Opinionated Rust implementation of the Uniswap
  V3 SDK with a focus on readability and performance
- ...

*(If you want to add project to the list, dm or open a PR)*
