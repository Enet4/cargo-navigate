# `cargo navigate`

[![Crates.io](https://img.shields.io/crates/v/cargo-navigate.svg)](https://crates.io/crates/cargo-navigate)
[![Build Status](https://travis-ci.org/Enet4/cargo-navigate.svg?branch=master)](https://travis-ci.org/Enet4/cargo-navigate)

This is the repository for the tools `cargo-navigate`, `cargo-repo` and `cargo-home`,
which will open your default Internet browser on a crate's URL.

The main tool allows you to navigate to:

- **repo**: the crate's repository;
- **docs**: the crate's documentation;
- **home**: the crate's homepage;
- **crates**: the crate's page on [crates.io](https://crates.io).

## Installing

```
cargo install cargo-navigate
```

## Using

```
cargo navigate <where> [crate]
cargo repo [crate]
cargo home [crate]
```

The argument _where_ can be one of "repo", "home", "crates", "docs", along with some other similar aliases.

When _crate_ is omitted, the crate in the working directory is assumed.
Otherwise, the crate is assumed to be registered in [crates.io](https://crates.io). If the "documentation"
metadata is not present when attempting to open the documentation, the browser will navigate to the crate's
[docs.rs](https://docs.rs) page.

Binary aliases are available for accessing the repository and homepage: `cargo repo` and `cargo home`.
Thus, `cargo repo` would be the Rust equivalent of `npm repo` in the JavaScript development environment.

### Examples:

```
cargo navigate docs rayon  # navigate to rayon's documentation
cargo repo                 # navigate to this crate's repository
cargo home rocket-rs       # navigate to Rocket's homepage
```

## Building and Installing from source

```
cargo build --release
cargo install
```

This project is in an alpha stage, so bugs and other issues might occur.
Nevertheless, your contributions are welcome!
