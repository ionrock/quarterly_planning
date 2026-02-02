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

### Technology Stack
- **Language:** Rust 1.75+
- **CLI:** clap v4 with derive macros
- **Config:** toml crate for parsing
- **Template:** handlebars for hook script generation

### Configuration Format (.hooks.toml)
```toml
[hooks.pre-commit]
commands = [
    { run = "cargo fmt --check", name = "format" },
    { run = "cargo clippy -- -D warnings", name = "lint" },
    { run = "cargo test --lib", name = "test" },
]
fail_fast = true  # Stop on first failure

[hooks.commit-msg]
commands = [
    { run = "commitlint --edit $1", name = "commitlint" },
]

[hooks.pre-push]
commands = [
    { run = "cargo test", name = "full-test" },
]
```

### Core Data Structures
```rust
#[derive(Debug, Deserialize)]
pub struct Config {
    pub hooks: HashMap<String, HookConfig>,
}

#[derive(Debug, Deserialize)]
pub struct HookConfig {
    pub commands: Vec<Command>,
    #[serde(default)]
    pub fail_fast: bool,
    #[serde(default)]
    pub parallel: bool,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub run: String,
    pub name: Option<String>,
    #[serde(default)]
    pub stage_files: Vec<String>,  // Glob patterns to restage
}

#[derive(Parser)]
#[command(name = "hooks", about = "Git hooks manager")]
pub enum Cli {
    /// Install hooks to .git/hooks
    Install {
        #[arg(short, long)]
        force: bool,
    },
    /// Uninstall managed hooks
    Uninstall,
    /// Run a hook manually
    Run {
        hook: String,
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Show hook status
    Status,
}
```

### Hook Script Template
```rust
const HOOK_TEMPLATE: &str = r#"#!/bin/sh
# Managed by hooks - do not edit
# Version: {{version}}
# Hook: {{hook_name}}

set -e

{{#each commands}}
echo "Running: {{name}}"
{{run}} {{#if ../args}}"$@"{{/if}}
{{#unless ../fail_fast}}|| true{{/unless}}

{{/each}}
"#;
```

### Installation Logic
```rust
pub fn install_hooks(config: &Config, git_dir: &Path, force: bool) -> Result<()> {
    let hooks_dir = git_dir.join("hooks");
    fs::create_dir_all(&hooks_dir)?;

    for (hook_name, hook_config) in &config.hooks {
        let hook_path = hooks_dir.join(hook_name);

        // Check for existing non-managed hook
        if hook_path.exists() && !is_managed_hook(&hook_path)? {
            if force {
                let backup = hook_path.with_extension("backup");
                fs::rename(&hook_path, &backup)?;
                println!("Backed up existing hook to {}", backup.display());
            } else {
                bail!("Existing hook at {}. Use --force to overwrite.", hook_path.display());
            }
        }

        // Generate and write hook script
        let script = generate_hook_script(hook_name, hook_config)?;
        fs::write(&hook_path, script)?;

        // Make executable (Unix)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)?;
        }

        println!("Installed {} hook", hook_name);
    }

    Ok(())
}

fn is_managed_hook(path: &Path) -> Result<bool> {
    let content = fs::read_to_string(path)?;
    Ok(content.contains("Managed by hooks"))
}
```

### Hook Execution
```rust
pub fn run_hook(config: &HookConfig, args: &[String]) -> Result<ExitCode> {
    let mut failed = false;

    for cmd in &config.commands {
        let name = cmd.name.as_deref().unwrap_or(&cmd.run);
        println!("\x1b[36m→ {}\x1b[0m", name);

        let shell = if cfg!(windows) { "cmd" } else { "sh" };
        let flag = if cfg!(windows) { "/C" } else { "-c" };

        let status = std::process::Command::new(shell)
            .arg(flag)
            .arg(&cmd.run)
            .args(args)
            .status()?;

        if !status.success() {
            println!("\x1b[31m✗ {} failed\x1b[0m", name);
            if config.fail_fast {
                return Ok(ExitCode::FAILURE);
            }
            failed = true;
        } else {
            println!("\x1b[32m✓ {}\x1b[0m", name);
        }
    }

    Ok(if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS })
}
```

### Git Directory Detection
```rust
pub fn find_git_dir() -> Result<PathBuf> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()?;

    if !output.status.success() {
        bail!("Not a git repository");
    }

    let path = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(PathBuf::from(path))
}
```

## Review Notes

(none yet)

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
