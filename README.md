#                                           Uniswap SDK Core Rust

**An Unofficial Uniswap SDK Core in Rust provides essential functionality for interacting with the Uniswap decentralized exchange.**

> **Warning**
> 
>   This is an unofficial uniswap library

## Quickstart
Add this to your Cargo.toml

[dependencies]
uniswap-core = "0.6.0";

And this to your code:

use ethers::prelude::*;

## Examples
The code below shows an example of how you can validate an address
```
// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.
use uniswap_core::prelude::*;

fn main() {
        let valid_address: &str = "0x1234567890123456789012345678901234567890";
        assert!(check_valid_ethereum_address(valid_address).is_ok());
}
```

## Acknowledgments

The Uniswap SDK Core in Rust is inspired by the original [Uniswap SDK]() and aims to provide similar functionality in the Rust programming language.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/Uniswap/sdk-core/tree/main) file for details.

## Contribution

Contributions are welcome! If you find a bug or have suggestions for improvements, feel free to open an issue or submit a pull request on the [GitHub repository](https://github.com/malik672/uniswap-sdk-core-rust).
