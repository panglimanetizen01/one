use crate::runtime::Value;

pub fn display_value(value: &Value) -> String { value.display() }

pub fn list_append(mut list: Value, value: Value) -> Option<Value> {
    if let Value::List(values) = &mut list { values.push(value); Some(list) } else { None }
}

pub fn list_remove(mut list: Value, value: &Value) -> Option<Value> {
    if let Value::List(values) = &mut list { if let Some(index) = values.iter().position(|item| item == value) { values.remove(index); } Some(list) } else { None }
}
