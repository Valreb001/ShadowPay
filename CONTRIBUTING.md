# Contributing to ShadowPay

Thank you for your interest in contributing! This document outlines our guidelines.

## Branch Naming

- Feature: `feature/description`
- Bug fix: `fix/description`
- Docs: `docs/description`
- Chore: `chore/description`

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): subject

body

footer
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
- `feat(contract): add pause/unpause functionality`
- `fix(proof): validate proof length correctly`
- `docs(readme): update deployment instructions`

## Pull Request Workflow

1. Create a feature branch from `develop`
2. Make your changes and commit with conventional messages
3. Push to your fork and open a PR against `develop`
4. Ensure all CI checks pass
5. Request review from maintainers
6. Merge after approval

## Testing

- Write tests for all new features
- Run `cargo test` before submitting PR
- Aim for >80% code coverage

## Code Style

- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Follow Rust naming conventions

## Security

- Never commit `.env` files or secret keys
- Report vulnerabilities privately via GitHub Security Advisories
- Use `cargo audit` to check dependencies
