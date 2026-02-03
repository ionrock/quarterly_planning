---
id: "test-020"
title: "Code Formatter"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a code formatter for a custom configuration language. Handles indentation, line wrapping, and comment alignment. Preserves semantic meaning while improving readability.

## Constraints

- Format 10,000 line file under 1 second
- Idempotent (formatting twice gives same result)

## Implementation Notes

- Written in Rust
- Parse-print architecture
- Configurable style options

## Review Notes

(none yet)

## Tickets

### Ticket 1: Parser

**Summary:** Parse configuration language to AST.

**Definition of Done:** All valid configs parse correctly.

#### Steps

1. **Create Rust project**
   - Run `cargo new configfmt --bin`
   - Add lib.rs for library code
   - Verify: project builds

2. **Define token types**
   - Create src/lexer.rs
   - Define Token enum: Identifier, String, Number, Equals, Comma, etc.
   - Verify: tokens compile

3. **Implement lexer**
   - Create Lexer struct with source and position
   - Implement next_token() -> Option<Token>
   - Track line and column for errors
   - Verify: lexer tokenizes sample input

4. **Handle string literals**
   - Support double-quoted strings
   - Handle escape sequences
   - Verify: strings tokenize correctly

5. **Handle comments**
   - Support # line comments
   - Preserve comments in token stream
   - Verify: comments captured

6. **Define AST node types**
   - Create src/ast.rs
   - Define: Document, Section, KeyValue, Value, Comment
   - Verify: AST types compile

7. **Create parser struct**
   - Create src/parser.rs
   - Accept token iterator
   - Track current and peek tokens
   - Verify: parser initializes

8. **Implement section parsing**
   - Parse [section.name] headers
   - Collect key-value pairs until next section
   - Verify: sections parsed

9. **Implement key-value parsing**
   - Parse key = value lines
   - Support nested keys (a.b.c)
   - Verify: key-values parsed

10. **Implement value parsing**
    - Parse strings, numbers, booleans
    - Parse arrays [a, b, c]
    - Parse inline tables {a = 1}
    - Verify: all value types parsed

11. **Handle multiline values**
    - Support multiline strings
    - Support multiline arrays
    - Verify: multiline values parsed

12. **Implement error recovery**
    - Report parse errors with location
    - Continue parsing after error
    - Verify: multiple errors reported

### Ticket 2: Formatter

**Summary:** Format AST back to source.

**Definition of Done:** Output is well-formatted.

#### Steps

1. **Define FormatConfig struct**
   - Include: indent_size, max_line_width, align_values
   - Add builder pattern
   - Verify: config works

2. **Create formatter struct**
   - Create src/formatter.rs
   - Accept AST and FormatConfig
   - Verify: formatter initializes

3. **Implement document formatting**
   - Format sections in order
   - Add blank lines between sections
   - Verify: document structure maintained

4. **Implement section header formatting**
   - Format [section] with consistent spacing
   - Verify: headers formatted

5. **Implement key-value formatting**
   - Format key = value with spaces around equals
   - Verify: key-values formatted

6. **Implement value alignment**
   - When enabled, align equals signs in section
   - Calculate max key length
   - Verify: values aligned

7. **Implement string formatting**
   - Choose single or double quotes based on content
   - Escape necessary characters
   - Verify: strings formatted correctly

8. **Implement array formatting**
   - Single line if fits within max_line_width
   - Multi-line with indentation if too long
   - Trailing comma on multi-line
   - Verify: arrays formatted

9. **Implement comment preservation**
   - Keep comments attached to following line
   - Preserve inline comments
   - Verify: comments preserved

10. **Implement line wrapping**
    - Break long lines at appropriate points
    - Indent continuation lines
    - Verify: lines wrap correctly

11. **Test idempotence**
    - Format output should format to itself
    - Write test that formats twice and compares
    - Verify: formatting is idempotent

### Ticket 3: CLI Tool

**Summary:** Create command-line interface.

**Definition of Done:** Tool formats files in place or to stdout.

#### Steps

1. **Add clap dependency**
   - Add clap to Cargo.toml
   - Verify: dependency resolves

2. **Define CLI arguments**
   - Create src/main.rs with clap derive
   - Accept file paths as arguments
   - Add --check, --write, --config options
   - Verify: args parsed

3. **Implement single file formatting**
   - Read file, parse, format, output
   - Verify: single file works

4. **Implement stdout output**
   - Default: print formatted output to stdout
   - Verify: output to stdout

5. **Implement in-place writing**
   - With --write flag, overwrite original file
   - Verify: file updated in place

6. **Implement check mode**
   - With --check, exit 1 if file would change
   - Don't modify file
   - Verify: check mode works

7. **Implement multiple file handling**
   - Accept multiple file paths
   - Process each in order
   - Verify: multiple files work

8. **Implement glob pattern support**
   - Accept patterns like "**/*.config"
   - Use glob crate
   - Verify: patterns expand

9. **Add config file support**
   - Look for .configfmt in project root
   - Override defaults with config values
   - Verify: config file loaded

10. **Implement diff output**
    - With --diff, show what would change
    - Use similar crate for diff
    - Verify: diff shown

11. **Add stdin support**
    - Accept input from stdin with - argument
    - Output to stdout
    - Verify: stdin works

12. **Benchmark performance**
    - Create 10,000 line test file
    - Measure formatting time
    - Verify: under 1 second
