# Contributing to mtrack/backend

Contributions are welcome!
Please read this guide before getting started.

## Code

If you plan to contribute code please take the following subsections into account.

### Implementation

Pay attention to security, usability, maintainability and performance (roughly in that order).
Generally, try to adhere to the coding-style of the current implementation to keep the code easy to read.
Specifically
- make sure that the code is formatted correctly by running `cargo fmt --all -- --check`.
- make sure that the linter does not complain too much by running `cargo clippy --all --benches --examples --tests --all-features` followed by `-- -W clippy::pedantic` or `-- -W clippy::nursery` if the output will be interpreted very carefully.

### Documentation

Document items (regardless of visibility) descending from the library-crate adhering to the current style.
For enums and traits this means writing a description of what they define (extensionally and intensionally, respectively) while for structs it means describing what they structure.
For function-like items this means writing a 'contract' consisting of:
1. a description of what the item does
2. a description of the arguments (if any)
3. a description of the side effects (if any)
4. a description of the preconditions, postconditions, and invariants (if any)
5. additional information relevant to callers (if there is some)
Furthermore
- for constructors, state what they construct.
- for structs, explain what their fields represent.
Only do this where it makes sense to do so.
Finally, make sure that the docs build by running `cargo doc --no-deps --document-private-items`.

### Testing

If you add code then also add tests as necessary.
Both integration- and unit-tests.
It is also suggested to include one documentation-test for each public API function.
Finally, make sure that all tests work as expected by running `cargo test`.

## GitHub

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.

## Git

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.

## Versioning

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.
