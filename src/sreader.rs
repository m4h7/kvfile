extern crate memmap;

use std::io::Result;
use std::path::Path;
use self::memmap::{Mmap, Protection};

pub struct SimpleReader {
    m: Mmap,
}

impl SimpleReader {
    pub fn open_file(s: &str) -> Result<SimpleReader> {
        let path = Path::new(s);
        Mmap::open_path(path, Protection::Read)
            .map(|file| SimpleReader { m: file })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.m.len()
    }

    #[inline]
    pub fn u32le(&self, pos: usize) -> u32 {
        let ptr = unsafe { self.m.as_slice() };

        (ptr[pos] as u32) |
        ((ptr[pos + 1] as u32) << 8) |
        ((ptr[pos + 2] as u32) << 16) |
        ((ptr[pos + 3] as u32) << 24)
    }

    #[inline]
    pub fn varbytes(&self,
                    pos: usize,
                    size: usize) -> &[u8] {
        let ptr = unsafe { self.m.as_slice() };
        return &ptr[pos..pos+size];
    }

    #[inline]
    pub fn align(pos: usize,
                 alignment: usize) -> usize {
        if pos % alignment != 0 {
            pos + alignment - (pos % alignment)
        } else {
            pos
        }
    }
}
