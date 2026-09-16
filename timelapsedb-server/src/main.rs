mod storage;

use crate::storage::sst_writer::SSTWriter;

fn main() {
    println!("Hello, world!");

    let writer = SSTWriter::new();
}
