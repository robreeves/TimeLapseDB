pub enum DataType {
    Int(i32),
    Float(f32),
    Bool(bool),
    Text(String),
}

impl DataType {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            DataType::Int(i) => i.to_be_bytes().to_vec(),
            DataType::Bool(b) => vec![*b as u8],
            DataType::Float(f) => f.to_be_bytes().to_vec(),
            DataType::Text(t) => t.as_bytes().to_vec()
        }
    }
}
