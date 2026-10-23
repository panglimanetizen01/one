#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Text,
    Number,
    Boolean,
    List,
    Map,
    Nothing,
    Any,
}

impl Type {
    pub fn name(self) -> &'static str {
        match self {
            Type::Text => "teks",
            Type::Number => "angka",
            Type::Boolean => "boolean",
            Type::List => "daftar",
            Type::Map => "pemetaan",
            Type::Nothing => "kosong",
            Type::Any => "apa saja",
        }
    }
}
