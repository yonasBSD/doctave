## [unreleased]

### 🚀 Features

- Add badges to README.md
- Add dprint config and fix formatting.
- Add goji config file.
- Add todo-tree workflow to CI. Runs alexandretrotel/todo-tree.
- Build with cargo-auditable to add SBOM to the release binary.

### 🐛 Bug Fixes

- Linting workflow should run on any branch.
- Linting errors.
- Linting errors and remove linkspector from CI
- Update .ignore
- Links in README.md.
- Clippy errors.
- Clippy errors.
- Missing permissions on linting workflow.
- Use clippy-sarif to upload proper SARIF.
- Actually install cargo-machete.
- Resolve false positives with cargo-machete.
- Better conventional commits error messages.
- Correct URL to latest git-cliff release.
- Cog check should ignore merge commits.
- PR Labels workflow should fire on forked repos.
- Remove empty permissions in workflow.
- Rename labeler config file extension.
- Update labeler config to latest format.
- Do not require scope when linting PRs.
- Generate changelog using all commits.
- Fetch-depth indent.
- Install cargo-binstall before installing cargo-nextest.
- Typo and todo-tree => tt.
- *(ci)* Move --no-fail-fast to nextest.toml config file.
- Cargo nexttest --all is outdated.
- *(ci)* Try alexandretrotel/todo-tree-action@v1.0.1.
- *(ci)* Cargo nextest run.

### 💼 Other

- Committing before a refactor
- Progress on getting new styles up and running
- Committing before rethink
- Committing before navigation refactor
- Making some changes to the config system to make other changes easy

### 📚 Documentation

- Add TODO.md

### ⚙️ Miscellaneous Tasks

- Update dependencies.
- Update dependencies and refactor.
- Update incompatible dependencies.
- Move docs and templates to lib folder.
- Add conventional commits check. Runs cocogitto.
- Don't exit after first fail during testing.
- Upload clippy SARIF results.
- Generate code coverage badge.
- Use cargo-binstall for faster setup.
- Add an issue management workflow.
- Use nightly toolchain.
- Use .codespellignore. Update test matrix.
- Use yonasBSD/doctave-markdown.
- Switch to typos-cli for spell checking.
- Run typos-cli manually to lint non-PRs.
- Switch to dtolnay/rust-toolchain.
- Add OSSF Scorecard workflow.
- Add TODO checker workflows.
- Add dependency review. Runs on PRs.
- Add cargo-machete. Update deps in scorecard.yaml
- Add labels to pull requests.
- Cleanup changelog workflow title.
- Add labeler.yaml config.
- Add greeting workflow to greet first time contributors.
- Update actions checkout.
- Upgrade codeql workflow.
- Update create-pull-request to v7.
- Update changelog workflow title.
- Update changelog (#10)
- Changelog workflow should have signed commits.
- Add permissions block to workflows.
- Use cargo-nextest for faster tests.
- Add alexandretrotel/todo-tree-action. Runs on PRs.
- Add lefthook, pre-commit. Fix formatting.
