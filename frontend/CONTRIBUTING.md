# Contributing to mtrack/frontend

Contributions are welcome!
But please read this guide before.

## Code

If you plan to contribute code please take the following subsections into account.

### Environment

The project expects you to have typescript and typescript-language-server installed globally.
The other dependencies can be installed locally with `npm install`.

### Implementation

Pay attention to security, usability and maintainability (roughly in that order).
Generally, try to adhere to the coding-style of the current implementation to keep reading the code easy.
Specifically, make sure that the linter does not complain too much by running `npm run lint`.

### Documentation

Document modules, interfaces and methods (functions declared as part of an interface) as well as public comatches (functions using `this` to generate an object) and public functions adhering to the current style.
For interfaces this means to write a description of what it defines (intensionally).
For methods, comatches and functions this means to write a 'contract' made up by:
1. a description of what the method/comatch/function does
2. a description of the arguments (if there are any)
3. a description of the side effects (if there are any)
4. a description of the preconditions, postconditions and invariants (if there are any)
5. additional information interesting to callers (if there is some)

### Testing

Add tests as necessary adhering to the current style. 
The guideline is to only test the public APIs of modules.
In the end, make sure that
- all tests work as expected by running `npm run test`.
- all pages are served as expected by `npm run serve` after installing with `npm run build`.

## GitHub

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.

## Git

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.

## Versioning

Please take the respective section of the [CONTRIBUTING.md](../CONTRIBUTING.md) at the top-level of the repo into account.
