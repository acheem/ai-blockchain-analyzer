# Git Workflow & Branching Strategy

This document outlines the Git workflow and branching strategy for the AI Blockchain Analyzer project following GitFlow methodology.

## 📊 Branch Structure Overview

```
master (production)
  │
  ├─── release/* (release candidates)
  │      │
  │      └─── test (testing environment)
  │             │
  │             └─── dev (development integration)
  │                    │
  │                    └─── feature/* (new features)
  │
  └─── hotfix/* (emergency fixes)
```

---

## 🌿 Branch Types & Purposes

### 1. **master** (Main/Production Branch)
- **Purpose**: Production-ready code
- **Protection**: Highly protected, requires PR approval
- **Deployment**: Automatically deploys to production
- **Lifetime**: Permanent
- **Merge from**: `release/*` branches only
- **Tag**: Every merge creates a version tag (e.g., `v1.0.0`)

```bash
# Only release branches merge into master
git checkout master
git merge release/v1.0.0 --no-ff
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin master --tags
```

**Rules**:
- ✅ Never commit directly to master
- ✅ Only merge from release branches
- ✅ Always use `--no-ff` (no fast-forward)
- ✅ Tag every release

---

### 2. **release/** (Release Candidate Branches)
- **Purpose**: Prepare for production release
- **Naming**: `release/v<version>` (e.g., `release/v1.2.0`)
- **Created from**: `test` branch
- **Merges to**: `master` and back to `dev`
- **Lifetime**: Temporary (deleted after release)
- **Activities**: Bug fixes, version bumps, documentation updates

```bash
# Create release branch from test
git checkout test
git pull origin test
git checkout -b release/v1.2.0

# Make final adjustments
# Update version in Cargo.toml
# Update CHANGELOG.md

# Commit changes
git add .
git commit -m "chore: prepare release v1.2.0"
git push origin release/v1.2.0

# After approval, merge to master
git checkout master
git merge release/v1.2.0 --no-ff
git tag -a v1.2.0 -m "Release version 1.2.0"
git push origin master --tags

# Merge back to dev to keep in sync
git checkout dev
git merge release/v1.2.0 --no-ff
git push origin dev

# Delete release branch
git branch -d release/v1.2.0
git push origin --delete release/v1.2.0
```

**Rules**:
- ✅ No new features, only bug fixes
- ✅ Update version numbers
- ✅ Final testing before production
- ✅ Merge to both master and dev

---

### 3. **test** (Testing/Staging Branch)
- **Purpose**: Integration testing and QA validation
- **Protection**: Protected, requires PR approval
- **Deployment**: Deploys to staging/test environment
- **Lifetime**: Permanent
- **Merge from**: `dev` branch
- **Merge to**: `release/*` branches

```bash
# Merge dev into test for testing phase
git checkout test
git pull origin test
git merge dev --no-ff
git push origin test

# Run automated tests
cargo test
cargo clippy
cargo fmt --check

# Deploy to test environment
# QA team performs manual testing
```

**Rules**:
- ✅ Only merge from dev branch
- ✅ Run full test suite before merging
- ✅ QA approval required
- ✅ Create release branch from here

---

### 4. **dev** (Development Integration Branch)
- **Purpose**: Integration branch for ongoing development
- **Protection**: Semi-protected, requires PR review
- **Deployment**: Deploys to development environment
- **Lifetime**: Permanent
- **Merge from**: `feature/*` branches
- **Merge to**: `test` branch

```bash
# Merge completed feature into dev
git checkout dev
git pull origin dev
git merge feature/blockchain-rpc-integration --no-ff
git push origin dev

# Run integration tests
cargo test --all-features

# Verify build
cargo build
```

**Rules**:
- ✅ Default branch for new feature branches
- ✅ Merge feature branches here
- ✅ Always run tests before pushing
- ✅ Keep synchronized with test/master

---

### 5. **feature/** (Feature Development Branches)
- **Purpose**: Develop new features in isolation
- **Naming**: `feature/<feature-name>` (e.g., `feature/llm-integration`)
- **Created from**: `dev` branch
- **Merges to**: `dev` branch only
- **Lifetime**: Temporary (deleted after merge)
- **Activities**: Feature development and unit testing

