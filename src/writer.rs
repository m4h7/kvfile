use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::error::Error;

pub struct KVFileWriter {
    f: File,
    pos: usize,
}

// u32 to little endian u8 array
#[inline]
fn u32tou8ale(v: u32) -> [u8; 4] {
    [v as u8, (v >> 8) as u8, (v >> 16) as u8, (v >> 24) as u8]
}


impl KVFileWriter {
    pub fn new(s: &str) -> KVFileWriter {
        let path = Path::new(s);
        let file = match File::create(&path) {
            Err(why) => {
                panic!("could not open file: {}: {}",
                       path.display(),
                       Error::description(&why))
            }
            Ok(file) => file,
        };
        KVFileWriter { f: file, pos: 0 }
    }

    fn write_header(self: &mut KVFileWriter, key: &[u8], value: &[u8]) {
        let key_length = key.len() as u32;
        let value_length = value.len() as u32;
        match self.f.write(&u32tou8ale(key_length)) {
            Ok(bwritten_k) => {
                // u32 in bytes
                assert!(bwritten_k == 4);
                self.pos += bwritten_k;

                match self.f.write(&u32tou8ale(value_length)) {
                    Ok(bwritten_v) => {
                        assert!(bwritten_v == value.len());
                        self.pos += bwritten_v;
                    }
                    Err(e) => panic!("error writing value {:}", Error::description(&e)),
                }
            }
            Err(e) => panic!("error writing key {:}", e),
        }
    }

    #[inline]
    fn align_position(self: &mut KVFileWriter, align: usize) {
        if self.pos % align != 0 {
            let diff = align - (self.pos % align);
            let empty: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            match self.f.write(&empty[0..diff]) {
                Ok(bwritten) => self.pos += bwritten,
                Err(e) => panic!("error writing alignment bytes: {:}", Error::description(&e)),
            }
        }
        assert!(self.f.seek(SeekFrom::Current(0)).ok().unwrap() % 4 == 0);
    }

    pub fn put(self: &mut KVFileWriter, key: &[u8], value: &[u8]) {
        self.align_position(4);
        self.write_header(key, value);
        match self.f.write(key) {
            Ok(b_written_k) => {
                assert!(b_written_k == key.len());
                self.pos += b_written_k;
            }
            Err(e) => panic!("error writing key {:}", Error::description(&e)),
        }
        match self.f.write(value) {
            Ok(b_written_v) => {
                assert!(b_written_v == value.len());
                self.pos += b_written_v;
            }
            Err(e) => panic!("error writing value {:}", Error::description(&e)),
        }
    }
}
