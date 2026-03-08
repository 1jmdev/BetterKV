use std::time::Instant;

use protocol::types::{BulkData, RespFrame};

use crate::args::Args;
use crate::client::Client;
use crate::discovery::discover_rtest_files;
use crate::model::{ExpectedValue, RunSummary, TestFailure};
use crate::output::{print_failures, render_frame, Ui};
use crate::parser::parse_test_file;

pub async fn run(args: Args) -> Result<RunSummary, String> {
    let started = Instant::now();
    let paths = discover_rtest_files(&args.path)?;
    if paths.is_empty() {
        return Err(format!("no .rtest files found under {}", args.path.display()));
    }

    let files = paths
        .iter()
        .map(|path| parse_test_file(path))
        .collect::<Result<Vec<_>, _>>()?;

    let total = files.iter().map(|file| file.cases.len()).sum();
    let ui = Ui::new(total, args.quiet);
    ui.set_discovery(files.len(), total);

    let mut client = Client::connect(&args).await?;
    let mut failures = Vec::new();
    let mut passed = 0usize;

    for file in files {
        for case in file.cases {
            let location = match &file.metadata.name {
                Some(name) => format!("{} :: {} :: {}", file.path.display(), name, case.name),
                None => format!("{} :: {}", file.path.display(), case.name),
            };
            ui.set_current_test(&location);

            let case_started = Instant::now();
            let result = run_case(&mut client, &case).await;
            let elapsed = case_started.elapsed();

            match result {
                Ok(()) => {
                    passed += 1;
                    ui.record_success(&location);
                }
                Err(error) => {
                    let failure = TestFailure {
                        path: file.path.clone(),
                        test_name: case.name,
                        elapsed,
                        error,
                    };
                    ui.record_failure(&location);
                    failures.push(failure);
                }
            }
        }
    }

    let summary = RunSummary {
        total,
        passed,
        failed: failures.len(),
        elapsed: started.elapsed(),
        failures,
    };

    ui.finish(&summary);
    print_failures(&summary.failures);

    Ok(summary)
}

async fn run_case(client: &mut Client, case: &crate::model::TestCase) -> Result<(), String> {
    client.flush_all().await.map_err(|err| format!("FLUSHALL failed: {err}"))?;

    for command in &case.setup {
        client
            .execute_raw(command)
            .await
            .map_err(|err| format!("setup command `{command}` failed: {err}"))?;
    }

    let mut last = None;
    for command in &case.run {
        let frame = client
            .execute_raw(command)
            .await
            .map_err(|err| format!("run command `{command}` failed: {err}"))?;
        last = Some(frame);
    }

    let actual = last.ok_or_else(|| "RUN section did not execute any command".to_string())?;
    validate_expected(&case.expect, &actual)?;

    for command in &case.cleanup {
        client
            .execute_raw(command)
            .await
            .map_err(|err| format!("cleanup command `{command}` failed: {err}"))?;
    }

    Ok(())
}

