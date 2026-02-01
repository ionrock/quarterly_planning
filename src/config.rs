//! Configuration loading: global (~/.config/qp) and local (.qp/config.toml).
//! Local overrides global.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

const DEFAULT_AGENT_COMMAND: &str = "claude";
const DEFAULT_OPTIMIZATION_STEPS: &[&str] = &["holes", "details", "breakdown", "deliverables"];

fn default_agent_command() -> String {
    DEFAULT_AGENT_COMMAND.to_string()
}

fn default_optimization_steps() -> Vec<String> {
    DEFAULT_OPTIMIZATION_STEPS.iter().map(|s| s.to_string()).collect()
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentConfig {
    #[serde(default = "default_agent_command")]
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReviewAgentConfig {
    #[serde(default = "default_agent_command")]
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub prompt: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptimizationConfig {
    #[serde(default = "default_optimization_steps")]
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigFile {
    #[serde(default)]
    pub agent: AgentConfig,
    #[serde(default)]
    pub review_agents: HashMap<String, ReviewAgentConfig>,
    #[serde(default)]
    pub optimization: OptimizationConfig,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            agent: AgentConfig {
                command: default_agent_command(),
                args: vec![],
            },
            review_agents: default_review_agents(),
            optimization: OptimizationConfig {
                steps: default_optimization_steps(),
            },
        }
    }
}

fn default_review_agents() -> HashMap<String, ReviewAgentConfig> {
    let mut m = HashMap::new();
    m.insert(
        "holes".to_string(),
        ReviewAgentConfig {
            command: DEFAULT_AGENT_COMMAND.to_string(),
            args: vec![],
            prompt: "Review this plan and identify weaknesses, missing considerations, edge cases, potential failures, and assumptions that need validation.".to_string(),
        },
    );
    m.insert(
        "details".to_string(),
        ReviewAgentConfig {
            command: DEFAULT_AGENT_COMMAND.to_string(),
            args: vec![],
            prompt: "Expand this plan with implementation details. Add specifics about technologies, APIs, data structures, and algorithms.".to_string(),
        },
    );
    m.insert(
        "breakdown".to_string(),
        ReviewAgentConfig {
            command: DEFAULT_AGENT_COMMAND.to_string(),
            args: vec![],
            prompt: "Break this plan into precise, atomic steps. Each step should be independently implementable and testable.".to_string(),
        },
    );
    m.insert(
        "deliverables".to_string(),
        ReviewAgentConfig {
            command: DEFAULT_AGENT_COMMAND.to_string(),
            args: vec![],
            prompt: "Define clear acceptance criteria for each component. What tests must pass? What can be demonstrated? How do we know it's done?".to_string(),
        },
    );
    m
}

/// Load config: merge global (if present) with local (if present). Local wins.
pub fn load_config(qp_root: Option<&std::path::Path>) -> Result<ConfigFile> {
    let mut config = ConfigFile::default();

    // Global: ~/.config/qp/config.toml
    if let Some(config_dir) = dirs::config_dir() {
        let global = config_dir.join("qp").join("config.toml");
        if global.exists() {
            let s = std::fs::read_to_string(&global)
                .with_context(|| format!("read {}", global.display()))?;
            let global_cfg: ConfigFile = toml::from_str(&s)
                .with_context(|| format!("parse {}", global.display()))?;
            merge_config(&mut config, &global_cfg);
        }
    }

    // Local: .qp/config.toml (takes precedence)
    if let Some(root) = qp_root {
        let local = root.join("config.toml");
        if local.exists() {
            let s = std::fs::read_to_string(&local)
                .with_context(|| format!("read {}", local.display()))?;
            let local_cfg: ConfigFile = toml::from_str(&s)
                .with_context(|| format!("parse {}", local.display()))?;
            merge_config(&mut config, &local_cfg);
        }
    }

    Ok(config)
}

fn merge_config(base: &mut ConfigFile, override_with: &ConfigFile) {
    if override_with.agent.command != default_agent_command() || !override_with.agent.args.is_empty() {
        base.agent = override_with.agent.clone();
    }
    for (k, v) in &override_with.review_agents {
        base.review_agents.insert(k.clone(), v.clone());
    }
    if !override_with.optimization.steps.is_empty() {
        base.optimization.steps = override_with.optimization.steps.clone();
    }
}

/// Resolve path to global config file (for display).
pub fn global_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("qp").join("config.toml"))
}

/// Serialize config to TOML string for writing to config.toml.
pub fn config_to_toml(config: &ConfigFile) -> Result<String> {
    toml::to_string_pretty(config).context("serialize config to TOML")
}
