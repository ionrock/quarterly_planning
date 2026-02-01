# qp — Quarterly Planning

A CLI for creating, reviewing, and optimizing plans that AI coding agents (Cursor, Claude, Aider, etc.) will implement. You draft plans with your AI tool, approve them, then run optimization steps that strengthen the plan before execution.

---

## Install

```bash
cargo install --path .
```

Or from this repo: `cargo build --release` and use `target/release/qp`.

---

## Quick start

1. **Initialize** a project (creates `.qp/` with config and plan format):

   ```bash
   qp init
   ```

   Use `qp init --no-interactive` to create `.qp` with defaults and no wizard.

2. **Create a plan** (spawns your configured AI agent to brainstorm):

   ```bash
   qp new "My feature"
   ```

   The agent runs in *plan mode*: it asks questions to clarify goals and scope. When you’re ready, ask it to write the full plan. It will be told to write to `.qp/plans/<id>/plan.md` using the required format.

3. **Save** the plan to the path qp prints (e.g. have the agent write the file, or paste the content yourself).

4. **Approve** the plan so it’s ready for optimization:

   ```bash
   qp approve <plan-id-or-title>
   ```

5. **Optimize** (run AI review steps: holes, details, breakdown, deliverables):

   ```bash
   qp optimize <plan-id-or-title>
   ```

   Use `qp optimize <plan> --step holes` to run one step, or `--force` to re-run steps.

6. **Review** optimization history and version snapshots:

   ```bash
   qp review <plan-id-or-title>
   ```

---

## Workflow and plan states

| State         | Meaning |
|---------------|--------|
| `draft`       | Being written or edited; not yet approved. |
| `approved`    | Ready for optimization. |
| `optimizing`  | Optimization steps are running. |
| `ready`       | Optimization complete; ready for implementation. |
| `in_progress` | Being implemented. |
| `completed`   | Done. |

Flow: **Create** → **Approve** → **Optimize** → **Ready** → (optional) mark **in_progress** / **completed**.

---

## Commands

| Command | Description |
|--------|-------------|
| `qp` or `qp list` | List plans in scope (current or nearest `.qp` up to repo root). |
| `qp new [name]` | Create a plan and spawn the AI agent for editing. |
| `qp show <plan>` | Print plan content (by id or slug). |
| `qp edit <plan>` | Spawn the AI agent to edit the plan. |
| `qp approve <plan>` | Mark plan as approved (enables optimization). |
| `qp optimize <plan>` | Run all optimization steps. |
| `qp optimize <plan> --step <name>` | Run a single step (e.g. `holes`, `details`). |
| `qp optimize <plan> --force` | Re-run steps even if already done. |
| `qp review <plan>` | Show optimization history and step status. |
| `qp delete <plan> --yes` | Remove a plan. |
| `qp status` | Same as `qp list`. |
| `qp stats` | Count of plans, completed, and with optimization. |
| `qp history <plan>` | List version snapshots for a plan. |
| `qp config` | Show current configuration. |
| `qp init` | Create `.qp` and walk through agent/plugins config. |
| `qp init --no-interactive` | Create `.qp` with default config only. |

---

## Directory structure

```
.qp/
├── config.toml       # Agent command, optimization steps, review-agent prompts
├── plan-format.md    # Canonical plan format (for AI tools and humans)
└── plans/
    └── <plan-id>/
        ├── plan.md   # Current plan (frontmatter + body)
        └── history/  # Version snapshots (e.g. v1.md, v2.md, v2.review.md)
```

**Discovery:** qp looks for `.qp` in the current directory, then walks up until a repo root (`.git`). The nearest `.qp` wins (supports multiple in a monorepo).

---

## Plan format

Plans are Markdown with YAML frontmatter. The exact spec is in **`.qp/plan-format.md`** (created by `qp init`). Summary:

- **Frontmatter (required):** `id`, `title`, `state`, `created_at`, `updated_at`. Optional: `review_cycles`, `review_steps`, `agent`, `review_agents`.
- **Body sections (required, order matters):** `## Overview`, `## Constraints`, `## Implementation Notes`, `## Review Notes`, `## Tickets`. Under Tickets, each item has **Summary** and **Definition of Done**.

When you run `qp new` or `qp edit`, the agent receives the path to `plan.md` and this structure so it can write or edit in the right place.

---

## Configuration

Config is merged from **global** (`~/.config/qp/config.toml`) and **local** (`.qp/config.toml`). Local overrides global.

```toml
[agent]
command = "cursor"   # or "claude", "aider", etc.
# args = ["--model", "opus"]

[optimization]
steps = ["holes", "details", "breakdown", "deliverables"]

[review_agents.holes]
command = "claude"
prompt = "Review this plan and identify weaknesses, missing considerations, edge cases, potential failures, and assumptions that need validation."

# ... review_agents.details, review_agents.breakdown, review_agents.deliverables
```

Optional plugins (extra steps) can be added in the init wizard or by editing config: e.g. `risk-check`, `strict-deliverables`, `dependencies`.

---

## Optimization steps (defaults)

| Step | Purpose |
|------|--------|
| **holes** | Find weaknesses, edge cases, and assumptions to validate. |
| **details** | Add technical depth: APIs, data structures, algorithms. |
| **breakdown** | Turn the plan into atomic, testable steps. |
| **deliverables** | Define acceptance criteria and definition of done. |

Each step runs your configured review agent (e.g. Claude) with the plan content and a step-specific prompt; the agent’s output is merged into the plan and a version is saved in `history/`.

---

## Aspirational / roadmap

- **`qp config set <key> <value>`** — Set config values from the CLI (today: edit config files by hand).
- **Richer stats** — Word-count deltas (draft → final), time in each state, which steps are run most, success rate.
- **Export** — Export plans or summaries (e.g. Markdown, JSON) for external tools or reporting.
- **Review diffs** — In `qp review`, show diffs between version snapshots (foundation exists; UX to be improved).
- **Git integration** — Optional hooks or commands to snapshot plans on commit or branch.
- **Per-plan customization** — Override optimization steps or prompts per plan.
- **Parallel optimization** — Run independent steps in parallel where safe.

---

## License

See repository for license details.
