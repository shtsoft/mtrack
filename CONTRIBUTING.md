# Contributing to mtrack

Contributions are welcome!
But please read this guide before.

## Code

If you plan to contribute code please take the respective CONTRIBUTING.md into account:

- [backend](backend/CONTRIBUTING.md)
- [frontend](frontend/CONTRIBUTING.md)

## GitHub

This project is hosted on GitHub.
In the folllowing it is briefly described how this project intends to leverage GitHub for development.

### Issues

GitHub Issues is how contributors shall communicate.
Issue-templates are provided to type communication a bit.
But in doubt, feel free to use blank issues.

### Workflows

When making a contribution keep an eye on the [workflows](.github/workflows).

In a nutshell, what they do is

- Audit-Backend
  * checking for known bugs in dependencies
- Audit-Frontend
  * checking for known bugs in dependencies
- CI-Backend
  * checking if tests run
  * checking for memory leak
  * checking format
  * checking for lints
  * checking documentation
- CI-Frontend
  * checking if tests run
  * checking for lints

### Pull Requests

As usual, if you can not directly push to the repository use pull requests for contributions.
A pull request-template is provided to assist you.
Otherwise try to keep pull requests small and focused (like a topic-branch - see [Git](#git)).
In any case, take all the other sections into account to ease the pain of merging your pull request.

## Git

This project uses git for version control and contributors are kindly asked to use git as described in the [git-book](https://git-scm.com/book/en/v2).
In the following, the most important aspects are recounted and a few things are nailed down.

### Commits

Commit in 'sensible' units.
In particular, do NOT
- misuse commits for your backup-strategy.
- rewrite your commit history after pushing.

Moreover run `git diff --check` before a commit to check for whitespace errors.

#### Messages

A commit message is written in the imperative (e.g.: 'Fix bug' instead of 'Fixed bug').
The first line of a commit message is a short description of the changeset in less than 50 characters.
Optionally, it can be followed by a blank second line and a more detailed explanatory text starting at the third line.
The text should i.p. motivate the change and explain the difference.

### Branching

This project's branching workflow is 'progressive-stability branching'.
This means that there is a permanent `development`-branch (organizing development) in addition to the `master`-branch.
`development` is merged into master whenever it is in a stable state.
Actual development takes place in so-called topic-branches (e.g. `iss4` to handle the fourth issue) emanating from `development`.
The topic-branches are merged back and deleted when the topic is treated.

## Versioning

This project uses [Semantic Versioning](https://semver.org) - read that if you do not already know it.
When `development` is in a state such that [Semantic Versioning](https://semver.org) requires a version-action then edit the `version`-field in the package configuration of all

- [backend](backend/Cargo.toml)
- [frontend](frontend/package.json)

accordingly and make a commit with message 'Release version X.Y.Z'.
Afterwards tag the commit (`git tag vX.Y.Z -m 'Release version X.Y.Z'`) and merge development back into master.
Finally, make a release on GitHub.
