//! Plan model: frontmatter, body, CRUD, and .qp directory init.

use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanState {
    Draft,
    Approved,
    Optimizing,
    Ready,
    InProgress,
    Completed,
}

impl std::fmt::Display for PlanState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanState::Draft => write!(f, "draft"),
            PlanState::Approved => write!(f, "approved"),
            PlanState::Optimizing => write!(f, "optimizing"),
            PlanState::Ready => write!(f, "ready"),
            PlanState::InProgress => write!(f, "in_progress"),
            PlanState::Completed => write!(f, "completed"),
        }
    }
}

impl std::str::FromStr for PlanState {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(PlanState::Draft),
            "approved" => Ok(PlanState::Approved),
            "optimizing" => Ok(PlanState::Optimizing),
            "ready" => Ok(PlanState::Ready),
            "in_progress" => Ok(PlanState::InProgress),
            "completed" => Ok(PlanState::Completed),
            _ => anyhow::bail!("unknown state: {}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewStepStatus {
    pub step: String,
    pub status: String, // "pending" | "done" | "failed"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanMeta {
    pub id: String,
    pub title: String,
    pub state: PlanState,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub review_cycles: u32,
    #[serde(default)]
    pub review_steps: Vec<ReviewStepStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_agents: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub meta: PlanMeta,
    pub body: String,
}

const PLAN_FRONTMATTER_DELIM: &str = "---";

/// Parse plan.md content into Plan.
pub fn parse_plan(content: &str) -> Result<Plan> {
    let (front, body) = split_frontmatter(content);
    let meta: PlanMeta = serde_yaml::from_str(front).context("parse plan frontmatter")?;
    let body = body.trim_start().to_string();
    Ok(Plan { meta, body })
}

fn split_frontmatter(content: &str) -> (&str, &str) {
    let content = content.trim_start();
    if !content.starts_with(PLAN_FRONTMATTER_DELIM) {
        return ("", content);
    }
    let rest = content[PLAN_FRONTMATTER_DELIM.len()..].trim_start();
    if let Some(pos) = rest.find(PLAN_FRONTMATTER_DELIM) {
        let front = rest[..pos].trim();
        let body = rest[pos + PLAN_FRONTMATTER_DELIM.len()..].trim_start();
        return (front, body);
    }
    ("", content)
}

/// Serialize Plan to markdown string.
pub fn serialize_plan(plan: &Plan) -> Result<String> {
    let front = serde_yaml::to_string(&plan.meta).context("serialize frontmatter")?;
    Ok(format!("---\n{}\n---\n\n{}", front.trim(), plan.body))
}

/// List plan IDs and their metadata for a .qp root.
pub fn list_plans(qp_root: &Path) -> Result<Vec<PlanMeta>> {
    let plans_dir = qp_root.join("plans");
    if !plans_dir.exists() {
        return Ok(vec![]);
    }
    let mut out = vec![];
    for e in std::fs::read_dir(&plans_dir).context("read plans dir")? {
        let e = e?;
        let path = e.path();
        if path.is_dir() {
            let plan_md = path.join("plan.md");
            if plan_md.exists() {
                let content = std::fs::read_to_string(&plan_md).context("read plan.md")?;
                if let Ok(plan) = parse_plan(&content) {
                    out.push(plan.meta);
                }
            }
        }
    }
    out.sort_by(|a, b| a.updated_at.cmp(&b.updated_at).reverse());
    Ok(out)
}

/// Load a single plan by id or by title slug (or title itself).
pub fn get_plan(qp_root: &Path, id_or_slug: &str) -> Result<Plan> {
    let slug_input = title_to_slug(id_or_slug);
    let plans_dir = qp_root.join("plans");
    for e in std::fs::read_dir(&plans_dir).context("read plans dir")? {
        let e = e?;
        let path = e.path();
        if path.is_dir() {
            let plan_md = path.join("plan.md");
            if plan_md.exists() {
                let content = std::fs::read_to_string(&plan_md).context("read plan.md")?;
                let plan = parse_plan(&content)?;
                let match_id = plan.meta.id == id_or_slug;
                let slug = title_to_slug(&plan.meta.title);
                let match_slug = slug == id_or_slug || slug == slug_input || plan.meta.title == id_or_slug;
                if match_id || match_slug {
                    return Ok(plan);
                }
            }
        }
    }
    anyhow::bail!("plan not found: {}", id_or_slug)
}

fn title_to_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Path to plan directory.
pub fn plan_dir(qp_root: &Path, plan_id: &str) -> PathBuf {
    qp_root.join("plans").join(plan_id)
}

/// Path to plan.md.
pub fn plan_md_path(qp_root: &Path, plan_id: &str) -> PathBuf {
    plan_dir(qp_root, plan_id).join("plan.md")
}

/// Save plan to .qp/plans/<id>/plan.md.
pub fn save_plan(qp_root: &Path, plan: &Plan) -> Result<()> {
    let dir = plan_dir(qp_root, &plan.meta.id);
    std::fs::create_dir_all(&dir).context("create plan dir")?;
    let path = plan_md_path(qp_root, &plan.meta.id);
    let s = serialize_plan(plan)?;
    std::fs::write(&path, s).context("write plan.md")?;
    Ok(())
}

/// Create a new plan with default template. Returns the new plan.
pub fn create_plan(qp_root: &Path, title: Option<&str>) -> Result<Plan> {
    ensure_qp_structure(qp_root)?;
    let id = Uuid::new_v4().to_string();
    let title = title.unwrap_or("Untitled Plan").to_string();
    let now = Utc::now().to_rfc3339();
    let meta = PlanMeta {
        id: id.clone(),
        title: title.clone(),
        state: PlanState::Draft,
        created_at: now.clone(),
        updated_at: now,
        review_cycles: 0,
        review_steps: vec![],
        agent: None,
        review_agents: None,
    };
    let body = default_plan_body();
    let plan = Plan { meta, body };
    save_plan(qp_root, &plan)?;
    Ok(plan)
}

fn default_plan_body() -> String {
    "## Ideas\n\n(Add goals and scope here. When ready, have the agent write the full plan.)".to_string()
}

/// Set plan state to Approved.
pub fn approve_plan(qp_root: &Path, id_or_slug: &str) -> Result<Plan> {
    let mut plan = get_plan(qp_root, id_or_slug)?;
    plan.meta.state = PlanState::Approved;
    plan.meta.updated_at = Utc::now().to_rfc3339();
    save_plan(qp_root, &plan)?;
    Ok(plan)
}

/// Delete plan directory and contents.
pub fn delete_plan(qp_root: &Path, id_or_slug: &str) -> Result<()> {
    let plan = get_plan(qp_root, id_or_slug)?;
    let dir = plan_dir(qp_root, &plan.meta.id);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).context("delete plan dir")?;
    }
    Ok(())
}

