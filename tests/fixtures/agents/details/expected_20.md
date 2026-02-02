---
id: "test-020"
title: "Code Formatter"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create an opinionated code formatter for JavaScript/TypeScript. Parses code into AST, applies formatting rules, and outputs formatted code. Similar to Prettier but with different style choices. Written in Rust for speed.

## Constraints

- Format 10,000 lines per second
- Support ES2024 and TypeScript 5.x syntax

## Implementation Notes

### Technology Stack
- **Language:** Rust 1.75+
- **Parser:** tree-sitter-javascript, tree-sitter-typescript
- **Algorithm:** Wadler-Lindig pretty printing
- **CLI:** clap v4

### Wadler-Lindig Document IR
```rust
#[derive(Clone, Debug)]
pub enum Doc {
    Nil,
    Text(String),
    Line,                    // Line break (or space if flattened)
    HardLine,               // Always line break
    Concat(Vec<Doc>),
    Nest(i32, Box<Doc>),    // Indent nested content
    Group(Box<Doc>),        // Try to fit on one line
    IfBreak(Box<Doc>, Box<Doc>),  // Different content if broken
}

impl Doc {
    pub fn text(s: impl Into<String>) -> Doc {
        Doc::Text(s.into())
    }

    pub fn concat(docs: Vec<Doc>) -> Doc {
        Doc::Concat(docs)
    }

    pub fn group(doc: Doc) -> Doc {
        Doc::Group(Box::new(doc))
    }

    pub fn nest(indent: i32, doc: Doc) -> Doc {
        Doc::Nest(indent, Box::new(doc))
    }

    pub fn join(sep: Doc, docs: Vec<Doc>) -> Doc {
        let mut result = Vec::new();
        for (i, doc) in docs.into_iter().enumerate() {
            if i > 0 {
                result.push(sep.clone());
            }
            result.push(doc);
        }
        Doc::concat(result)
    }
}
```

### Pretty Printer
```rust
pub struct Printer {
    width: usize,
    indent_width: usize,
}

impl Printer {
    pub fn print(&self, doc: &Doc) -> String {
        let mut output = String::new();
        let mut stack = vec![(0, Mode::Break, doc)];

        while let Some((indent, mode, doc)) = stack.pop() {
            match doc {
                Doc::Nil => {}
                Doc::Text(s) => output.push_str(s),
                Doc::Line => match mode {
                    Mode::Flat => output.push(' '),
                    Mode::Break => {
                        output.push('\n');
                        output.push_str(&" ".repeat(indent));
                    }
                },
                Doc::HardLine => {
                    output.push('\n');
                    output.push_str(&" ".repeat(indent));
                }
                Doc::Concat(docs) => {
                    for d in docs.iter().rev() {
                        stack.push((indent, mode, d));
                    }
                }
                Doc::Nest(i, inner) => {
                    stack.push((indent + (*i as usize), mode, inner));
                }
                Doc::Group(inner) => {
                    let flat_len = self.flat_len(inner);
                    let fits = self.fits(self.width - self.current_col(&output), flat_len);
                    let new_mode = if fits { Mode::Flat } else { Mode::Break };
                    stack.push((indent, new_mode, inner));
                }
                Doc::IfBreak(broken, flat) => {
                    let doc = match mode {
                        Mode::Break => broken,
                        Mode::Flat => flat,
                    };
                    stack.push((indent, mode, doc));
                }
            }
        }
        output
    }
}
```

### AST to Doc Conversion
```rust
pub fn format_node(node: &Node, source: &str) -> Doc {
    match node.kind() {
        "program" => format_program(node, source),
        "function_declaration" => format_function(node, source),
        "arrow_function" => format_arrow_function(node, source),
        "call_expression" => format_call(node, source),
        "object" => format_object(node, source),
        "array" => format_array(node, source),
        "binary_expression" => format_binary(node, source),
        "if_statement" => format_if(node, source),
        _ => Doc::text(node.utf8_text(source.as_bytes()).unwrap()),
    }
}

fn format_object(node: &Node, source: &str) -> Doc {
    let pairs: Vec<Doc> = node
        .children_by_field_name("pair", &mut node.walk())
        .map(|p| format_node(&p, source))
        .collect();

    if pairs.is_empty() {
        return Doc::text("{}");
    }

    Doc::group(Doc::concat(vec![
        Doc::text("{"),
        Doc::nest(2, Doc::concat(vec![
            Doc::Line,
            Doc::join(
                Doc::concat(vec![Doc::text(","), Doc::Line]),
                pairs,
            ),
        ])),
        Doc::Line,
        Doc::text("}"),
    ]))
}

fn format_function(node: &Node, source: &str) -> Doc {
    let name = node.child_by_field_name("name").map(|n| n.utf8_text(source.as_bytes()).unwrap());
    let params = node.child_by_field_name("parameters");
    let body = node.child_by_field_name("body");

    Doc::concat(vec![
        Doc::text("function"),
        name.map(|n| Doc::concat(vec![Doc::text(" "), Doc::text(n)])).unwrap_or(Doc::Nil),
        params.map(|p| format_params(&p, source)).unwrap_or(Doc::text("()")),
        Doc::text(" "),
        body.map(|b| format_node(&b, source)).unwrap_or(Doc::text("{}")),
    ])
}
```

### CLI Interface
```rust
#[derive(Parser)]
#[command(name = "fmt", about = "Code formatter")]
struct Cli {
    /// Files to format (or stdin if none)
    files: Vec<PathBuf>,

    /// Write formatted output back to files
    #[arg(short, long)]
    write: bool,

    /// Check if files are formatted (exit 1 if not)
    #[arg(long)]
    check: bool,

    /// Line width (default: 80)
    #[arg(long, default_value = "80")]
    width: usize,

    /// Indent width (default: 2)
    #[arg(long, default_value = "2")]
    indent: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let formatter = Formatter::new(cli.width, cli.indent);

    if cli.files.is_empty() {
        // Format stdin
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        let output = formatter.format(&input)?;
        print!("{}", output);
    } else {
        for path in &cli.files {
            let input = fs::read_to_string(path)?;
            let output = formatter.format(&input)?;

            if cli.check {
                if input != output {
                    eprintln!("{}: not formatted", path.display());
                    std::process::exit(1);
                }
            } else if cli.write {
                fs::write(path, &output)?;
            } else {
                print!("{}", output);
            }
        }
    }
    Ok(())
}
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Parser Integration

**Summary:** Integrate tree-sitter for JS/TS parsing.

**Definition of Done:** Code is parsed into AST correctly.

### Ticket 2: Formatting Engine

**Summary:** Implement pretty printing algorithm.

**Definition of Done:** Code is formatted according to rules.

### Ticket 3: CLI Interface

**Summary:** Build CLI for file and stdin formatting.

**Definition of Done:** Files can be formatted in place or to stdout.
