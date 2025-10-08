# Branch Protection Rules

This document describes how to configure GitHub branch protection rules to enforce CI checks before merging to the main branch.

## Prerequisites

- Repository admin access
- GitHub Actions workflows configured (ci.yml and pr.yml)

## Required Protection Rules for `main` Branch

### Step 1: Navigate to Branch Protection Settings

1. Go to your repository on GitHub
2. Click **Settings** > **Branches**
3. Under "Branch protection rules", click **Add rule**
4. Enter `main` in the "Branch name pattern" field

### Step 2: Configure Protection Rules

Enable the following settings:

#### ✅ Require a pull request before merging
- **Require approvals**: 1 (recommended)
- **Dismiss stale pull request approvals when new commits are pushed**: ✓
- **Require review from Code Owners**: ✓ (if you have a CODEOWNERS file)

#### ✅ Require status checks to pass before merging
- **Require branches to be up to date before merging**: ✓

**Required status checks** (must pass before merging):
- `CI Success` (from ci.yml)
- `Check Formatting` (from ci.yml)
- `Clippy Lints` (from ci.yml)
- `Build (x86_64-unknown-uefi)` (from ci.yml)
- `Build (aarch64-unknown-uefi)` (from ci.yml)
- `Build (i686-unknown-uefi)` (from ci.yml)
- `Run Tests` (from ci.yml)
- `PR Ready for Review` (from pr.yml)

**Note**: Status checks will only appear in the list after they run at least once. Make a test PR to populate the list.

#### ✅ Require conversation resolution before merging
This ensures all review comments are addressed.

#### ✅ Require signed commits (Optional but recommended)
Ensures commit authenticity.

#### ✅ Require linear history (Optional)
Prevents merge commits, enforces rebase or squash merges.

#### ✅ Do not allow bypassing the above settings
Applies rules to administrators as well.

### Step 3: Additional Recommended Settings

#### Include administrators
- ✓ **Include administrators**: Ensures even admins must follow the rules

#### Restrictions (Optional)
- **Restrict who can push to matching branches**: Limit to specific users/teams
- **Allow force pushes**: ✗ (disabled)
- **Allow deletions**: ✗ (disabled)

## Workflow Overview

### Main CI Workflow (.github/workflows/ci.yml)

**Triggers**: Push to main/develop, pull requests to main
**Required Jobs**:
1. **format**: Checks code formatting with `cargo fmt`
2. **clippy**: Runs lints with warnings as errors
3. **build-multi-arch**: Builds library for x86_64, aarch64, i686
4. **test**: Runs unit and integration tests
5. **ci-success**: Final gate requiring all jobs to pass

**Optional Jobs**:
- **build-uefi-app**: Example UEFI application build

### PR Workflow (.github/workflows/pr.yml)

**Triggers**: Pull requests to main/develop
**Jobs**:
1. **pr-checks**: Validates PR title, checks for large files and secrets
2. **run-ci**: Invokes the main CI workflow
3. **code-quality**: Checks documentation and code comments
4. **pr-ready**: Final gate for PR approval

## Testing Your Configuration

### 1. Create a test branch
```bash
git checkout -b test-branch-protection
```

### 2. Make a trivial change
```bash
echo "# Test" >> test.txt
git add test.txt
git commit -m "test: branch protection"
git push origin test-branch-protection
```

### 3. Create a pull request
- Go to GitHub and create a PR from `test-branch-protection` to `main`
- Observe the status checks running
- Try to merge before checks complete (should be blocked)
- Try to merge after checks pass (should succeed if approvals are met)

### 4. Test failure scenarios
```bash
# Test formatting failure
echo "fn main(){}" >> src/lib.rs  # Intentionally bad formatting
git add src/lib.rs
git commit -m "test: formatting failure"
git push
```

The PR should show failing checks and block merging.

## Troubleshooting

### Status checks not appearing
- Wait for GitHub Actions to run at least once
- Refresh the branch protection settings page
- Check Actions tab to ensure workflows completed

### Checks passing but merge blocked
- Ensure required reviewers have approved
- Check that all conversations are resolved
- Verify branch is up to date with base

### Admin override needed
- Temporarily disable "Include administrators" in settings
- Merge with override (document why)
- Re-enable protection rules

## CI Check Descriptions

| Check | Purpose | Failure Impact |
|-------|---------|----------------|
| Check Formatting | Ensures consistent code style | **BLOCKS MERGE** |
| Clippy Lints | Catches common mistakes and non-idiomatic code | **BLOCKS MERGE** |
| Build (multi-arch) | Verifies code compiles on all UEFI targets | **BLOCKS MERGE** |
| Run Tests | Validates functionality | **BLOCKS MERGE** |
| PR Checks | Validates PR metadata and scans for issues | **BLOCKS MERGE** |
| Code Quality | Checks documentation (informational) | Informational only |
| Build UEFI App | Tests example application | Informational only |

## Maintenance

### Adding new required checks
1. Update this document
2. Add check to GitHub branch protection settings
3. Notify team members

### Removing checks
1. Update branch protection rules
2. Update this document
3. Consider why the check is no longer needed

## Emergency Procedures

### Critical hotfix needed
If a critical security fix must bypass CI:

1. Create emergency branch: `hotfix/critical-issue`
2. Admin temporarily disables "Include administrators"
3. Admin merges with documented justification
4. Post-merge: Verify all checks would pass
5. Re-enable full protection rules

**Document all bypasses in CHANGELOG.md**

## References

- [GitHub Branch Protection Documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- [UEFI Specification 2.10](https://uefi.org/specifications)
- [Rust UEFI Target Documentation](https://doc.rust-lang.org/nightly/rustc/platform-support/unknown-uefi.html)
