use serde_json::{Map, Number, Value as JsonValue};

use types::value::Entry;

use super::{normalize_index, JsonError, JsonPath, JsonPathToken};

pub(crate) fn json_entry(entry: &Entry) -> Result<&JsonValue, JsonError> {
    entry.as_json().ok_or(JsonError::WrongType)
}

pub(crate) fn json_entry_mut(entry: &mut Entry) -> Result<&mut JsonValue, JsonError> {
    entry.as_json_mut().ok_or(JsonError::WrongType)
}

pub(crate) fn collect_matches<'a>(
    value: &'a JsonValue,
    tokens: &[JsonPathToken],
    out: &mut Vec<&'a JsonValue>,
) {
    if tokens.is_empty() {
        out.push(value);
        return;
    }
    match &tokens[0] {
        JsonPathToken::Field(field) => {
            if let JsonValue::Object(object) = value {
                if let Ok(field) = std::str::from_utf8(field) {
                    if let Some(child) = object.get(field) {
                        collect_matches(child, &tokens[1..], out);
                    }
                }
            }
        }
        JsonPathToken::RecursiveField(field) => {
            if let Ok(field) = std::str::from_utf8(field) {
                collect_recursive_field_matches(value, field, &tokens[1..], out);
            }
        }
        JsonPathToken::Index(index) => {
            if let JsonValue::Array(array) = value {
                if let Some(index) = normalize_index(array.len(), *index) {
                    if let Some(child) = array.get(index) {
                        collect_matches(child, &tokens[1..], out);
                    }
                }
            }
        }
        JsonPathToken::Wildcard => match value {
            JsonValue::Array(array) => {
                for child in array {
                    collect_matches(child, &tokens[1..], out);
                }
            }
            JsonValue::Object(object) => {
                for child in object.values() {
                    collect_matches(child, &tokens[1..], out);
                }
            }
            _ => {}
        },
    }
}

fn collect_recursive_field_matches<'a>(
    value: &'a JsonValue,
    field: &str,
    remaining_tokens: &[JsonPathToken],
    out: &mut Vec<&'a JsonValue>,
) {
    match value {
        JsonValue::Object(object) => {
            if let Some(child) = object.get(field) {
                collect_matches(child, remaining_tokens, out);
            }
            for child in object.values() {
                collect_recursive_field_matches(child, field, remaining_tokens, out);
            }
        }
        JsonValue::Array(array) => {
            for child in array {
                collect_recursive_field_matches(child, field, remaining_tokens, out);
            }
        }
        _ => {}
    }
}

pub(crate) fn get_matches<'a>(value: &'a JsonValue, path: &JsonPath) -> Vec<&'a JsonValue> {
    let mut out = Vec::new();
    collect_matches(value, &path.tokens, &mut out);
    out
}

pub(crate) fn ensure_path_mut<'a>(
    value: &'a mut JsonValue,
    tokens: &[JsonPathToken],
) -> Result<&'a mut JsonValue, JsonError> {
    if tokens.is_empty() {
        return Ok(value);
    }
    match &tokens[0] {
        JsonPathToken::Field(field) => {
            let field = std::str::from_utf8(field).map_err(|_| JsonError::Syntax)?;
            if !value.is_object() {
                if value.is_null() {
                    *value = JsonValue::Object(Map::new());
                } else {
                    return Err(JsonError::Path);
                }
            }
            let object = value.as_object_mut().ok_or(JsonError::Path)?;
            let child = object.entry(field.to_owned()).or_insert(JsonValue::Null);
            ensure_path_mut(child, &tokens[1..])
        }
        JsonPathToken::Index(index) => {
            let array = value.as_array_mut().ok_or(JsonError::Path)?;
            let index = normalize_index(array.len(), *index).ok_or(JsonError::Path)?;
            let child = array.get_mut(index).ok_or(JsonError::Path)?;
            ensure_path_mut(child, &tokens[1..])
        }
        JsonPathToken::RecursiveField(_) | JsonPathToken::Wildcard => Err(JsonError::Path),
    }
}

