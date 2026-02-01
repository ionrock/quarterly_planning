//! Interactive init wizard: walk user through configuring agent, skills, and plugins.

use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use crate::config::{ConfigFile, ReviewAgentConfig};

const DEFAULT_STEPS: &[&str] = &["holes", "details", "breakdown", "deliverables"];

/// Predefined agent profiles: command and description.
pub const AGENT_PROFILES: &[(&str, &str, &str)] = &[
    ("cursor", "agent", "Cursor CLI (agent)"),
    ("claude", "claude", "Claude CLI"),
    ("aider", "aider", "Aider"),
];

/// Predefined review-agent prompts (step name -> prompt).
fn default_prompts() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert(
        "holes".to_string(),
        "Review this plan and identify weaknesses, missing considerations, edge cases, potential failures, and assumptions that need validation.".to_string(),
    );
    m.insert(
        "details".to_string(),
        "Expand this plan with implementation details. Add specifics about technologies, APIs, data structures, and algorithms.".to_string(),
    );
    m.insert(
        "breakdown".to_string(),
        "Break this plan into precise, atomic steps. Each step should be independently implementable and testable.".to_string(),
    );
    m.insert(
        "deliverables".to_string(),
        "Define clear acceptance criteria for each component. What tests must pass? What can be demonstrated? How do we know it's done?".to_string(),
    );
    m
}

/// Optional plugins: extra optimization steps with prompts.
pub const PLUGINS: &[(&str, &str)] = &[
    (
        "risk-check",
        "Identify risks, failure modes, and mitigation strategies for this plan. Output concrete mitigations.",
    ),
    (
        "strict-deliverables",
        "For each deliverable, define strict acceptance criteria: automated test or manual check, and a one-sentence definition of done.",
    ),
    (
        "dependencies",
        "List explicit dependencies between tasks and any external blockers. Output a dependency section for the plan.",
    ),
];

fn read_line(prompt: &str, default: Option<&str>) -> Result<String> {
    let mut stdout = io::stdout();
    if let Some(d) = default {
        write!(stdout, "{} [{}]: ", prompt, d)?;
    } else {
        write!(stdout, "{} ", prompt)?;
    }
    stdout.flush()?;
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line)?;
    let line = line.trim().to_string();
    Ok(if line.is_empty() && default.is_some() {
        default.unwrap().to_string()
    } else {
        line
    })
}

fn read_line_optional(prompt: &str) -> Result<String> {
    read_line(prompt, Some(""))
}

/// Run the interactive wizard and return a ConfigFile to write.
pub fn run_wizard(agent_command: &str) -> Result<ConfigFile> {
    let prompts = default_prompts();

    println!("\n  qp init – configure agent, skills, and plugins\n");

    // 1) Agent command (pre-filled from profile)
    let cmd = read_line("Agent command", Some(agent_command))?;
    let cmd = if cmd.is_empty() { agent_command.to_string() } else { cmd };

    // 2) Extra args
    let args_str = read_line_optional("Extra agent args (space-separated, optional)")?;
    let args: Vec<String> = args_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // 3) Optimization steps
    println!("\n  Optimization steps: holes, details, breakdown, deliverables");
    let steps_str = read_line("Use all default steps?", Some("y"))?;
    let use_default_steps = steps_str.is_empty() || steps_str.eq_ignore_ascii_case("y") || steps_str.eq_ignore_ascii_case("yes");
    let steps: Vec<String> = if use_default_steps {
        DEFAULT_STEPS.iter().map(|s| s.to_string()).collect()
    } else {
        let custom = read_line("Comma-separated step names (e.g. holes,details,deliverables)", None)?;
        custom.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    };

    // 4) Plugins (extra steps)
    println!("\n  Optional plugins (add extra review steps):");
    for (name, desc) in PLUGINS {
        println!("    - {}: {}", name, desc);
    }
    let plugins_str = read_line("Enable plugins (comma-separated names, or 'none')", Some("none"))?;
    let mut plugin_steps: Vec<String> = vec![];
    if !plugins_str.is_empty() && !plugins_str.eq_ignore_ascii_case("none") {
        for name in plugins_str.split(',') {
            let name = name.trim();
            if PLUGINS.iter().any(|(n, _)| *n == name) {
                plugin_steps.push(name.to_string());
            }
        }
    }

    // Build review_agents: steps + plugin steps; resolve prompt from default_prompts or PLUGINS
    let mut review_agents: HashMap<String, ReviewAgentConfig> = HashMap::new();
    let ra = |_step: &str, prompt: &str| ReviewAgentConfig {
        command: cmd.clone(),
        args: args.clone(),
        prompt: prompt.to_string(),
    };
    for step in &steps {
        let prompt_opt = prompts.get(step)
            .map(String::as_str)
            .or_else(|| PLUGINS.iter().find(|(n, _)| *n == step).map(|(_, p)| *p));
        if let Some(prompt) = prompt_opt {
            review_agents.insert(step.clone(), ra(step, prompt));
        }
    }
    for step in &plugin_steps {
        if let Some((_, prompt)) = PLUGINS.iter().find(|(n, _)| *n == step) {
            review_agents.insert(step.clone(), ra(step, prompt));
        }
    }

    let mut all_steps = steps;
    all_steps.extend(plugin_steps);

    let config = ConfigFile {
        agent: crate::config::AgentConfig {
            command: cmd,
            args,
        },
        review_agents,
        optimization: crate::config::OptimizationConfig { steps: all_steps },
    };

    Ok(config)
}

/// Prompt for agent profile and return (profile_name, command).
pub fn choose_agent_profile() -> Result<(String, String)> {
    println!("\n  Choose agent profile (used for creating/editing plans and for review steps):");
    for (i, (_name, cmd, desc)) in AGENT_PROFILES.iter().enumerate() {
        println!("    {}) {} – command: {}", i + 1, desc, cmd);
    }
    println!("    {}) Custom – you'll enter the command", AGENT_PROFILES.len() + 1);

    let n = read_line("Profile", Some("1"))?;
    let n: usize = n.parse().unwrap_or(1);
    if n >= 1 && n <= AGENT_PROFILES.len() {
        let (name, cmd, _) = AGENT_PROFILES[n - 1];
        return Ok((name.to_string(), cmd.to_string()));
    }
    let custom = read_line("Custom command (e.g. agent, claude, npx -y tsx agent)", Some("agent"))?;
    Ok(("custom".to_string(), if custom.is_empty() { "agent".to_string() } else { custom }))
}
