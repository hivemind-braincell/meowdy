# meowdy

[![CI]][workflow]

## Installation

### Using `cargo`

```console
$ cargo install meowdy
```

## Contributing

This project uses a `rust-toolchain` file to specify which version of the Rust compiler should 
be used, which `rustup` should detect and install for you.

This repository also contains a [Nix flake](https://nixos.wiki/wiki/Flakes) that sets up 
an appropriate Rust toolchain. Run `nix develop` or `direnv allow` to load the `devShell` flake 
output, according to your preference.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or 
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the 
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any 
additional terms or conditions.

[CI]: https://github.com/hivemind-braincell/meowdy/actions/workflows/ci.yml/badge.svg?branch=main
[workflow]: https://github.com/hivemind-braincell/meowdy/actions/workflows/ci.yml

