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


/// Return whether text starts with a literal prefix.
pub fn text_starts_with(value: &Value, prefix: &Value) -> Option<Value> {
    match (value, prefix) {
        (Value::Text(text), Value::Text(prefix)) => Some(Value::Boolean(text.starts_with(prefix))),
        _ => None,
    }
}

/// Return whether text ends with a literal suffix.
pub fn text_ends_with(value: &Value, suffix: &Value) -> Option<Value> {
    match (value, suffix) {
        (Value::Text(text), Value::Text(suffix)) => Some(Value::Boolean(text.ends_with(suffix))),
        _ => None,
    }
}

/// Apply Unicode lowercase mapping.
pub fn text_lowercase(value: &Value) -> Option<Value> {
    match value {
        Value::Text(text) => Some(Value::Text(text.to_lowercase())),
        _ => None,
    }
}

/// Apply Unicode uppercase mapping.
pub fn text_uppercase(value: &Value) -> Option<Value> {
    match value {
        Value::Text(text) => Some(Value::Text(text.to_uppercase())),
        _ => None,
    }
}

/// Join a list of text values with a literal separator.
pub fn list_join_text(value: &Value, separator: &Value) -> Option<Value> {
    match (value, separator) {
        (Value::List(values), Value::Text(separator)) => {
            let parts = values.iter().map(|value| match value {
                Value::Text(text) => Some(text.as_str()),
                _ => None,
            }).collect::<Option<Vec<_>>>()?;
            Some(Value::Text(parts.join(separator)))
        }
        _ => None,
    }
}

/// Sum a list of integers, rejecting non-integer elements and overflow.
pub fn list_sum_numbers(value: &Value) -> Option<Value> {
    match value {
        Value::List(values) => values.iter().try_fold(0_i64, |sum, value| match value {
            Value::Number(number) => sum.checked_add(*number),
            _ => None,
        }).map(Value::Number),
        _ => None,
    }
}

