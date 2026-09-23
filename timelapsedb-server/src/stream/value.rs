use crate::stream::data_type::DataType;

pub struct Value {
    pub timestamp: u64,
    pub value: DataType,
}

impl Value {
    pub fn new(timestamp: u64, value: DataType) -> Self {
        Value { timestamp, value }
    }
}
