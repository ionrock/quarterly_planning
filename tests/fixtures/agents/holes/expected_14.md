---
id: "test-014"
title: "Git Hooks Manager"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a CLI tool that manages Git hooks across repositories. Users define hooks in a config file, and the tool installs/updates them. Supports pre-commit, commit-msg, and pre-push hooks. Written in Rust.

## Constraints

- Must work with any Git repository
- No runtime dependencies beyond Git

## Implementation Notes

- Read hook configuration from .hooks.toml
- Generate shell scripts for each hook type
- Support running multiple commands per hook
- Provide install and uninstall commands

## Review Notes

### Identified Weaknesses

1. **Existing hooks handling**: What if the user already has hooks? Overwrite, merge, or backup?

2. **Cross-platform shell compatibility**: Shell scripts differ between bash, zsh, fish, Windows.

3. **No hook bypass mechanism**: How do users skip hooks when needed (--no-verify awareness)?

4. **Missing hook versioning**: How do users know if hooks need reinstalling after config changes?

5. **No partial failure handling**: What if one command in a hook fails? Continue or abort?

### Edge Cases

- What if .hooks.toml doesn't exist?
- Worktrees and submodules (different .git locations)?
- What if hooks directory doesn't exist or isn't writable?
- Commands with special characters or quotes?
- What about hooks that need stdin (commit-msg gets message file)?
- Relative vs absolute paths in commands?

### Assumptions to Validate

- Is .hooks.toml the right config file name (vs .pre-commit-config.yaml convention)?
- Should this be compatible with husky/pre-commit configuration?
- Is team-wide hook sharing via the config file the goal?
- Do we need to support hook arguments/parameters?
- Should hooks run in parallel or sequentially?

### Potential Failures

- Permission denied writing to .git/hooks
- Generated script has syntax errors
- Command not found in PATH during hook execution
- Hook timeout (long-running linters)
- Circular dependency if hook triggers git operations
- Path issues in monorepos with nested git repos

## Tickets

### Ticket 1: Config Parser

**Summary:** Parse .hooks.toml configuration file.

**Definition of Done:** Configuration is correctly parsed into structs.

### Ticket 2: Hook Generator

**Summary:** Generate shell scripts from hook configuration.

**Definition of Done:** Generated scripts execute configured commands.

### Ticket 3: CLI Commands

**Summary:** Implement install, uninstall, and run commands.

**Definition of Done:** Hooks are correctly installed in .git/hooks.
