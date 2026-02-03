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
            prompt: "Review this plan and identify gaps, risks, and issues. Output the complete plan with an updated Review Notes section (## Review Notes) containing these four subsections:\n\n\
### Identified Weaknesses\n\
List at least 5 specific weaknesses in the plan. Focus on missing security considerations, unclear requirements, underspecified behavior, missing error handling, and architectural gaps. Be concrete and actionable.\n\n\
### Edge Cases\n\
List at least 5 edge cases that the plan doesn't address. Think about boundary conditions, error states, concurrent access, invalid inputs, and failure scenarios.\n\n\
### Assumptions to Validate\n\
List at least 4 assumptions the plan makes that should be verified before implementation. These are things that could change the approach if they turn out to be false.\n\n\
### Potential Failures\n\
List at least 4 ways the implementation could fail in production. Consider infrastructure failures, data issues, scaling problems, and operational concerns.\n\n\
Output the entire plan with the Review Notes section populated. Keep all other sections (Overview, Constraints, Implementation Notes, Tickets) unchanged.".to_string(),
        },
    );
    m.insert(
        "details".to_string(),
        ReviewAgentConfig {
            command: DEFAULT_AGENT_COMMAND.to_string(),
            args: vec![],
            prompt: "Expand this plan with implementation details.\n\n\
CRITICAL: Your output must be the COMPLETE, EXPANDED plan - not a summary or description of changes.\n\
Do NOT ask for permission. Do NOT describe what you would add. Just output the full plan.\n\n\
Add an ## Implementation Notes section (if not present, or expand existing) with:\n\n\
### Technology Stack\n\
Specify exact versions and libraries. Include language version, key dependencies with versions, build tools, and testing frameworks.\n\n\
### Data Structures\n\
Define the core data structures with actual code. Show structs/classes, interfaces/traits, and type definitions. Include field types and documentation.\n\n\
### Algorithms & Logic\n\
Document key algorithms with pseudocode or actual code. Explain the approach and any important implementation details.\n\n\
### API Design\n\
If applicable, show endpoint signatures, request/response schemas, and error formats.\n\n\
Your response must START with the plan's YAML frontmatter (---) and include ALL sections: Overview, Constraints, Implementation Notes, Review Notes, and Tickets.\n\
Do NOT write meta-commentary about the plan. Output ONLY the plan content.".to_string(),
        },
    );
    m.insert(
        "breakdown".to_string(),
        ReviewAgentConfig {
            command: DEFAULT_AGENT_COMMAND.to_string(),
            args: vec![],
            prompt: "Break this plan into precise, atomic steps.\n\n\
CRITICAL: Your output must be the COMPLETE plan with detailed steps added - not a summary or description.\n\
Do NOT ask for permission. Do NOT describe what steps you would add. Output the full plan with steps.\n\n\
For each ticket in the ## Tickets section, add a #### Steps subsection containing:\n\
- Numbered steps (1, 2, 3, etc.) that are atomic and independently implementable\n\
- Each step should have a clear action and a verification method\n\
- Steps should be small enough to complete in under 2 hours\n\
- Include specific commands, file names, and technical details\n\
- End each step with 'Verify:' describing how to confirm completion\n\n\
Example step format:\n\
1. **Create user model file**\n\
   - Create src/models/user.ts with User interface\n\
   - Include id, email, name, createdAt fields\n\
   - Verify: TypeScript compiles without errors\n\n\
Your response must START with the plan's YAML frontmatter (---) and include ALL sections.\n\
Output ONLY the plan content with steps added. No meta-commentary.".to_string(),
        },
    );
    m.insert(
        "deliverables".to_string(),
        ReviewAgentConfig {
            command: DEFAULT_AGENT_COMMAND.to_string(),
            args: vec![],
            prompt: "Add clear acceptance criteria for each ticket in the plan.\n\n\
CRITICAL: Your output must be the COMPLETE plan with acceptance criteria added - not a summary or description.\n\
Do NOT ask for permission. Do NOT describe what criteria you would add. Output the full plan with criteria.\n\n\
For each ticket in the ## Tickets section, add these subsections:\n\n\
#### Acceptance Criteria\n\
Numbered groups of specific, testable requirements. Use checkbox format for each item:\n\
1. **Category Name**\n\
   - [ ] Specific testable requirement\n\
   - [ ] Another testable requirement\n\n\
#### Demo Script\n\
Concrete commands or code showing how to verify the feature works. Include expected output.\n\n\
#### Test Requirements\n\
Specific tests that must pass, with checkboxes:\n\
- [ ] Test description\n\
- [ ] Another test\n\n\
Focus on:\n\
- What specific behaviors must be verified?\n\
- What can be demonstrated to stakeholders?\n\
- How do we know the ticket is complete?\n\n\
Your response must START with the plan's YAML frontmatter (---) and include ALL sections.\n\
Output ONLY the plan content with acceptance criteria added. No meta-commentary.".to_string(),
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
