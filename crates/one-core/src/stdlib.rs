use std::collections::BTreeMap;

use crate::runtime::Value;

/// Render a runtime value using One's stable, human-readable representation.
pub fn display_value(value: &Value) -> String {
    value.display()
}

/// Append a value to a list. Returns None when the input is not a list.
pub fn list_append(mut list: Value, value: Value) -> Option<Value> {
    if let Value::List(values) = &mut list {
        values.push(value);
        Some(list)
    } else {
        None
    }
}

/// Remove the first equal value from a list. A missing value is not an error.
pub fn list_remove(mut list: Value, value: &Value) -> Option<Value> {
    if let Value::List(values) = &mut list {
        if let Some(index) = values.iter().position(|item| item == value) {
            values.remove(index);
        }
        Some(list)
    } else {
        None
    }
}

/// Number of Unicode scalar values in text (not UTF-8 bytes or grapheme clusters).
pub fn text_length(value: &Value) -> Option<Value> {
    match value {
        Value::Text(text) => i64::try_from(text.chars().count()).ok().map(Value::Number),
        _ => None,
    }
}

/// Return whether text contains the given literal substring.
pub fn text_contains(value: &Value, needle: &Value) -> Option<Value> {
    match (value, needle) {
        (Value::Text(text), Value::Text(needle)) => Some(Value::Boolean(text.contains(needle))),
        _ => None,
    }
}

/// Trim Unicode whitespace from both ends of a text value.
pub fn text_trim(value: &Value) -> Option<Value> {
    match value {
        Value::Text(text) => Some(Value::Text(text.trim().to_owned())),
        _ => None,
    }
}

/// Replace all literal, non-empty occurrences of a substring.
pub fn text_replace(value: &Value, from: &Value, to: &Value) -> Option<Value> {
    match (value, from, to) {
        (Value::Text(text), Value::Text(from), Value::Text(to)) if !from.is_empty() => {
            Some(Value::Text(text.replace(from, to)))
        }
        _ => None,
    }
}

/// Split text on a literal non-empty separator. Empty input yields one empty field.
pub fn text_split(value: &Value, separator: &Value) -> Option<Value> {
    match (value, separator) {
        (Value::Text(text), Value::Text(separator)) if !separator.is_empty() => Some(
            Value::List(
                text.split(separator)
                    .map(|part| Value::Text(part.to_owned()))
                    .collect(),
            ),
        ),
        _ => None,
    }
}

/// Parse a base-10 signed 64-bit integer without accepting surrounding whitespace.
pub fn text_to_integer(value: &Value) -> Option<Value> {
    match value {
        Value::Text(text) => text.parse::<i64>().ok().map(Value::Number),
        _ => None,
    }
}

/// Return a list's element count.
pub fn list_length(value: &Value) -> Option<Value> {
    match value {
        Value::List(values) => i64::try_from(values.len()).ok().map(Value::Number),
        _ => None,
    }
}

/// Read a list element by zero-based index. Negative and out-of-range indices fail.
pub fn list_get(value: &Value, index: &Value) -> Option<Value> {
    match (value, index) {
        (Value::List(values), Value::Number(index)) => {
            usize::try_from(*index).ok().and_then(|index| values.get(index).cloned())
        }
        _ => None,
    }
}

/// Read a text-keyed mapping without mutating it.
pub fn map_get(value: &Value, key: &Value) -> Option<Value> {
    match (value, key) {
        (Value::Map(values), Value::Text(key)) => values.get(key).cloned(),
        _ => None,
    }
}

/// Insert or replace a text-keyed mapping entry, returning the updated mapping.
pub fn map_set(mut value: Value, key: &Value, entry: Value) -> Option<Value> {
    match (&mut value, key) {
        (Value::Map(values), Value::Text(key)) => {
            values.insert(key.clone(), entry);
            Some(value)
        }
        _ => None,
    }
}

/// Return the absolute value unless it cannot be represented by i64.
pub fn number_abs(value: &Value) -> Option<Value> {
    match value {
        Value::Number(number) => number.checked_abs().map(Value::Number),
        _ => None,
    }
}

/// Checked integer division; zero and i64::MIN / -1 are rejected.
pub fn number_divide(left: &Value, right: &Value) -> Option<Value> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.checked_div(*right).map(Value::Number),
        _ => None,
    }
}