```bash
# Create feature branch from dev
git checkout dev
git pull origin dev
git checkout -b feature/llm-integration

# Develop the feature
# Make multiple commits
git add src/services/llm.rs
git commit -m "feat: add LLM service module"

git add src/services/ai.rs
git commit -m "feat: integrate LLM into AI service"

git add tests/llm_tests.rs
git commit -m "test: add LLM integration tests"

# Push feature branch
git push origin feature/llm-integration

# Create Pull Request to dev
# After approval, merge to dev
git checkout dev
git pull origin dev
git merge feature/llm-integration --no-ff
git push origin dev

# Delete feature branch
git branch -d feature/llm-integration
git push origin --delete feature/llm-integration
```

**Naming Conventions**:
- `feature/blockchain-rpc-integration` - New blockchain RPC client
- `feature/llm-integration` - LLM API integration
- `feature/risk-scoring` - Risk scoring algorithm
- `feature/multi-chain-support` - Multiple blockchain support
- `feature/websocket-api` - WebSocket endpoint

**Rules**:
- ✅ Always branch from latest dev
- ✅ Small, focused features
- ✅ Include unit tests
- ✅ Rebase on dev before merging
- ✅ Delete after successful merge

---

### 6. **hotfix/** (Emergency Production Fixes)
- **Purpose**: Critical bug fixes for production
- **Naming**: `hotfix/<issue-name>` (e.g., `hotfix/memory-leak`)
- **Created from**: `master` branch
- **Merges to**: `master` AND `dev`
- **Lifetime**: Temporary (deleted after merge)
- **Version**: Patch version bump (e.g., 1.0.0 → 1.0.1)

```bash
# Create hotfix from master
git checkout master
git pull origin master
git checkout -b hotfix/critical-security-fix

# Fix the critical issue
git add src/routes.rs
git commit -m "fix: patch critical security vulnerability CVE-2025-1234"

# Update version (patch bump)
# Cargo.toml: 1.0.0 → 1.0.1

git add Cargo.toml
git commit -m "chore: bump version to 1.0.1"

# Merge to master
git checkout master
git merge hotfix/critical-security-fix --no-ff
git tag -a v1.0.1 -m "Hotfix: critical security patch"
git push origin master --tags

# Merge to dev to keep in sync
git checkout dev
git merge hotfix/critical-security-fix --no-ff
git push origin dev

# Merge to test
git checkout test
git merge hotfix/critical-security-fix --no-ff
git push origin test

# Delete hotfix branch
git branch -d hotfix/critical-security-fix
git push origin --delete hotfix/critical-security-fix
```

**Rules**:
- ✅ Only for critical production issues
- ✅ Must merge to master, dev, and test
- ✅ Requires immediate deployment
- ✅ Patch version bump

---

## 🔄 Complete Workflow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    GITFLOW WORKFLOW                             │
└─────────────────────────────────────────────────────────────────┘

MASTER (production - v1.0.0)
  │
  │◄────────── merge ────────── RELEASE/v1.1.0
  │                                    │
  │                                    │◄──── final fixes ────
  │                                    │
  │                             TEST (staging)
  │                                    │
  │                                    │◄──── merge ──── DEV
  │                                                        │
  │                                                        │◄── FEATURE/blockchain-rpc
  │                                                        │       (development)
  │                                                        │
  │                                                        │◄── FEATURE/llm-integration
  │                                                        │       (development)
  │                                                        │
  │                                                        │◄── FEATURE/risk-scoring
  │                                                                (development)
  │
  │◄──── HOTFIX/security-patch ────►DEV
        (emergency fixes)             (keep in sync)
```

---

## 🚀 Development Workflow Steps

### Starting New Feature Development

```bash
# 1. Update local dev branch
git checkout dev
git pull origin dev

# 2. Create feature branch
git checkout -b feature/new-awesome-feature

# 3. Develop and commit
git add .
git commit -m "feat: implement awesome feature"

