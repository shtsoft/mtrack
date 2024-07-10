# mtrack/backend

[![GPL licensed][license-badge]][license-url]

[license-badge]: https://img.shields.io/badge/license-GPL-blue.svg
[license-url]: ./Cargo.toml

The **Rust** backend of [mtrack](https://github.com/shtsoft/mtrack).

- Rust:
  * safety:
    + thoroughly testet
    + memory-safety: no `unsafe`-code
    + ...

### Installation

Having a clone of the [mtrack repo](https://github.com/shtsoft/mtrack), generate the mtrack binary by running the following commands in the context of the repo top-level:

```console
user@host:~$ cd backend
user@host:~$ cargo build --release
```

Then copy the resulting binary `target/release/mtrack` to the appropriate location.

### Usage

To get a usage description just run the app with the 'help'-argument:

```console
user@host:~$ mtrack --help
```

## Contributing

If you want to contribute: [CONTRIBUTING](CONTRIBUTING.md).

### Security

For security-related issues see: [SECURITY](../SECURITY.md).
