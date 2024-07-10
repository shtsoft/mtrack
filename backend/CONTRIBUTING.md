# Contributing to mtrack/backend

Contributions are welcome!
But please read this guide before.

## Code

If you plan to contribute code please take the following subsections into account.

### Implementation

Pay attention to security, usability, maintainability and performance (roughly in that order).
Generally, try to adhere to the coding-style of the current implementation to keep reading the code easy.
Specifically,
- make sure that the code is formatted correctly by running `cargo fmt --all -- --check`.
- make sure that the linter does not complain too much by running `cargo clippy --all --benches --examples --tests --all-features` followed by `-- -W clippy::pedantic` or `-- -W clippy::nursery` if output will be interpreted very carefully.

### Documentation

Document items (regardless of visibility) descending from the library-crate adhering to the current style.
For enums and traits this means to write a description of what they define (extensionally and intensionally, respectively) while for structs it means to say what they structure.
For function-like items this means to write a 'contract' made up by:
1. a description of what the item does
2. a description of the arguments (if there are any)
3. a description of the side effects (if there are any)
4. a description of the preconditions, postconditions and invariants (if there are any)
5. additional information interesting to callers (if there is some)
Furthermore, it means
- for constructors to say what they construct
- for structs to say what their fields mean
but only if it really makes sense to do so.
In the end, make sure that the docs build by running `cargo doc --no-deps --document-private-items`.

### Testing

If you add code then also add tests as necessary.
Both integration- and unit-tests.
It is also suggested to include one documentation-test for each public API function.
In the end, make sure that all tests work as expected by running `cargo test`.

## GitHub

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.

## Git

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.

## Versioning

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.
