#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JsonSetMode {
    Any,
    Nx,
    Xx,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JsonType {
    Null,
    Boolean,
    Integer,
    Number,
    String,
    Array,
    Object,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JsonSetResult {
    pub applied: bool,
    pub ttl_preserved: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum JsonError {
    WrongType,
    Syntax,
    Path,
    KeyMissing,
}

impl JsonType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Boolean => "boolean",
            Self::Integer => "integer",
            Self::Number => "number",
            Self::String => "string",
            Self::Array => "array",
            Self::Object => "object",
        }
    }
}
