use bytes::{Buf, BytesMut};
use protocol::parser::{self, ParseError};
use protocol::types::RespFrame;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

const SAMPLE_EVERY: u64 = 32_768;

pub fn encode_resp_parts(parts: &[&[u8]]) -> Vec<u8> {
    let mut out = Vec::with_capacity(parts.iter().map(|part| part.len() + 16).sum::<usize>() + 16);
    out.push(b'*');
    append_u64(&mut out, parts.len() as u64);
    out.extend_from_slice(b"\r\n");

    for part in parts {
        out.push(b'$');
        append_u64(&mut out, part.len() as u64);
        out.extend_from_slice(b"\r\n");
        out.extend_from_slice(part);
        out.extend_from_slice(b"\r\n");
    }
    out
}

pub fn make_key(base: &[u8], sequence: u64) -> Vec<u8> {
    if sequence == 0 {
        return base.to_vec();
    }

    let mut key = Vec::with_capacity(base.len() + 1 + 20);
    key.extend_from_slice(base);
    key.push(b':');
    append_u64(&mut key, sequence);
    key
}

pub fn repeat_payload(one: &[u8], count: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(one.len() * count);
    for _ in 0..count {
        out.extend_from_slice(one);
    }
    out
}

pub async fn read_n_responses(
    stream: &mut TcpStream,
    parse_buf: &mut BytesMut,
    expected: usize,
) -> Result<(), String> {
    let mut parsed = 0usize;

    while parsed < expected {
        loop {
            if parsed >= expected {
                return Ok(());
            }
            match parser::parse_frame(parse_buf) {
                Ok(Some(frame)) => {
                    if let RespFrame::Error(message) = frame {
                        return Err(format!("server returned error: {message}"));
                    }
                    parsed += 1;
                }
                Ok(None) | Err(ParseError::Incomplete) => break,
                Err(ParseError::Protocol(err)) => {
                    return Err(format!("protocol error: {err}"));
                }
            }
        }

        let read = stream
            .read_buf(parse_buf)
            .await
            .map_err(|err| format!("read failed: {err}"))?;
        if read == 0 {
            return Err("connection closed by server".to_string());
        }
    }

    Ok(())
}

pub async fn read_n_fixed_mget_responses(
    stream: &mut TcpStream,
    parse_buf: &mut BytesMut,
    expected: usize,
    value_len: usize,
    strict: bool,
    seen: &mut u64,
) -> Result<(), String> {
    let expected_frame = encode_fixed_mget_response(value_len);
    read_n_fixed_responses(stream, parse_buf, expected, &expected_frame, strict, seen).await
}

pub async fn read_n_fixed_hgetall_responses(
    stream: &mut TcpStream,
    parse_buf: &mut BytesMut,
    expected: usize,
    value_len: usize,
    strict: bool,
    seen: &mut u64,
) -> Result<(), String> {
    let expected_frame = encode_fixed_hgetall_response(value_len);
    read_n_fixed_responses(stream, parse_buf, expected, &expected_frame, strict, seen).await
}

async fn read_n_fixed_responses(
    stream: &mut TcpStream,
    parse_buf: &mut BytesMut,
    expected_count: usize,
    expected_frame: &[u8],
    strict: bool,
    seen: &mut u64,
) -> Result<(), String> {
    let frame_len = expected_frame.len();

    for _ in 0..expected_count {
        while parse_buf.len() < frame_len {
            let read = stream
                .read_buf(parse_buf)
                .await
                .map_err(|err| format!("read failed: {err}"))?;
            if read == 0 {
                return Err("connection closed by server".to_string());
            }
        }

        if parse_buf[0] != expected_frame[0] {
            return Err(format!(
                "unexpected response type byte: expected {:?}, got {:?}",
                expected_frame[0] as char,
                parse_buf[0] as char
            ));
        }

        let validate = strict || *seen == 0 || *seen % SAMPLE_EVERY == 0;
        *seen += 1;
        if validate && &parse_buf[..frame_len] != expected_frame {
            return Err("sampled response validation failed".to_string());
        }

        parse_buf.advance(frame_len);
    }

    Ok(())
}

fn encode_fixed_mget_response(value_len: usize) -> Vec<u8> {
    let value = vec![b'x'; value_len];
    let mut out = Vec::with_capacity(4 + 2 * (value_len + 16));
    out.extend_from_slice(b"*2\r\n");
    append_bulk(&mut out, &value);
    append_bulk(&mut out, &value);
    out
}

fn encode_fixed_hgetall_response(value_len: usize) -> Vec<u8> {
    let value = vec![b'x'; value_len];
    let mut out = Vec::with_capacity(4 + value_len + 24);
    out.extend_from_slice(b"*2\r\n");
    append_bulk(&mut out, b"field");
    append_bulk(&mut out, &value);
    out
}

fn append_bulk(out: &mut Vec<u8>, value: &[u8]) {
    out.push(b'$');
    append_u64(out, value.len() as u64);
    out.extend_from_slice(b"\r\n");
    out.extend_from_slice(value);
    out.extend_from_slice(b"\r\n");
}

fn append_u64(out: &mut Vec<u8>, value: u64) {
    let mut tmp = itoa::Buffer::new();
    out.extend_from_slice(tmp.format(value).as_bytes());
}
