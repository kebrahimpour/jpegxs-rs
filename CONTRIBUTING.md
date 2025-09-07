# Contributing to JPEG XS Codec

Thank you for your interest in contributing to our JPEG XS codec implementation! This guide outlines our development workflow, coding standards, and contribution process.

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+
- Git
- GitHub account
- Familiarity with JPEG XS standard (helpful but not required)

### Setup

```bash
git clone https://github.com/kebrahimpour/jpegxs-rs.git
cd jpegxs-rs
cargo build --all-features
cargo test --all-features
```

## 🔄 Development Workflow

We use a **feature branch + Pull Request workflow**. All contributions must go through pull requests.

### Step 1: Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/jpegxs-rs.git
   cd jpegxs-rs
   git remote add upstream https://github.com/kebrahimpour/jpegxs-rs.git
   ```

### Step 2: Create Feature Branch

Create a new branch from `main` for your work:

```bash
git checkout main
git pull upstream main
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-description
# or
git checkout -b docs/documentation-improvement
```

**Branch naming conventions:**
- `feature/` - New features
- `fix/` - Bug fixes  
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `perf/` - Performance improvements
- `test/` - Test additions/improvements
- `chore/` - Maintenance tasks

### Step 3: Make Changes

- Write your code following our coding standards (see below)
- Add tests for new functionality
- Update documentation as needed
- Follow conventional commit format (see below)

### Step 4: Test Locally

Before opening a PR, ensure all checks pass locally:

```bash
# Run tests
cargo test --all-features

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Run security audit
cargo audit

# Build documentation
cargo doc --all-features --no-deps
```

### Step 5: Open Pull Request

1. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Open a Pull Request on GitHub using our PR template
3. Fill out all sections of the PR template completely
4. Request review from maintainers

## 📝 Coding Standards

### Rust Code Style

- **Formatting**: Use `rustfmt` with project settings (`cargo fmt`)
- **Linting**: Address all `clippy` warnings (`cargo clippy`)
- **Naming**: Follow Rust naming conventions
  - `snake_case` for functions, variables, modules
  - `PascalCase` for types, traits, enums
  - `SCREAMING_SNAKE_CASE` for constants
- **Documentation**: Add doc comments for all public APIs
- **Error Handling**: Use `Result<T, E>` and proper error types
- **Safety**: Minimize `unsafe` code, document when necessary

### Code Organization

- Keep functions focused and reasonably sized
- Use meaningful variable and function names
- Group related functionality in modules
- Write self-documenting code with clear intent
- Add inline comments for complex algorithms

### Testing Requirements

- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test complete workflows
- **Property tests**: Use property-based testing for codec correctness
- **Benchmarks**: Include performance tests for critical paths
- **Coverage**: Aim for high test coverage on new code

### Dependencies

- Prefer standard library when possible
- Minimize external dependencies
- Use well-maintained, security-audited crates
- Document rationale for new dependencies in PR

## 📋 Commit Message Convention

We use **Conventional Commits** format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring without behavior changes
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks, dependency updates
- `ci`: CI/CD changes
- `build`: Build system changes

### Examples

```bash
# Good commit messages
feat(encoder): add SIMD acceleration for DWT transform
fix(decoder): handle malformed bitstream gracefully
docs(api): add usage examples for CLI interface
perf(core): optimize memory allocation in codec pipeline
test(integration): add roundtrip tests for all quality levels
chore: update dependencies to latest versions

# Bad commit messages
fix stuff
add feature
update
wip
```

### Commit Best Practices

- Use imperative mood ("add" not "added")
- Keep first line under 72 characters
- Provide context in body for non-trivial changes
- Reference issues when applicable (`fixes #123`)
- Make atomic commits (one logical change per commit)

## 🧪 Local Testing and Linting

### Essential Commands

```bash
# Run all tests with full feature set
cargo test --all-features

# Run tests for specific package
cargo test -p jpegxs-core

# Run integration tests
cargo test --test integration

# Format code
cargo fmt --all

# Check formatting without changing files
cargo fmt --all -- --check

# Run clippy linter
cargo clippy --all-targets --all-features

# Run clippy with strict warnings
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
cargo audit

# Build documentation
cargo doc --all-features --no-deps --open
```

