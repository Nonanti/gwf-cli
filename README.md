# GWF - Git Workflow Automator

Streamline your Git workflows with powerful automation and best practices.

[![Build Status](https://github.com/Nonanti/gwf-cli/actions/workflows/main.yml/badge.svg)](https://github.com/Nonanti/gwf-cli/actions/workflows/main.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.82%2B-orange.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/gwf.svg)](https://crates.io/crates/gwf)
[![Downloads](https://img.shields.io/crates/d/gwf.svg)](https://crates.io/crates/gwf)
## Features

- **Smart Branch Management** - Create and manage feature, hotfix, and release branches
- **Automated Workflows** - Streamline common Git operations with single commands
- **Conventional Commits** - Interactive commit creation following best practices
- **Branch Synchronization** - Keep branches up-to-date with configurable strategies
- **Cleanup Automation** - Remove merged branches safely
- **Multi-Platform** - Works on Linux, macOS, and Windows
- **Repository Analytics** - View commit statistics and contributor data
- **Interactive Undo** - Safely revert recent Git operations
- **Version Tagging** - Semantic versioning support

## Installation

### Quick Install

#### From GitHub Releases (Recommended)

```bash
# Linux
curl -LO https://github.com/Nonanti/gwf-cli/releases/latest/download/gwf-linux-amd64.tar.gz
tar xzf gwf-linux-amd64.tar.gz
sudo mv gwf /usr/local/bin/

# macOS
curl -LO https://github.com/Nonanti/gwf-cli/releases/latest/download/gwf-macos-amd64.tar.gz
tar xzf gwf-macos-amd64.tar.gz
sudo mv gwf /usr/local/bin/

# Windows (PowerShell)
Invoke-WebRequest -Uri https://github.com/Nonanti/gwf-cli/releases/latest/download/gwf-windows-amd64.zip -OutFile gwf.zip
Expand-Archive gwf.zip -DestinationPath .
# Add gwf.exe to your PATH
```

#### From Source

```bash
cargo install gwf
```

## Quick Start

```bash
# Initialize in your repository
gwf init

# Create a feature branch
gwf feature awesome-feature

# Make changes and commit
gwf commit

# Create a pull request
gwf pr

# Clean up after merge
gwf cleanup
```

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `init` | Initialize GWF in repository | `gwf init` |
| `feature` | Create feature branch | `gwf feature user-auth` |
| `hotfix` | Create hotfix branch | `gwf hotfix security-patch` |
| `release` | Create release branch | `gwf release 1.2.0` |
| `commit` | Create conventional commit | `gwf commit` |
| `sync` | Sync with remote | `gwf sync` |
| `cleanup` | Remove merged branches | `gwf cleanup` |
| `pr` | Create pull request | `gwf pr` |
| `standup` | Generate standup report | `gwf standup` |
| `config` | Manage configuration | `gwf config --edit` |
| `completions` | Generate shell completions | `gwf completions bash` |

## Configuration

GWF uses `.gwf.toml` for configuration:

```toml
[workflows]
feature_branch_prefix = "feature/"
hotfix_branch_prefix = "hotfix/"
release_branch_prefix = "release/"
main_branch = "main"

[commits]
conventional = true
sign_commits = false

[sync]
strategy = "rebase"  # or "merge"
auto_stash = true

[cleanup]
delete_merged = true
protect_branches = ["main", "master", "develop"]
```

## Workflow Examples

### Feature Development

```bash
gwf feature new-feature       # Create feature branch
# ... make changes ...
gwf commit                    # Interactive commit
gwf sync                      # Sync with upstream
gwf pr                        # Create pull request
gwf cleanup                   # Clean up after merge
```

### Hotfix Deployment

```bash
gwf hotfix critical-fix       # Create hotfix branch
# ... fix issue ...
gwf commit -m "fix: resolve critical issue"
gwf pr --target main
gwf release patch --tag       # Create patch release
```

### Release Management

```bash
gwf release minor             # Create release branch
gwf commit -m "chore: bump version"
gwf pr --title "Release v1.2.0"
# After merge
git tag v1.2.0 && git push --tags
```

## Shell Completions

Enable auto-completion for your shell:

```bash
# Bash
gwf completions bash > ~/.bash_completion.d/gwf

# Zsh
gwf completions zsh > ~/.zfunc/_gwf

# Fish
gwf completions fish > ~/.config/fish/completions/gwf.fish

# PowerShell
gwf completions powershell | Out-String | Invoke-Expression
```

## Requirements

- Git 2.0+
- GitHub CLI (optional, for PR features)

## Known Issues

- PR creation only works with GitHub CLI currently
- `gwf bisect` automated mode may not work with all test scripts
- Remote sync sometimes fails with large repositories
- Stats command can be slow on repos with 10k+ commits

## Roadmap

- [ ] GitLab and Bitbucket support for PR creation
- [ ] Interactive rebase helper
- [ ] Commit message templates
- [ ] Hook management system
- [ ] Plugin architecture

## Contributing

Contributions are welcome! Please check out the [issues](https://github.com/Nonanti/gwf-cli/issues) or submit a pull request.

## License

MIT License - see [LICENSE](LICENSE) for details.

---


Built with Rust for speed and reliability.

