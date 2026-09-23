use super::constants::{SST_FOOTER_MAGIC, SST_HEADER_MAGIC, SST_VERSION};
use crate::stream::value::Value;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

pub struct SSTWriter {
    values: HashMap<u32, Vec<Value>>,
}

impl SSTWriter {
    pub fn new() -> Self {
        SSTWriter {
            values: HashMap::new(),
        }
    }

    // TODO better error type
    pub fn insert(&mut self, stream_id: u32, value: Value) -> Result<(), io::Error> {
        let values = self.values.entry(stream_id).or_insert_with(Vec::new);
        if let Some(last_val) = values.last() {
            if value.timestamp < last_val.timestamp {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "StreamId {} last timestamp is {}. New write cannot write older timestamp {}.",
                        stream_id, last_val.timestamp, value.timestamp
                    ),
                ));
            }
        }

        values.push(value);
        Ok(())
    }

    pub fn flush(&self, path: &PathBuf) -> Result<(), io::Error> {
        let file = OpenOptions::new().write(true).create_new(true).open(path)?;
        let mut writer = BufWriter::new(file);

        self.write_header(&mut writer)?;
        // TODO write other parts of SST
        self.write_footer(&mut writer)?;

        writer.flush()?;
        Ok(())
    }

    fn write_header(&self, file: &mut BufWriter<File>) -> Result<(), io::Error> {
        file.write_all(&SST_HEADER_MAGIC)?;
        file.write_all(&SST_VERSION)?;
        Ok(())
    }

    fn write_footer(&self, file: &mut BufWriter<File>) -> Result<(), io::Error> {
        let mock_min_timestamp: u64 = 123;
        let mock_max_timestamp: u64 = 345;

        file.write_all(&mock_min_timestamp.to_be_bytes())?;
        file.write_all(&mock_max_timestamp.to_be_bytes())?;
        file.write_all(&SST_FOOTER_MAGIC)?;
        Ok(())
    }
}
