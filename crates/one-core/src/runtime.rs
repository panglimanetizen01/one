use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    Number(i64),
    Boolean(bool),
    List(Vec<Value>),
    Nothing,
}

impl Value {
    pub fn display(&self) -> String {
        match self {
            Value::Text(v) => v.clone(),
            Value::Number(v) => v.to_string(),
            Value::Boolean(v) => if *v { "benar".into() } else { "salah".into() },
            Value::List(values) => format!("[{}]", values.iter().map(Value::display).collect::<Vec<_>>().join(", ")),
            Value::Nothing => "tidak ada".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Default for Environment { fn default() -> Self { Self::new() } }

impl Environment {
    pub fn new() -> Self { Self { scopes: vec![HashMap::new()] } }
    pub fn push(&mut self) { self.scopes.push(HashMap::new()); }
    pub fn pop(&mut self) { if self.scopes.len() > 1 { self.scopes.pop(); } }
    pub fn define(&mut self, name: impl Into<String>, value: Value) { self.scopes.last_mut().unwrap().insert(name.into(), value); }
    pub fn contains_current(&self, name: &str) -> bool { self.scopes.last().is_some_and(|scope| scope.contains_key(name)) }
    pub fn define_if_absent(&mut self, name: impl Into<String>, value: Value) -> bool {
        let name = name.into();
        let Some(scope) = self.scopes.last_mut() else { return false; };
        if scope.contains_key(&name) { return false; }
        scope.insert(name, value);
        true
    }
    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) { scope.insert(name.to_string(), value); return true; }
        }
        false
    }
    pub fn get(&self, name: &str) -> Option<Value> { self.scopes.iter().rev().find_map(|scope| scope.get(name).cloned()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_environment_has_a_root_scope_and_accepts_definitions() {
        let mut environment = Environment::default();
        environment.define("nilai", Value::Number(7));
        assert_eq!(environment.get("nilai"), Some(Value::Number(7)));
    }
}
