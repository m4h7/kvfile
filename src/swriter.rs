use std::fs::File;
use std::io::prelude::*;
use std::io::{SeekFrom, Result};
use std::path::Path;
use std::error::Error;

pub struct SimpleWriter {
    f: File,
    pos: usize,
    sync: bool,
}

impl SimpleWriter {
    pub fn create_file(s: &str) -> Result<SimpleWriter> {
        let path = Path::new(s);
        File::create(&path)
            .map(|file| SimpleWriter { f: file, pos: 0, sync: false })
    }

    #[inline]
    pub fn tell(&mut self) -> usize {
        self.pos
    }

    #[inline]
    pub fn varbytes(&mut self, v: &[u8]) -> bool {
        match self.f.write(v) {
            Ok(bwritten) => {
                self.pos += bwritten;
                bwritten == v.len()
            }
            Err(_) => false
        }
    }

    #[inline]
    pub fn le32_at(&mut self,
                   v: u32,
                   pos: usize) {
        let seekres = self.f.seek(
            SeekFrom::Start(
                pos as u64));
        assert!(seekres.ok().unwrap() == pos as u64);
        self.le32write(v);
        let seekback = self.f.seek(
            SeekFrom::Start(
                self.pos as u64));
        assert!(seekback.ok().unwrap() == self.pos as u64);
    }

    fn le32write(&mut self, v: u32) -> bool {
        let buf = [v as u8,
                   (v >> 8) as u8,
                   (v >> 16) as u8,
                   (v >> 24) as u8];

        let result = self.f.write(&buf);
        match result {
            Ok(bwritten) => {
                bwritten == buf.len()
            },
            Err(_) => {
                false
            }
        }
    }

    #[inline]
    pub fn le32(&mut self, v: u32) -> bool {
        let r = self.le32write(v);
        self.pos += 4;
        r
    }

    #[inline]
    pub fn sync(&mut self) {
        if (self.sync) {
            self.f.sync_all();
        }
    }

    #[inline]
    pub fn align_position(self: &mut SimpleWriter,
                          align: usize) {
        if self.pos % align != 0 {
            let diff = align - (self.pos % align);
            let empty: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
            match self.f.write(&empty[0..diff]) {
                Ok(bwritten) => self.pos += bwritten,
                Err(e) => panic!("error writing alignment bytes: {:}",
                                 Error::description(&e)),
            }
        }
        assert!(self.f.seek(SeekFrom::Current(0)).ok().unwrap() % 4 == 0);
    }

}

