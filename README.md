<div align="center">
  <a href="https://github.com/zeeshanlakhani/test-template" target="_blank">
    <img src="https://raw.githubusercontent.com/zeeshanlakhani/test-template/main/assets/a_logo.png" alt="test-template Logo" width="100"></img>
  </a>

  <h1 align="center">test-template</h1>

  <p>
    <a href="https://crates.io/crates/test-template">
      <img src="https://img.shields.io/crates/v/test-template?label=crates" alt="Crate">
    </a>
    <a href="https://npmjs.com/package/test-template">
      <img src="https://img.shields.io/npm/v/test-template" alt="Npm">
    </a>
    <a href="https://github.com/zeeshanlakhani/test-template/actions?query=">
      <img src="https://github.com/zeeshanlakhani/test-template/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/zeeshanlakhani/test-template/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>
    <a href="https://docs.rs/test-template">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>
  </p>
</div>

<div align="center"><sub>:warning: Work in progress :warning:</sub></div>

##

## Outline

- [Crates](#crates)
- [Usage and Installation](#usage-and-installation)
- [Testing the Project](#testing-the-project)
- [Setting-up test-template-wasm](#setting-up-test-template-wasm)
- [Contributing](#contributing)
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)

## Crates

- [test-template](https://github.com/zeeshanlakhani/test-template/tree/main/test-template)
- [test-template-wasm](https://github.com/zeeshanlakhani/test-template/tree/main/)

## Usage and Installation

### Using `cargo`

This is just for the rust-only `test-template` binary application:

```console
$ cargo install test-template
```

### test-template-wasm Usage

Due to the reliance on [wasm-pack][wasm-pack], `test-template-wasm` is only
available as a library.

- Add the following to the `[dependencies]` section of your `Cargo.toml` file
  for using `test-template-wasm` crate/workspace:

```toml
test-template-wasm = "0.1.0"
```

## Testing the Project

- Run tests for crate/workspace `test-template`:

  ```console
  cd test-template && cargo test
  ```

- To run tests for crate/workspace `test-template-wasm`, follow
  the instructions in [test-template-wasm](./test-template-wasm#testing-the-project),
  which leverages [wasm-pack][wasm-pack].

## Setting-up test-template-wasm

The Wasm targetted version of this project relies on [wasm-pack][wasm-pack]
for building, testing, and publishing artifacts sutiable for
[Node.js][node-js], web broswers, or bundlers like [webpack][webpack].

Please read more on working with `wasm-pack` directly in
[test-template-wasm](./test-template-wasm#set-up).

## Contributing

:balloon: We're thankful for any feedback and help in improving our project!
We have a [contributing guide](./CONTRIBUTING.md) to help you get involved. We
also adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md).

### Formatting

For formatting Rust in particular, please use `cargo +nightly fmt` as it uses
specific nightly features we recommend. **Make sure you have nightly
installed**.

### Pre-commit Hook

This library recommends using [pre-commit][pre-commit] for running pre-commit
hooks. Please run this before every commit and/or push.

- Once installed, Run `pre-commit install` and `pre-commit install --hook-type commit-msg`
  to setup the pre-commit hooks locally. This will reduce failed CI builds.

- If you are doing interim commits locally, and for some reason if you _don't_
  want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

### Recommended Development Flow

- We recommend installing and leveraging [cargo-watch][cargo-watch],
  [cargo-expand][cargo-expand] and [irust][irust] for Rust development.

### Conventional Commits

This project *lightly* follows the [Conventional Commits
convention][commit-spec-site] to help explain
commit history and tie in with our release process. The full specification
can be found [here][commit-spec]. We recommend prefixing your commits with
a type of `fix`, `feat`, `docs`, `ci`, `refactor`, etc..., structured like so:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Getting Help

For usage questions, usecases, or issues please open an issue in our repository.

We would be happy to try to answer your question or try opening a new issue on Github.

## External Resources

These are references to specifications, talks and presentations, etc.

## License

This project is licensed under the [Apache License 2.0](./LICENSE), or
[http://www.apache.org/licenses/LICENSE-2.0][apache].


[apache]: https://www.apache.org/licenses/LICENSE-2.0
[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/
[irust]: https://github.com/sigmaSd/IRust
[mit]: http://opensource.org/licenses/MIT
[node-js]: https://nodejs.dev/en/
[pre-commit]: https://pre-commit.com/
[wasm-pack]: https://rustwasm.github.io/docs/wasm-pack/
[webpack]: https://webpack.js.org/
