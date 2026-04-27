# mtrack

[![CI-Backend][actions-badge-backend]][actions-url-backend]
[![CI-Frontend][actions-badge-frontend]][actions-url-frontend]

[actions-badge-backend]: https://github.com/shtsoft/mtrack/actions/workflows/ci-backend.yaml/badge.svg
[actions-url-backend]: https://github.com/shtsoft/mtrack/actions/workflows/ci-backend.yaml
[actions-badge-frontend]: https://github.com/shtsoft/mtrack/actions/workflows/ci-frontend.yaml/badge.svg
[actions-url-frontend]: https://github.com/shtsoft/mtrack/actions/workflows/ci-frontend.yaml

A web app that allows 'download' class clients to track the positions of 'upload' class clients.
The entry points are:

- `https://domain.tld:PORT` for download users.
- `https://domain.tld:PORT/postpos` for upload users.

## Deployment

To deploy mtrack, first clone the repo.
Then, follow the installation and usage instructions for the [backend](backend) and [frontend](frontend).

## Contributing

If you want to contribute: [CONTRIBUTING](CONTRIBUTING.md).

### Security

For security-related issues see: [SECURITY](SECURITY.md).
