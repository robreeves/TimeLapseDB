mod storage;

use crate::storage::sst_writer::SSTWriter;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let writer = SSTWriter::new();

    let sst_path = PathBuf::from("./test.sst");
    writer.write(&sst_path)?;

    Ok(())
}
