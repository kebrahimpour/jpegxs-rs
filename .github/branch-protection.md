# GitHub Branch Protection Configuration

## Main Branch Protection Rules

After creating the repository, configure the following branch protection rules for `main`:

### Required Settings

1. **Require a pull request before merging**
   - Require approvals: 1
   - Dismiss stale pull request approvals when new commits are pushed
   - Require review from CODEOWNERS
   - Require approval of the most recent reviewable push

2. **Require status checks to pass before merging**
   - Require branches to be up to date before merging
   - Required status checks:
     - `lint`
     - `test (ubuntu-latest, stable)`
     - `test (macos-latest, stable)`
     - `build (ubuntu-latest)`
     - `security`

3. **Require conversation resolution before merging**

4. **Require linear history**

5. **Include administrators**

6. **Restrict who can push to matching branches**
   - Add: kebrahimpour

### GitHub Copilot Integration

1. Enable Copilot for pull request summaries
2. Enable Copilot code review suggestions
3. Configure Copilot to review:
   - Code quality
   - Security issues
   - Performance concerns
   - Best practices

### Automated Review Setup

1. Install GitHub Copilot app for the repository
2. Configure `.github/copilot-config.yml`:

```yaml
review:
  auto_review_enabled: true
  review_level: thorough

suggestions:
  auto_suggest_improvements: true

security:
  scan_for_secrets: true
  scan_for_vulnerabilities: true
```

## Development Branch Protection

For `develop` branch:

1. **Require a pull request before merging**
   - No approval required (for rapid iteration)

2. **Require status checks to pass**
   - Same as main branch

3. **Do not require linear history** (allow merge commits)

## Setting Up Protection via CLI

```bash
# After creating the repository
gh repo edit --enable-issues --enable-wiki=false
gh repo edit --default-branch main

# Note: Branch protection rules must be configured via GitHub web UI
echo "Please configure branch protection rules at:"
echo "https://github.com/kebrahimpour/jpegxs-rs/settings/branches"
```
