use super::constants::{SST_FOOTER_MAGIC, SST_HEADER_MAGIC, SST_VERSION};
use std::fmt;
use std::io::{self, BufReader, Read};
use std::{fs::File, path::PathBuf};

pub struct SSTReader {
    path: PathBuf,
    reader: BufReader<File>,
    version: u8,
    min_timestamp: u64,
    max_timestamp: u64,
    index_offset: u32,
    index_length: u32,
}

impl SSTReader {
    pub fn new(path: &PathBuf) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let mut sst_reader = SSTReader {
            path: path.clone(),
            reader: BufReader::new(file),
            version: 0,
            min_timestamp: 0,
            max_timestamp: 0,
            index_offset: 0,
            index_length: 0,
        };
        sst_reader.initialize()?;
        Ok(sst_reader)
    }

    fn initialize(&mut self) -> Result<(), io::Error> {
        self.read_header()?;
        self.read_footer()?;
        // TODO set struct fields
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

        Ok(())
    }

    fn read_footer(&mut self) -> Result<(), io::Error> {
        let mut min_timestamp_buf = [0u8; 8];
        self.reader.read_exact(&mut min_timestamp_buf)?;
        self.min_timestamp = u64::from_be_bytes(min_timestamp_buf);

        let mut max_timestamp_buf = [0u8; 8];
        self.reader.read_exact(&mut max_timestamp_buf)?;
        self.max_timestamp = u64::from_be_bytes(max_timestamp_buf);

        let mut magic = [0u8; SST_FOOTER_MAGIC.len()];
        self.reader.read_exact(&mut magic)?;
        if magic != SST_FOOTER_MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Incorrect footer magic in '{}'. Found '{}'",
                    self.path.display(),
                    String::from_utf8_lossy(&magic)
                ),
            ));
        }

        Ok(())
    }
}

impl fmt::Display for SSTReader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SSTReader(path={}, version={}, min_timestamp={}, max_timestamp={})",
            self.path.display(),
            self.version,
            self.min_timestamp,
            self.max_timestamp
        )
    }
}
