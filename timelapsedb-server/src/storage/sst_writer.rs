use super::constants::{SST_FOOTER_MAGIC, SST_HEADER_MAGIC, SST_VERSION};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

pub struct SSTWriter {}

impl SSTWriter {
    pub fn new() -> Self {
        SSTWriter {}
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), io::Error> {
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
