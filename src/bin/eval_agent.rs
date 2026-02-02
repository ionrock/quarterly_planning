//! Test harness for evaluating review agent performance.
//!
//! Runs an agent against test fixtures and uses LLM-as-judge to score outputs.
//!
//! Usage:
//!   eval-agent --agent holes --fixtures tests/fixtures/agents/holes
//!
//! Environment:
//!   ANTHROPIC_API_KEY - Required for LLM-as-judge scoring

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(name = "eval-agent", about = "Evaluate review agent performance")]
struct Args {
    /// Agent name (e.g., "holes", "details", "breakdown", "deliverables")
    #[arg(short, long)]
    agent: String,

    /// Path to fixtures directory
    #[arg(short, long, default_value = "tests/fixtures/agents")]
    fixtures: PathBuf,

    /// Agent command to run (default: claude)
    #[arg(long, default_value = "claude")]
    command: String,

    /// Additional args for agent command
    #[arg(long)]
    args: Vec<String>,

    /// Only run specific test case(s) by number (e.g., --only 1,5,10)
    #[arg(long)]
    only: Option<String>,

    /// Skip LLM judge and just run agent (for debugging)
    #[arg(long)]
    skip_judge: bool,

    /// Output detailed results to JSON file
    #[arg(long)]
    output: Option<PathBuf>,

