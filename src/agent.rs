//! Spawn AI agent subprocess with a prompt; capture stdin/stdout as needed.
//! Supports interactive (spawn and attach) and one-shot (pass plan + prompt, get output).

use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::io::{Read, Write};

/// Run agent in one-shot mode: pass full prompt on stdin, capture stdout.
/// Used for optimization steps: we have the plan content + step prompt, we want the revised plan.
pub fn run_agent_oneshot(
    command: &str,
    args: &[String],
    prompt: &str,
    plan_content: &str,
) -> Result<String> {
    let full_input = format!(
        "{}\n\n---\n\nPlan to review/revise:\n\n{}",
        prompt, plan_content
    );
    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| format!("spawn {} {}", command, args.join(" ")))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(full_input.as_bytes()).context("write stdin")?;
        stdin.flush().context("flush stdin")?;
    }
    let mut stdout = String::new();
    if let Some(mut out) = child.stdout.take() {
        out.read_to_string(&mut stdout).context("read stdout")?;
    }
    let status = child.wait().context("wait for agent")?;
    if !status.success() {
        anyhow::bail!("agent exited with {}", status);
    }
    Ok(stdout)
}

/// Spawn agent interactively (no stdin pipe). User collaborates in their terminal.
/// Used for `qp new` and `qp edit`. Caller should wait or detach as desired.
pub fn run_agent_interactive(command: &str, args: &[String], initial_prompt: Option<&str>) -> Result<std::process::Child> {
    let mut cmd = Command::new(command);
    cmd.args(args);
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());
    if let Some(p) = initial_prompt {
        // Some CLIs accept a prompt as first arg; others need to be told.
        // Common: `agent "prompt"` or `claude "prompt"`. We'll pass as first arg.
        cmd.arg(p);
    }
    let child = cmd.spawn().with_context(|| format!("spawn {} {}", command, args.join(" ")))?;
    Ok(child)
}