# 4. Push feature branch
git push origin feature/new-awesome-feature

# 5. Create Pull Request to dev
# (Use GitHub/GitLab UI)

# 6. After code review and approval, merge to dev
git checkout dev
git pull origin dev
git merge feature/new-awesome-feature --no-ff
git push origin dev

# 7. Delete feature branch
git branch -d feature/new-awesome-feature
git push origin --delete feature/new-awesome-feature
```

### Testing & QA Phase

```bash
# 1. Merge dev into test
git checkout test
git pull origin test
git merge dev --no-ff
git push origin test

# 2. Automated tests run in CI/CD
# 3. QA team performs manual testing
# 4. If bugs found, create bug fix branches from dev
```

### Release Process

```bash
# 1. Create release branch from test
git checkout test
git pull origin test
git checkout -b release/v1.2.0

# 2. Update version numbers
# Edit Cargo.toml: version = "1.2.0"
# Update CHANGELOG.md

git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 1.2.0"

# 3. Final testing and bug fixes (if needed)
# Only critical fixes allowed

# 4. Merge to master
git checkout master
git pull origin master
git merge release/v1.2.0 --no-ff
git tag -a v1.2.0 -m "Release version 1.2.0"
git push origin master --tags

# 5. Merge back to dev
git checkout dev
git merge release/v1.2.0 --no-ff
git push origin dev

# 6. Merge back to test
git checkout test
git merge release/v1.2.0 --no-ff
git push origin test

# 7. Delete release branch
git branch -d release/v1.2.0
git push origin --delete release/v1.2.0
```

### Hotfix Process

```bash
# 1. Create hotfix from master
git checkout master
git pull origin master
git checkout -b hotfix/critical-issue

# 2. Fix the issue
git add .
git commit -m "fix: resolve critical production issue"

# 3. Update version (patch bump)
git add Cargo.toml
git commit -m "chore: bump version to 1.2.1"

# 4. Merge to master
git checkout master
git merge hotfix/critical-issue --no-ff
git tag -a v1.2.1 -m "Hotfix: critical issue"
git push origin master --tags

# 5. Merge to all active branches
git checkout dev
git merge hotfix/critical-issue --no-ff
git push origin dev

git checkout test
git merge hotfix/critical-issue --no-ff
git push origin test

# 6. Delete hotfix branch
git branch -d hotfix/critical-issue
git push origin --delete hotfix/critical-issue
```

---

## 📝 Commit Message Conventions

Follow [Conventional Commits](https://www.conventionalcommits.org/):

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD configuration changes
- `build`: Build system changes

### Examples

```bash
# Feature
git commit -m "feat(ai): add LLM integration for transaction analysis"

# Bug fix
git commit -m "fix(blockchain): handle RPC timeout errors gracefully"

# Documentation
git commit -m "docs: update API documentation with new endpoints"

# Breaking change
git commit -m "feat(api)!: change response format for analyze_tx endpoint

BREAKING CHANGE: Response now includes additional risk_factors field"

# Multiple changes
git commit -m "feat(services): integrate OpenAI API

- Add OpenAI client module
- Implement prompt engineering
- Add error handling for API failures
- Update tests for AI service

Closes #42"
```

---

## 🔒 Branch Protection Rules

### master
- ✅ Require pull request reviews (2 approvers)
- ✅ Require status checks to pass
- ✅ Require branches to be up to date
- ✅ Include administrators
- ✅ Restrict who can push
- ✅ Require signed commits

### test
- ✅ Require pull request reviews (1 approver)
- ✅ Require status checks to pass
- ✅ Require branches to be up to date

### dev
- ✅ Require pull request reviews (1 approver)
- ✅ Require status checks to pass

### release/*
- ✅ Require pull request reviews (2 approvers)
- ✅ Require status checks to pass
- ✅ Require branches to be up to date

---

## 🧪 CI/CD Integration

### Automated Checks on All Branches

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [master, dev, test, 'release/**', 'feature/**']
  pull_request:
    branches: [master, dev, test]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all-features
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Check formatting
        run: cargo fmt -- --check
```

### Deployment Pipeline

