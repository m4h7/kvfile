use swriter::SimpleWriter;
use std::io::Result;

pub struct KVFileWriter {
    w: SimpleWriter
}

impl KVFileWriter {

    pub fn create_file(path: &str) -> Result<KVFileWriter> {
        SimpleWriter::create_file(path)
            .map(|w| KVFileWriter { w: w })
    }

    pub fn put(self: &mut KVFileWriter, key: &[u8], value: &[u8]) {
        self.w.align_position(4);
        if !self.w.le32(key.len() as u32) {
            panic!("error writing key len");
        }
        if !self.w.le32(value.len() as u32) {
            panic!("error writing value len");
        }
        if !self.w.varbytes(key) {
            panic!("error writing key");
        }
        if !self.w.varbytes(value) {
            panic!("error writing value");
        }
    }
}
