use super::constants::{SST_FOOTER_MAGIC, SST_HEADER_MAGIC, SST_VERSION};
use std::io::{self, BufReader, Error, Read};
use std::{fs::File, path::PathBuf};

pub struct SSTReader {
    path: PathBuf,
    reader: BufReader<File>,
    version: u8,
}

impl SSTReader {
    pub fn new(path: &PathBuf) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let mut sst_reader = SSTReader {
            path: path.clone(),
            reader: BufReader::new(file),
            version: 0,
        };
        sst_reader.initialize()?;
        Ok(sst_reader)
    }

    fn initialize(&mut self) -> Result<(), io::Error> {
        // TODO set struct fields
        self.read_header()?;
        Ok(())
    }

    fn read_header(&mut self) -> Result<(), io::Error> {
        let mut magic = [0u8; SST_HEADER_MAGIC.len()];
        self.reader.read_exact(&mut magic)?;
        if magic != SST_HEADER_MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Incorrect header magic in '{}'. Found '{}'",
                    self.path.display(),
                    String::from_utf8_lossy(&magic)
                ),
            ));
        }

        let mut version = [0u8; SST_VERSION.len()];
        self.reader.read_exact(&mut version)?;
        println!("header version: {}", String::from_utf8_lossy(&version));
        if version != SST_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Incorrect version in '{}'. Found '{}'",
                    self.path.display(),
                    String::from_utf8_lossy(&version)
                ),
            ));
        }

        //TODO validate

        Ok(())
    }
}
