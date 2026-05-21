# Contributing to CBOR-Web

Thank you for considering contributing to CBOR-Web! This guide will help you get started.

## 🎯 How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues as you might find out that you're not the first one to report it.

**When creating a bug report, include:**

- A clear and descriptive title
- Description of what you expected vs what happened
- Steps to reproduce the issue
- The CBOR file URL or attach the file if possible
- Your environment (parser library, version, language)

👉 Use the [Bug Report Template](?template=bug_report.md) when creating an issue.

### Suggesting Features

Feature requests are welcome! Before submitting, please:

1. Check if the feature already exists or has been requested
2. Consider if it aligns with the project's goals (simplicity, AI-agent friendly)
3. Think about backwards compatibility

**When creating a feature request, include:**

- Clear description of the problem you're solving
- Proposed solution
- Alternative solutions you've considered
- Use case (who benefits: AI agents, humans, both?)

👉 Use the [Feature Request Template](?template=feature_request.md) when creating an issue.

### Submitting Changes

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes**
4. **Validate** your CBOR files (see below)
5. **Commit** with clear messages:
   ```bash
   git commit -m "Add amazing feature: description"
   ```
6. **Push** to your fork:
   ```bash
   git push origin feature/amazing-feature
   ```
7. **Open a Pull Request** on GitHub

### CBOR Validation

Before submitting any CBOR files, ensure they are valid:

```bash
# Install cbor2 if you haven't
pip install cbor2

# Validate syntax
python -m cbor2.tool your-file.cbor

# Or use Python directly
python -c "import cbor2; cbor2.load(open('your-file.cbor', 'rb'))"
```

**Required structure:**
- Root must be a CBOR map
- Must contain keys: 0 (type), 1 (version), 2 (metadata), 3 (pages)
- Must use tag 55799 (self-described CBOR)

### Code Style Guidelines

- **Keep it simple**: CBOR-Web prioritizes simplicity over features
- **Follow RFC 8949**: All CBOR encoding must comply with the standard
- **Document new block types**: Add examples in README if introducing new blocks
- **Test with multiple parsers**: Ensure compatibility across languages

## 📋 Development Setup

### Prerequisites

- Python 3.8+ (for validation tools)
- Git

### Installation

```bash
# Clone your fork
git clone https://github.com/YOUR-USERNAME/cbor-web-read.git
cd cbor-web-read

# Install dependencies
pip install cbor2 jsonschema

# Run validation
python -c "
import cbor2
from pathlib import Path
for f in Path('examples').glob('*.cbor'):
    data = cbor2.load(open(f, 'rb'))
    print(f'✓ {f.name}')
"
```

## ❓ Questions?

- **General questions**: Start a [Discussion](../../discussions/categories/q-a)
- **Feature ideas**: Discuss in [Ideas category](../../discussions/categories/ideas) before opening an issue
- **Show your implementation**: Post in [Show and tell](../../discussions/categories/show-and-tell)

## 🏷️ Pull Request Process

1. Ensure all CBOR files validate correctly
2. Update documentation if needed (README, examples)
3. Add yourself to contributors if it's your first PR
4. Be responsive to review feedback
5. Squash commits if you have many small fixes

## 📜 License

By contributing to CBOR-Web, you agree that your contributions will be licensed under **CC0 1.0 Universal** (Public Domain). This means anyone can use your contributions for any purpose without attribution.

## 🙏 Thank You!

Your contributions make CBOR-Web better for everyone. Whether it's a typo fix, a new example, or a major feature, we appreciate your time and effort!

---

**Need more help?** Check out:
- [README.md](../README.md) - Project overview
- [STORY.md](../STORY.md) - Vision and background
- [GITHUB_SETUP_GUIDE.md](../GITHUB_SETUP_GUIDE.md) - Repository configuration

*Last updated: 2026*
