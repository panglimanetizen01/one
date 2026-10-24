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

/// Checked signed-integer remainder. Zero divisors and i64::MIN % -1 are rejected.
pub fn number_modulo(left: &Value, right: &Value) -> Option<Value> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.checked_rem(*right).map(Value::Number),
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

/// Maximum serialized JSON output, in UTF-8 bytes.
const MAX_JSON_OUTPUT_BYTES: usize = 16 * 1024 * 1024;

/// Append text only while keeping the complete JSON output within its byte limit.
fn push_json_bounded(output: &mut String, fragment: &str) -> Option<()> {
    if output.len().checked_add(fragment.len())? > MAX_JSON_OUTPUT_BYTES {
        return None;
    }
    output.push_str(fragment);
    Some(())
}

/// Append a JSON string with all required escapes, enforcing the output byte limit.
fn quote_json_bounded(text: &str, output: &mut String) -> Option<()> {
    push_json_bounded(output, "\"")?;
    for character in text.chars() {
        match character {
            '"' => push_json_bounded(output, "\\\"")?,
            '\\' => push_json_bounded(output, "\\\\")?,
            '\u{0008}' => push_json_bounded(output, "\\b")?,
            '\u{000c}' => push_json_bounded(output, "\\f")?,
            '\n' => push_json_bounded(output, "\\n")?,
            '\r' => push_json_bounded(output, "\\r")?,
            '\t' => push_json_bounded(output, "\\t")?,
            character if character <= '\u{001f}' => {
                let escaped = format!("\\u{:04x}", u32::from(character));
                push_json_bounded(output, &escaped)?;
            }
            character => {
                let mut encoded = [0_u8; 4];
                push_json_bounded(output, character.encode_utf8(&mut encoded))?;
            }
        }
    }
    push_json_bounded(output, "\"")
}

/// Serialize a runtime value as deterministic JSON. Nesting deeper than 128 levels or output
/// larger than 16 MiB is rejected. Map keys are emitted in deterministic sorted order.
pub fn value_to_json(value: &Value) -> Option<Value> {
    fn encode(value: &Value, depth: usize, output: &mut String) -> Option<()> {
        if depth > 128 {
            return None;
        }
        match value {
            Value::Text(text) => quote_json_bounded(text, output),
            Value::Number(number) => push_json_bounded(output, &number.to_string()),
            Value::Boolean(true) => push_json_bounded(output, "true"),
            Value::Boolean(false) => push_json_bounded(output, "false"),
            Value::Nothing => push_json_bounded(output, "null"),
            Value::List(values) => {
                push_json_bounded(output, "[")?;
                for (index, value) in values.iter().enumerate() {
                    if index > 0 {
                        push_json_bounded(output, ",")?;
                    }
                    encode(value, depth + 1, output)?;
                }
                push_json_bounded(output, "]")
            }
            Value::Map(values) => {
                push_json_bounded(output, "{")?;
                for (index, (key, value)) in values.iter().enumerate() {
                    if index > 0 {
                        push_json_bounded(output, ",")?;
                    }
                    quote_json_bounded(key, output)?;
                    push_json_bounded(output, ":")?;
                    encode(value, depth + 1, output)?;
                }
                push_json_bounded(output, "}")
            }
        }
    }

    let mut output = String::new();
    encode(value, 0, &mut output)?;
    Some(Value::Text(output))
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

struct JsonParser<'a> {
    source: &'a str,
    position: usize,
}

impl JsonParser<'_> {
    fn parse(source: &str) -> Option<Value> {
        let mut parser = JsonParser { source, position: 0 };
        let value = parser.parse_value(0)?;
        parser.skip_whitespace();
        (parser.position == source.len()).then_some(value)
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.position..)?.chars().next()
    }

    fn next(&mut self) -> Option<char> {
        let value = self.peek()?;
        self.position += value.len_utf8();
        Some(value)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\r' | '\n')) {
            let _ = self.next();
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
            self.next()?;
        }
        match self.peek()? {
            '0' => {
                self.next()?;
                if matches!(self.peek(), Some('0'..='9')) {
                    return None;
                }
            }
            '1'..='9' => {
                self.next()?;
                while matches!(self.peek(), Some('0'..='9')) {
                    self.next()?;
                }
            }
            _ => return None,
        }
        if matches!(self.peek(), Some('.' | 'e' | 'E')) {
            return None;
        }
        self.source.get(start..self.position)?.parse::<i64>().ok().map(Value::Number)
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
            self.next()?;
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
            self.next()?;
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

