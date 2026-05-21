# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in CBOR-Web, please report it responsibly.

### How to Report

**DO NOT** create a public GitHub issue for security vulnerabilities.

Instead, please use GitHub's private vulnerability reporting feature:

1. Go to the **Security** tab of this repository
2. Click on **Report a vulnerability**
3. Provide details about the vulnerability

Or contact us directly at: **security@ploteddie-bit.github.io** (if available)

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)
- CBOR file example (if applicable)

### Response Time

We aim to acknowledge all security reports within **48 hours**.

### Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 3.0.x   | :white_check_mark: |
| < 3.0   | :x:                |

## Security Considerations

CBOR-Web is designed with security in mind:

1. **No executable content**: CBOR files contain only structured data - no code execution
2. **Self-describing format**: Tag 55799 ensures format verification and prevents confusion attacks
3. **Immutable content**: Files are pre-generated, not dynamically executed on the server
4. **No client-side code**: No JavaScript execution required for parsing
5. **Binary format**: Prevents injection attacks common in text-based formats
6. **Strict typing**: CBOR's type system prevents type confusion vulnerabilities

### Known Limitations

- Large CBOR files could potentially cause memory issues in parsers (DoS risk)
- Always validate CBOR files from untrusted sources before parsing
- Use streaming parsers for very large files when possible

For more details, see the full [CBOR-Web Specification](https://github.com/ploteddie-bit/cbor-web).

## Acknowledgments

We appreciate responsible disclosure and will credit researchers who report valid security issues (with permission).

---

*Last updated: 2026*
