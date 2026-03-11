use protocol::types::RespFrame;

pub(super) fn parse_score(raw: &[u8]) -> Result<f64, RespFrame> {
    match std::str::from_utf8(raw) {
        Ok(value) => value
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
            .ok_or_else(|| RespFrame::Error("ERR value is not a valid float".to_string())),
        Err(_) => Err(RespFrame::Error(
            "ERR value is not a valid float".to_string(),
        )),
    }
}

pub(super) fn parse_score_bound(raw: &[u8]) -> Result<(f64, bool), RespFrame> {
    let exclusive = raw.first() == Some(&b'(');
    let value = if exclusive { &raw[1..] } else { raw };
    let score = match value {
        b"-inf" => f64::NEG_INFINITY,
        b"+inf" | b"inf" => f64::INFINITY,
        _ => parse_score(value)?,
    };
    Ok((score, exclusive))
}
