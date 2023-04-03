# Contributing to tic_tac_toe_rust

Thank you for considering contributing to tic_tac_toe_rust! We welcome contributions from anyone, regardless of experience level or background. Please take a moment to review this guide before submitting your pull request.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Code of Conduct

This project has a [Code of Conduct](https://github.com/jungersa/tic_tac_toe_rust/blob/master/CODE_OF_CONDUCT.md). By participating in this project, you agree to abide by its terms.

## Contributing

### Testing
We appreciate any contributions that improve the project, but we ask that you test any new features or changes thoroughly before submitting a pull request. Please make sure that your code passes all tests and that no existing functionality is broken.

To run the tests, use the following command:
```shell
cargo test --all-features --workspace
```
## Reporting issues

If you find a bug or want to request a new feature, please submit an issue. Be sure to include a clear description of the problem or feature request, and any relevant code snippets or error messages.


Before reporting an issue on the
[issue tracker](https://github.com/jungersa/tic_tac_toe_rust/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Please submit all changes via pull request. Make sure your pull request includes a clear description of the changes you're making and why they're necessary. If your changes include a new feature, please make sure it's thoroughly tested.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/jungersa/tic_tac_toe_rust/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/jungersa/tic_tac_toe_rust
cd tic_tac_toe_rust
cargo test
```

### Useful Commands

- Build and run release version:

  ```shell
  cargo build --release && cargo run --release
  ```

- Run Clippy:

  ```shell
  cargo clippy --all-targets --all-features --workspace
  ```

- Run all tests:

  ```shell
  cargo test --all-features --workspace
  ```

- Check to see if there are code formatting issues

  ```shell
  cargo fmt --all -- --check
  ```

- Format the code in the project

  ```shell
  cargo fmt --all
  ```