/// Return the smaller integer.
pub fn number_min(left: &Value, right: &Value) -> Option<Value> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Some(Value::Number((*left).min(*right))),
        _ => None,
    }
}

/// Return the larger integer.
pub fn number_max(left: &Value, right: &Value) -> Option<Value> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => Some(Value::Number((*left).max(*right))),
        _ => None,
    }
}

/// Construct a deterministic text-keyed mapping.
pub fn map_from_entries(entries: impl IntoIterator<Item = (String, Value)>) -> Value {
    Value::Map(entries.into_iter().collect::<BTreeMap<_, _>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_length_counts_unicode_scalar_values_not_bytes() {
        assert_eq!(text_length(&Value::Text("kucing 🐈".into())), Some(Value::Number(8)));
    }

    #[test]
    fn text_operations_are_literal_and_type_checked() {
        let text = Value::Text("  apel,jeruk,apel  ".into());
        assert_eq!(text_trim(&text), Some(Value::Text("apel,jeruk,apel".into())));
        assert_eq!(text_contains(&text, &Value::Text("jeruk".into())), Some(Value::Boolean(true)));
        assert_eq!(text_replace(&text, &Value::Text("apel".into()), &Value::Text("pir".into())), Some(Value::Text("  pir,jeruk,pir  ".into())));
        assert_eq!(text_split(&Value::Text("a,,b".into()), &Value::Text(",".into())), Some(Value::List(vec![Value::Text("a".into()), Value::Text("".into()), Value::Text("b".into())])));
        assert_eq!(text_split(&text, &Value::Text("".into())), None);
        assert_eq!(text_length(&Value::Number(1)), None);
    }

    #[test]
    fn integer_conversion_rejects_whitespace_and_overflow() {
        assert_eq!(text_to_integer(&Value::Text("-42".into())), Some(Value::Number(-42)));
        assert_eq!(text_to_integer(&Value::Text(" 42".into())), None);
        assert_eq!(text_to_integer(&Value::Text("9223372036854775808".into())), None);
    }

    #[test]
    fn list_operations_check_types_and_bounds() {
        let list = Value::List(vec![Value::Text("a".into()), Value::Text("b".into())]);
        assert_eq!(list_length(&list), Some(Value::Number(2)));
        assert_eq!(list_get(&list, &Value::Number(1)), Some(Value::Text("b".into())));
        assert_eq!(list_get(&list, &Value::Number(-1)), None);
        assert_eq!(list_get(&list, &Value::Number(2)), None);
        assert_eq!(list_get(&list, &Value::Text("0".into())), None);
    }

    #[test]
    fn maps_are_deterministic_and_set_replaces_existing_key() {
        let map = map_from_entries([
            ("b".into(), Value::Number(2)),
            ("a".into(), Value::Number(1)),
        ]);
        assert_eq!(map.display(), "{a: 1, b: 2}");
        let updated = map_set(map, &Value::Text("a".into()), Value::Number(3)).unwrap();
        assert_eq!(map_get(&updated, &Value::Text("a".into())), Some(Value::Number(3)));
        assert_eq!(map_get(&updated, &Value::Text("missing".into())), None);
        assert_eq!(map_set(Value::Number(1), &Value::Text("x".into()), Value::Nothing), None);
    }

    #[test]
    fn integer_helpers_reject_overflow_division_by_zero_and_wrong_types() {
        assert_eq!(number_abs(&Value::Number(-7)), Some(Value::Number(7)));
        assert_eq!(number_abs(&Value::Number(i64::MIN)), None);
        assert_eq!(number_divide(&Value::Number(7), &Value::Number(2)), Some(Value::Number(3)));
        assert_eq!(number_divide(&Value::Number(1), &Value::Number(0)), None);
        assert_eq!(number_divide(&Value::Number(i64::MIN), &Value::Number(-1)), None);
        assert_eq!(number_min(&Value::Number(4), &Value::Number(-2)), Some(Value::Number(-2)));
        assert_eq!(number_max(&Value::Number(4), &Value::Number(-2)), Some(Value::Number(4)));
        assert_eq!(number_max(&Value::Text("4".into()), &Value::Number(2)), None);
    }
}
