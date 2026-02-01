//! CLI: clap subcommands, default to list. Requires qp root from discovery.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::io::IsTerminal;
use std::path::PathBuf;

use crate::config::load_config;
use crate::discovery::find_qp_root;
use crate::plan::{self, PlanState};
use crate::optimize;

#[derive(Parser)]
#[command(name = "qp")]
#[command(about = "Quarterly planning: create and optimize plans for AI coding agents")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List plans in scope (default when no subcommand)
    List,
    /// Create a new plan (spawns AI agent)
    New {
        #[arg(value_name = "NAME")]
        name: Option<String>,
    },
    /// Show a plan by id or slug
    Show {
        #[arg(value_name = "PLAN")]
        plan: String,
    },
    /// Edit a plan (spawns AI agent)
    Edit {
        #[arg(value_name = "PLAN")]
        plan: String,
    },
    /// Mark plan as approved, ready for optimization
    Approve {
        #[arg(value_name = "PLAN")]
        plan: String,
    },
    /// Delete a plan
    Delete {
        #[arg(value_name = "PLAN")]
        plan: String,
        #[arg(long)]
        yes: bool,
    },
    /// Run optimization steps
    Optimize {
        #[arg(value_name = "PLAN")]
        plan: String,
        #[arg(long, value_name = "STEP")]
        step: Option<String>,
        #[arg(long)]
        force: bool,
    },
    /// Show optimization history and diffs
    Review {
        #[arg(value_name = "PLAN")]
        plan: String,
    },
    /// Overview of all plans
    Status,
    /// Show statistics
    Stats,
    /// Show version history for a plan
    History {
        #[arg(value_name = "PLAN")]
        plan: String,
    },
    /// Show current configuration
    Config {
        #[arg(long, value_name = "KEY")]
        set: Option<String>,
        #[arg(value_name = "VALUE", num_args = 0..)]
        value: Vec<String>,
    },
    /// Initialize .qp directory and walk through agent, skills, and plugins config
    Init {
        /// Skip interactive wizard; create .qp with default config only
        #[arg(long)]
        no_interactive: bool,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let cwd = std::env::current_dir().context("current dir")?;
    let qp_root = find_qp_root(&cwd);

    match &cli.cmd {
        None => cmd_list(qp_root.as_deref())?,
        Some(Commands::List) => cmd_list(qp_root.as_deref())?,
        Some(Commands::New { name }) => cmd_new(qp_root.as_deref(), name.as_deref())?,
        Some(Commands::Show { plan }) => cmd_show(qp_root.as_deref(), plan)?,
        Some(Commands::Edit { plan }) => cmd_edit(qp_root.as_deref(), plan)?,
        Some(Commands::Approve { plan }) => cmd_approve(qp_root.as_deref(), plan)?,
        Some(Commands::Delete { plan, yes }) => cmd_delete(qp_root.as_deref(), plan, *yes)?,
        Some(Commands::Optimize { plan, step, force }) => {
            cmd_optimize(qp_root.as_deref(), plan, step.as_deref(), *force)?
        }
        Some(Commands::Review { plan }) => cmd_review(qp_root.as_deref(), plan)?,
        Some(Commands::Status) => cmd_status(qp_root.as_deref())?,
        Some(Commands::Stats) => cmd_stats(qp_root.as_deref())?,
        Some(Commands::History { plan }) => cmd_history(qp_root.as_deref(), plan)?,
        Some(Commands::Config { set, value }) => cmd_config(qp_root.as_deref(), set, value)?,
        Some(Commands::Init { no_interactive }) => cmd_init(&cwd, *no_interactive)?,
    }
    Ok(())
}

fn require_qp_root(qp_root: Option<&std::path::Path>) -> Result<PathBuf> {
    match qp_root {
        Some(p) => Ok(p.to_path_buf()),
        None => {
            eprintln!("{} No .qp directory found. Run from a project with .qp or run `qp init` first.", "error:".red());
            std::process::exit(1);
        }
    }
}

