extern crate memmap;

use std::path::Path;
use std::error::Error;
use self::memmap::{Mmap, Protection};

#[inline]
fn u8aletou32(ptr: &[u8]) -> u32 {
    (ptr[0] as u32) | ((ptr[1] as u32) << 8) | ((ptr[2] as u32) << 16) | ((ptr[3] as u32) << 24)
}

pub struct KVFileReader {
    m: Mmap,
}

impl KVFileReader {
    pub fn new(s: &str) -> KVFileReader {
        let path = Path::new(s);

        let mmap = match Mmap::open_path(path, Protection::Read) {
            Err(why) => {
                panic!("could not open file: {}: {}",
                       path.display(),
                       Error::description(&why))
            }
            Ok(file) => file,
        };

        KVFileReader { m: mmap }
    }

    pub fn key(&self, pos: usize) -> &[u8] {
        let ptr = unsafe { self.m.as_slice() };
        let len = u8aletou32(&ptr[pos..pos + 4]) as usize;
        return &ptr[pos + 8..(pos + 8 + len)];
    }

    pub fn value(&self, pos: usize) -> &[u8] {
        let ptr = unsafe { self.m.as_slice() };
        let keylen = u8aletou32(&ptr[pos..pos + 4]) as usize;
        let valuelen = u8aletou32(&ptr[pos + 4..pos + 8]) as usize;
        return &ptr[(pos + 8 + keylen)..(pos + 8 + keylen + valuelen)];
    }

    pub fn next(&self, pos: usize) -> usize {
        let ptr = unsafe { self.m.as_slice() };
        let keylen = u8aletou32(&ptr[pos..pos + 4]) as usize;
        let valuelen = u8aletou32(&ptr[pos + 4..pos + 8]) as usize;
        let newpos = pos + 4 + 4 + keylen + valuelen;

        let align = 4;
        if newpos % align != 0 {
            newpos + align - (pos % align)
        } else {
            newpos
        }
    }

    pub fn len(&self) -> usize {
        self.m.len()
    }
}
