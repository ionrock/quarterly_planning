//! Optimization pipeline: run configured review steps in sequence, save versions, track state.

use anyhow::{Context, Result};
use std::path::Path;

use crate::agent;
use crate::config::ConfigFile;
use crate::plan::{
    self, ensure_review_steps, record_review_step, save_version_snapshot,
    Plan, PlanState,
};

/// Run a single optimization step: load plan, run review agent, merge result (replace body with agent output), save version, record step.
pub fn run_step(
    qp_root: &Path,
    plan_id: &str,
    step_name: &str,
    config: &ConfigFile,
) -> Result<Plan> {
    let review_agent = config
        .review_agents
        .get(step_name)
        .with_context(|| format!("unknown step: {}", step_name))?;
    let mut plan = plan::get_plan(qp_root, plan_id)?;
    plan.meta.state = PlanState::Optimizing;
    plan::save_plan(qp_root, &plan)?;

    let plan_content = crate::plan::serialize_plan(&plan)?;
    let version_before = (plan.meta.review_cycles + 1) as u32;
    let snapshot_before = plan_content.clone();
    save_version_snapshot(
        qp_root,
        &plan.meta.id,
        version_before,
        &snapshot_before,
        None,
    )?;

    let output = agent::run_agent_oneshot(
        &review_agent.command,
        &review_agent.args,
        &review_agent.prompt,
        &plan_content,
    )?;

    // Agent output may be raw markdown (revised plan) or markdown with frontmatter.
    // If it looks like a full plan (has --- and body), use body only; else append as review notes.
    plan.body = parse_agent_output(&output, &plan);
    plan.meta.updated_at = chrono::Utc::now().to_rfc3339();
    plan::save_plan(qp_root, &plan)?;

    save_version_snapshot(
        qp_root,
        &plan.meta.id,
        version_before + 1,
        &crate::plan::serialize_plan(&plan)?,
        None,
    )?;
    record_review_step(qp_root, &plan.meta.id, step_name, "done")?;

    let mut plan = plan::get_plan(qp_root, plan_id)?;
    let all_done = config
        .optimization
        .steps
        .iter()
        .all(|s| plan.meta.review_steps.iter().any(|r| r.step == *s && r.status == "done"));
    if all_done {
        plan.meta.state = PlanState::Ready;
    } else {
        plan.meta.state = PlanState::Approved; // back to approved for next step
    }
    plan.meta.updated_at = chrono::Utc::now().to_rfc3339();
    plan::save_plan(qp_root, &plan)?;
    Ok(plan)
}

/// If agent returned full plan (frontmatter + body), return its body; else append output as review notes and return combined body.
fn parse_agent_output(output: &str, fallback_plan: &Plan) -> String {
    let trimmed = output.trim();
    if trimmed.starts_with("---") {
        if let Ok(parsed) = plan::parse_plan(trimmed) {
            return parsed.body;
        }
    }
    let marker = "## Review Notes";
    let mut body = fallback_plan.body.clone();
    let block = format!("Optimization output:\n\n{}", trimmed);
    if let Some(pos) = body.find(marker) {
        let insert = pos + marker.len();
        body.insert_str(insert, &format!("\n\n### {}\n\n", block));
    } else {
        body.push_str(&format!("\n\n## Review Notes\n\n### {}\n\n", block));
    }
    body
}

/// Run all optimization steps in order. Skips steps already done unless --force.
pub fn run_all_steps(
    qp_root: &Path,
    plan_id: &str,
    config: &ConfigFile,
    force: bool,
) -> Result<Vec<Plan>> {
    ensure_review_steps(qp_root, plan_id, &config.optimization.steps)?;
    let mut results = vec![];
    for step in &config.optimization.steps {
        let plan = plan::get_plan(qp_root, plan_id)?;
        let already_done = plan
            .meta
            .review_steps
            .iter()
            .any(|r| r.step == *step && r.status == "done");
        if already_done && !force {
            continue;
        }
        let plan = run_step(qp_root, plan_id, step, config)?;
        results.push(plan);
    }
    Ok(results)
}
