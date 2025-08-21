# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-08-22

### Added
- Initial release of Git Workflow Automator (GWF)
- Feature branch management with `gwf feature` command
- Hotfix workflow support with `gwf hotfix` command  
- Release management with `gwf release` command
- Branch synchronization with `gwf sync` command
- Smart cleanup of merged branches with `gwf cleanup` command
- Conventional commits support with `gwf commit` command
- Pull request creation with `gwf pr` command
- Daily standup reports with `gwf standup` command
- Configuration management with `gwf config` command
- Shell completions for bash, zsh, fish, and PowerShell
- Comprehensive CI/CD pipeline with GitHub Actions
- Security auditing and dependency management
- Multi-platform support (Linux, macOS, Windows)
- Interactive and non-interactive modes
- Colored terminal output for better UX

### Technical Details
- Built with Rust for performance and reliability
- Uses git2 for Git operations
- Async runtime with Tokio
- Interactive prompts with dialoguer
- Progress indicators with indicatif
- Conventional commit format support
- Configurable workflows via .gwf.toml

[0.1.0]: https://github.com/Nonanti/gwf-cli/releases/tag/v0.1.0