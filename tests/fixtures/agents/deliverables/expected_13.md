---
id: "test-013"
title: "Git Repository Analyzer"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a tool that analyzes Git repositories for code quality metrics, contributor statistics, and change patterns. Generates reports and visualizations.

## Constraints

- Analyze repositories with 100k+ commits
- Complete analysis in under 5 minutes

## Implementation Notes

- Python with GitPython
- SQLite for caching
- Matplotlib for visualizations

## Review Notes

(none yet)

## Tickets

### Ticket 1: Commit Analysis

**Summary:** Parse and categorize commits.

**Definition of Done:** All commits extracted with metadata.

#### Acceptance Criteria

1. **Commit Extraction**
   - [ ] Extract: hash, author, date, message, files changed
   - [ ] Support: additions, deletions, renames per file
   - [ ] Handle merge commits appropriately
   - [ ] Parse all branches or specific branch

2. **Commit Categorization**
   - [ ] Detect conventional commits (feat, fix, docs, etc.)
   - [ ] Categorize by file type changed
   - [ ] Tag breaking changes
   - [ ] Identify refactoring commits (high churn, no behavior change)

3. **Author Statistics**
   - [ ] Commits per author
   - [ ] Lines added/removed per author
   - [ ] Active days per author
   - [ ] First and last commit dates

4. **Time Analysis**
   - [ ] Commits by hour of day
   - [ ] Commits by day of week
   - [ ] Commits over time (weekly/monthly)
   - [ ] Identify quiet and active periods

5. **Caching**
   - [ ] Cache parsed commits in SQLite
   - [ ] Incremental updates (only new commits)
   - [ ] Cache invalidation on force push

#### Demo Script
```bash
# Analyze repository
git-analyzer analyze ./my-repo --output analysis.db

# Output:
# Parsing commits... 50,000 commits found
# Analyzing authors... 125 contributors
# Building time series... done
# Saved to analysis.db

# Quick stats
git-analyzer stats ./my-repo
# Repository: my-repo
# Total commits: 50,000
# Contributors: 125
# Active since: 2020-01-15
# Most active day: Tuesday
# Most active hour: 10:00-11:00

# Author leaderboard
git-analyzer authors ./my-repo --top 10
# Author          Commits  Lines Added  Lines Removed
# alice@ex.com    5,234    150,000      80,000
# bob@ex.com      3,102    90,000       45,000
# ...
```

#### Test Requirements
- [ ] Test with small repo (100 commits)
- [ ] Test with large repo (100k+ commits)
- [ ] Test incremental cache update
- [ ] Test merge commit handling
- [ ] Test conventional commit parsing
- [ ] Benchmark: 100k commits in under 2 minutes

### Ticket 2: Code Metrics

**Summary:** Calculate complexity and churn metrics.

**Definition of Done:** Metrics calculated for each file.

#### Acceptance Criteria

1. **File Churn**
   - [ ] Times each file was modified
   - [ ] Total lines added/removed per file
   - [ ] Authors who modified each file
   - [ ] Recent vs historical churn

2. **Hotspot Detection**
   - [ ] Files with high churn + high complexity
   - [ ] Rank files by "hotspot score"
   - [ ] Identify files changed together frequently
   - [ ] Detect code ownership (primary author per file)

3. **Complexity Metrics**
   - [ ] Lines of code per file
   - [ ] Cyclomatic complexity (for supported languages)
   - [ ] Nesting depth
   - [ ] Function/method count

4. **Change Coupling**
   - [ ] Files that change together (>50% correlation)
   - [ ] Identify hidden dependencies
   - [ ] Suggest module boundaries

5. **Trend Analysis**
   - [ ] Complexity over time
   - [ ] Churn over time
   - [ ] Growing vs shrinking files
   - [ ] Abandoned files (no changes in 6+ months)

#### Demo Script
```bash
# Calculate metrics
git-analyzer metrics ./my-repo

# Show hotspots
git-analyzer hotspots ./my-repo --top 20
# File                          Churn  Complexity  Hotspot Score
# src/core/engine.py            89     45          4,005
# src/api/handlers.py           67     38          2,546
# src/utils/helpers.py          112    22          2,464

# Show change coupling
git-analyzer coupling ./my-repo --threshold 0.5
# Files changed together (>50% of the time):
# src/models/user.py <-> src/api/user_handler.py (78%)
# tests/test_auth.py <-> src/auth/login.py (65%)

# Show complexity trend
git-analyzer trend ./my-repo --file src/core/engine.py
# Date        Lines  Complexity
# 2024-01     500    25
# 2024-04     650    32
# 2024-07     800    45  # ← Growing complexity
```

#### Test Requirements
- [ ] Test churn calculation accuracy
- [ ] Test hotspot ranking algorithm
- [ ] Test change coupling detection
- [ ] Test complexity calculation (Python, JS)
- [ ] Test trend analysis over time

### Ticket 3: Report Generation

**Summary:** Generate HTML reports with charts.

**Definition of Done:** Reports display all metrics clearly.

#### Acceptance Criteria

1. **HTML Report**
   - [ ] Single-file HTML report (embedded CSS/JS)
   - [ ] Executive summary at top
   - [ ] Interactive charts
   - [ ] Sortable tables

2. **Visualizations**
   - [ ] Commit activity heatmap (GitHub-style)
   - [ ] Author contribution pie chart
   - [ ] Commits over time line chart
   - [ ] File churn bar chart
   - [ ] Hotspot treemap

3. **Report Sections**
   - [ ] Overview: total commits, authors, timespan
   - [ ] Authors: top contributors, activity patterns
   - [ ] Files: hotspots, complexity, churn
   - [ ] Trends: metrics over time

4. **Export Formats**
   - [ ] HTML (default)
   - [ ] JSON (for programmatic use)
   - [ ] Markdown (for GitHub/GitLab)
   - [ ] PDF (via headless browser)

5. **Customization**
   - [ ] Date range filter
   - [ ] Branch filter
   - [ ] File pattern filter (e.g., "src/**/*.py")
   - [ ] Report template customization

#### Demo Script
```bash
# Generate HTML report
git-analyzer report ./my-repo --output report.html
# Report generated: report.html

# Open in browser
open report.html

# Report shows:
# - Summary card: 50k commits, 125 authors, 4 years
# - Heatmap of commit activity
# - Top 10 authors chart
# - Hotspot treemap (clickable)
# - Complexity trend chart

# Generate JSON for CI integration
git-analyzer report ./my-repo --format json --output metrics.json

# Generate for specific date range
git-analyzer report ./my-repo --since 2024-01-01 --until 2024-06-30

# Generate Markdown for PR comment
git-analyzer report ./my-repo --format markdown --output METRICS.md
```

#### Test Requirements
- [ ] Test HTML report renders correctly
- [ ] Test all chart types
- [ ] Test JSON output schema
- [ ] Test date range filtering
- [ ] Test file pattern filtering
- [ ] Visual regression test for charts
