use bytes::{Buf, BytesMut};
use memchr::memchr;
use smallvec::SmallVec;
use thiserror::Error;

use crate::types::{BulkData, RespFrame};
use types::value::CompactArg;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("incomplete frame")]
    Incomplete,
    #[error("protocol error: {0}")]
    Protocol(String),
}

pub fn parse_frame(src: &mut BytesMut) -> Result<Option<RespFrame>, ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_frame");
    if src.is_empty() {
        return Ok(None);
    }

    match parse_value(src, 0) {
        Ok((frame, consumed)) => {
            src.advance(consumed);
            Ok(Some(frame))
        }
        Err(ParseError::Incomplete) => Ok(None),
        Err(err) => Err(err),
    }
}

pub fn parse_command_into(
    src: &mut BytesMut,
    args: &mut Vec<CompactArg>,
) -> Result<Option<()>, ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_command_into");
    if src.is_empty() {
        return Ok(None);
    }

    if src[0] == b'*' {
        return parse_command_array_into(src, args);
    }

    parse_inline_command_into(src, args)
}

fn parse_inline_command_into(
    src: &mut BytesMut,
    args: &mut Vec<CompactArg>,
) -> Result<Option<()>, ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_inline_command_into");
    let consumed = match parse_line_bytes(src, 0) {
        Ok(value) => value,
        Err(ParseError::Incomplete) => return Ok(None),
        Err(err) => return Err(err),
    }
    .1;

    args.clear();

    let line_end = consumed - 2;
    let mut start = 0;
    let mut saw_first = false;
    while start < line_end {
        while start < line_end && src[start].is_ascii_whitespace() {
            start += 1;
        }
        if start == line_end {
            break;
        }

        let mut end = start + 1;
        while end < line_end && !src[end].is_ascii_whitespace() {
            end += 1;
        }

        if !saw_first {
            src[start..end].make_ascii_uppercase();
            saw_first = true;
        }
        args.push(CompactArg::from_slice(&src[start..end]));
        start = end + 1;
    }

    if args.is_empty() {
        return Err(ParseError::Protocol("empty inline command".to_string()));
    }

    src.advance(consumed);
    Ok(Some(()))
}

fn parse_command_array_into(
    src: &mut BytesMut,
    args: &mut Vec<CompactArg>,
) -> Result<Option<()>, ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_command_array_into");
    let (line, mut cursor) = match parse_line_bytes(src, 1) {
        Ok(value) => value,
        Err(ParseError::Incomplete) => return Ok(None),
        Err(err) => return Err(err),
    };

    let length =
        parse_decimal(line).ok_or(ParseError::Protocol("invalid array length".to_string()))?;
    if length < 0 {
        return Err(ParseError::Protocol("invalid command array".to_string()));
    }

    let argc = length as usize;
    args.clear();
    if args.capacity() < argc {
        args.reserve(argc - args.capacity());
    }

    for index in 0..argc {
        if cursor >= src.len() {
            return Ok(None);
        }

        match src[cursor] {
            b'$' => {
                let (bulk_len_raw, bulk_header_end) = match parse_line_bytes(src, cursor + 1) {
                    Ok(value) => value,
                    Err(ParseError::Incomplete) => return Ok(None),
                    Err(err) => return Err(err),
                };

                let bulk_len = parse_decimal(bulk_len_raw)
                    .ok_or(ParseError::Protocol("invalid bulk length".to_string()))?;
                if bulk_len < 0 {
                    return Err(ParseError::Protocol("invalid argument type".to_string()));
                }

                let size = bulk_len as usize;
                if src.len() < bulk_header_end + size + 2 {
                    return Ok(None);
                }

                let end = bulk_header_end + size;
                if src[end] != b'\r' || src[end + 1] != b'\n' {
                    return Err(ParseError::Protocol("missing bulk terminator".to_string()));
                }

                if index == 0 {
                    src[bulk_header_end..end].make_ascii_uppercase();
                }
                args.push(CompactArg::from_slice(&src[bulk_header_end..end]));
                cursor = end + 2;
            }
            b'+' => {
                let (simple, next) = match parse_line_bytes(src, cursor + 1) {
                    Ok(value) => value,
                    Err(ParseError::Incomplete) => return Ok(None),
                    Err(err) => return Err(err),
                };
                if std::str::from_utf8(simple).is_err() {
                    return Err(ParseError::Protocol("invalid utf8 line".to_string()));
                }
                let simple_start = cursor + 1;
                let simple_end = next - 2;
                if index == 0 {
                    src[simple_start..simple_end].make_ascii_uppercase();
                }
                args.push(CompactArg::from_slice(&src[simple_start..simple_end]));
                cursor = next;
            }
            _ => {
                return Err(ParseError::Protocol("invalid argument type".to_string()));
            }
        }
    }

    src.advance(cursor);
    Ok(Some(()))
}

fn parse_value(src: &[u8], offset: usize) -> Result<(RespFrame, usize), ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_value");
    if offset >= src.len() {
        return Err(ParseError::Incomplete);
    }

    match src[offset] {
        b'+' => parse_simple(src, offset),
        b'-' => parse_error(src, offset),
        b':' => parse_integer(src, offset),
        b'$' => parse_bulk(src, offset),
        b'*' => parse_array(src, offset),
        _ => parse_inline(src, offset),
    }
}

