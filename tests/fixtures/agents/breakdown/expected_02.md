---
id: "test-002"
title: "CLI Log Analyzer"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a command-line tool that analyzes log files and generates summary reports. The tool should parse various log formats, extract key metrics, and output reports in multiple formats (JSON, CSV, text).

## Constraints

- Must handle log files up to 1GB
- Support at least 3 common log formats

## Implementation Notes

- Use Python with argparse for CLI
- Stream processing for large files
- Pluggable parser architecture

## Review Notes

(none yet)

## Tickets

### Ticket 1: CLI Framework

**Summary:** Set up the command-line interface structure.

**Definition of Done:** CLI accepts input file and output format arguments.

#### Steps

1. **Create project structure**
   - Create log_analyzer/ package directory with __init__.py
   - Create log_analyzer/cli.py for CLI entry point
   - Verify: `python -c "import log_analyzer"` succeeds

2. **Set up pyproject.toml**
   - Create pyproject.toml with project metadata
   - Add dependencies: none for core (stdlib only initially)
   - Verify: `pip install -e .` succeeds

3. **Create main CLI parser**
   - Import argparse in cli.py
   - Create ArgumentParser with description
   - Verify: `python -m log_analyzer --help` shows help text

4. **Add input file argument**
   - Add positional argument 'input_file' of type Path
   - Add validation that file exists
   - Verify: `python -m log_analyzer nonexistent.log` shows error

5. **Add output format argument**
   - Add --format/-f argument with choices ['json', 'csv', 'text']
   - Set default to 'text'
   - Verify: `python -m log_analyzer test.log -f json` parses format

6. **Add output file argument**
   - Add --output/-o argument for output file path
   - Default to stdout if not specified
   - Verify: argument parsed correctly

7. **Add log format argument**
   - Add --log-format argument with choices ['apache', 'nginx', 'syslog', 'auto']
   - Set default to 'auto' for auto-detection
   - Verify: `python -m log_analyzer test.log --log-format apache` works

8. **Create entry point script**
   - Add [project.scripts] section to pyproject.toml
   - Map 'loganalyzer' to 'log_analyzer.cli:main'
   - Verify: `loganalyzer --help` works after reinstall

### Ticket 2: Log Parsers

**Summary:** Implement parsers for different log formats.

**Definition of Done:** Can parse Apache, nginx, and syslog formats.

#### Steps

1. **Create parser base class**
   - Create log_analyzer/parsers/__init__.py
   - Create log_analyzer/parsers/base.py with abstract Parser class
   - Define abstract method parse_line(line: str) -> Optional[LogEntry]
   - Verify: class can be imported

2. **Define LogEntry dataclass**
   - Create log_analyzer/models.py
   - Define LogEntry with timestamp, level, message, source, extra fields
   - Verify: LogEntry can be instantiated

3. **Implement Apache log parser**
   - Create log_analyzer/parsers/apache.py
   - Parse Combined Log Format: %h %l %u %t "%r" %>s %b "%{Referer}i" "%{User-agent}i"
   - Extract: ip, timestamp, method, path, status, size, referer, user_agent
   - Verify: parses sample Apache log line correctly

4. **Implement nginx log parser**
   - Create log_analyzer/parsers/nginx.py
   - Parse default nginx format (similar to Apache combined)
   - Handle nginx-specific fields like request_time
   - Verify: parses sample nginx log line correctly

5. **Implement syslog parser**
   - Create log_analyzer/parsers/syslog.py
   - Parse RFC 3164 format: <PRI>TIMESTAMP HOSTNAME TAG: MESSAGE
   - Extract: priority, timestamp, hostname, tag, message
   - Verify: parses sample syslog line correctly

6. **Create parser registry**
   - Create get_parser(format: str) -> Parser function in parsers/__init__.py
   - Map format names to parser classes
   - Verify: get_parser('apache') returns ApacheParser instance

7. **Implement format auto-detection**
   - Create detect_format(sample_lines: List[str]) -> str function
   - Try each parser, return first that successfully parses >80% of lines
   - Verify: correctly identifies Apache, nginx, and syslog files

8. **Add streaming file reader**
   - Create log_analyzer/reader.py with stream_lines(path, chunk_size) generator
   - Use mmap for efficient reading of large files
   - Verify: can iterate over 100MB file without loading into memory

### Ticket 3: Report Generation

**Summary:** Generate reports in multiple formats.

**Definition of Done:** Outputs valid JSON, CSV, and text reports.

#### Steps

1. **Create report data structure**
   - Create log_analyzer/report.py
   - Define AnalysisReport dataclass with: total_lines, error_count, warning_count, time_range, top_ips, top_paths, status_distribution
   - Verify: dataclass instantiates correctly

2. **Create analyzer class**
   - Create log_analyzer/analyzer.py with Analyzer class
   - Accept parser instance in constructor
   - Verify: Analyzer can be instantiated

3. **Implement line-by-line analysis**
   - Add analyze_line(entry: LogEntry) method to update internal counters
   - Track: line count, error count, timestamps, IP frequency, path frequency
   - Verify: counters update correctly for sample entries

4. **Implement report generation**
   - Add generate_report() -> AnalysisReport method
   - Compute final statistics from accumulated data
   - Verify: returns populated AnalysisReport

5. **Create formatter base class**
   - Create log_analyzer/formatters/__init__.py
   - Create log_analyzer/formatters/base.py with abstract Formatter class
   - Define abstract method format(report: AnalysisReport) -> str
   - Verify: class can be imported

6. **Implement JSON formatter**
   - Create log_analyzer/formatters/json_fmt.py
   - Use json.dumps with indent=2 for pretty printing
   - Verify: `json.loads(output)` succeeds

7. **Implement CSV formatter**
   - Create log_analyzer/formatters/csv_fmt.py
   - Output summary stats as rows: metric,value
   - Verify: output is valid CSV parseable by csv.reader

8. **Implement text formatter**
   - Create log_analyzer/formatters/text_fmt.py
   - Create human-readable report with headers and tables
   - Verify: output is readable in terminal

9. **Create formatter registry**
   - Create get_formatter(format: str) -> Formatter in formatters/__init__.py
   - Map format names to formatter classes
   - Verify: get_formatter('json') returns JsonFormatter

10. **Wire up CLI to analysis pipeline**
    - In cli.main(): load file, detect/get parser, create analyzer
    - Stream lines through parser and analyzer
    - Generate report and format output
    - Verify: `loganalyzer sample.log -f json` produces valid output

11. **Handle output destination**
    - Write to --output file if specified, else stdout
    - Verify: `loganalyzer sample.log -o report.json -f json` creates file
