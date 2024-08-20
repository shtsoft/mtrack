# mtrack/backend

[![GPL licensed][license-badge]][license-url]

[license-badge]: https://img.shields.io/badge/license-GPL-blue.svg
[license-url]: ./Cargo.toml

The backend of [mtrack](https://github.com/shtsoft/mtrack).

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

(Also have a look at the [tests-data directory](tests-data) to see what is needed to run mtrack. In particular, note that the PHC strings in the user databases are bcrypt.)

## Contributing

If you want to contribute: [CONTRIBUTING](CONTRIBUTING.md).

### Security

For security-related issues see: [SECURITY](../SECURITY.md).
