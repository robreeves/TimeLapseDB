mod storage;

use crate::storage::sst_writer::SSTWriter;
use std::fs::{create_dir_all, remove_file};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let sst_path = PathBuf::from("./temp/test.sst");
    if sst_path.exists() {
        remove_file(&sst_path)?;
    }
    if let Some(parent) = sst_path.parent() {
        create_dir_all(parent)?;
    }

    let writer = SSTWriter::new();
    writer.write(&sst_path)?;
    Ok(())
}
