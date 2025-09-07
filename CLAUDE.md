- no ai generated signatures in comments and commit messages

# Internal Documentation Management

## Session Documentation
- All session summaries, TODOs, and internal planning documents should be stored in the `internal_docs/` folder
- This folder is gitignored and will not be committed to the repository
- When creating session documentation, always use paths like:
  - `internal_docs/SESSION-SUMMARY-{date}.md`
  - `internal_docs/NEXT-SESSION-PLAN.md`
  - `internal_docs/TODO-{topic}.md`
  - `internal_docs/validation-results/`

## Guidelines for Internal vs Public Documentation
- **Internal (goes in internal_docs/):**
  - Session summaries and handoffs
  - Development TODOs and planning
  - Test results and validation logs
  - Any temporary or work-in-progress notes
  
- **Public (goes in root or docs/):**
  - README.md - Main project documentation
  - DEVELOPMENT-SUMMARY.md - High-level development overview
  - FUTURE-IDEAS.md - Public roadmap and enhancement ideas
  - API documentation
  - User guides and tutorials

## Reading/Writing Session Files
- When starting a new session, check for existing session files in `internal_docs/`
- When ending a session, save summary to `internal_docs/SESSION-SUMMARY-{date}.md`
- Keep the `internal_docs/` folder organized with subdirectories as needed