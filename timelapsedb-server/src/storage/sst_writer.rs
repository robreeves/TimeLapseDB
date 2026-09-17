use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;

const HEADER_MAGIC: [u8; 12] = *b"_TLDB-START_";
const FOOTER_MAGIC: [u8; 10] = *b"_TLDB-END_";

pub struct SSTWriter {}

impl SSTWriter {
    pub fn new() -> Self {
        SSTWriter {}
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), io::Error> {
        let file = OpenOptions::new().write(true).create_new(true).open(path)?;

        // TODO write SST to disk
        self.write_header(&file)?;
        self.write_footer(&file)?;
        Ok(())
    }

    fn write_header(&self, file: &File) -> Result<(), io::Error> {
        // TODO
        Ok(())
    }

    fn write_footer(&self, file: &File) -> Result<(), io::Error> {
        // TODO
        Ok(())
    }
}
