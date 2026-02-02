---
id: "test-017"
title: "CI/CD Pipeline Generator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a tool that generates CI/CD pipeline configurations from a simple YAML spec. Supports GitHub Actions, GitLab CI, and CircleCI output formats. Detects project type and suggests appropriate pipelines.

## Constraints

- Generated configs must be valid for each platform
- Support Node.js, Python, Go, and Rust projects

## Implementation Notes

### Technology Stack
- **Language:** Python 3.11+
- **CLI:** Typer for command interface
- **Templates:** Jinja2 for config generation
- **Validation:** pydantic for schema validation

### Input Schema (pipeline.yaml)
```yaml
name: my-app
language: node  # node, python, go, rust

stages:
  - name: test
    steps:
      - run: npm test

  - name: build
    steps:
      - run: npm run build
    artifacts:
      - dist/

  - name: deploy
    needs: [build]
    environment: production
    steps:
      - run: ./deploy.sh
    only:
      - main

cache:
  paths:
    - node_modules/
    - .npm/

env:
  NODE_ENV: production
```

### Pydantic Models
```python
from pydantic import BaseModel, Field
from typing import Optional
from enum import Enum

class Language(str, Enum):
    NODE = "node"
    PYTHON = "python"
    GO = "go"
    RUST = "rust"

class Step(BaseModel):
    run: Optional[str] = None
    name: Optional[str] = None
    uses: Optional[str] = None  # For GH Actions
    with_: Optional[dict] = Field(None, alias="with")

class Stage(BaseModel):
    name: str
    steps: list[Step]
    needs: list[str] = []
    artifacts: list[str] = []
    environment: Optional[str] = None
    only: list[str] = []  # Branch filters

class Pipeline(BaseModel):
    name: str
    language: Language
    stages: list[Stage]
    cache: Optional[dict] = None
    env: dict[str, str] = {}
```

### Project Detection
```python
from pathlib import Path

def detect_project(path: Path) -> Language:
    """Auto-detect project language from files."""
    if (path / "package.json").exists():
        return Language.NODE
    if (path / "pyproject.toml").exists() or (path / "requirements.txt").exists():
        return Language.PYTHON
    if (path / "go.mod").exists():
        return Language.GO
    if (path / "Cargo.toml").exists():
        return Language.RUST
    raise ValueError("Could not detect project type")

def get_default_pipeline(language: Language) -> Pipeline:
    """Generate sensible defaults for detected language."""
    templates = {
        Language.NODE: {
            "stages": [
                {"name": "install", "steps": [{"run": "npm ci"}]},
                {"name": "lint", "steps": [{"run": "npm run lint"}]},
                {"name": "test", "steps": [{"run": "npm test"}]},
                {"name": "build", "steps": [{"run": "npm run build"}]},
            ],
            "cache": {"paths": ["node_modules/", "~/.npm"]},
        },
        Language.PYTHON: {
            "stages": [
                {"name": "install", "steps": [{"run": "pip install -e '.[dev]'"}]},
                {"name": "lint", "steps": [{"run": "ruff check ."}]},
                {"name": "test", "steps": [{"run": "pytest"}]},
            ],
            "cache": {"paths": [".venv/", "~/.cache/pip"]},
        },
        # ... go, rust
    }
    return Pipeline(name="ci", language=language, **templates[language])
```

### GitHub Actions Generator
```python
GITHUB_TEMPLATE = '''
name: {{ pipeline.name }}

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

{% if pipeline.env %}
env:
{% for key, value in pipeline.env.items() %}
  {{ key }}: {{ value }}
{% endfor %}
{% endif %}

jobs:
{% for stage in pipeline.stages %}
  {{ stage.name }}:
    runs-on: ubuntu-latest
{% if stage.needs %}
    needs: [{{ stage.needs | join(", ") }}]
{% endif %}
{% if stage.environment %}
    environment: {{ stage.environment }}
{% endif %}
{% if stage.only %}
    if: github.ref == 'refs/heads/{{ stage.only[0] }}'
{% endif %}
    steps:
      - uses: actions/checkout@v4
{% if pipeline.cache %}
      - uses: actions/cache@v3
        with:
          path: |
{% for p in pipeline.cache.paths %}
            {{ p }}
{% endfor %}
          key: ${{ runner.os }}-{{ pipeline.language }}-${{ hashFiles('**/lockfile') }}
{% endif %}
{% for step in stage.steps %}
      - name: {{ step.name or step.run[:50] }}
        run: {{ step.run }}
{% endfor %}
{% if stage.artifacts %}
      - uses: actions/upload-artifact@v3
        with:
          name: {{ stage.name }}-artifacts
          path: |
{% for artifact in stage.artifacts %}
            {{ artifact }}
{% endfor %}
{% endif %}
{% endfor %}
'''

def generate_github_actions(pipeline: Pipeline) -> str:
    template = jinja2.Template(GITHUB_TEMPLATE)
    return template.render(pipeline=pipeline)
```

### CLI Interface
```python
import typer

app = typer.Typer()

@app.command()
def generate(
    input: Path = typer.Option("pipeline.yaml", help="Input pipeline spec"),
    output: Path = typer.Option(None, help="Output file"),
    platform: str = typer.Option("github", help="github, gitlab, or circleci"),
):
    """Generate CI/CD configuration from pipeline spec."""
    pipeline = Pipeline.parse_file(input)

    generators = {
        "github": generate_github_actions,
        "gitlab": generate_gitlab_ci,
        "circleci": generate_circleci,
    }

    config = generators[platform](pipeline)

    if output:
        output.write_text(config)
        typer.echo(f"Generated {output}")
    else:
        typer.echo(config)

@app.command()
def init(
    path: Path = typer.Option(".", help="Project path"),
):
    """Initialize pipeline.yaml from detected project type."""
    language = detect_project(path)
    pipeline = get_default_pipeline(language)
    (path / "pipeline.yaml").write_text(pipeline.yaml())
    typer.echo(f"Created pipeline.yaml for {language.value} project")
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Project Detection

**Summary:** Detect project type and extract relevant configuration.

**Definition of Done:** Correctly identifies language and framework.

### Ticket 2: Pipeline Model

**Summary:** Define abstract pipeline model with stages and steps.

**Definition of Done:** Model represents common CI/CD concepts.

### Ticket 3: Config Generators

**Summary:** Generate platform-specific YAML from pipeline model.

**Definition of Done:** Generated configs are valid for all platforms.
