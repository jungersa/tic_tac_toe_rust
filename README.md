# tic_tac_toe_rust

[![Crates.io](https://img.shields.io/crates/v/tic_tac_toe_rust.svg)](https://crates.io/crates/tic_tac_toe_rust)
[![Docs.rs](https://docs.rs/tic_tac_toe_rust/badge.svg)](https://docs.rs/tic_tac_toe_rust)
[![CI](https://github.com/jungersa/tic_tac_toe_rust/workflows/CI/badge.svg)](https://github.com/jungersa/tic_tac_toe_rust/actions)


A command-line tic-tac-toe game written in Rust, with a computer player that uses the minimax algorithm.
This project was created to learn Rust and how to implement a functional AI for a game.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

To install the game, you will need to have cargo installed, as well as the Rust toolchain. You can install the Rust toolchain by following [this guide](https://www.rust-lang.org/tools/install).


### Installation

To install the game, simply run the following command in your terminal:

```bash
$ cargo install tic_tac_toe_rust
```

### Usage
To start a new game, run the following command:

```bash
$ tic_tac_toe_rust
```
To make a move, simply enter the row and column numbers of the cell you want to place your symbol in.

## Running the tests

To run the automated tests for this system, run the following command:
```bash
$ cargo test
```
### Break down into end to end tests

The end-to-end tests ensure that the game is functional, including proper player moves and win/loss/draw conditions. It also tests all the functions in the game module. And every traits implemented on every structs.


### And coding style tests

To test the coding style, run the following command:
```bash
$ cargo fmt --all -- --check
```


## Built With

* [Rust](https://www.rust-lang.org/) - Rust

## Contributing

Please read [CONTRIBUTING.md](https://github.com/jungersa/tic_tac_toe_rust/blob/master/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Support

If you encounter any issues while using the game, please open an issue on the GitHub repository. And if you have any questions, feel free to contact me on my github account. If you would like to contribute to the project, please read the [CONTRIBUTING.md](https://github.com/jungersa/tic_tac_toe_rust/blob/master/CONTRIBUTING.md) file.

## Roadmap

Future releases may another way to play the game (e.g. a GUI).

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/jungersa/tic_tac_toe_rust/tags).

## Authors

* **Arnaud Jungers** - *Initial work* - [jungersa](https://github.com/jungersa)

See also the list of [contributors](https://github.com/jungersa/tic_tac_toe_rust/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
