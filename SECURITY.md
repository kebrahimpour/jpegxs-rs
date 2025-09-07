# Security Policy

## Reporting Security Vulnerabilities

We take the security of jpegxs-rs seriously. If you believe you have found a security vulnerability in this project, please report it to us as described below.

### Preferred Reporting Method

**Please use GitHub Security Advisories to report security vulnerabilities.** This is the preferred method as it allows for private disclosure and coordination.

1. Go to the [Security tab](https://github.com/kebrahimpour/jpegxs-rs/security) of this repository
2. Click on "Report a vulnerability"
3. Fill out the advisory form with details about the vulnerability
4. Submit the report

### Alternative Reporting Method

If you cannot use GitHub Security Advisories, you may also report vulnerabilities via email:
- Email: k1.ebrahimpour@gmail.com
- Subject: [SECURITY] jpegxs-rs vulnerability report

### What to Include

When reporting a security vulnerability, please include:
- A description of the vulnerability
- Steps to reproduce the issue
- Affected versions
- Impact assessment
- Any potential mitigations or workarounds
- Your contact information (if you want to be credited)

## Response Timeline

We are committed to addressing security vulnerabilities promptly:

- **Initial Response**: Within 48 hours of receiving the report
- **Investigation**: We will investigate and assess the vulnerability within 7 days
- **Resolution**: For confirmed vulnerabilities:
  - Critical/High severity: Patch within 14 days
  - Medium severity: Patch within 30 days
  - Low severity: Patch in next regular release cycle
- **Disclosure**: Public disclosure will be coordinated with the reporter

## Security Best Practices

### For Users

- Always use the latest stable version of jpegxs-rs
- Validate input data before processing with the codec
- Run the codec in sandboxed environments when processing untrusted data
- Monitor for security updates and apply them promptly

### For Contributors

- Follow secure coding practices
- Validate all inputs and handle errors gracefully
- Use memory-safe Rust patterns and avoid unsafe code when possible
- Run security audits on dependencies regularly
- Include security considerations in pull request descriptions

## Security Features

- **Memory Safety**: Built in Rust with memory safety guarantees
- **Input Validation**: Robust validation of JPEG XS bitstreams
- **Error Handling**: Graceful handling of malformed or malicious input
- **Dependency Management**: Regular security audits of dependencies

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| Latest  | :white_check_mark: |
| < 1.0   | :x:                |

*Note: As this is a pre-1.0 project, we currently only support the latest development version.*

## Security Audit

This project includes automated security auditing:
- Dependency vulnerability scanning via `cargo audit`
- Regular updates of security-sensitive dependencies
- Static analysis tools integration in CI/CD pipeline

## Acknowledgments

We appreciate security researchers and users who responsibly report vulnerabilities. Contributors who report valid security issues will be acknowledged in our security advisories (with their permission).

## Questions

If you have questions about this security policy or need clarification on any security-related matters, please contact k1.ebrahimpour@gmail.com.
