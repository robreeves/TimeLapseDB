const HEADER_MAGIC: [u8; 12] = *b"_TLDB-START_";
const FOOTER_MAGIC: [u8; 10] = *b"_TLDB-END_";

pub struct SSTWriter {}

impl SSTWriter {
    pub fn new() -> Self {
        SSTWriter {  }
    }
}