pub(crate) fn get_mut_exact<'a>(
    value: &'a mut JsonValue,
    tokens: &[JsonPathToken],
) -> Result<Option<&'a mut JsonValue>, JsonError> {
    if tokens.is_empty() {
        return Ok(Some(value));
    }
    match &tokens[0] {
        JsonPathToken::Field(field) => {
            let JsonValue::Object(object) = value else {
                return Ok(None);
            };
            let Ok(field) = std::str::from_utf8(field) else {
                return Err(JsonError::Syntax);
            };
            let Some(child) = object.get_mut(field) else {
                return Ok(None);
            };
            get_mut_exact(child, &tokens[1..])
        }
        JsonPathToken::Index(index) => {
            let JsonValue::Array(array) = value else {
                return Ok(None);
            };
            let Some(index) = normalize_index(array.len(), *index) else {
                return Ok(None);
            };
            let Some(child) = array.get_mut(index) else {
                return Ok(None);
            };
            get_mut_exact(child, &tokens[1..])
        }
        JsonPathToken::RecursiveField(_) | JsonPathToken::Wildcard => Err(JsonError::Path),
    }
}

pub(crate) fn delete_exact(
    value: &mut JsonValue,
    tokens: &[JsonPathToken],
) -> Result<bool, JsonError> {
    if tokens.is_empty() {
        return Ok(false);
    }
    if tokens.len() == 1 {
        match &tokens[0] {
            JsonPathToken::Field(field) => {
                let JsonValue::Object(object) = value else {
                    return Ok(false);
                };
                let Ok(field) = std::str::from_utf8(field) else {
                    return Err(JsonError::Syntax);
                };
                return Ok(object.remove(field).is_some());
            }
            JsonPathToken::Index(index) => {
                let JsonValue::Array(array) = value else {
                    return Ok(false);
                };
                let Some(index) = normalize_index(array.len(), *index) else {
                    return Ok(false);
                };
                array.remove(index);
                return Ok(true);
            }
            JsonPathToken::RecursiveField(_) | JsonPathToken::Wildcard => {
                return Err(JsonError::Path)
            }
        }
    }
    match &tokens[0] {
        JsonPathToken::Field(field) => {
            let JsonValue::Object(object) = value else {
                return Ok(false);
            };
            let Ok(field) = std::str::from_utf8(field) else {
                return Err(JsonError::Syntax);
            };
            let Some(child) = object.get_mut(field) else {
                return Ok(false);
            };
            delete_exact(child, &tokens[1..])
        }
        JsonPathToken::Index(index) => {
            let JsonValue::Array(array) = value else {
                return Ok(false);
            };
            let Some(index) = normalize_index(array.len(), *index) else {
                return Ok(false);
            };
            let Some(child) = array.get_mut(index) else {
                return Ok(false);
            };
            delete_exact(child, &tokens[1..])
        }
        JsonPathToken::RecursiveField(_) | JsonPathToken::Wildcard => Err(JsonError::Path),
    }
}

pub(crate) fn clear_value(value: &mut JsonValue) -> bool {
    match value {
        JsonValue::Array(array) => {
            let changed = !array.is_empty();
            array.clear();
            changed
        }
        JsonValue::Object(object) => {
            let changed = !object.is_empty();
            object.clear();
            changed
        }
        JsonValue::Number(_) => {
            *value = JsonValue::Number(Number::from(0));
            true
        }
        _ => false,
    }
}

pub(crate) fn merge_value(target: &mut JsonValue, patch: JsonValue) {
    match (target, patch) {
        (JsonValue::Object(target), JsonValue::Object(patch)) => {
            for (key, value) in patch {
                if value.is_null() {
                    target.remove(&key);
                } else if let Some(current) = target.get_mut(&key) {
                    merge_value(current, value);
                } else {
                    target.insert(key, value);
                }
            }
        }
        (target, patch) => *target = patch,
    }
}
