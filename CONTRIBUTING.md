# Contributing to mtrack

Contributions are welcome!
Please read this guide before getting started.

## Code

If you plan to contribute code please take the respective CONTRIBUTING.md into account:

- [backend](backend/CONTRIBUTING.md)
- [frontend](frontend/CONTRIBUTING.md)

## GitHub

This project is hosted on GitHub.
The following section describes how this project leverages GitHub for development.

### Issues

GitHub Issues is the primary way for contributors to communicate.
Issue-templates are provided to type communication a bit.
But when in doubt, feel free to use blank issues.

### Workflows

When making a contribution, keep an eye on the [workflows](.github/workflows).

In a nutshell, what they do is

- Audit-Backend
  * checking for known bugs in dependencies
- Audit-Frontend
  * checking for known bugs in dependencies
- CI-Backend
  * checking that tests run
  * checking for memory leaks
  * checking code formatting
  * checking for lints
  * checking documentation
- CI-Frontend
  * checking that tests run
  * checking for lints

### Pull Requests

If you cannot push directly to the repository, please use pull requests for contributions.
A pull request-template is provided to assist you.
Otherwise try to keep pull requests small and focused (like a topic-branch - see [Git](#git)).
In any case, keep the other sections in mind to make merging your pull request as smooth as possible.

## Git

This project uses git for version control and contributors are kindly asked to use git as described in the [git-book](https://git-scm.com/book/en/v2).
The most important aspects are outlined below.

### Commits

Commit in 'sensible' units.
In particular, do NOT
- misuse commits as a backup-strategy.
- rewrite your commit history after pushing.

Additionally, run `git diff --check` before a commit to check for whitespace errors.

#### Messages

Commit messages should be written in the imperative mood (e.g.: 'Fix bug' instead of 'Fixed bug').
The first line should be a short description of the changeset in fewer than 50 characters.
Optionally, this can be followed by a blank second line and a more detailed explanatory text starting on the third line.
The text should motivate the change and explain the differences.

### Branching

This project uses a 'progressive-stability' branching workflow.
This means there is a permanent `development`-branch (organizing development) in addition to the `master`-branch.
`development` is merged into master whenever it is in a stable state.
Actual development takes place in so-called topic-branches (e.g.: `iss4` to handle the fourth issue) branching off from `development`.
Topic-branches are merged back and deleted once the task is completed.

## Versioning

This project uses [Semantic Versioning](https://semver.org) - read it if you do not already know it.
When `development` reaches a state where [Semantic Versioning](https://semver.org) requires a version-action, edit the `version`-field in the package configuration of all

- [backend](backend/Cargo.toml)
- [frontend](frontend/package.json)

accordingly and make a commit with the message 'Release version X.Y.Z'.
Afterward tag the commit (`git tag vX.Y.Z -m 'Release version X.Y.Z'`) and merge development back into master.
Finally, make a release on GitHub.