    /// Model to use for judging (default: claude-sonnet-4-20250514)
    #[arg(long, default_value = "claude-sonnet-4-20250514")]
    judge_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestCase {
    number: u32,
    input_path: PathBuf,
    expected_path: PathBuf,
    input_content: String,
    expected_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestResult {
    test_case: u32,
    score: f64,
    reasoning: String,
    actual_output: String,
    agent_time_ms: u64,
    judge_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EvalReport {
    agent: String,
    total_cases: usize,
    completed_cases: usize,
    average_score: f64,
    min_score: f64,
    max_score: f64,
    results: Vec<TestResult>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Load agent prompt from config
    let agent_prompt = get_agent_prompt(&args.agent)?;
    println!(
        "{} {} agent evaluation",
        "Starting".green().bold(),
        args.agent
    );
    println!("Agent prompt: {}", agent_prompt.dimmed());
    println!();

    // Load test cases
    let fixtures_dir = args.fixtures.join(&args.agent);
    let test_cases = load_test_cases(&fixtures_dir, args.only.as_deref())?;
    println!("Loaded {} test cases", test_cases.len());
    println!();

    // Run evaluation
    let mut results = Vec::new();
    for (i, test_case) in test_cases.iter().enumerate() {
        println!(
            "{} Test case {} ({}/{})",
            "Running".cyan().bold(),
            test_case.number,
            i + 1,
            test_cases.len()
        );

        // Run agent
        let agent_start = Instant::now();
        let actual_output = run_agent(
            &args.command,
            &args.args,
            &agent_prompt,
            &test_case.input_content,
        )?;
        let agent_time_ms = agent_start.elapsed().as_millis() as u64;
        println!("  Agent completed in {}ms", agent_time_ms);

        // Score with LLM judge
        let (score, reasoning, judge_time_ms) = if args.skip_judge {
            (0.0, "Skipped".to_string(), 0)
        } else {
            let judge_start = Instant::now();
            let (s, r) = judge_output(
                &args.judge_model,
                &test_case.input_content,
                &test_case.expected_content,
                &actual_output,
                &agent_prompt,
            )?;
            let judge_time = judge_start.elapsed().as_millis() as u64;
            (s, r, judge_time)
        };

        let score_color = if score >= 8.0 {
            format!("{:.1}", score).green()
        } else if score >= 6.0 {
            format!("{:.1}", score).yellow()
        } else {
            format!("{:.1}", score).red()
        };
        println!("  Score: {}/10", score_color);
        println!("  Reasoning: {}", reasoning.dimmed());
        println!();

        results.push(TestResult {
            test_case: test_case.number,
            score,
            reasoning,
            actual_output,
            agent_time_ms,
            judge_time_ms,
        });
    }

    // Generate report
    let report = generate_report(&args.agent, &results);
    print_report(&report);

    // Save to file if requested
    if let Some(output_path) = args.output {
        let json = serde_json::to_string_pretty(&report)?;
        fs::write(&output_path, json)?;
        println!("Results saved to {}", output_path.display());
    }

    Ok(())
}

fn get_agent_prompt(agent_name: &str) -> Result<String> {
    // Default prompts matching src/config.rs
    let prompt = match agent_name {
        "holes" => "Review this plan and identify gaps, risks, and issues. Output the complete plan with an updated Review Notes section (## Review Notes) containing these four subsections:\n\n\
### Identified Weaknesses\n\
List at least 5 specific weaknesses in the plan. Focus on missing security considerations, unclear requirements, underspecified behavior, missing error handling, and architectural gaps. Be concrete and actionable.\n\n\
### Edge Cases\n\
List at least 5 edge cases that the plan doesn't address. Think about boundary conditions, error states, concurrent access, invalid inputs, and failure scenarios.\n\n\
### Assumptions to Validate\n\
List at least 4 assumptions the plan makes that should be verified before implementation. These are things that could change the approach if they turn out to be false.\n\n\
### Potential Failures\n\
List at least 4 ways the implementation could fail in production. Consider infrastructure failures, data issues, scaling problems, and operational concerns.\n\n\
Output the entire plan with the Review Notes section populated. Keep all other sections (Overview, Constraints, Implementation Notes, Tickets) unchanged.",
        "details" => "Expand this plan with implementation details. Add specifics about technologies, APIs, data structures, and algorithms.",
        "breakdown" => "Break this plan into precise, atomic steps. Each step should be independently implementable and testable.",
        "deliverables" => "Define clear acceptance criteria for each component. What tests must pass? What can be demonstrated? How do we know it's done?",
        _ => anyhow::bail!("Unknown agent: {}", agent_name),
    };
    Ok(prompt.to_string())
}

fn load_test_cases(fixtures_dir: &PathBuf, only: Option<&str>) -> Result<Vec<TestCase>> {
    let filter_numbers: Option<Vec<u32>> = only.map(|s| {
        s.split(',')
            .filter_map(|n| n.trim().parse().ok())
            .collect()
    });

    let mut test_cases = Vec::new();

    for entry in fs::read_dir(fixtures_dir).context("read fixtures directory")? {
        let entry = entry?;
        let filename = entry.file_name().to_string_lossy().to_string();

        if filename.starts_with("input_") && filename.ends_with(".md") {
            let number: u32 = filename
                .strip_prefix("input_")
                .unwrap()
                .strip_suffix(".md")
                .unwrap()
                .parse()
                .context("parse test case number")?;

            // Apply filter if specified
            if let Some(ref nums) = filter_numbers {
                if !nums.contains(&number) {
                    continue;
                }
            }

            let input_path = entry.path();
            let expected_path = fixtures_dir.join(format!("expected_{:02}.md", number));

            if !expected_path.exists() {
                anyhow::bail!("Missing expected file: {}", expected_path.display());
            }

            let input_content = fs::read_to_string(&input_path)?;
            let expected_content = fs::read_to_string(&expected_path)?;

            test_cases.push(TestCase {
                number,
                input_path,
                expected_path,
                input_content,
                expected_content,
            });
        }
    }

    test_cases.sort_by_key(|tc| tc.number);
    Ok(test_cases)
}

fn run_agent(command: &str, args: &[String], prompt: &str, plan_content: &str) -> Result<String> {
    let full_input = format!(
        "{}\n\n---\n\nPlan to review/revise:\n\n{}",
        prompt, plan_content
    );

    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| format!("spawn {} {}", command, args.join(" ")))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(full_input.as_bytes())
            .context("write stdin")?;
        stdin.flush().context("flush stdin")?;
    }

    let mut stdout = String::new();
    if let Some(mut out) = child.stdout.take() {
        out.read_to_string(&mut stdout).context("read stdout")?;
    }

    let status = child.wait().context("wait for agent")?;
    if !status.success() {
        anyhow::bail!("agent exited with {}", status);
    }

    Ok(stdout)
}

fn judge_output(
    model: &str,
    input: &str,
    expected: &str,
    actual: &str,
    agent_prompt: &str,
) -> Result<(f64, String)> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .context("ANTHROPIC_API_KEY environment variable required for judging")?;

