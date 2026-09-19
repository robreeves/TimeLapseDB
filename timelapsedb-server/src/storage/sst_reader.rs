use std::io::{self, BufReader};
use std::{fs::File, path::PathBuf};
use super::constants::{HEADER_MAGIC, FOOTER_MAGIC, VERSION};

pub struct SSTReader {
    reader: BufReader<File>,
    version: u8,
}

impl SSTReader {
    pub fn new(path: &PathBuf) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let sst_reader = SSTReader {
            reader: BufReader::new(file),
            version: 0
        };
        sst_reader.initialize()?;
        Ok(sst_reader)
    }

    fn initialize(&self) -> Result<(), io::Error> {
        // TODO set struct fields
        Ok(())
    }

    fn read_header(&self) -> Result<(), io::Error> {
        let mut magic = [0u8; HEADER_MAGIC.len()];
        //TODO

        Ok(())
    }
}