/// Format Unix epoch seconds as a Gregorian UTC timestamp (YYYY-MM-DDTHH:MM:SSZ).
/// Years outside 0000..=9999 and non-integer inputs are rejected.
pub fn unix_seconds_to_utc_text(value: &Value) -> Option<Value> {
    let Value::Number(seconds) = value else {
        return None;
    };

    const SECONDS_PER_DAY: i64 = 86_400;
    let days = seconds.div_euclid(SECONDS_PER_DAY);
    let second_of_day = seconds.rem_euclid(SECONDS_PER_DAY);
    let shifted_days = days.checked_add(719_468)?;
    // The negative branch biases the value before truncating division,
    // matching the Gregorian civil-from-days algorithm for year zero and earlier.
    let era = (if shifted_days >= 0 {
        shifted_days
    } else {
        shifted_days.checked_sub(146_096)?
    }) / 146_097;
    let day_of_era = shifted_days.checked_sub(era.checked_mul(146_097)?)?;
    let year_of_era = (day_of_era
        - day_of_era / 1_460
        + day_of_era / 36_524
        - day_of_era / 146_096)
        / 365;
    let mut year = year_of_era.checked_add(era.checked_mul(400)?)?;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += if month <= 2 { 1 } else { 0 };

    if !(0..=9_999).contains(&year) {
        return None;
    }

    let hour = second_of_day / 3_600;
    let minute = (second_of_day % 3_600) / 60;
    let second = second_of_day % 60;
    Some(Value::Text(format!(
        "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z"
    )))
}

/// Parse a strict UTC timestamp (YYYY-MM-DDTHH:MM:SSZ) into Unix epoch seconds.
/// The accepted calendar is proleptic Gregorian with years 0000 through 9999.
/// Offsets, fractional seconds, leap seconds, and non-canonical forms are rejected.
pub fn utc_text_to_unix_seconds(value: &Value) -> Option<Value> {
    let Value::Text(text) = value else {
        return None;
    };
    let bytes = text.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return None;
    }

    let parse_digits = |start: usize, end: usize| -> Option<i64> {
        bytes.get(start..end)?.iter().try_fold(0_i64, |value, byte| {
            if byte.is_ascii_digit() {
                value.checked_mul(10)?.checked_add((*byte - b'0') as i64)
            } else {
                None
            }
        })
    };
    let year = parse_digits(0, 4)?;
    let month = parse_digits(5, 7)?;
    let day = parse_digits(8, 10)?;
    let hour = parse_digits(11, 13)?;
    let minute = parse_digits(14, 16)?;
    let second = parse_digits(17, 19)?;

    if !(1..=12).contains(&month)
        || !(0..=23).contains(&hour)
        || !(0..=59).contains(&minute)
        || !(0..=59).contains(&second)
    {
        return None;
    }
    let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => return None,
    };
    if !(1..=days_in_month).contains(&day) {
        return None;
    }

    // Howard Hinnant's civil-from-days inverse, using Euclidean division so
    // year 0000 and dates before 1970 remain correct.
    let adjusted_year = year - if month <= 2 { 1 } else { 0 };
    let era = adjusted_year.div_euclid(400);
    let year_of_era = adjusted_year - era * 400;
    let adjusted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days_since_epoch = era * 146_097 + day_of_era - 719_468;
    let seconds_of_day = hour * 3_600 + minute * 60 + second;
    let unix_seconds = days_since_epoch
        .checked_mul(86_400)?
        .checked_add(seconds_of_day)?;
    Some(Value::Number(unix_seconds))
}

