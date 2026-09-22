use crate::stream::data_type::DataType;

pub struct Value {
    timestamp: u64,
    value: DataType,
}

impl Value {
    pub fn new(timestamp: u64, value: DataType) -> Self {
        Value { timestamp, value }
    }
}
