# Contributing to mtrack/frontend

Contributions are welcome!
Please read this guide before getting started.

## Code

If you plan to contribute code please take the following subsections into account.

### Environment

The project expects you to have typescript and typescript-language-server installed globally.
The other dependencies can be installed locally with `npm install`.

### Implementation

Pay attention to security, usability, maintainability and performance (roughly in that order).
Generally, try to adhere to the coding-style of the current implementation to keep the code easy to read.
Specifically, make sure that the linter does not complain too much by running `npm run lint`.

### Documentation

Document modules, interfaces and methods (functions declared as part of an interface) as well as public comatches (functions using `this` to generate an object) and public functions adhering to the current style.
For interfaces this means to write a description of what they define (intensionally).
For methods, comatches and functions this means writing a 'contract' consisting of:
1. a description of what the method/comatch/function does
2. a description of the arguments (if any)
3. a description of the side effects (if any)
4. a description of the preconditions, postconditions and invariants (if any)
5. additional information relevant to callers (if there is some)

### Testing

Add tests as necessary adhering to the current style. 
The guideline is to only test the public APIs of modules.
Finally, make sure that
- all tests work as expected by running `npm run test`.
- all pages are served as expected by `npm run serve` after installing with `npm run build`.

## GitHub

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.

## Git

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.

## Versioning

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.