fn cmd_list(qp_root: Option<&std::path::Path>) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let plans = plan::list_plans(&root)?;
    if plans.is_empty() {
        println!("No plans. Create one with: qp new [name]");
        return Ok(());
    }
    println!("{}", "Plans:".bold());
    for m in plans {
        let state_color = match m.state {
            PlanState::Draft => "yellow",
            PlanState::Approved => "blue",
            PlanState::Optimizing => "cyan",
            PlanState::Ready => "green",
            PlanState::InProgress => "magenta",
            PlanState::Completed => "white",
        };
        let state_str = format!("{}", m.state);
        let state_display = match state_color {
            "yellow" => state_str.yellow(),
            "blue" => state_str.blue(),
            "cyan" => state_str.cyan(),
            "green" => state_str.green(),
            "magenta" => state_str.magenta(),
            _ => state_str.normal(),
        };
        println!("  {}  {}  {}", m.id, state_display, m.title);
    }
    Ok(())
}

fn cmd_new(qp_root: Option<&std::path::Path>, name: Option<&str>) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let config = load_config(Some(&root))?;
    let plan = plan::create_plan(&root, name)?;
    println!("Created plan: {} ({})", plan.meta.title, plan.meta.id);
    println!("Spawning agent for editing: {} {}", config.agent.command, config.agent.args.join(" "));
    let prompt = format!(
        "Create or refine a quarterly plan. Save the result as a single markdown file with YAML frontmatter (id, title, state, created_at, updated_at, review_cycles, review_steps) and sections: Overview, Constraints, Implementation Notes, Review Notes, Tickets. Each ticket: TICKET: <title>, Summary:, Definition of Done:."
    );
    let mut child = crate::agent::run_agent_interactive(
        &config.agent.command,
        &config.agent.args,
        Some(&prompt),
    )?;
    let _ = child.wait();
    println!("After editing in the agent, save content to: {}", plan::plan_md_path(&root, &plan.meta.id).display());
    Ok(())
}

fn cmd_show(qp_root: Option<&std::path::Path>, plan_ref: &str) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let plan = plan::get_plan(&root, plan_ref)?;
    let out = plan::serialize_plan(&plan)?;
    print!("{}", out);
    Ok(())
}

fn cmd_edit(qp_root: Option<&std::path::Path>, plan_ref: &str) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let config = load_config(Some(&root))?;
    let plan = plan::get_plan(&root, plan_ref)?;
    let path = plan::plan_md_path(&root, &plan.meta.id);
    println!("Spawning agent to edit: {} {}", config.agent.command, config.agent.args.join(" "));
    let prompt = format!(
        "Edit this plan. Keep YAML frontmatter and sections: Overview, Constraints, Implementation Notes, Review Notes, Tickets. Plan path: {}",
        path.display()
    );
    let mut child = crate::agent::run_agent_interactive(
        &config.agent.command,
        &config.agent.args,
        Some(&prompt),
    )?;
    let _ = child.wait();
    println!("Save updated content to: {}", path.display());
    Ok(())
}

fn cmd_approve(qp_root: Option<&std::path::Path>, plan_ref: &str) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let plan = plan::approve_plan(&root, plan_ref)?;
    println!("Approved: {} ({})", plan.meta.title, plan.meta.id);
    Ok(())
}

fn cmd_delete(qp_root: Option<&std::path::Path>, plan_ref: &str, yes: bool) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let plan = plan::get_plan(&root, plan_ref)?;
    if !yes {
        eprintln!("Delete plan \"{}\" ({})? Use --yes to confirm.", plan.meta.title, plan.meta.id);
        std::process::exit(1);
    }
    plan::delete_plan(&root, plan_ref)?;
    println!("Deleted: {}", plan.meta.title);
    Ok(())
}

