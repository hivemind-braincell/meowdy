# meowdy

[![CI]][workflow]

## Vision and Design
Meowdy! That's the name of this western-style cat-based RPG. You play as Claws Eastwood as you try to escape your backwater country hometown and find your fortune out West. Do odd-jobs and favours around town to earn money, you'll need it to pay for a ride outta here! 

This game was built by three people in 24 hours as part of the 2022 Wackathon. Two of the three writers had no previous experience in Rust.

This game uses the Rust game engine "Bevy", and illustrates that not all games have to be written in more traditional languages (like C++ or C#). Rust is a new and emerging language with a great community and a focus on safety, speed and ease of use.

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

