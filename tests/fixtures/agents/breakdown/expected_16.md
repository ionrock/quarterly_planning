---
id: "test-016"
title: "Markdown Parser"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a CommonMark-compliant Markdown parser that converts Markdown to HTML. Supports extensions for tables, task lists, and syntax highlighting.

## Constraints

- Full CommonMark compliance
- Parse 1MB document under 100ms

## Implementation Notes

- Written in Rust for performance
- Two-pass parsing (blocks then inlines)
- Extension system for custom syntax

## Review Notes

(none yet)

## Tickets

### Ticket 1: Block Parsing

**Summary:** Parse block-level elements.

**Definition of Done:** Paragraphs, headers, lists, and code blocks parsed.

#### Steps

1. **Create Rust library crate**
   - Run `cargo new markdown --lib`
   - Verify: `cargo build` succeeds

2. **Define AST node types**
   - Create src/ast.rs
   - Define enum for block types: Document, Paragraph, Heading, CodeBlock, List, etc.
   - Verify: enum compiles

3. **Create line scanner**
   - Create src/scanner.rs
   - Iterate lines with peek capability
   - Track line numbers for errors
   - Verify: scanner works

4. **Implement blank line detection**
   - Check for lines with only whitespace
   - Used to separate blocks
   - Verify: blank lines detected

5. **Implement ATX heading parser**
   - Detect lines starting with 1-6 #
   - Extract heading level and text
   - Verify: headings parsed

6. **Implement setext heading parser**
   - Detect = or - underlines
   - Convert previous paragraph to heading
   - Verify: setext headings parsed

7. **Implement fenced code block parser**
   - Detect opening ``` or ~~~
   - Collect lines until closing fence
   - Extract info string (language)
   - Verify: fenced code blocks parsed

8. **Implement indented code block parser**
   - Detect 4+ space indentation
   - Collect consecutive indented lines
   - Verify: indented code blocks parsed

9. **Implement unordered list parser**
   - Detect -, *, + list markers
   - Handle nested lists via indentation
   - Verify: unordered lists parsed

10. **Implement ordered list parser**
    - Detect number + period markers
    - Track list start number
    - Handle nested lists
    - Verify: ordered lists parsed

11. **Implement blockquote parser**
    - Detect > prefix
    - Recursively parse blockquote content
    - Verify: blockquotes parsed

12. **Implement paragraph parser**
    - Collect lines until blank or other block
    - Handle lazy continuation
    - Verify: paragraphs parsed

### Ticket 2: Inline Parsing

**Summary:** Parse inline elements within blocks.

**Definition of Done:** Links, emphasis, and code spans parsed.

#### Steps

1. **Define inline AST nodes**
   - Add to ast.rs: Text, Emphasis, Strong, Code, Link, Image
   - Verify: inline types compile

2. **Create inline scanner**
   - Create src/inline_scanner.rs
   - Character-level scanning
   - Track position for error reporting
   - Verify: scanner works

3. **Implement code span parser**
   - Detect backtick delimiters
   - Handle varying backtick counts
   - Verify: code spans parsed

4. **Implement emphasis parser**
   - Detect * and _ delimiters
   - Handle single (em) vs double (strong)
   - Verify: emphasis parsed

5. **Handle nested emphasis**
   - Parse ***text*** as nested strong+em
   - Use delimiter stack algorithm
   - Verify: nesting works

6. **Implement link parser**
   - Parse [text](url) syntax
   - Parse [text][ref] reference links
   - Verify: links parsed

7. **Implement image parser**
   - Parse ![alt](url) syntax
   - Verify: images parsed

8. **Implement autolink parser**
   - Detect <url> syntax
   - Detect <email> syntax
   - Verify: autolinks parsed

9. **Implement escape handling**
   - Handle backslash escapes
   - Escape special characters
   - Verify: escapes work

10. **Implement hard line break**
    - Detect trailing spaces or backslash
    - Create <br> in output
    - Verify: line breaks parsed

11. **Implement entity parsing**
    - Parse &entity; and &#code;
    - Decode to Unicode characters
    - Verify: entities decoded

12. **Integrate inline parser with blocks**
    - Parse inline content of paragraphs, headings, etc.
    - Verify: full document parses

### Ticket 3: HTML Rendering

**Summary:** Convert AST to HTML output.

**Definition of Done:** Valid HTML generated for all elements.

#### Steps

1. **Create HTML renderer trait**
   - Create src/render.rs
   - Define render(node) -> String method
   - Verify: trait compiles

2. **Implement document rendering**
   - Render children sequentially
   - Join with newlines
   - Verify: document renders

3. **Implement heading rendering**
   - Generate <h1> through <h6> tags
   - Render inline content
   - Verify: headings render

4. **Implement paragraph rendering**
   - Generate <p> tags
   - Render inline content
   - Verify: paragraphs render

5. **Implement code block rendering**
   - Generate <pre><code> tags
   - HTML-escape content
   - Add language class if present
   - Verify: code blocks render

6. **Implement list rendering**
   - Generate <ul> or <ol> tags
   - Generate <li> for items
   - Handle nested lists
   - Verify: lists render

7. **Implement blockquote rendering**
   - Generate <blockquote> tags
   - Render nested content
   - Verify: blockquotes render

8. **Implement link rendering**
   - Generate <a href="url"> tags
   - HTML-escape URL
   - Render link text
   - Verify: links render

9. **Implement image rendering**
   - Generate <img src="url" alt="text">
   - HTML-escape attributes
   - Verify: images render

10. **Implement emphasis rendering**
    - Generate <em> and <strong> tags
    - Verify: emphasis renders

11. **Implement HTML escaping**
    - Escape <, >, &, ", '
    - Apply to text content
    - Verify: special chars escaped

12. **Add pretty printing option**
    - Indent nested elements
    - Add newlines for readability
    - Verify: output readable

13. **Run CommonMark spec tests**
    - Download official test suite
    - Run all test cases
    - Verify: passes spec tests
