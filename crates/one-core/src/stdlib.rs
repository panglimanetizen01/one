use crate::runtime::Value;

pub fn display_value(value: &Value) -> String { value.display() }

pub fn list_append(mut list: Value, value: Value) -> Option<Value> {
    if let Value::List(values) = &mut list { values.push(value); Some(list) } else { None }
}

pub fn list_remove(mut list: Value, value: &Value) -> Option<Value> {
    if let Value::List(values) = &mut list { if let Some(index) = values.iter().position(|item| item == value) { values.remove(index); } Some(list) } else { None }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display_value_uses_runtime_display_for_each_value_kind() {
        assert_eq!(display_value(&Value::Text("halo".into())), "halo");
        assert_eq!(display_value(&Value::Number(42)), "42");
        assert_eq!(display_value(&Value::Boolean(true)), "benar");
        assert_eq!(display_value(&Value::Boolean(false)), "salah");
        assert_eq!(display_value(&Value::List(vec![Value::Number(1), Value::Text("x".into())])), "[1, x]");
        assert_eq!(display_value(&Value::List(vec![])), "[]");
        assert_eq!(display_value(&Value::Nothing), "tidak ada");
    }

    #[test]
    fn list_append_appends_exactly_one_value_and_preserves_existing_order() {
        let result = list_append(
            Value::List(vec![Value::Number(1), Value::Number(2)]),
            Value::Number(3),
        );
        assert_eq!(
            result,
            Some(Value::List(vec![Value::Number(1), Value::Number(2), Value::Number(3)]))
        );
    }

    #[test]
    fn list_append_supports_an_empty_list() {
        assert_eq!(
            list_append(Value::List(vec![]), Value::Text("awal".into())),
            Some(Value::List(vec![Value::Text("awal".into())]))
        );
    }

    #[test]
    fn list_append_returns_none_for_non_list_input() {
        assert_eq!(list_append(Value::Number(1), Value::Number(2)), None);
    }

    #[test]
    fn list_remove_removes_only_the_first_equal_value() {
        let result = list_remove(
            Value::List(vec![Value::Number(1), Value::Number(2), Value::Number(2), Value::Number(3)]),
            &Value::Number(2),
        );
        assert_eq!(
            result,
            Some(Value::List(vec![Value::Number(1), Value::Number(2), Value::Number(3)]))
        );
    }

    #[test]
    fn list_remove_preserves_list_when_value_is_absent() {
        let input = Value::List(vec![Value::Number(1), Value::Number(3)]);
        assert_eq!(
            list_remove(input.clone(), &Value::Number(2)),
            Some(input)
        );
    }

    #[test]
    fn list_remove_supports_an_empty_list() {
        assert_eq!(
            list_remove(Value::List(vec![]), &Value::Number(1)),
            Some(Value::List(vec![]))
        );
    }

    #[test]
    fn list_remove_returns_none_for_non_list_input() {
        assert_eq!(list_remove(Value::Number(1), &Value::Number(1)), None);
    }
}
