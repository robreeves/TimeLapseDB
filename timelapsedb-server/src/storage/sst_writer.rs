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
        // TODO write SST to disk
        self.write_header()?;
        self.write_footer()?;
        Ok(())
    }

    fn write_header(&self) -> Result<(), io::Error> {
        // TODO
        Ok(())
    }

    fn write_footer(&self) -> Result<(), io::Error> {
        // TODO
        Ok(())
    }
}