/// Default config.toml content when not using the wizard.
pub fn default_config_toml() -> &'static str {
    r#"[agent]
command = "claude"

[optimization]
steps = ["holes", "details", "breakdown", "deliverables"]

[review_agents.holes]
command = "claude"
prompt = "Review this plan and identify weaknesses, missing considerations, edge cases, potential failures, and assumptions that need validation."

[review_agents.details]
command = "claude"
prompt = "Expand this plan with implementation details. Add specifics about technologies, APIs, data structures, and algorithms."

[review_agents.breakdown]
command = "claude"
prompt = "Break this plan into precise, atomic steps. Each step should be independently implementable and testable."

[review_agents.deliverables]
command = "claude"
prompt = "Define clear acceptance criteria for each component. What tests must pass? What can be demonstrated? How do we know it's done?"
"#
}

/// Initialize .qp directory: config.toml, plans/, etc.
/// If config_toml is Some, write that content; else write default if config.toml doesn't exist.
pub fn init_qp(qp_root: &Path, config_toml: Option<&str>) -> Result<()> {
    std::fs::create_dir_all(qp_root).context("create .qp")?;
    std::fs::create_dir_all(qp_root.join("plans")).context("create .qp/plans")?;
    let config_path = qp_root.join("config.toml");
    let to_write = match config_toml {
        Some(s) => s.to_string(),
        None if !config_path.exists() => default_config_toml().to_string(),
        _ => return Ok(()),
    };
    if !config_path.exists() || config_toml.is_some() {
        std::fs::write(&config_path, to_write).context("write config.toml")?;
    }
    Ok(())
}