fn parse_inline(src: &[u8], offset: usize) -> Result<(RespFrame, usize), ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_inline");
    let (line, consumed) = parse_line_bytes(src, offset)?;

    let mut parts = SmallVec::<[RespFrame; 8]>::new();
    let mut start = 0;
    while start < line.len() {
        while start < line.len() && line[start].is_ascii_whitespace() {
            start += 1;
        }
        if start == line.len() {
            break;
        }

        let mut end = start + 1;
        while end < line.len() && !line[end].is_ascii_whitespace() {
            end += 1;
        }

        parts.push(RespFrame::Bulk(Some(BulkData::Arg(
            CompactArg::from_slice(&line[start..end]),
        ))));
        start = end + 1;
    }

    if parts.is_empty() {
        return Err(ParseError::Protocol("empty inline command".to_string()));
    }

    Ok((RespFrame::Array(Some(parts.into_vec())), consumed))
}

fn parse_simple(src: &[u8], offset: usize) -> Result<(RespFrame, usize), ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_simple");
    let (line, consumed) = parse_line_bytes(src, offset + 1)?;
    let text = std::str::from_utf8(line)
        .map_err(|_| ParseError::Protocol("invalid utf8 line".to_string()))?
        .to_owned();
    Ok((RespFrame::Simple(text), consumed))
}

fn parse_error(src: &[u8], offset: usize) -> Result<(RespFrame, usize), ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_error");
    let (line, consumed) = parse_line_bytes(src, offset + 1)?;
    let text = std::str::from_utf8(line)
        .map_err(|_| ParseError::Protocol("invalid utf8 line".to_string()))?
        .to_owned();
    Ok((RespFrame::Error(text), consumed))
}

fn parse_integer(src: &[u8], offset: usize) -> Result<(RespFrame, usize), ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_integer");
    let (line, consumed) = parse_line_bytes(src, offset + 1)?;
    let value = parse_decimal(line).ok_or(ParseError::Protocol("invalid integer".to_string()))?;
    Ok((RespFrame::Integer(value), consumed))
}

fn parse_bulk(src: &[u8], offset: usize) -> Result<(RespFrame, usize), ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_bulk");
    let (line, mut cursor) = parse_line_bytes(src, offset + 1)?;
    let length =
        parse_decimal(line).ok_or(ParseError::Protocol("invalid bulk length".to_string()))?;

    if length < 0 {
        return Ok((RespFrame::Bulk(None), cursor));
    }

    let size = length as usize;
    if src.len() < cursor + size + 2 {
        return Err(ParseError::Incomplete);
    }

    let end = cursor + size;
    let payload = BulkData::Arg(CompactArg::from_slice(&src[cursor..end]));
    cursor = end;

    if src[cursor] != b'\r' || src[cursor + 1] != b'\n' {
        return Err(ParseError::Protocol("missing bulk terminator".to_string()));
    }

    cursor += 2;
    Ok((RespFrame::Bulk(Some(payload)), cursor))
}

fn parse_array(src: &[u8], offset: usize) -> Result<(RespFrame, usize), ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_array");
    let (line, mut cursor) = parse_line_bytes(src, offset + 1)?;
    let length =
        parse_decimal(line).ok_or(ParseError::Protocol("invalid array length".to_string()))?;

    if length < 0 {
        return Ok((RespFrame::Array(None), cursor));
    }

    let mut items = SmallVec::<[RespFrame; 8]>::with_capacity(length as usize);
    for _ in 0..length {
        let (item, consumed) = parse_value(src, cursor)?;
        cursor = consumed;
        items.push(item);
    }

    Ok((RespFrame::Array(Some(items.into_vec())), cursor))
}

fn parse_line_bytes(src: &[u8], from: usize) -> Result<(&[u8], usize), ParseError> {
    let _trace = profiler::scope("protocol::parser::parse_line_bytes");
    let end = find_crlf(src, from).ok_or(ParseError::Incomplete)?;
    Ok((&src[from..end], end + 2))
}

#[inline(always)]
fn find_crlf(src: &[u8], from: usize) -> Option<usize> {
    let _trace = profiler::scope("protocol::parser::find_crlf");
    let haystack = &src[from..];
    let mut offset = 0;
    while offset < haystack.len() {
        let pos = memchr(b'\r', &haystack[offset..])?;
        let abs = offset + pos;
        if abs + 1 < haystack.len() && haystack[abs + 1] == b'\n' {
            return Some(from + abs);
        }
        offset = abs + 1;
    }
    None
}

fn parse_decimal(raw: &[u8]) -> Option<i64> {
    let _trace = profiler::scope("protocol::parser::parse_decimal");
    if raw.is_empty() {
        return None;
    }

    let mut index = 0;
    let mut negative = false;
    if raw[0] == b'-' {
        negative = true;
        index = 1;
    }
    if index >= raw.len() {
        return None;
    }

    let mut value: i64 = 0;
    while index < raw.len() {
        let digit = raw[index].wrapping_sub(b'0');
        if digit > 9 {
            return None;
        }
        value = value.checked_mul(10)?;
        value = value.checked_add(i64::from(digit))?;
        index += 1;
    }

    if negative {
        value.checked_neg()
    } else {
        Some(value)
    }
}
