use super::JsonError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonPathToken {
    Field(Vec<u8>),
    RecursiveField(Vec<u8>),
    Index(i64),
    Wildcard,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonPath {
    pub tokens: Vec<JsonPathToken>,
}

impl JsonPath {
    pub fn root() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn parse(raw: &[u8]) -> Result<Self, JsonError> {
        if raw == b"$" || raw == b"." {
            return Ok(Self::root());
        }
        if raw.first().copied() != Some(b'$') {
            return Err(JsonError::Syntax);
        }

        let mut index = 1usize;
        let mut tokens = Vec::new();
        while index < raw.len() {
            match raw[index] {
                b'.' => {
                    index += 1;
                    if index >= raw.len() {
                        return Err(JsonError::Syntax);
                    }
                    if raw[index] == b'.' {
                        index += 1;
                        if index >= raw.len() {
                            return Err(JsonError::Syntax);
                        }
                        let start = index;
                        while index < raw.len() {
                            match raw[index] {
                                b'.' | b'[' => break,
                                _ => index += 1,
                            }
                        }
                        if start == index {
                            return Err(JsonError::Syntax);
                        }
                        tokens.push(JsonPathToken::RecursiveField(raw[start..index].to_vec()));
                        continue;
                    }
                    if raw[index] == b'*' {
                        tokens.push(JsonPathToken::Wildcard);
                        index += 1;
                        continue;
                    }

                    let start = index;
                    while index < raw.len() {
                        match raw[index] {
                            b'.' | b'[' => break,
                            _ => index += 1,
                        }
                    }
                    if start == index {
                        return Err(JsonError::Syntax);
                    }
                    tokens.push(JsonPathToken::Field(raw[start..index].to_vec()));
                }
                b'[' => {
                    index += 1;
                    if index >= raw.len() {
                        return Err(JsonError::Syntax);
                    }
                    if raw[index] == b'*' {
                        index += 1;
                        if raw.get(index) != Some(&b']') {
                            return Err(JsonError::Syntax);
                        }
                        index += 1;
                        tokens.push(JsonPathToken::Wildcard);
                        continue;
                    }

                    let start = index;
                    if raw[index] == b'-' {
                        index += 1;
                    }
                    while index < raw.len() && raw[index].is_ascii_digit() {
                        index += 1;
                    }
                    if start == index || raw.get(index) != Some(&b']') {
                        return Err(JsonError::Syntax);
                    }
                    let value = std::str::from_utf8(&raw[start..index])
                        .ok()
                        .and_then(|value| value.parse::<i64>().ok())
                        .ok_or(JsonError::Syntax)?;
                    index += 1;
                    tokens.push(JsonPathToken::Index(value));
                }
                _ => return Err(JsonError::Syntax),
            }
        }

        Ok(Self { tokens })
    }
}
