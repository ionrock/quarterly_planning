---
id: "test-016"
title: "Markdown Parser"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a CommonMark-compliant Markdown parser that converts Markdown to HTML. Support extensions like tables, footnotes, and syntax highlighting. Written in TypeScript with streaming support for large documents.

## Constraints

- Pass CommonMark spec tests
- Parse 1MB documents in under 100ms

## Implementation Notes

### Technology Stack
- **Language:** TypeScript 5.x
- **Testing:** Vitest with CommonMark spec suite
- **Build:** esbuild, outputs ESM and CJS

### AST Node Types
```typescript
type Node =
  | Document
  | Paragraph
  | Heading
  | CodeBlock
  | BlockQuote
  | List
  | ListItem
  | ThematicBreak
  | HtmlBlock
  | Text
  | Emphasis
  | Strong
  | Code
  | Link
  | Image
  | HardBreak
  | SoftBreak
  | HtmlInline;

interface BaseNode {
  type: string;
  position?: Position;
}

interface Position {
  start: { line: number; column: number; offset: number };
  end: { line: number; column: number; offset: number };
}

interface Parent extends BaseNode {
  children: Node[];
}

interface Heading extends Parent {
  type: 'heading';
  depth: 1 | 2 | 3 | 4 | 5 | 6;
}

interface CodeBlock extends BaseNode {
  type: 'code';
  lang?: string;
  meta?: string;
  value: string;
}

interface Link extends Parent {
  type: 'link';
  url: string;
  title?: string;
}
```

### Block Parser (Two-Phase)
```typescript
class BlockParser {
  private lines: string[];
  private pos: number = 0;

  parse(input: string): Document {
    this.lines = input.split(/\r?\n/);
    this.pos = 0;

    const doc: Document = { type: 'document', children: [] };

    while (this.pos < this.lines.length) {
      const block = this.parseBlock();
      if (block) {
        doc.children.push(block);
      }
    }

    return doc;
  }

  private parseBlock(): Node | null {
    const line = this.lines[this.pos];

    // ATX Heading: # Heading
    if (/^#{1,6}(?:\s|$)/.test(line)) {
      return this.parseAtxHeading();
    }

    // Fenced code block: ```
    if (/^`{3,}|^~{3,}/.test(line)) {
      return this.parseFencedCode();
    }

    // Block quote: >
    if (/^>/.test(line)) {
      return this.parseBlockQuote();
    }

    // Thematic break: ---, ***, ___
    if (/^(?:[-*_]\s*){3,}$/.test(line.trim())) {
      this.pos++;
      return { type: 'thematicBreak' };
    }

    // List item: -, *, +, or 1.
    if (/^[-*+]\s|^\d+\.\s/.test(line)) {
      return this.parseList();
    }

    // Paragraph (default)
    return this.parseParagraph();
  }
}
```

### Inline Parser
```typescript
class InlineParser {
  private text: string;
  private pos: number = 0;

  parse(text: string): Node[] {
    this.text = text;
    this.pos = 0;
    const nodes: Node[] = [];

    while (this.pos < this.text.length) {
      const node = this.parseInline();
      if (node) nodes.push(node);
    }

    return this.mergeTextNodes(nodes);
  }

  private parseInline(): Node | null {
    const char = this.text[this.pos];

    // Emphasis: *text* or _text_
    if (char === '*' || char === '_') {
      return this.parseEmphasis(char);
    }

    // Code span: `code`
    if (char === '`') {
      return this.parseCodeSpan();
    }

    // Link: [text](url)
    if (char === '[') {
      return this.parseLink();
    }

    // Autolink: <url>
    if (char === '<') {
      return this.parseAutolink();
    }

    // Escape: \*
    if (char === '\\' && this.pos + 1 < this.text.length) {
      this.pos += 2;
      return { type: 'text', value: this.text[this.pos - 1] };
    }

    // Regular text
    return this.parseText();
  }
}
```

### HTML Renderer
```typescript
class HtmlRenderer {
  render(node: Node): string {
    switch (node.type) {
      case 'document':
        return node.children.map(c => this.render(c)).join('\n');

      case 'heading':
        const tag = `h${node.depth}`;
        return `<${tag}>${this.renderChildren(node)}</${tag}>`;

      case 'paragraph':
        return `<p>${this.renderChildren(node)}</p>`;

      case 'code':
        const lang = node.lang ? ` class="language-${this.escape(node.lang)}"` : '';
        return `<pre><code${lang}>${this.escape(node.value)}</code></pre>`;

      case 'emphasis':
        return `<em>${this.renderChildren(node)}</em>`;

      case 'strong':
        return `<strong>${this.renderChildren(node)}</strong>`;

      case 'link':
        const title = node.title ? ` title="${this.escape(node.title)}"` : '';
        return `<a href="${this.escapeUrl(node.url)}"${title}>${this.renderChildren(node)}</a>`;

      case 'text':
        return this.escape(node.value);

      // ... other node types
    }
  }

  private escape(str: string): string {
    return str
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;');
  }

  private escapeUrl(url: string): string {
    // Prevent XSS via javascript: URLs
    if (/^javascript:/i.test(url)) {
      return '';
    }
    return encodeURI(url);
  }
}
```

### Main API
```typescript
export function parse(markdown: string): Document {
  const blockParser = new BlockParser();
  const doc = blockParser.parse(markdown);
  // Second pass: parse inline content
  processInlines(doc);
  return doc;
}

export function render(markdown: string): string {
  const doc = parse(markdown);
  const renderer = new HtmlRenderer();
  return renderer.render(doc);
}
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Block Parser

**Summary:** Parse block-level elements (paragraphs, headers, lists, code blocks).

**Definition of Done:** All CommonMark block elements parse correctly.

### Ticket 2: Inline Parser

**Summary:** Parse inline elements (emphasis, links, code spans).

**Definition of Done:** All CommonMark inline elements parse correctly.

### Ticket 3: HTML Renderer

**Summary:** Render AST to HTML with XSS protection.

**Definition of Done:** Output is valid, safe HTML.