    let judge_prompt = format!(
        r#"You are evaluating the output of an AI agent that reviews software plans.

The agent was given this task:
"{agent_prompt}"

## Original Plan (Input)
{input}

## Expected Output (Reference)
{expected}

## Actual Output (To Evaluate)
{actual}

## Evaluation Criteria

Score the actual output from 1-10 based on:
1. **Completeness** (1-3 points): Does it identify similar types of issues as the expected output? (weaknesses, edge cases, assumptions, failures)
2. **Quality** (1-3 points): Are the identified issues relevant, specific, and actionable?
3. **Coverage** (1-2 points): Does it cover the major gaps that the expected output covers?
4. **Format** (1-2 points): Is the output well-structured and follows a similar format to expected?

Note: The actual output does NOT need to match the expected output exactly. It should demonstrate similar analytical depth and identify comparable (not identical) issues.

Respond in this exact JSON format:
{{"score": <number 1-10>, "reasoning": "<brief explanation of score>"}}
"#
    );

    let request_body = serde_json::json!({
        "model": model,
        "max_tokens": 1024,
        "messages": [
            {
                "role": "user",
                "content": judge_prompt
            }
        ]
    });

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .context("send request to Anthropic API")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        anyhow::bail!("Anthropic API error {}: {}", status, body);
    }

    let response_json: serde_json::Value = response.json().context("parse API response")?;

    let content = response_json["content"][0]["text"]
        .as_str()
        .context("extract response text")?;

    // Parse the JSON response from the judge
    let judge_response: serde_json::Value =
        serde_json::from_str(content).context("parse judge JSON response")?;

    let score = judge_response["score"]
        .as_f64()
        .context("extract score from judge response")?;
    let reasoning = judge_response["reasoning"]
        .as_str()
        .unwrap_or("No reasoning provided")
        .to_string();

    Ok((score, reasoning))
}

fn generate_report(agent: &str, results: &[TestResult]) -> EvalReport {
    let scores: Vec<f64> = results.iter().map(|r| r.score).collect();
    let average_score = if scores.is_empty() {
        0.0
    } else {
        scores.iter().sum::<f64>() / scores.len() as f64
    };
    let min_score = scores.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    EvalReport {
        agent: agent.to_string(),
        total_cases: results.len(),
        completed_cases: results.len(),
        average_score,
        min_score: if min_score.is_infinite() {
            0.0
        } else {
            min_score
        },
        max_score: if max_score.is_infinite() {
            0.0
        } else {
            max_score
        },
        results: results.to_vec(),
    }
}

fn print_report(report: &EvalReport) {
    println!("{}", "═".repeat(60));
    println!("{}", "EVALUATION REPORT".bold());
    println!("{}", "═".repeat(60));
    println!("Agent: {}", report.agent.cyan());
    println!("Test cases: {}", report.total_cases);
    println!();

    let avg_color = if report.average_score >= 8.0 {
        format!("{:.2}", report.average_score).green()
    } else if report.average_score >= 6.0 {
        format!("{:.2}", report.average_score).yellow()
    } else {
        format!("{:.2}", report.average_score).red()
    };

    println!("Average Score: {}/10", avg_color);
    println!(
        "Score Range: {:.1} - {:.1}",
        report.min_score, report.max_score
    );
    println!();

    println!("{}", "Score Distribution:".bold());
    let mut buckets = [0usize; 10];
    for result in &report.results {
        let bucket = ((result.score - 0.01).max(0.0) as usize).min(9);
        buckets[bucket] += 1;
    }
    for (i, count) in buckets.iter().enumerate() {
        let bar = "█".repeat(*count);
        println!("  {}-{}: {} {}", i + 1, i + 1, bar, count);
    }
    println!();

    // Show lowest scoring cases for improvement focus
    let mut sorted_results = report.results.clone();
    sorted_results.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

    println!("{}", "Lowest Scoring Cases (focus for improvement):".bold());
    for result in sorted_results.iter().take(5) {
        println!(
            "  Case {}: {:.1}/10 - {}",
            result.test_case,
            result.score,
            result.reasoning.chars().take(60).collect::<String>()
        );
    }
    println!("{}", "═".repeat(60));
}
