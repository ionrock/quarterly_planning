---
id: "test-018"
title: "PDF Generation Service"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a PDF generation service from HTML templates. Supports dynamic data binding, headers/footers, and page numbering. Handles high-volume report generation.

## Constraints

- Generate 100 pages in under 5 seconds
- Pixel-perfect output matching design

## Implementation Notes

- Playwright for rendering
- Handlebars for templates
- Redis queue for batch jobs

## Review Notes

(none yet)

## Tickets

### Ticket 1: Template Engine

**Summary:** HTML templates with data binding.

**Definition of Done:** Templates render with dynamic data.

#### Acceptance Criteria

1. **Template Syntax**
   - [ ] Handlebars templating ({{variable}})
   - [ ] Loops: {{#each items}}
   - [ ] Conditionals: {{#if condition}}
   - [ ] Partials: {{> partial_name}}
   - [ ] Helpers: {{formatDate date "YYYY-MM-DD"}}

2. **Built-in Helpers**
   - [ ] formatDate: date formatting
   - [ ] formatNumber: number with locale
   - [ ] formatCurrency: currency display
   - [ ] uppercase, lowercase, capitalize

3. **Template Storage**
   - [ ] Templates stored in filesystem or S3
   - [ ] Template versioning
   - [ ] Hot reload in development
   - [ ] Template validation on upload

4. **Data Binding**
   - [ ] JSON data input
   - [ ] Nested object access: {{user.address.city}}
   - [ ] Array iteration with index
   - [ ] Missing data shows empty (configurable)

5. **CSS Support**
   - [ ] Inline styles
   - [ ] External CSS files
   - [ ] CSS @page rules for print
   - [ ] @media print queries

#### Demo Script
```javascript
import { TemplateEngine } from '@company/pdf-service';

const engine = new TemplateEngine();

// Register template
await engine.registerTemplate('invoice', `
<!DOCTYPE html>
<html>
<head>
  <style>
    @page { size: A4; margin: 2cm; }
    .total { font-weight: bold; }
  </style>
</head>
<body>
  <h1>Invoice #{{invoiceNumber}}</h1>
  <p>Date: {{formatDate date "MMMM D, YYYY"}}</p>

  <table>
    <tr><th>Item</th><th>Qty</th><th>Price</th></tr>
    {{#each items}}
    <tr>
      <td>{{name}}</td>
      <td>{{quantity}}</td>
      <td>{{formatCurrency price "USD"}}</td>
    </tr>
    {{/each}}
  </table>

  <p class="total">Total: {{formatCurrency total "USD"}}</p>
</body>
</html>
`);

// Render template
const html = await engine.render('invoice', {
  invoiceNumber: 'INV-001',
  date: new Date(),
  items: [
    { name: 'Widget', quantity: 2, price: 29.99 },
    { name: 'Gadget', quantity: 1, price: 49.99 }
  ],
  total: 109.97
});
```

#### Test Requirements
- [ ] Test variable interpolation
- [ ] Test loops and conditionals
- [ ] Test all built-in helpers
- [ ] Test partial includes
- [ ] Test missing data handling
- [ ] Test CSS rendering

### Ticket 2: PDF Rendering

**Summary:** Convert HTML to PDF with Playwright.

**Definition of Done:** PDFs match visual design exactly.

#### Acceptance Criteria

1. **Page Setup**
   - [ ] Paper size: A4, Letter, Legal, custom
   - [ ] Orientation: portrait, landscape
   - [ ] Margins: configurable per side
   - [ ] Background graphics (colors, images)

2. **Headers and Footers**
   - [ ] Header HTML template
   - [ ] Footer HTML template
   - [ ] Page number: {{pageNumber}} of {{totalPages}}
   - [ ] Different first page header/footer

3. **Page Breaks**
   - [ ] CSS: page-break-before, page-break-after
   - [ ] Avoid breaking inside elements
   - [ ] Orphan/widow control

4. **Fonts**
   - [ ] System fonts
   - [ ] Custom fonts via @font-face
   - [ ] Font subsetting for smaller PDFs
   - [ ] Emoji support

5. **Output Quality**
   - [ ] Vector text (selectable, searchable)
   - [ ] High-resolution images (300 DPI)
   - [ ] Accurate color reproduction
   - [ ] PDF/A compliance (optional)

#### Demo Script
```javascript
import { PdfRenderer } from '@company/pdf-service';

const renderer = new PdfRenderer();

// Simple render
const pdf = await renderer.render(html, {
  format: 'A4',
  margin: { top: '2cm', bottom: '2cm', left: '1.5cm', right: '1.5cm' },
  printBackground: true
});

// With header and footer
const pdf = await renderer.render(html, {
  format: 'Letter',
  headerTemplate: `
    <div style="font-size: 10px; width: 100%; text-align: center;">
      Company Name - Confidential
    </div>
  `,
  footerTemplate: `
    <div style="font-size: 10px; width: 100%; text-align: center;">
      Page <span class="pageNumber"></span> of <span class="totalPages"></span>
    </div>
  `,
  displayHeaderFooter: true
});

// Save to file
await fs.writeFile('invoice.pdf', pdf);

// Or return as base64
const base64 = pdf.toString('base64');
```

#### Test Requirements
- [ ] Test all paper sizes
- [ ] Test header/footer rendering
- [ ] Test page number accuracy
- [ ] Test page break handling
- [ ] Test custom fonts
- [ ] Visual comparison with reference PDFs
- [ ] Benchmark: 100 pages in under 5 seconds

### Ticket 3: Batch Processing

**Summary:** Queue for bulk PDF generation.

**Definition of Done:** Batch jobs complete reliably.

#### Acceptance Criteria

1. **Batch Job Submission**
   - [ ] Submit batch with multiple items
   - [ ] Each item: template, data, output path
   - [ ] Maximum batch size: 1000 items
   - [ ] Returns batch ID

2. **Processing**
   - [ ] Items processed in parallel (configurable concurrency)
   - [ ] Progress tracking per item
   - [ ] Continue on item failure
   - [ ] Resource pooling (browser instances)

3. **Output Options**
   - [ ] Individual PDFs per item
   - [ ] Merged PDF (all items combined)
   - [ ] ZIP archive of all PDFs
   - [ ] Upload to S3

4. **Status and Results**
   - [ ] GET /batches/{id} returns status
   - [ ] Items: total, completed, failed
   - [ ] Failed items with error details
   - [ ] Webhook on completion

5. **Performance**
   - [ ] Warm browser pool for fast startup
   - [ ] Reuse browser contexts
   - [ ] Memory management for large batches
   - [ ] Timeout handling per item

#### Demo Script
```bash
# Submit batch job
curl -X POST http://localhost:8000/api/batches \
  -H "Content-Type: application/json" \
  -d '{
    "template": "invoice",
    "items": [
      {"data": {"invoiceNumber": "INV-001", ...}, "output": "invoices/001.pdf"},
      {"data": {"invoiceNumber": "INV-002", ...}, "output": "invoices/002.pdf"},
      ...
    ],
    "options": {
      "format": "A4",
      "concurrency": 10,
      "outputFormat": "zip",
      "s3Bucket": "reports"
    },
    "webhook": "https://app.example.com/webhooks/pdf"
  }'
# Response: {"batch_id": "batch-abc123", "items_count": 100}

# Check status
curl http://localhost:8000/api/batches/batch-abc123
# {
#   "batch_id": "batch-abc123",
#   "status": "processing",
#   "progress": {
#     "total": 100,
#     "completed": 45,
#     "failed": 2
#   },
#   "started_at": "2024-01-15T10:00:00Z",
#   "eta_seconds": 30
# }

# Get results when complete
curl http://localhost:8000/api/batches/batch-abc123
# {
#   "status": "completed",
#   "output_url": "s3://reports/batches/abc123/invoices.zip",
#   "failed_items": [
#     {"index": 23, "error": "Template rendering failed: missing field 'total'"}
#   ]
# }
```

#### Test Requirements
- [ ] Test batch submission
- [ ] Test parallel processing
- [ ] Test individual vs merged output
- [ ] Test failure handling (continue on error)
- [ ] Test webhook delivery
- [ ] Load test: 1000 PDFs in batch
- [ ] Memory usage under load
