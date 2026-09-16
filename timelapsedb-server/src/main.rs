mod storage;

use crate::storage::sst_writer::SSTWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let writer = SSTWriter::new();
    writer.write()?;
    Ok(())
}