### Pre-commit Hooks

We recommend setting up pre-commit hooks:

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install hooks
pre-commit install

# Run hooks manually
pre-commit run --all-files
```

### Performance Testing

```bash
# Run benchmarks
cargo bench

# Profile specific benchmark
cargo bench --bench codec_performance

# Run with profiling
cargo build --release
perf record target/release/jpegxs encode input.png output.jxs
```

## ✅ Pull Request Checklist

Before submitting your PR, ensure you've completed our PR template checklist:

### Code Quality
- [ ] Code follows project lint/format guidelines (`cargo fmt`, `cargo clippy`)
- [ ] All tests are passing (`cargo test --all-features`)
- [ ] New functionality includes appropriate tests
- [ ] Documentation updated (if applicable)
- [ ] Security audit passes (`cargo audit`)
- [ ] No new warnings introduced

### PR Requirements
- [ ] PR template is completely filled out
- [ ] Purpose section clearly describes the change
- [ ] Approach section explains technical implementation
- [ ] Tests section describes testing methodology
- [ ] Risks section identifies potential issues
- [ ] Migration/Docs section covers impact on users
- [ ] All checklist items are addressed

### Review Process
- [ ] Self-review completed
- [ ] Ready for maintainer review
- [ ] Responsive to feedback
- [ ] Squash commits if requested

**Reference**: See our [Pull Request Template](.github/PULL_REQUEST_TEMPLATE.md) for the complete structure your PR should follow.

## 🎯 Types of Contributions

### Code Contributions
- **Performance optimizations** (SIMD, memory usage, algorithmic improvements)
- **New features** (see FUTURE-IDEAS.md for roadmap)
- **Bug fixes** and stability improvements  
- **Platform support** (Windows, ARM, embedded systems)
- **Testing and validation** (more test cases, edge case handling)

### Non-Code Contributions
- **Documentation** improvements and examples
- **Issue reporting** with reproducible test cases
- **Performance benchmarks** and analysis
- **Code reviews** and feedback
- **Community support** in discussions

## 🏗️ Project Structure

```
jpegxs-rs/
├── crates/
│   ├── jpegxs-core/     # Core codec implementation
│   ├── jpegxs-cli/      # Command-line interface
│   ├── jpegxs-io/       # I/O utilities and bit operations
│   └── jpegxs-ffi/      # FFI bindings for cross-validation
├── commercial/
│   └── jpegxs-core-clean/  # Clean-room implementation
├── tests/               # Integration tests
├── benches/             # Performance benchmarks
├── test-data/           # Test images and validation files
├── tools/               # Development and validation scripts
└── .github/             # GitHub workflows and templates
```

## 💡 Good First Issues

Looking for easy ways to contribute? Try:

- **Adding test cases** for edge conditions
- **Improving error messages** with more context
- **Writing examples** and tutorials
- **Optimizing memory allocations** in hot paths
- **Adding documentation comments** to public APIs
- **Fixing clippy warnings** and improving code quality
- **Adding benchmark cases** for performance tracking

## 🔒 Security Considerations

- Report security vulnerabilities through GitHub Security Advisories
- Be cautious with unsafe code - document thoroughly
- Validate all inputs, especially in codec parsing
- Consider fuzzing for new parsing code
- Run `cargo audit` regularly

## 📄 License

By contributing, you agree that your contributions will be licensed under the same terms as the project. See [LICENSE](LICENSE) and [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md) for details.

## 📧 Questions and Support

- **Issues**: Open a GitHub issue for bugs or feature requests
- **Discussions**: Use GitHub Discussions for questions
- **Email**: k1.ebrahimpour@gmail.com for complex questions
- **Documentation**: Check README.md and DEVELOPMENT-SUMMARY.md

## 🎉 Recognition

Contributors are acknowledged in:
- Changelog updates and release notes
- Project documentation and README
- GitHub contributor graphs
- Special recognition for significant contributions

We appreciate all contributions, no matter how small!

## 📚 Additional Resources

- [JPEG XS Standard (ISO/IEC 21122)](https://www.iso.org/standard/74504.html)
- [Project Roadmap](FUTURE-IDEAS.md)
- [Development Summary](DEVELOPMENT-SUMMARY.md)
- [Security Policy](SECURITY.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
