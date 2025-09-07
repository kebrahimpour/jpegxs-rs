# Documentation Structure

## Public Documentation (in repository root)

All public-facing documentation is maintained in the repository root:

- `README.md` - Main project documentation
- `LICENSE` - Dual licensing terms
- `COMMERCIAL_LICENSE.md` - Commercial licensing details
- `LICENSE-COMMERCIAL.md` - Commercial license agreement
- `LICENSE-REQUEST.md` - Commercial license request template
- `PATENT_NOTICE.md` - Essential patent requirements
- `COMPLIANCE_REPORT.md` - ISO/IEC 21122-1:2024 compliance
- `LICENSING_FAQ.md` - Licensing questions and answers
- `LICENSING-GUIDE.md` - Clean-room vs reference guide
- `DEVELOPMENT-SUMMARY.md` - Development overview
- `FUTURE-IDEAS.md` - Roadmap and enhancement ideas

## Private Documentation (local only)

The following directories are gitignored and kept locally:

### `/docs/` - ISO Standard Documents
- Contains extracted ISO/IEC 21122-1:2024 standard
- Copyrighted material - not for public distribution
- Used for reference during clean-room implementation
- **STATUS**: Excluded from public repository

### `/internal_docs/` - Development Documentation
- Session summaries and handoffs
- Development TODOs and planning
- Test results and validation logs
- Work-in-progress notes
- **STATUS**: Excluded from public repository

## API Documentation

API documentation can be generated from source:

```bash
cargo doc --no-deps --open
```

This generates documentation for:
- `jpegxs-core` - Core codec implementation
- `jpegxs-cli` - Command-line interface
- `jpegxs-io` - I/O utilities
- `jpegxs-core-clean` - Clean-room DWT

## Commercial Documentation

For commercial customers, the following documents are essential:
1. `COMMERCIAL_LICENSE.md` - License terms and tiers
2. `PATENT_NOTICE.md` - Patent pool requirements
3. `COMPLIANCE_REPORT.md` - Technical compliance details
4. `LICENSE-REQUEST.md` - How to request a license

## Contributing Documentation

When contributing to the project:
- Update `README.md` for user-facing changes
- Add technical details to inline code documentation
- Use `internal_docs/` for development notes (not committed)
- Keep ISO standard references in `/docs/` (not committed)