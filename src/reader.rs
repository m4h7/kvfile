use sreader::SimpleReader;
use std::io::Result;

pub struct KVFileReader {
    c: SimpleReader,
}

impl KVFileReader {

    // create a new KVFileReader from a string
    pub fn open_file(path: &str) -> Result<KVFileReader> {
        let sr = SimpleReader::open_file(path);
        match sr {
            Err(why) => Err(why),
            Ok(reader) => Ok(KVFileReader { c: reader }),
        }
    }

    pub fn key(&self, pos: usize) -> &[u8] {
        let len = self.c.u32le(pos) as usize;
        self.c.varbytes(pos + 4 + 4, len)
    }

    pub fn value(&self, pos: usize) -> &[u8] {
        let keylen = self.c.u32le(pos) as usize;
        let valuelen = self.c.u32le(pos + 4) as usize;
        self.c.varbytes(pos + 4 + 4 + keylen, valuelen)
    }

    pub fn next(&self, pos: usize) -> usize {
        let keylen = self.c.u32le(pos) as usize;
        let valuelen = self.c.u32le(pos + 4) as usize;
        let newpos = pos + 4 + 4 + keylen + valuelen;
        SimpleReader::align(newpos, 4)
    }

    pub fn len(&self) -> usize {
        self.c.len()
    }
}
