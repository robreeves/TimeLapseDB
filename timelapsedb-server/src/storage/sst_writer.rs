use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

const HEADER_MAGIC: [u8; 12] = *b"_TLDB-START_";
const FOOTER_MAGIC: [u8; 10] = *b"_TLDB-END_";
const VERSION: [u8; 1] = *b"0";

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
        file.write_all(&HEADER_MAGIC)?;
        file.write_all(&VERSION)?;
        Ok(())
    }

    fn write_footer(&self, file: &mut BufWriter<File>) -> Result<(), io::Error> {
        file.write_all(&FOOTER_MAGIC)?;
        Ok(())
    }
}
