# Contributing to VitaStellar Contracts

Thank you for your interest in contributing to VitaStellar Contracts! This document provides guidelines and standards for contributing to the project.

## Security

**Found a vulnerability?** Do not open a public issue. Please read our [Security Policy](./.github/SECURITY.md) for the responsible disclosure process, severity classification matrix, response SLAs, and bug bounty program. Report vulnerabilities to **security@vitastellar.io** or via [GitHub Private Security Advisory](https://github.com/0xVida/VitaStellar-Contracts/security/advisories/new).

## Development Workflow

1. **Fork the repository** and create a feature branch
2. **Follow coding standards** outlined in [CODING_STANDARDS.md](./docs/CODING_STANDARDS.md)
3. **Write tests** for new functionality
4. **Review the contract review checklist** in [docs/contract-review-checklist.md](./docs/contract-review-checklist.md)
5. **Run linting and tests** before submitting
6. **Submit a pull request** with a clear description

## Code Quality Standards

### Naming Conventions
All code must follow the naming conventions defined in [CODING_STANDARDS.md](./docs/CODING_STANDARDS.md) and [CONTRACT_NAMING_CONVENTIONS.md](./docs/CONTRACT_NAMING_CONVENTIONS.md):

- **Functions**: `snake_case`
- **Types**: `PascalCase`  
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`
- **Error enums**: Always use `Error`, never `Err`
- **File names**: `snake_case` (e.g., `lib.rs`, `errors.rs`, `types.rs`, `events.rs`, `test.rs`)
- **Contract directories**: `snake_case`

Run the naming check script before submitting PRs:
```bash
bash scripts/check-naming.sh
```

### Code Style
- Use Rust 2021 edition
- Follow Rustfmt formatting (run `cargo fmt`)
- Adhere to Clippy linting rules (run `cargo clippy`)

### Documentation
- Document all public APIs with `///` doc comments
- Include examples for complex functions
- Update relevant documentation when changing functionality

## Testing Requirements

### Unit Tests
- Write tests for all new functionality
- Test edge cases and error conditions
- Mock external dependencies where appropriate

### Integration Tests
- Test contract interactions
- Verify cross-contract calls work correctly
- Ensure upgrade paths are tested

## Pull Request Process

1. **Ensure code compiles** without warnings
2. **Run all tests** and verify they pass
3. **Update documentation** if needed
4. **Describe changes** in the PR description
5. **Link related issues** if applicable

### PR Description Format

Every PR that resolves one or more issues **must** include a closing keyword in the PR **body** (not just the title):

- `Closes #N`
- `Fixes #N`
- `Resolves #N`

GitHub only auto-closes issues when these keywords appear in the PR description body.
Without them, issues remain open after the fix lands and require manual cleanup.

See the [GitHub documentation on closing keywords](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue#linking-a-pull-request-to-an-issue-using-a-keyword) for the full list of supported keywords and placement rules.

Example PR body:
```
Closes #42
Closes #43
```

### PR Review Checklist
- [ ] Code follows naming conventions
- [ ] Tests are included and pass
- [ ] Documentation is updated
- [ ] No new Clippy warnings
- [ ] Code is properly formatted
- [ ] Contract review checklist items have been considered for correctness, safety, and testing
- [ ] If the change touches authentication, access control, cryptography, or financial logic, confirm no new vulnerabilities are introduced (see [Security Policy](./.github/SECURITY.md))

## Contract Review Checklist
Use the shared contract review checklist for all smart contract and contract-related pull requests: [docs/contract-review-checklist.md](./docs/contract-review-checklist.md)

## Development Setup

### Prerequisites
- Rust toolchain (stable)
- Soroban CLI
- Cargo make (optional)

### Local Development
```bash
# Clone the repository
git clone <repository-url>
cd VitaStellar-Contracts

# Install dependencies
cargo build

# Run tests
cargo test

# Run linter
cargo clippy -- -D warnings

# Format code
cargo fmt
```

### CI/CD Pipeline
The project uses GitHub Actions for CI/CD. The pipeline includes:
- Build verification
- Linting (Clippy)
- Testing
- Formatting check

## Getting Help
- Review existing documentation in the `docs/` directory
- Check open issues for known problems
- Looking for where to start? Run `make first-issue` to see a list of beginner-friendly tasks!
- Ask questions in pull request discussions

## License
By contributing, you agree that your contributions will be licensed under the project's MIT license.