<div align="center">
  <a href="https://github.com/zeeshanlakhani/test-template" target="_blank">
    <img src="https://raw.githubusercontent.com/zeeshanlakhani/test-template/main/assets/a_logo.png" alt="test-template Logo" width="100"></img>
  </a>

  <h1 align="center">test-template-wasm</h1>

  <p>
    <a href="https://crates.io/crates/test-template-wasm">
      <img src="https://img.shields.io/crates/v/test-template-wasm?label=crates" alt="Crate">
    </a>
    <a href="https://npmjs.com/package/test-template">
      <img src="https://img.shields.io/npm/v/test-template" alt="npm">
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

Description.

## Outline

- [Set-up](#set-up)
- [Build for Javascript](#build-for-javascript)
- [Testing the Project](#testing-the-project)
- [Publishing a Package](#publishing-a-package)
- [License](#license)

## Set-up

- Install [`wasm-pack`][wasm-pack]

  ```console
  cargo install wasm-pack
  ```

We'll use [`wasm-pack`][wasm-pack] for building, testing, and publishing
our Wasm project.

### Build for Javascript

The `wasm-pack build` command will compile the code in this directory into
Wasm and generate a `pkg` folder by default, containing the Wasm binary, a
Javascript-wrapper file, the test-template-wasm README (and version), and a
`package.json` file.

- Targetting node:

  ```console
  wasm-pack build --target nodejs
  ```

- Targetting browswers:

  ```console
  wasm-pack build --target web
  ```

- Targetting bundlers like [webpack][webpack]:

  ```console
  wasm-pack build --target bundler
  ```

## Testing the Project

For running tests in the current directory, use one of these commands:

- Run tests expected to execute in [Node.js][node-js]:

```console
wasm-pack test --node
```

- Run browser tests in a headless browwer:

```console
wasm-pack test --headless --firefox --chrome --safari
```

*Note*: Make sure you have the appropriate browser installed when running
locally.

## Publishing a Package

Once you've [built the package](#build-for-javascript), which lives under
`pkg` by default (or a sub-directory of your choosing), you can pack and
publish it to [npm][npm] via (given credentials):

```console
wasm-pack publish
```

## License

This project is licensed under the [Apache License 2.0](./LICENSE), or
[http://www.apache.org/licenses/LICENSE-2.0][apache].


[apache]: https://www.apache.org/licenses/LICENSE-2.0
[mit]: http://opensource.org/licenses/MIT
[node-js]: https://nodejs.dev/en/
[npm]: https://www.npmjs.com/
[wasm-pack]: https://rustwasm.github.io/docs/wasm-pack/
[webpack]: https://webpack.js.org/
