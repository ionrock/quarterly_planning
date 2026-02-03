---
id: "test-023"
title: "API Documentation Generator"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Build a tool that generates API documentation from OpenAPI/Swagger specs. Produces static HTML with interactive examples, search, and dark mode.

## Constraints

- Support OpenAPI 3.0 and 3.1
- Generated site works offline

## Implementation Notes

- Written in TypeScript
- Vite for static generation
- Fuse.js for search

## Review Notes

(none yet)

## Tickets

### Ticket 1: Spec Parsing

**Summary:** Parse and validate OpenAPI specs.

**Definition of Done:** Valid specs parsed, invalid rejected.

#### Steps

1. **Create TypeScript project**
   - Run `npm init -y && npm install -D typescript`
   - Create tsconfig.json
   - Verify: project compiles

2. **Install OpenAPI parser**
   - Run `npm install @apidevtools/swagger-parser`
   - Verify: package installed

3. **Create spec loader**
   - Create src/loader.ts
   - Accept file path or URL
   - Verify: specs load

4. **Implement spec validation**
   - Use SwaggerParser.validate()
   - Return validation errors
   - Verify: invalid specs rejected

5. **Detect OpenAPI version**
   - Check openapi field (3.0.x, 3.1.x)
   - Handle version differences
   - Verify: versions detected

6. **Resolve $ref references**
   - Use SwaggerParser.dereference()
   - Handle circular refs
   - Verify: refs resolved

7. **Extract API metadata**
   - Extract info.title, info.version, info.description
   - Extract servers list
   - Verify: metadata extracted

8. **Extract paths and operations**
   - Iterate paths object
   - Extract method, summary, description, parameters
   - Verify: operations extracted

9. **Extract schemas**
   - Get components.schemas
   - Build schema reference map
   - Verify: schemas extracted

10. **Create normalized data model**
    - Define TypeScript interfaces for extracted data
    - Verify: model compiles

### Ticket 2: HTML Generation

**Summary:** Generate static documentation.

**Definition of Done:** Documentation renders in browser.

#### Steps

1. **Set up Vite project**
   - Run `npm create vite@latest -- --template vanilla-ts`
   - Configure for library mode
   - Verify: Vite builds

2. **Create HTML template structure**
   - Create index.html with layout
   - Include navigation sidebar
   - Include content area
   - Verify: template renders

3. **Create CSS styles**
   - Create base styles for documentation
   - Add responsive design
   - Verify: styles apply

4. **Implement dark mode**
   - Add dark mode CSS variables
   - Toggle based on system preference
   - Add manual toggle button
   - Verify: dark mode works

5. **Generate navigation sidebar**
   - List all endpoints grouped by tag
   - Link to endpoint sections
   - Verify: navigation works

6. **Generate endpoint documentation**
   - Create section per endpoint
   - Show method badge, path, description
   - Verify: endpoints documented

7. **Generate parameter tables**
   - List path, query, header parameters
   - Show name, type, required, description
   - Verify: parameters documented

8. **Generate request body docs**
   - Show schema for request body
   - Include example if available
   - Verify: request body documented

9. **Generate response docs**
   - Document each response status
   - Show response schema
   - Include examples
   - Verify: responses documented

10. **Generate schema documentation**
    - Create collapsible schema views
    - Show properties with types
    - Verify: schemas documented

11. **Build static output**
    - Configure Vite to build static HTML
    - Inline CSS and JS for offline use
    - Verify: single HTML file works

### Ticket 3: Interactive Features

**Summary:** Add search and try-it-out.

**Definition of Done:** Users can search and test API.

#### Steps

1. **Install Fuse.js**
   - Run `npm install fuse.js`
   - Verify: package installed

2. **Build search index**
   - Index endpoint paths, summaries, descriptions
   - Index parameter names
   - Verify: index built

3. **Create search input**
   - Add search box to header
   - Verify: input renders

4. **Implement search functionality**
   - Use Fuse.js for fuzzy search
   - Display results in dropdown
   - Verify: search works

5. **Add keyboard navigation**
   - Arrow keys to navigate results
   - Enter to select
   - Escape to close
   - Verify: keyboard works

6. **Navigate to search result**
   - Scroll to and highlight selected endpoint
   - Verify: navigation works

7. **Create try-it-out form**
   - Generate form inputs for parameters
   - Add request body textarea
   - Verify: form renders

8. **Build request from form**
   - Construct URL with path and query params
   - Add headers
   - Add body for POST/PUT
   - Verify: request built correctly

9. **Send request**
   - Use fetch API
   - Handle CORS (document limitations)
   - Verify: requests sent

10. **Display response**
    - Show status code
    - Show response headers
    - Pretty-print JSON body
    - Verify: response displayed

11. **Add copy buttons**
    - Copy curl command
    - Copy response
    - Verify: copy works

12. **Save form state**
    - Persist form values in localStorage
    - Restore on page load
    - Verify: state persisted

13. **Test offline functionality**
    - Disable network
    - Verify docs still work
    - Verify: offline works