fn validate_expected(expected: &ExpectedValue, actual: &RespFrame) -> Result<(), String> {
    match expected {
        ExpectedValue::Any => Ok(()),
        ExpectedValue::Simple(value) => match actual {
            RespFrame::Simple(actual) if actual == value => Ok(()),
            RespFrame::SimpleStatic(actual) if actual == value => Ok(()),
            _ => Err(mismatch(expected, actual)),
        },
        ExpectedValue::Bulk(None) => match actual {
            RespFrame::Bulk(None) => Ok(()),
            _ => Err(mismatch(expected, actual)),
        },
        ExpectedValue::Bulk(Some(value)) => match actual {
            RespFrame::Bulk(Some(BulkData::Arg(actual))) if actual.as_slice() == value.as_bytes() => Ok(()),
            RespFrame::Bulk(Some(BulkData::Value(actual))) if actual.as_slice() == value.as_bytes() => Ok(()),
            _ => Err(mismatch(expected, actual)),
        },
        ExpectedValue::Integer(value) => match actual {
            RespFrame::Integer(actual) if actual == value => Ok(()),
            _ => Err(mismatch(expected, actual)),
        },
        ExpectedValue::ErrorAny => match actual {
            RespFrame::Error(_) | RespFrame::ErrorStatic(_) => Ok(()),
            _ => Err(mismatch(expected, actual)),
        },
        ExpectedValue::ErrorPrefix(prefix) => match actual {
            RespFrame::Error(actual) if actual.starts_with(prefix) => Ok(()),
            RespFrame::ErrorStatic(actual) if actual.starts_with(prefix) => Ok(()),
            _ => Err(mismatch(expected, actual)),
        },
        ExpectedValue::EmptyArray => match actual {
            RespFrame::Array(Some(items)) if items.is_empty() => Ok(()),
            RespFrame::BulkOptions(items) if items.is_empty() => Ok(()),
            RespFrame::BulkValues(items) if items.is_empty() => Ok(()),
            _ => Err(mismatch(expected, actual)),
        },
        ExpectedValue::Array { items, unordered } => validate_array(items, *unordered, actual),
        ExpectedValue::Regex(regex) => {
            let rendered = render_frame(actual);
            if regex.is_match(&rendered) {
                Ok(())
            } else {
                Err(format!(
                    "expected response to match `{}`, got:\n{}",
                    regex.as_str(),
                    rendered
                ))
            }
        }
    }
}

fn validate_array(items: &[ExpectedValue], unordered: bool, actual: &RespFrame) -> Result<(), String> {
    let actual_items = match actual {
        RespFrame::Array(Some(items)) => items,
        _ => return Err(mismatch(&ExpectedValue::Array { items: items.to_vec(), unordered }, actual)),
    };

    if actual_items.len() != items.len() {
        return Err(format!(
            "expected {} array item(s), got {}:\n{}",
            items.len(),
            actual_items.len(),
            render_frame(actual)
        ));
    }

    if !unordered {
        for (expected_item, actual_item) in items.iter().zip(actual_items.iter()) {
            validate_expected(expected_item, actual_item)?;
        }
        return Ok(());
    }

    let mut used = vec![false; actual_items.len()];
    for expected_item in items {
        let mut matched = false;
        for (index, actual_item) in actual_items.iter().enumerate() {
            if used[index] {
                continue;
            }
            if validate_expected(expected_item, actual_item).is_ok() {
                used[index] = true;
                matched = true;
                break;
            }
        }
        if !matched {
            return Err(format!(
                "could not match unordered item `{}` in:\n{}",
                expected_to_string(expected_item),
                render_frame(actual)
            ));
        }
    }

    Ok(())
}

fn mismatch(expected: &ExpectedValue, actual: &RespFrame) -> String {
    format!(
        "expected {}, got:\n{}",
        expected_to_string(expected),
        render_frame(actual)
    )
}

fn expected_to_string(expected: &ExpectedValue) -> String {
    match expected {
        ExpectedValue::Any => "(any)".to_string(),
        ExpectedValue::Simple(value) => value.clone(),
        ExpectedValue::Bulk(None) => "(nil)".to_string(),
        ExpectedValue::Bulk(Some(value)) => format!("\"{}\"", value),
        ExpectedValue::Integer(value) => format!("(integer) {value}"),
        ExpectedValue::ErrorAny => "(error)".to_string(),
        ExpectedValue::ErrorPrefix(prefix) => format!("(error) {prefix}"),
        ExpectedValue::EmptyArray => "(empty array)".to_string(),
        ExpectedValue::Array { items, unordered } => {
            let prefix = if *unordered { "(unordered) " } else { "" };
            format!("{prefix}array[{}]", items.len())
        }
        ExpectedValue::Regex(regex) => format!("(match) {}", regex.as_str()),
    }
}