/// Construct a deterministic text-keyed mapping.
pub fn map_from_entries(entries: impl IntoIterator<Item = (String, Value)>) -> Value {
    Value::Map(entries.into_iter().collect::<BTreeMap<_, _>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_seconds_format_as_valid_bounded_gregorian_utc_timestamps() {
        assert_eq!(
            unix_seconds_to_utc_text(&Value::Number(0)),
            Some(Value::Text("1970-01-01T00:00:00Z".into()))
        );
        assert_eq!(
            unix_seconds_to_utc_text(&Value::Number(-1)),
            Some(Value::Text("1969-12-31T23:59:59Z".into()))
        );
        assert_eq!(
            unix_seconds_to_utc_text(&Value::Number(-62_167_219_200)),
            Some(Value::Text("0000-01-01T00:00:00Z".into()))
        );
        assert_eq!(
            unix_seconds_to_utc_text(&Value::Number(951_782_400)),
            Some(Value::Text("2000-02-29T00:00:00Z".into()))
        );
        assert_eq!(
            unix_seconds_to_utc_text(&Value::Number(1_709_164_800)),
            Some(Value::Text("2024-02-29T00:00:00Z".into()))
        );
        assert_eq!(unix_seconds_to_utc_text(&Value::Text("0".into())), None);
        assert_eq!(
            unix_seconds_to_utc_text(&Value::Number(253_402_300_799)),
            Some(Value::Text("9999-12-31T23:59:59Z".into()))
        );
        assert_eq!(unix_seconds_to_utc_text(&Value::Number(253_402_300_800)), None);
    }

    #[test]
    fn utc_text_parses_strict_bounded_gregorian_timestamps() {
        assert_eq!(
            utc_text_to_unix_seconds(&Value::Text("1970-01-01T00:00:00Z".into())),
            Some(Value::Number(0))
        );
        assert_eq!(
            utc_text_to_unix_seconds(&Value::Text("1969-12-31T23:59:59Z".into())),
            Some(Value::Number(-1))
        );
        assert_eq!(
            utc_text_to_unix_seconds(&Value::Text("0000-01-01T00:00:00Z".into())),
            Some(Value::Number(-62_167_219_200))
        );
        assert_eq!(
            utc_text_to_unix_seconds(&Value::Text("2000-02-29T00:00:00Z".into())),
            Some(Value::Number(951_782_400))
        );
        assert_eq!(
            utc_text_to_unix_seconds(&Value::Text("9999-12-31T23:59:59Z".into())),
            Some(Value::Number(253_402_300_799))
        );

        for invalid in [
            "2023-02-29T00:00:00Z",
            "2024-13-01T00:00:00Z",
            "2024-01-00T00:00:00Z",
            "2024-01-01T24:00:00Z",
            "2024-01-01T00:60:00Z",
            "2024-01-01T00:00:60Z",
            "2024-01-01T00:00:00+00:00",
            "2024-01-01T00:00:00z",
            "2024-01-01T00:00:00.1Z",
        ] {
            assert_eq!(
                utc_text_to_unix_seconds(&Value::Text(invalid.into())),
                None,
                "invalid UTC timestamp accepted: {invalid}"
            );
        }
        assert_eq!(utc_text_to_unix_seconds(&Value::Number(0)), None);
    }

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
    fn json_serialization_escapes_every_json_control_class() {
        let value = Value::Text("\0\u{0008}\u{000c}\n\r\t\u{001f}".into());
        assert_eq!(
            value_to_json(&value),
            Some(Value::Text(r#""\u0000\b\f\n\r\t\u001f""#.into()))
        );
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
    fn json_serializer_enforces_the_utf8_output_byte_limit() {
        let exact_limit = Value::Text("x".repeat(MAX_JSON_OUTPUT_BYTES - 2));
        let serialized = value_to_json(&exact_limit).expect("JSON exactly at the output limit should serialize");
        let Value::Text(serialized) = serialized else { panic!("serializer must return text"); };
        assert_eq!(serialized.len(), MAX_JSON_OUTPUT_BYTES);
        drop(serialized);

        let over_limit = Value::Text("x".repeat(MAX_JSON_OUTPUT_BYTES - 1));
        assert_eq!(value_to_json(&over_limit), None);
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
        assert_eq!(number_modulo(&Value::Number(7), &Value::Number(3)), Some(Value::Number(1)));
        assert_eq!(number_modulo(&Value::Number(-7), &Value::Number(3)), Some(Value::Number(-1)));
        assert_eq!(number_modulo(&Value::Number(7), &Value::Number(0)), None);
        assert_eq!(number_modulo(&Value::Number(i64::MIN), &Value::Number(-1)), None);
        assert_eq!(number_modulo(&Value::Text("7".into()), &Value::Number(3)), None);
        assert_eq!(number_min(&Value::Number(4), &Value::Number(-2)), Some(Value::Number(-2)));
        assert_eq!(number_max(&Value::Number(4), &Value::Number(-2)), Some(Value::Number(4)));
        assert_eq!(number_max(&Value::Text("4".into()), &Value::Number(2)), None);
    }

    #[test]
    fn json_parser_handles_raw_unicode_and_integer_boundaries_without_byte_cursor_errors() {
        assert_eq!(
            json_to_value(&Value::Text("\"猫😀\"".into())),
            Some(Value::Text("猫😀".into()))
        );
        assert_eq!(
            json_to_value(&Value::Text(i64::MIN.to_string())),
            Some(Value::Number(i64::MIN))
        );
        assert_eq!(
            json_to_value(&Value::Text(i64::MAX.to_string())),
            Some(Value::Number(i64::MAX))
        );
        for invalid in ["-", "01", "-01", "9223372036854775808", "-9223372036854775809", "1e2", "1.0"] {
            assert_eq!(json_to_value(&Value::Text(invalid.into())), None, "invalid integer form: {invalid}");
        }
    }

    #[test]
    fn json_parser_accepts_valid_input_exactly_at_the_byte_limit() {
        let padding = " ".repeat(MAX_JSON_INPUT_BYTES - 4);
        let source = Value::Text(format!("null{padding}"));
        let Value::Text(source_text) = &source else { panic!("expected text"); };
        assert_eq!(source_text.len(), MAX_JSON_INPUT_BYTES);
        assert_eq!(json_to_value(&source), Some(Value::Nothing));
    }

    #[test]
    fn json_parser_rejects_valid_json_one_byte_over_the_limit() {
        // This would parse successfully without the size guard; whitespace after null is valid JSON.
        let padding = " ".repeat(MAX_JSON_INPUT_BYTES - 3);
        let source = Value::Text(format!("null{padding}"));
        let Value::Text(source_text) = &source else { panic!("expected text"); };
        assert_eq!(source_text.len(), MAX_JSON_INPUT_BYTES + 1);
        assert_eq!(json_to_value(&source), None);
    }

    #[test]
    fn json_parser_limits_input_by_utf8_bytes_not_character_count() {
        // The character count is below the limit, but the UTF-8 byte count is above it.
        let source = Value::Text(format!("\"{}\"", "é".repeat(MAX_JSON_INPUT_BYTES / 2)));
        let Value::Text(source_text) = &source else { panic!("expected text"); };
        assert!(source_text.chars().count() < MAX_JSON_INPUT_BYTES);
        assert!(source_text.len() > MAX_JSON_INPUT_BYTES);
        assert_eq!(json_to_value(&source), None);
    }

    #[test]
    fn json_parser_rejects_empty_and_excessively_nested_input() {
        assert_eq!(json_to_value(&Value::Text(String::new())), None);
        let nested = format!("{}0{}", "[".repeat(129), "]".repeat(129));
        assert_eq!(json_to_value(&Value::Text(nested)), None);
    }

    #[test]
    fn json_parser_rejects_malformed_string_escapes_and_container_syntax() {
        // JSON strings must escape control characters and use only the RFC-defined escapes.
        for invalid in [
            "\\\"raw\\nline\\\"",
            "\\\"bad\\qescape\\\"",
            "\\\"trailing slash\\\\",
            "\\\"\\uDC00\\\"",
            "\\\"\\uD800\\u0041\\\"",
            "[1,]",
            "{\\\"a\\\":1,}",
            "{a:1}",
            "[1] trailing",
            "01",
            "+1",
            "--1",
        ] {
            assert_eq!(
                json_to_value(&Value::Text(invalid.into())),
                None,
                "malformed JSON must be rejected: {invalid:?}"
            );
        }

        // Negative zero is valid JSON and has the same integer value in One.
        assert_eq!(
            json_to_value(&Value::Text("-0".into())),
            Some(Value::Number(0))
        );
    }

}