```
feature/* → dev      → Deploy to dev environment
dev       → test     → Deploy to staging environment
test      → release  → No deployment (preparation)
release   → master   → Deploy to production
hotfix    → master   → Deploy to production (immediate)
```

---

## 📋 Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Feature (new functionality)
- [ ] Bug fix (non-breaking fix)
- [ ] Hotfix (critical production fix)
- [ ] Documentation update
- [ ] Refactoring
- [ ] Performance improvement

## Related Issues
Closes #<issue_number>

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex code
- [ ] Documentation updated
- [ ] No new warnings generated
- [ ] Tests pass locally
- [ ] Dependent changes merged

## Screenshots (if applicable)
```

---

## 🏷️ Version Numbering (Semantic Versioning)

Format: `MAJOR.MINOR.PATCH` (e.g., `1.2.3`)

- **MAJOR**: Breaking changes (1.0.0 → 2.0.0)
- **MINOR**: New features, backward compatible (1.0.0 → 1.1.0)
- **PATCH**: Bug fixes, backward compatible (1.0.0 → 1.0.1)

### Examples
- `1.0.0` - Initial release
- `1.1.0` - Added LLM integration (new feature)
- `1.1.1` - Fixed RPC timeout bug (patch)
- `2.0.0` - Changed API response format (breaking change)

---

## 📊 Branch Lifecycle Summary

| Branch | Created From | Merges To | Lifetime | Purpose |
|--------|-------------|-----------|----------|---------|
| `master` | - | - | Permanent | Production code |
| `release/*` | `test` | `master`, `dev` | Temporary | Release preparation |
| `test` | - | `release/*` | Permanent | QA testing |
| `dev` | - | `test` | Permanent | Integration |
| `feature/*` | `dev` | `dev` | Temporary | Feature development |
| `hotfix/*` | `master` | `master`, `dev`, `test` | Temporary | Emergency fixes |

---

## 🎯 Best Practices

### Do's ✅
- Create feature branches for all new work
- Keep feature branches small and focused
- Merge dev into test regularly
- Test thoroughly before creating release branches
- Tag all production releases
- Write descriptive commit messages
- Rebase feature branches on dev before merging
- Delete branches after merging
- Use `--no-ff` for important merges

### Don'ts ❌
- Don't commit directly to master
- Don't merge untested code to test
- Don't create long-lived feature branches
- Don't skip code reviews
- Don't merge without passing CI checks
- Don't forget to merge hotfixes to all branches
- Don't use force push on shared branches
- Don't include multiple features in one branch

---

## 🆘 Common Scenarios

### Scenario 1: Feature needs update from dev
```bash
git checkout feature/my-feature
git fetch origin
git rebase origin/dev
# Resolve conflicts if any
git push --force-with-lease origin feature/my-feature
```

### Scenario 2: Urgent bug in production
```bash
# Use hotfix process
git checkout master
git pull origin master
git checkout -b hotfix/urgent-bug
# Fix and follow hotfix workflow
```

### Scenario 3: Feature is not ready for release
```bash
# Don't merge to test yet
# Keep working on feature branch
# Merge other completed features first
```

### Scenario 4: Release has critical bug
```bash
# Fix in release branch
git checkout release/v1.2.0
git add .
git commit -m "fix: critical bug in release"
# Continue with release process
# Bug fix will be merged to master and dev
```

---

## 📞 Git Commands Cheat Sheet

```bash
# Create and switch to new branch
git checkout -b feature/new-feature

# Update local branch
git pull origin dev

# View all branches
git branch -a

# Delete local branch
git branch -d feature/old-feature

# Delete remote branch
git push origin --delete feature/old-feature

# View commit history
git log --oneline --graph --all

# Rebase on dev
git rebase origin/dev

# Interactive rebase (squash commits)
git rebase -i HEAD~3

# Create tag
git tag -a v1.0.0 -m "Release 1.0.0"

# Push tags
git push origin --tags

# Stash changes
git stash save "work in progress"

# Apply stashed changes
git stash pop
```

---

**Last Updated**: December 8, 2025

**Document Version**: 1.0.0
