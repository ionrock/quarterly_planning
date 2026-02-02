---
id: "test-002"
title: "CLI Tool for Log Analysis"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a command-line tool that parses application logs, extracts errors, and generates summary reports. Written in Python, it will support multiple log formats and output reports in JSON or HTML.

## Constraints

- Must work on Linux and macOS
- Should handle log files up to 1GB

## Implementation Notes

### Technology Stack
- **Language:** Python 3.11+
- **CLI Framework:** Click (preferred over argparse for better UX)
- **HTML Templating:** Jinja2 for report generation
- **Testing:** pytest with fixtures for sample log files

### CLI Interface
```bash
loganalyze [OPTIONS] <input_file>

Options:
  -f, --format [auto|apache|nginx|json]  Log format (default: auto)
  -o, --output <file>                     Output file (default: stdout)
  -t, --type [json|html]                  Report type (default: json)
  -l, --level [error|warn|info|debug]    Minimum level to include
  --from <datetime>                       Start time filter (ISO 8601)
  --to <datetime>                         End time filter (ISO 8601)
  -v, --verbose                           Verbose output
  --version                               Show version
```

### Log Format Patterns
```python
# Apache Combined Log Format
APACHE_PATTERN = r'(?P<ip>\S+) \S+ \S+ \[(?P<time>[^\]]+)\] "(?P<request>[^"]*)" (?P<status>\d+) (?P<size>\S+)'

# Nginx Error Log
NGINX_PATTERN = r'(?P<time>\d{4}/\d{2}/\d{2} \d{2}:\d{2}:\d{2}) \[(?P<level>\w+)\] (?P<pid>\d+)#(?P<tid>\d+): (?P<message>.*)'

# JSON Log (expected fields)
JSON_FIELDS = ['timestamp', 'level', 'message', 'error', 'stack_trace']
```

### Data Structures
```python
@dataclass
class LogEntry:
    timestamp: datetime
    level: LogLevel  # enum: DEBUG, INFO, WARN, ERROR, FATAL
    message: str
    source_file: str
    line_number: int
    raw_line: str
    metadata: dict[str, Any]

@dataclass
class ErrorSummary:
    pattern: str           # Normalized error pattern
    count: int
    first_seen: datetime
    last_seen: datetime
    sample_messages: list[str]  # Up to 3 examples

@dataclass
class Report:
    input_file: str
    total_lines: int
    parsed_lines: int
    error_count: int
    warn_count: int
    time_range: tuple[datetime, datetime]
    errors_by_hour: dict[str, int]
    top_errors: list[ErrorSummary]  # Top 10 by count
```

### Streaming Algorithm for Large Files
```python
def process_large_file(path: Path, parser: LogParser) -> Report:
    """Process file in chunks to handle 1GB+ files."""
    aggregator = ReportAggregator()

    with open(path, 'r', buffering=64*1024) as f:
        for line_num, line in enumerate(f, 1):
            try:
                entry = parser.parse(line, line_num)
                aggregator.add(entry)
            except ParseError:
                aggregator.record_unparsed(line_num)

            if line_num % 100_000 == 0:
                logger.info(f"Processed {line_num:,} lines")

    return aggregator.finalize()
```

### Error Pattern Normalization
- Replace UUIDs with `<UUID>`
- Replace IP addresses with `<IP>`
- Replace timestamps with `<TIMESTAMP>`
- Replace file paths with `<PATH>`
- Replace numbers with `<N>` (except HTTP status codes)

### Output Formats
- **JSON:** Structured report with all fields, suitable for processing
- **HTML:** Styled report with charts using inline CSS (no external dependencies)

## Review Notes

(none yet)

## Tickets

### Ticket 1: CLI Framework

**Summary:** Set up Python project with argparse and basic command structure.

**Definition of Done:** CLI accepts input file and output format arguments.

### Ticket 2: Log Parsers

**Summary:** Implement parsers for Apache, nginx, and JSON log formats.

**Definition of Done:** All three formats are parsed correctly.

### Ticket 3: Report Generation

**Summary:** Create JSON and HTML report generators.

**Definition of Done:** Reports contain error summaries and are properly formatted.
