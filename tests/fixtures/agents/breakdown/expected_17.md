---
id: "test-017"
title: "CI/CD Pipeline Generator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a tool that generates CI/CD pipeline configurations from a simple specification. Supports GitHub Actions, GitLab CI, and CircleCI as output targets.

## Constraints

- Generated configs must be valid YAML
- Support common language ecosystems

## Implementation Notes

- Written in Python
- Template-based generation
- Extensible for new platforms

## Review Notes

(none yet)

## Tickets

### Ticket 1: Project Detection

**Summary:** Detect project type and dependencies.

**Definition of Done:** Correctly identifies language and build system.

#### Steps

1. **Create Python package structure**
   - Create pipelinegen/ directory
   - Create __init__.py, detector.py
   - Verify: package importable

2. **Define ProjectInfo dataclass**
   - Include: language, version, build_tool, test_command, dependencies
   - Verify: dataclass works

3. **Create detector base class**
   - Define abstract detect(path) -> Optional[ProjectInfo]
   - Verify: base class compiles

4. **Implement Python project detector**
   - Check for pyproject.toml, setup.py, requirements.txt
   - Extract Python version from config
   - Detect pytest, tox, poetry
   - Verify: Python projects detected

5. **Implement Node.js project detector**
   - Check for package.json
   - Extract Node version from engines
   - Detect npm, yarn, pnpm
   - Verify: Node projects detected

6. **Implement Go project detector**
   - Check for go.mod
   - Extract Go version
   - Detect go test command
   - Verify: Go projects detected

7. **Implement Rust project detector**
   - Check for Cargo.toml
   - Extract Rust edition/version
   - Detect cargo test command
   - Verify: Rust projects detected

8. **Implement Java project detector**
   - Check for pom.xml, build.gradle
   - Detect Maven or Gradle
   - Extract Java version
   - Verify: Java projects detected

9. **Create detector registry**
   - Register all detectors
   - Run in order until match
   - Verify: detection works

10. **Handle multi-language projects**
    - Detect all languages present
    - Return primary and secondary
    - Verify: monorepos handled

### Ticket 2: Pipeline Templates

**Summary:** Create templates for each CI platform.

**Definition of Done:** Templates generate valid configurations.

#### Steps

1. **Install Jinja2**
   - Add jinja2 to dependencies
   - Verify: package installed

2. **Create template directory structure**
   - Create templates/github/, templates/gitlab/, templates/circleci/
   - Verify: directories created

3. **Create GitHub Actions base template**
   - Create templates/github/base.yml.j2
   - Define workflow structure with on, jobs
   - Verify: template valid Jinja2

4. **Create GitHub Actions job templates**
   - Create test.yml.j2, build.yml.j2, deploy.yml.j2
   - Verify: job templates work

5. **Create GitLab CI base template**
   - Create templates/gitlab/base.yml.j2
   - Define stages and jobs structure
   - Verify: template valid

6. **Create GitLab CI job templates**
   - Create test.yml.j2, build.yml.j2, deploy.yml.j2
   - Verify: job templates work

7. **Create CircleCI base template**
   - Create templates/circleci/base.yml.j2
   - Define version, jobs, workflows
   - Verify: template valid

8. **Create CircleCI job templates**
   - Create orb references and job definitions
   - Verify: job templates work

9. **Create language-specific templates**
   - Python setup, Node setup, Go setup, etc.
   - Include caching configurations
   - Verify: language templates work

10. **Create template loader**
    - Load templates from package
    - Support custom template directories
    - Verify: templates load correctly

### Ticket 3: Configuration Generation

**Summary:** Generate complete pipeline configs.

**Definition of Done:** Generated configs work in CI systems.

#### Steps

1. **Create generator interface**
   - Define generate(project_info, platform) -> str
   - Verify: interface compiles

2. **Implement GitHub Actions generator**
   - Assemble templates for detected project
   - Generate complete workflow file
   - Verify: valid YAML output

3. **Implement GitLab CI generator**
   - Assemble templates for detected project
   - Generate .gitlab-ci.yml
   - Verify: valid YAML output

4. **Implement CircleCI generator**
   - Assemble templates for detected project
   - Generate .circleci/config.yml
   - Verify: valid YAML output

5. **Add caching configuration**
   - Generate cache keys based on lock files
   - Add cache restore and save steps
   - Verify: caching configured

6. **Add matrix builds**
   - Support multiple versions in config
   - Generate matrix strategy
   - Verify: matrix works

7. **Add deployment stages**
   - Generate staging and production deploys
   - Add environment protection
   - Verify: deployment stages work

8. **Create CLI interface**
   - Add generate command
   - Accept --platform, --output options
   - Verify: CLI works

9. **Validate generated YAML**
   - Parse generated output
   - Check for syntax errors
   - Verify: output valid YAML

10. **Write generated file**
    - Write to appropriate location
    - .github/workflows/ for GitHub
    - Verify: file written correctly

11. **Add dry-run option**
    - Print generated config without writing
    - Verify: dry-run works

12. **Test with real projects**
    - Generate configs for sample projects
    - Run in actual CI systems
    - Verify: pipelines execute successfully
