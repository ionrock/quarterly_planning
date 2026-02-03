---
id: "test-011"
title: "Static Site Generator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a static site generator with Markdown support, templating, and incremental builds. Generates HTML from content files with frontmatter metadata.

## Constraints

- Build 1000 pages in under 10 seconds
- Hot reload in development mode

## Implementation Notes

- Rust for performance
- Liquid templates
- CommonMark for Markdown

## Review Notes

(none yet)

## Tickets

### Ticket 1: Content Processing

**Summary:** Parse Markdown with frontmatter and render to HTML.

**Definition of Done:** All Markdown features render correctly.

#### Acceptance Criteria

1. **Frontmatter Parsing**
   - [ ] YAML frontmatter between --- delimiters
   - [ ] Required fields: title
   - [ ] Optional fields: date, tags, layout, draft
   - [ ] Custom fields accessible in templates
   - [ ] Invalid frontmatter shows clear error with line number

2. **Markdown Rendering**
   - [ ] CommonMark spec compliance
   - [ ] Code blocks with syntax highlighting (highlight.js classes)
   - [ ] Tables (GitHub-flavored Markdown)
   - [ ] Task lists (- [ ] and - [x])
   - [ ] Footnotes
   - [ ] Auto-linked URLs

3. **Content Enhancements**
   - [ ] Automatic heading IDs for anchor links
   - [ ] Table of contents generation
   - [ ] Image optimization (srcset generation)
   - [ ] External links open in new tab

4. **Draft Handling**
   - [ ] draft: true excludes from production build
   - [ ] draft: true included with --drafts flag
   - [ ] Visual indicator in dev mode

#### Demo Script
```bash
# Create content file
cat > content/posts/hello.md << 'EOF'
---
title: "Hello World"
date: 2024-01-15
tags: [intro, test]
---

# Welcome

This is my first post with **bold** and `code`.

```python
print("Hello, World!")
```

| Column 1 | Column 2 |
|----------|----------|
| A        | B        |
EOF

# Build site
ssg build

# Check output
cat public/posts/hello/index.html
# <h1 id="welcome">Welcome</h1>
# <p>This is my first post with <strong>bold</strong> and <code>code</code>.</p>
# <pre><code class="language-python">print("Hello, World!")</code></pre>
# <table>...</table>

# Build with drafts
ssg build --drafts
```

#### Test Requirements
- [ ] Test frontmatter parsing (valid and invalid)
- [ ] Test all Markdown features
- [ ] Test syntax highlighting for 10+ languages
- [ ] Test draft filtering
- [ ] Benchmark: parse 1000 files in under 2 seconds

### Ticket 2: Template Engine

**Summary:** Liquid templates with layouts and partials.

**Definition of Done:** Templates render with variables and includes.

#### Acceptance Criteria

1. **Layout System**
   - [ ] Default layout in layouts/default.html
   - [ ] Page specifies layout in frontmatter
   - [ ] Nested layouts supported
   - [ ] {{ content }} inserts page content

2. **Variables**
   - [ ] {{ page.title }}, {{ page.date }}, etc.
   - [ ] {{ site.title }}, {{ site.url }} from config
   - [ ] {{ collections.posts }} lists all posts
   - [ ] Custom variables from frontmatter

3. **Includes/Partials**
   - [ ] {% include "header.html" %}
   - [ ] Partials in includes/ directory
   - [ ] Variables passed to includes
   - [ ] Recursive includes supported

4. **Filters and Tags**
   - [ ] Date formatting: {{ page.date | date: "%B %d, %Y" }}
   - [ ] String filters: upcase, downcase, slugify
   - [ ] Array filters: sort, reverse, first, last
   - [ ] Conditional: {% if %}, {% unless %}
   - [ ] Loops: {% for post in collections.posts %}

5. **Error Handling**
   - [ ] Missing variable returns empty string (configurable)
   - [ ] Missing include shows error with file path
   - [ ] Syntax errors show line number

#### Demo Script
```bash
# Create layout
cat > layouts/default.html << 'EOF'
<!DOCTYPE html>
<html>
<head><title>{{ page.title }} | {{ site.title }}</title></head>
<body>
  {% include "header.html" %}
  <main>{{ content }}</main>
  {% include "footer.html" %}
</body>
</html>
EOF

# Create include
cat > includes/header.html << 'EOF'
<header>
  <nav>
    {% for item in site.nav %}
      <a href="{{ item.url }}">{{ item.title }}</a>
    {% endfor %}
  </nav>
</header>
EOF

# Build and verify
ssg build
cat public/posts/hello/index.html
# Verify layout and includes rendered
```

#### Test Requirements
- [ ] Test layout inheritance
- [ ] Test all Liquid filters
- [ ] Test variable scoping
- [ ] Test include with parameters
- [ ] Test error messages for invalid templates

### Ticket 3: Build System

**Summary:** Incremental builds with dependency tracking.

**Definition of Done:** Only changed files are rebuilt.

#### Acceptance Criteria

1. **Full Build**
   - [ ] `ssg build` builds all content
   - [ ] Output to public/ directory (configurable)
   - [ ] Clean output before full build
   - [ ] Copy static assets (images, CSS, JS)

2. **Incremental Build**
   - [ ] `ssg build --incremental` only rebuilds changed files
   - [ ] Track file modification times
   - [ ] Rebuild dependents when include/layout changes
   - [ ] Cache parsed templates

3. **Dependency Tracking**
   - [ ] Track which templates use which includes
   - [ ] Track which pages use which layouts
   - [ ] Invalidate all dependents on change
   - [ ] Dependency graph persisted between builds

4. **Development Mode**
   - [ ] `ssg serve` starts local server
   - [ ] File watcher triggers incremental build
   - [ ] WebSocket for live reload
   - [ ] Browser auto-refreshes on change

5. **Performance**
   - [ ] Parallel content processing
   - [ ] Build 1000 pages in under 10 seconds
   - [ ] Incremental build for single file under 100ms

#### Demo Script
```bash
# Full build
ssg build
# Built 150 pages in 1.2s

# Modify one file
echo "Updated content" >> content/posts/hello.md

# Incremental build
ssg build --incremental
# Rebuilt 1 page in 45ms

# Modify layout (affects many pages)
echo "<!-- updated -->" >> layouts/default.html
ssg build --incremental
# Rebuilt 150 pages in 1.1s (all pages use this layout)

# Development server
ssg serve --port 3000
# Server running at http://localhost:3000
# Watching for changes...

# Edit file in another terminal
echo "New paragraph" >> content/posts/hello.md
# [Browser auto-reloads]
```

#### Test Requirements
- [ ] Test full build output correctness
- [ ] Test incremental build detects changes
- [ ] Test dependency invalidation
- [ ] Test live reload WebSocket
- [ ] Benchmark: 1000 pages full build
- [ ] Benchmark: single file incremental build
