# qp - Quarterly Planning Tool

A CLI tool for creating, reviewing, and optimizing plans that will be implemented by AI coding agents.

## Core Concepts

### Workflow
1. **List (default)**: Running `qp` shows plans in scope
2. **Create**: User runs `qp new` → spawns AI tool → user collaborates to create a plan
3. **Save**: Plan is saved as markdown in `.qp/plans/`
4. **Approve**: User marks plan as approved
5. **Optimize**: Sequential optimization steps run via AI subprocesses
6. **Execute**: Final plan is ready for implementation

### Plan Lifecycle States
- `draft` - Initial creation, not yet approved
- `approved` - Ready for optimization
- `optimizing` - Currently undergoing review steps
- `ready` - Optimization complete, ready for execution
- `in_progress` - Being implemented
- `completed` - Done

## Directory Structure

```
.qp/
├── config.toml              # Local configuration
├── plans/
│   └── <plan-id>/
│       ├── plan.md          # The current plan
│       ├── metadata.toml    # State, timestamps, stats
│       └── history/
│           ├── v1.md        # Original draft
│           ├── v2.md        # After first optimization
│           ├── v2.review.md # Review notes for v2
│           └── ...
└── stats.toml               # Aggregate statistics
```

### Plan Discovery Rules

- `qp` looks for a `.qp` in the current directory first.
- If not found, it walks up parent directories until a repository root.
- It stops at the repo root (or filesystem root) and does not search beyond it.
- This allows multiple `.qp` directories in monorepos; nearest wins.

## Configuration

```toml
# ~/.config/qp/config.toml (global) or .qp/config.toml (local)

[agent]
command = "agent"  # or "claude", "aider", etc.
# args = ["--model", "opus"]  # optional extra args

[review_agents.holes]
command = "agent"
prompt = "Review this plan and identify weaknesses, missing considerations, edge cases, potential failures, and assumptions that need validation."

[review_agents.details]
command = "agent"
prompt = "Expand this plan with implementation details. Add specifics about technologies, APIs, data structures, and algorithms."

[review_agents.breakdown]
command = "agent"
prompt = "Break this plan into precise, atomic steps. Each step should be independently implementable and testable."

[review_agents.deliverables]
command = "agent"
prompt = "Define clear acceptance criteria for each component. What tests must pass? What can be demonstrated? How do we know it's done?"

[optimization]
steps = ["holes", "details", "breakdown", "deliverables"]
```

## Commands

### Plan Management
- `qp` - Default to `qp list` for plans in scope
- `qp new [name]` - Create a new plan (spawns AI agent)
- `qp list` - List all plans with status
- `qp show <plan>` - Display a plan
- `qp edit <plan>` - Edit a plan (spawns AI agent)
- `qp approve <plan>` - Mark plan as approved, ready for optimization
- `qp delete <plan>` - Remove a plan

### Optimization
- `qp optimize <plan>` - Run all optimization steps
- `qp optimize <plan> --step holes` - Run specific optimization step
- `qp review <plan>` - Show optimization history and diffs

### Status & Stats
- `qp status` - Overview of all plans
- `qp stats` - Show statistics (transformations, word counts, etc.)
- `qp history <plan>` - Show version history

### Configuration
- `qp config` - Show current configuration
- `qp config set agent.command "agent"` - Set config value
- `qp init` - Initialize .qp directory in current folder

## Optimization Steps (Detail)

Each step spawns a subprocess with its configured review agent and prompt:

### 1. Holes (`holes`)
**Output**: List of concerns with suggested mitigations

### 2. Details (`details`)  
**Output**: Enhanced plan with technical depth

### 3. Breakdown (`breakdown`)
**Output**: Numbered task list with dependencies

### 4. Deliverables (`deliverables`)
**Output**: Testable criteria and definition of done

## Plan Format (Markdown)

Each plan is a markdown file with frontmatter and a task hierarchy.

### Frontmatter (required)

- `id` - plan identifier
- `title` - human-friendly title
- `state` - lifecycle state (`draft`, `approved`, `optimizing`, `ready`, `in_progress`, `completed`)
- `created_at` - timestamp
- `updated_at` - timestamp
- `review_cycles` - count of completed optimization steps
- `review_steps` - list with status per step
- `agent` - creation agent command and args
- `review_agents` - configured review agents and prompts used

### Plan Body (required sections)

- `## Overview` - goals and context
- `## Constraints` - scope, assumptions, and risks
- `## Implementation Notes` - technical details
- `## Review Notes` - accumulated review summaries
- `## Tickets` - hierarchical task list

### Ticket Format (required)

Each ticket is a heading with nested subsections:

```
### TICKET: <title>

Summary: <short description>

Definition of Done:
- ...
- ...
```

Tickets may include nested sub-tickets using deeper heading levels.

## Statistics Tracked

- Plans created/completed
- Average optimizations per plan
- Word count changes (draft → final)
- Time in each state
- Steps most commonly run
- Plan success rate

## Implementation Phases

### Phase 1: Core CLI Structure
- [ ] Set up clap for argument parsing
- [ ] Config file loading (global + local)
- [ ] `.qp` directory initialization
- [ ] Basic plan CRUD (create, read, update, delete)

### Phase 2: Agent Integration
- [ ] Subprocess spawning for configured AI tool
- [ ] Prompt templating system
- [ ] Capture and save agent output
- [ ] Handle agent tool variations (claude, agent, aider)

### Phase 3: Optimization Pipeline
- [ ] Optimization step definitions
- [ ] Sequential step execution
- [ ] Version history tracking
- [ ] Diff generation between versions

### Phase 4: Review & Stats
- [ ] History display with diffs
- [ ] Statistics collection and storage
- [ ] Aggregate reporting
- [ ] Export capabilities

## Dependencies (Suggested)

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
tokio = { version = "1", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
similar = "2"  # for diffs
colored = "2"  # terminal colors
dirs = "5"     # config directories
```

## Open Questions

1. Should optimization steps be customizable per-plan?
2. How to handle agent failures mid-optimization?
3. Should we support parallel optimization branches?
4. Integration with git for plan versioning?
5. How verbose should the agent prompts be?