fn ensure_qp_structure(qp_root: &Path) -> Result<()> {
    std::fs::create_dir_all(qp_root.join("plans")).context("create plans dir")?;
    Ok(())
}

/// Update plan body and updated_at; optionally update state.
pub fn update_plan_body(
    qp_root: &Path,
    plan_id: &str,
    body: &str,
    state: Option<PlanState>,
) -> Result<Plan> {
    let mut plan = get_plan(qp_root, plan_id)?;
    plan.body = body.to_string();
    plan.meta.updated_at = Utc::now().to_rfc3339();
    if let Some(s) = state {
        plan.meta.state = s;
    }
    save_plan(qp_root, &plan)?;
    Ok(plan)
}

/// Append to Review Notes section and save.
pub fn append_review_notes(qp_root: &Path, plan_id: &str, section: &str, notes: &str) -> Result<Plan> {
    let mut plan = get_plan(qp_root, plan_id)?;
    let marker = "## Review Notes";
    if let Some(pos) = plan.body.find(marker) {
        let insert = pos + marker.len();
        let block = format!("\n\n### {}\n\n{}\n", section, notes);
        plan.body.insert_str(insert, &block);
    } else {
        plan.body.push_str(&format!("\n\n## Review Notes\n\n### {}\n\n{}\n", section, notes));
    }
    plan.meta.updated_at = Utc::now().to_rfc3339();
    save_plan(qp_root, &plan)?;
    Ok(plan)
}

/// Record a completed review step and optionally bump review_cycles.
pub fn record_review_step(
    qp_root: &Path,
    plan_id: &str,
    step_name: &str,
    status: &str,
) -> Result<()> {
    let mut plan = get_plan(qp_root, plan_id)?;
    let now = Utc::now().to_rfc3339();
    let mut found = false;
    for rs in &mut plan.meta.review_steps {
        if rs.step == step_name {
            rs.status = status.to_string();
            rs.completed_at = Some(now.clone());
            found = true;
            break;
        }
    }
    if !found {
        plan.meta.review_steps.push(ReviewStepStatus {
            step: step_name.to_string(),
            status: status.to_string(),
            completed_at: Some(now),
        });
    }
    if status == "done" {
        plan.meta.review_cycles += 1;
    }
    plan.meta.updated_at = Utc::now().to_rfc3339();
    save_plan(qp_root, &plan)?;
    Ok(())
}

/// Ensure review_steps has entries for each step name; set pending if missing.
pub fn ensure_review_steps(qp_root: &Path, plan_id: &str, step_names: &[String]) -> Result<()> {
    let mut plan = get_plan(qp_root, plan_id)?;
    for name in step_names {
        if !plan.meta.review_steps.iter().any(|r| r.step == *name) {
            plan.meta.review_steps.push(ReviewStepStatus {
                step: name.clone(),
                status: "pending".to_string(),
                completed_at: None,
            });
        }
    }
    save_plan(qp_root, &plan)?;
    Ok(())
}

/// Save a version snapshot to history/ and return path.
pub fn save_version_snapshot(
    qp_root: &Path,
    plan_id: &str,
    version: u32,
    content: &str,
    review_notes: Option<&str>,
) -> Result<PathBuf> {
    let dir = plan_dir(qp_root, plan_id).join("history");
    std::fs::create_dir_all(&dir).context("create history dir")?;
    let path = dir.join(format!("v{}.md", version));
    std::fs::write(&path, content).context("write version snapshot")?;
    if let Some(notes) = review_notes {
        let notes_path = dir.join(format!("v{}.review.md", version));
        std::fs::write(&notes_path, notes).context("write review notes")?;
    }
    Ok(path)
}
