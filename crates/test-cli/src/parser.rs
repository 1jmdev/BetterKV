use std::fs;
use std::path::Path;

use regex::Regex;

use crate::model::{ExpectedValue, FileMetadata, TestCase, TestFile};

pub fn parse_test_file(path: &Path) -> Result<TestFile, String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;

    let lines = raw.lines().collect::<Vec<_>>();
    let mut metadata = FileMetadata::default();
    let mut cases = Vec::new();
    let mut index = 0usize;

    while index < lines.len() {
        let line = lines[index].trim();
        if line.starts_with("@name ") {
            metadata.name = Some(line[6..].trim().to_string());
            index += 1;
            continue;
        }
        if line.starts_with("@group ") {
            metadata.group = Some(line[7..].trim().to_string());
            index += 1;
            continue;
        }
        if line.starts_with("@since ") {
            metadata.since = Some(line[7..].trim().to_string());
            index += 1;
            continue;
        }
        if line.starts_with("===") {
            let (case, next) = parse_case(&lines, index, path)?;
            cases.push(case);
            index = next;
            continue;
        }
        index += 1;
    }

    if cases.is_empty() {
        return Err(format!("{} does not contain any tests", path.display()));
    }

    Ok(TestFile {
        path: path.to_path_buf(),
        metadata,
        cases,
    })
}

fn parse_case(lines: &[&str], start: usize, path: &Path) -> Result<(TestCase, usize), String> {
    let header = lines[start].trim();
    let mut index = start + 1;

    let name = if let Some(name) = header.strip_prefix("=== TEST:") {
        name.trim().to_string()
    } else {
        while index < lines.len() && ignored_line(lines[index]) {
            index += 1;
        }
        let Some(name_line) = lines.get(index).map(|value| value.trim()) else {
            return Err(format!(
                "{}:{} missing test name",
                path.display(),
                start + 1
            ));
        };
        let Some(name) = name_line.strip_prefix("--- TEST:") else {
            return Err(format!(
                "{}:{} expected `--- TEST:` after test separator",
                path.display(),
                start + 1
            ));
        };
        index += 1;
        name.trim().to_string()
    };

    let mut setup = Vec::new();
    let mut run = Vec::new();
    let mut cleanup = Vec::new();
    let mut expect_lines = Vec::new();
    let mut section = Section::None;

    while index < lines.len() {
        let raw_line = lines[index];
        let line = raw_line.trim();
        if index != start && line.starts_with("===") {
            break;
        }
        if ignored_line(raw_line) {
            index += 1;
            continue;
        }

        match line {
            "SETUP:" => section = Section::Setup,
            "RUN:" => section = Section::Run,
            "EXPECT:" => section = Section::Expect,
            "CLEANUP:" => section = Section::Cleanup,
            _ => match section {
                Section::Setup => setup.push(line.to_string()),
                Section::Run => run.push(line.to_string()),
                Section::Expect => expect_lines.push(line.to_string()),
                Section::Cleanup => cleanup.push(line.to_string()),
                Section::None => {
                    return Err(format!(
                        "{}:{} unexpected content outside a section",
                        path.display(),
                        index + 1
                    ));
                }
            },
        }

        index += 1;
    }

    if run.is_empty() {
        return Err(format!(
            "{}:{} test `{name}` is missing RUN section",
            path.display(),
            start + 1
        ));
    }
    if expect_lines.is_empty() {
        return Err(format!(
            "{}:{} test `{name}` is missing EXPECT section",
            path.display(),
            start + 1
        ));
    }

    let expect = parse_expected(&expect_lines, path, &name)?;
    Ok((
        TestCase {
            name,
            setup,
            run,
            expect,
            cleanup,
        },
        index,
    ))
}

fn parse_expected(lines: &[String], path: &Path, test_name: &str) -> Result<ExpectedValue, String> {
    if lines.len() == 1 {
        return parse_scalar_expected(&lines[0], false, path, test_name);
    }

    let mut unordered = false;
    let mut slice = lines;
    if let Some(first) = lines.first() {
        if first == "(unordered)" {
            unordered = true;
            slice = &lines[1..];
        }
    }

    if slice.is_empty() {
        return Err(format!(
            "{} test `{test_name}` has `(unordered)` with no array items",
            path.display()
        ));
    }

    let mut items = Vec::with_capacity(slice.len());
    for line in slice {
        let Some((_, value)) = line.split_once(')') else {
            return Err(format!(
                "{} test `{test_name}` has invalid array line `{line}`",
                path.display()
            ));
        };
        items.push(parse_scalar_expected(value.trim(), true, path, test_name)?);
    }

    Ok(ExpectedValue::Array { items, unordered })
}

fn parse_scalar_expected(
    raw: &str,
    in_array: bool,
    path: &Path,
    test_name: &str,
) -> Result<ExpectedValue, String> {
    if raw == "(any)" {
        return Ok(ExpectedValue::Any);
    }
    if raw == "(nil)" {
        return Ok(ExpectedValue::Bulk(None));
    }
    if raw == "(empty array)" || raw == "(empty list or set)" {
        return Ok(ExpectedValue::EmptyArray);
    }
    if raw == "(error)" {
        return Ok(ExpectedValue::ErrorAny);
    }
    if let Some(rest) = raw.strip_prefix("(error) ") {
        return Ok(ExpectedValue::ErrorPrefix(rest.trim().to_string()));
    }
    if let Some(rest) = raw.strip_prefix("(integer) ") {
        let value = rest.trim().parse::<i64>().map_err(|err| {
            format!(
                "{} test `{test_name}` has invalid integer expectation `{raw}`: {err}",
                path.display()
            )
        })?;
        return Ok(ExpectedValue::Integer(value));
    }
    if let Some(rest) = raw.strip_prefix("(match) ") {
        let regex = Regex::new(rest.trim()).map_err(|err| {
            format!(
                "{} test `{test_name}` has invalid regex `{}`: {err}",
                path.display(),
                rest.trim()
            )
        })?;
        return Ok(ExpectedValue::Regex(regex));
    }
    if raw.starts_with('"') {
        return Ok(ExpectedValue::Bulk(Some(
            parse_quoted_string(raw)
                .map_err(|err| format!("{} test `{test_name}`: {err}", path.display()))?,
        )));
    }
    if !in_array {
        return Ok(ExpectedValue::Simple(raw.to_string()));
    }

    Ok(ExpectedValue::Simple(raw.to_string()))
}

fn parse_quoted_string(raw: &str) -> Result<String, String> {
    if !raw.ends_with('"') || raw.len() < 2 {
        return Err(format!("invalid quoted string `{raw}`"));
    }

    let mut out = String::new();
    let mut chars = raw[1..raw.len() - 1].chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }

        let Some(escaped) = chars.next() else {
            return Err(format!("invalid escape in `{raw}`"));
        };

        match escaped {
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            other => return Err(format!("unsupported escape `\\{other}` in `{raw}`")),
        }
    }

    Ok(out)
}

fn ignored_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || trimmed.starts_with('#')
}

#[derive(Debug, Clone, Copy)]
enum Section {
    None,
    Setup,
    Run,
    Expect,
    Cleanup,
}