/// Serialize a runtime value as deterministic JSON. Nesting deeper than 128 levels is rejected.
pub fn value_to_json(value: &Value) -> Option<Value> {
    fn quote(text: &str) -> String {
        let mut output = String::from("\"");
        for character in text.chars() {
            match character {
                '"' => output.push_str("\\\""),
                '\\' => output.push_str("\\\\"),
                '\u{0008}' => output.push_str("\\b"),
                '\u{000c}' => output.push_str("\\f"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                character if character <= '\u{001f}' => {
                    output.push_str(&format!("\\u{:04x}", u32::from(character)));
                }
                character => output.push(character),
            }
        }
        output.push('"');
        output
    }

    fn encode(value: &Value, depth: usize) -> Option<String> {
        if depth > 128 {
            return None;
        }
        match value {
            Value::Text(text) => Some(quote(text)),
            Value::Number(number) => Some(number.to_string()),
            Value::Boolean(boolean) => Some(boolean.to_string()),
            Value::Nothing => Some("null".to_owned()),
            Value::List(values) => {
                let encoded = values.iter()
                    .map(|value| encode(value, depth + 1))
                    .collect::<Option<Vec<_>>>()?;
                Some(format!("[{}]", encoded.join(",")))
            }
            Value::Map(values) => {
                let encoded = values.iter()
                    .map(|(key, value)| Some(format!("{}:{}", quote(key), encode(value, depth + 1)?)))
                    .collect::<Option<Vec<_>>>()?;
                Some(format!("{{{}}}", encoded.join(",")))
            }
        }
    }

    encode(value, 0).map(Value::Text)
}

const MAX_JSON_INPUT_BYTES: usize = 16 * 1024 * 1024;

/// Parse JSON containing text, signed 64-bit integers, booleans, null, arrays, and objects.
/// Decimal/exponent numbers are rejected because the runtime has no floating-point value type.
pub fn json_to_value(value: &Value) -> Option<Value> {
    match value {
        Value::Text(source) if source.len() <= MAX_JSON_INPUT_BYTES => JsonParser::parse(source),
        Value::Text(_) => None,
        _ => None,
    }
}

struct JsonParser {
    characters: Vec<char>,
    position: usize,
}

impl JsonParser {
    fn parse(source: &str) -> Option<Value> {
        let mut parser = Self {
            characters: source.chars().collect(),
            position: 0,
        };
        let value = parser.parse_value(0)?;
        parser.skip_whitespace();
        (parser.position == parser.characters.len()).then_some(value)
    }

    fn peek(&self) -> Option<char> {
        self.characters.get(self.position).copied()
    }

    fn next(&mut self) -> Option<char> {
        let value = self.peek()?;
        self.position += 1;
        Some(value)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\r' | '\n')) {
            self.position += 1;
        }
    }

    fn parse_value(&mut self, depth: usize) -> Option<Value> {
        if depth > 128 {
            return None;
        }
        self.skip_whitespace();
        match self.peek()? {
            '"' => self.parse_string().map(Value::Text),
            '[' => self.parse_array(depth),
            '{' => self.parse_object(depth),
            't' => self.parse_literal("true").map(|()| Value::Boolean(true)),
            'f' => self.parse_literal("false").map(|()| Value::Boolean(false)),
            'n' => self.parse_literal("null").map(|()| Value::Nothing),
            '-' | '0'..='9' => self.parse_integer(),
            _ => None,
        }
    }

    fn parse_literal(&mut self, literal: &str) -> Option<()> {
        for expected in literal.chars() {
            if self.next()? != expected {
                return None;
            }
        }
        Some(())
    }

    fn parse_integer(&mut self) -> Option<Value> {
        let start = self.position;
        if self.peek() == Some('-') {
            self.position += 1;
        }
        match self.peek()? {
            '0' => {
                self.position += 1;
                if matches!(self.peek(), Some('0'..='9')) {
                    return None;
                }
            }
            '1'..='9' => {
                self.position += 1;
                while matches!(self.peek(), Some('0'..='9')) {
                    self.position += 1;
                }
            }
            _ => return None,
        }
        if matches!(self.peek(), Some('.' | 'e' | 'E')) {
            return None;
        }
        let literal = self.characters[start..self.position].iter().collect::<String>();
        literal.parse::<i64>().ok().map(Value::Number)
    }

    fn parse_hex_quad(&mut self) -> Option<u16> {
        let mut value = 0_u16;
        for _ in 0..4 {
            let digit = self.next()?.to_digit(16)?;
            value = value.checked_mul(16)?.checked_add(u16::try_from(digit).ok()?)?;
        }
        Some(value)
    }

    fn parse_string(&mut self) -> Option<String> {
        if self.next()? != '"' {
            return None;
        }
        let mut output = String::new();
        loop {
            let character = self.next()?;
            match character {
                '"' => return Some(output),
                '\\' => {
                    let escaped = self.next()?;
                    match escaped {
                        '"' => output.push('"'),
                        '\\' => output.push('\\'),
                        '/' => output.push('/'),
                        'b' => output.push('\u{0008}'),
                        'f' => output.push('\u{000c}'),
                        'n' => output.push('\n'),
                        'r' => output.push('\r'),
                        't' => output.push('\t'),
                        'u' => {
                            let first = self.parse_hex_quad()?;
                            let scalar = if (0xd800..=0xdbff).contains(&first) {
                                if self.next()? != '\\' || self.next()? != 'u' {
                                    return None;
                                }
                                let second = self.parse_hex_quad()?;
                                if !(0xdc00..=0xdfff).contains(&second) {
                                    return None;
                                }
                                0x1_0000
                                    + ((u32::from(first) - 0xd800) << 10)
                                    + (u32::from(second) - 0xdc00)
                            } else if (0xdc00..=0xdfff).contains(&first) {
                                return None;
                            } else {
                                u32::from(first)
                            };
                            output.push(char::from_u32(scalar)?);
                        }
                        _ => return None,
                    }
                }
                character if u32::from(character) <= 0x1f => return None,
                character => output.push(character),
            }
        }
    }

    fn parse_array(&mut self, depth: usize) -> Option<Value> {
        if self.next()? != '[' {
            return None;
        }
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.peek() == Some(']') {
            self.position += 1;
            return Some(Value::List(values));
        }
        loop {
            values.push(self.parse_value(depth + 1)?);
            self.skip_whitespace();
            match self.next()? {
                ']' => return Some(Value::List(values)),
                ',' => self.skip_whitespace(),
                _ => return None,
            }
        }
    }

    fn parse_object(&mut self, depth: usize) -> Option<Value> {
        if self.next()? != '{' {
            return None;
        }
        self.skip_whitespace();
        let mut values = BTreeMap::new();
        if self.peek() == Some('}') {
            self.position += 1;
            return Some(Value::Map(values));
        }
        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            if self.next()? != ':' {
                return None;
            }
            let value = self.parse_value(depth + 1)?;
            if values.insert(key, value).is_some() {
                return None;
            }
            self.skip_whitespace();
            match self.next()? {
                '}' => return Some(Value::Map(values)),
                ',' => self.skip_whitespace(),
                _ => return None,
            }
        }
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
    fn text_case_prefix_suffix_and_list_helpers_are_checked() {
        assert_eq!(text_starts_with(&Value::Text("apel".into()), &Value::Text("ap".into())), Some(Value::Boolean(true)));
        assert_eq!(text_ends_with(&Value::Text("apel".into()), &Value::Text("el".into())), Some(Value::Boolean(true)));
        assert_eq!(text_lowercase(&Value::Text("Ä".into())), Some(Value::Text("ä".into())));
        assert_eq!(text_uppercase(&Value::Text("ß".into())), Some(Value::Text("SS".into())));
        let list = Value::List(vec![Value::Text("a".into()), Value::Text("b".into())]);
        assert_eq!(list_join_text(&list, &Value::Text(" | ".into())), Some(Value::Text("a | b".into())));
        assert_eq!(list_sum_numbers(&Value::List(vec![Value::Number(2), Value::Number(3)])), Some(Value::Number(5)));
        assert_eq!(list_sum_numbers(&Value::List(vec![Value::Number(i64::MAX), Value::Number(1)])), None);
        assert_eq!(list_join_text(&Value::List(vec![Value::Number(1)]), &Value::Text(",".into())), None);
    }

    #[test]
    fn json_serialization_escapes_text_and_orders_mapping_keys() {
        let value = map_from_entries([
            ("z".into(), Value::Text("baris\n\"".into())),
            ("a".into(), Value::List(vec![Value::Number(2), Value::Nothing])),
        ]);
        assert_eq!(
            value_to_json(&value),
            Some(Value::Text(r#"{"a":[2,null],"z":"baris\n\""}"#.into()))
        );
        assert_eq!(value_to_json(&Value::Boolean(true)), Some(Value::Text("true".into())));
    }

    #[test]
    fn json_serialization_rejects_excessive_nesting() {
        let mut value = Value::Nothing;
        for _ in 0..130 {
            value = Value::List(vec![value]);
        }
        assert_eq!(value_to_json(&value), None);
    }

    #[test]
    fn json_parser_round_trips_supported_values_and_rejects_ambiguous_numbers() {
        let source = Value::Text(r#"{"z":"line\n\"","a":[2,true,null]}"#.into());
        let parsed = json_to_value(&source).expect("supported JSON should parse");
        assert_eq!(
            value_to_json(&parsed),
            Some(Value::Text(r#"{"a":[2,true,null],"z":"line\n\""}"#.into()))
        );
        assert_eq!(
            json_to_value(&Value::Text(r#""\uD83D\uDE00""#.into())),
            Some(Value::Text("😀".into()))
        );
        assert_eq!(json_to_value(&Value::Text("[1.5]".into())), None);
        assert_eq!(json_to_value(&Value::Text(r#"{"a":1,"a":2}"#.into())), None);
        assert_eq!(json_to_value(&Value::Text(r#""\uD800""#.into())), None);
        assert_eq!(json_to_value(&Value::Text("true trailing".into())), None);
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

    #[test]
    fn json_parser_accepts_valid_input_exactly_at_the_byte_limit() {
        let padding = " ".repeat(MAX_JSON_INPUT_BYTES - 4);
        let source = Value::Text(format!("null{padding}"));
        assert_eq!(source.as_text().map(str::len), Some(MAX_JSON_INPUT_BYTES));
        assert_eq!(json_to_value(&source), Some(Value::Nothing));
    }

    #[test]
    fn json_parser_rejects_valid_json_one_byte_over_the_limit() {
        // This would parse successfully without the size guard; whitespace after null is valid JSON.
        let padding = " ".repeat(MAX_JSON_INPUT_BYTES - 3);
        let source = Value::Text(format!("null{padding}"));
        assert_eq!(source.as_text().map(str::len), Some(MAX_JSON_INPUT_BYTES + 1));
        assert_eq!(json_to_value(&source), None);
    }

    #[test]
    fn json_parser_limits_input_by_utf8_bytes_not_character_count() {
        // The character count is below the limit, but the UTF-8 byte count is above it.
        let source = Value::Text(format!("\"{}\"", "é".repeat(MAX_JSON_INPUT_BYTES / 2)));
        assert!(source.as_text().unwrap().chars().count() < MAX_JSON_INPUT_BYTES);
        assert!(source.as_text().unwrap().len() > MAX_JSON_INPUT_BYTES);
        assert_eq!(json_to_value(&source), None);
    }

    #[test]
    fn json_parser_rejects_empty_and_excessively_nested_input() {
        assert_eq!(json_to_value(&Value::Text(String::new())), None);
        let nested = format!("{}0{}", "[".repeat(129), "]".repeat(129));
        assert_eq!(json_to_value(&Value::Text(nested)), None);
    }
}