fn cmd_optimize(
    qp_root: Option<&std::path::Path>,
    plan_ref: &str,
    step: Option<&str>,
    force: bool,
) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let config = load_config(Some(&root))?;
    if let Some(s) = step {
        let _ = optimize::run_step(&root, plan_ref, s, &config)?;
        println!("Step {} completed.", s);
    } else {
        let results = optimize::run_all_steps(&root, plan_ref, &config, force)?;
        println!("Ran {} optimization step(s).", results.len());
    }
    Ok(())
}

fn cmd_review(qp_root: Option<&std::path::Path>, plan_ref: &str) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let plan = plan::get_plan(&root, plan_ref)?;
    let history_dir = plan::plan_dir(&root, &plan.meta.id).join("history");
    if !history_dir.exists() {
        println!("No optimization history yet.");
        return Ok(());
    }
    println!("{} Review steps:", plan.meta.title);
    for rs in &plan.meta.review_steps {
        println!("  {}: {}", rs.step, rs.status);
    }
    println!("\nVersion snapshots in: {}", history_dir.display());
    Ok(())
}

fn cmd_status(qp_root: Option<&std::path::Path>) -> Result<()> {
    cmd_list(qp_root)
}

fn cmd_stats(qp_root: Option<&std::path::Path>) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let plans = plan::list_plans(&root)?;
    let total = plans.len();
    let completed = plans.iter().filter(|p| matches!(p.state, PlanState::Completed)).count();
    let with_reviews = plans.iter().filter(|p| p.review_cycles > 0).count();
    println!("Plans: {} total, {} completed, {} with optimization", total, completed, with_reviews);
    Ok(())
}

fn cmd_history(qp_root: Option<&std::path::Path>, plan_ref: &str) -> Result<()> {
    let root = require_qp_root(qp_root)?;
    let plan = plan::get_plan(&root, plan_ref)?;
    let history_dir = plan::plan_dir(&root, &plan.meta.id).join("history");
    if !history_dir.exists() {
        println!("No version history.");
        return Ok(());
    }
    let mut entries: Vec<_> = std::fs::read_dir(&history_dir)?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.path());
    for e in entries {
        let name = e.file_name().to_string_lossy().into_owned();
        if name.ends_with(".md") && !name.ends_with(".review.md") {
            println!("  {}", name);
        }
    }
    Ok(())
}

fn cmd_config(
    qp_root: Option<&std::path::Path>,
    set: &Option<String>,
    _value: &[String],
) -> Result<()> {
    if set.is_some() {
        eprintln!("qp config set is not implemented yet. Edit config manually: .qp/config.toml or ~/.config/qp/config.toml");
        return Ok(());
    }
    let root = qp_root;
    let config = load_config(root)?;
    println!("agent.command = \"{}\"", config.agent.command);
    println!("agent.args = {:?}", config.agent.args);
    println!("optimization.steps = {:?}", config.optimization.steps);
    for (name, ra) in &config.review_agents {
        println!("review_agents.{} command = \"{}\"", name, ra.command);
        println!("review_agents.{} prompt = \"{}\"", name, ra.prompt);
    }
    if let Some(p) = crate::config::global_config_path() {
        println!("(global config: {})", p.display());
    }
    Ok(())
}

fn cmd_init(cwd: &std::path::Path, no_interactive: bool) -> Result<()> {
    let qp_dir = cwd.join(".qp");
    let config_toml: Option<String> = if no_interactive || !std::io::stdin().is_terminal() {
        None
    } else {
        let (profile_name, command) = crate::init_wizard::choose_agent_profile()?;
        let config = crate::init_wizard::run_wizard(&command)?;
        let toml = crate::config::config_to_toml(&config)?;
        println!("\n  Writing config for agent: {} ({})", profile_name, command);
        Some(toml)
    };
    plan::init_qp(&qp_dir, config_toml.as_deref())?;
    println!("Initialized {}", qp_dir.display());
    Ok(())
}
