---
id: "test-014"
title: "Git Hooks Manager"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a tool for managing Git hooks across projects. Supports shareable hook configurations, automatic installation, and multiple hook types.

## Constraints

- Must not interfere with existing hooks
- Cross-platform (Linux, macOS, Windows)

## Implementation Notes

- Written in Node.js
- YAML configuration files
- npm package distribution

## Review Notes

(none yet)

## Tickets

### Ticket 1: Hook Installation

**Summary:** Install hooks in git repositories.

**Definition of Done:** Hooks installed and executable.

#### Steps

1. **Create npm package**
   - Run `npm init`
   - Set name to "githooks-manager"
   - Add bin entry for CLI
   - Verify: package.json correct

2. **Create CLI entry point**
   - Create src/cli.js with shebang
   - Use commander for argument parsing
   - Verify: `npx . --help` works

3. **Find git directory**
   - Search for .git folder upward from cwd
   - Handle worktrees (.git file pointing to gitdir)
   - Verify: finds .git in nested directories

4. **Detect existing hooks**
   - List files in .git/hooks
   - Check for non-sample hooks
   - Verify: existing hooks detected

5. **Backup existing hooks**
   - Move existing hooks to .git/hooks/backup/
   - Record backup in metadata file
   - Verify: backups created

6. **Create hook wrapper script**
   - Create src/templates/hook-wrapper.sh
   - Template that calls our runner
   - Verify: template valid shell script

7. **Install hook wrapper**
   - Copy wrapper to .git/hooks/{hook-type}
   - Set executable permission (chmod +x)
   - Verify: hook file created and executable

8. **Support Windows**
   - Create .cmd wrapper for Windows
   - Handle path differences
   - Verify: works on Windows

9. **Create uninstall command**
   - Remove installed wrappers
   - Restore backups if present
   - Verify: clean uninstall

10. **Add install command to CLI**
    - `githooks install` installs all hooks
    - Verify: command works

### Ticket 2: Configuration

**Summary:** Define hooks via configuration file.

**Definition of Done:** Config file parsed and hooks generated.

#### Steps

1. **Define config file schema**
   - Create src/schema.js with JSON Schema
   - Define hooks, commands, options structure
   - Verify: schema valid

2. **Install YAML parser**
   - Add js-yaml dependency
   - Verify: package installed

3. **Create config loader**
   - Create src/config.js
   - Look for .githooks.yml in repo root
   - Verify: config file found

4. **Parse YAML config**
   - Parse file with js-yaml
   - Validate against schema
   - Verify: valid config parsed

5. **Handle missing config**
   - Return empty config if file missing
   - Create template on init command
   - Verify: missing config handled

6. **Define hook types structure**
   - Support: pre-commit, commit-msg, pre-push, etc.
   - Map to git hook names
   - Verify: all hook types supported

7. **Define command structure**
   - Support: command string, files glob, pass_filenames
   - Verify: command structure parsed

8. **Support extending shared configs**
   - Add extends field for remote configs
   - Fetch and merge extended configs
   - Verify: extension works

9. **Create config init command**
   - Generate default .githooks.yml
   - Include common hooks as examples
   - Verify: init creates valid config

10. **Validate config on load**
    - Check for unknown hook types
    - Validate command syntax
    - Verify: invalid config errors clearly

### Ticket 3: Hook Execution

**Summary:** Execute configured hook scripts.

**Definition of Done:** Scripts run with proper context.

#### Steps

1. **Create hook runner**
   - Create src/runner.js
   - Accept hook type and context
   - Verify: runner callable

2. **Load config in runner**
   - Find and parse config file
   - Get commands for hook type
   - Verify: config loaded

3. **Determine files to check**
   - For pre-commit: staged files
   - For pre-push: commits to push
   - Verify: correct files identified

4. **Filter files by glob pattern**
   - Use minimatch for glob matching
   - Filter staged files by command glob
   - Verify: glob filtering works

5. **Execute single command**
   - Use child_process.spawn
   - Pass file list if configured
   - Capture stdout/stderr
   - Verify: command executes

6. **Handle command failure**
   - Check exit code
   - Print error output
   - Exit hook with failure
   - Verify: failures propagate

7. **Execute commands in order**
   - Run commands sequentially
   - Stop on first failure
   - Verify: order respected

8. **Support parallel execution**
   - Add parallel option to config
   - Run commands concurrently
   - Verify: parallel faster

9. **Pass environment to commands**
   - Set GIT_HOOKS_* env variables
   - Include hook type, file list
   - Verify: env available to commands

10. **Handle commit-msg hook**
    - Read commit message from file
    - Pass to configured commands
    - Verify: commit-msg works

11. **Support verbose mode**
    - Add --verbose flag
    - Print command output even on success
    - Verify: verbose shows output

12. **Add skip option**
    - Support SKIP_HOOKS=1 env var
    - Skip all hooks when set
    - Verify: skip works
