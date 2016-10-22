use sreader::SimpleReader;
use std::io::Result;

pub struct ColumnReader {
    c: SimpleReader,
}

impl ColumnReader {

    pub fn open_file(path: &str) -> Result<ColumnReader> {
        let sr = SimpleReader::open_file(path);
        match sr {
            Err(why) => Err(why),
            Ok(reader) => Ok(ColumnReader { c: reader }),
        }
    }

    pub fn cols(&self, pos: usize) -> usize {
        self.c.u32le(pos) as usize
    }

    pub fn col(&self,
               pos: usize,
               colidx: usize) -> (usize, usize) {
        let mut cpos = 0;
        for n in 0..colidx {
            cpos += self.c.u32le(pos + n * 4) as usize;
        }
        let size = self.c.u32le(pos + colidx * 4) as usize;
        (cpos, size)
    }

    pub fn value(&self,
                 pos: usize,
                 colidx: usize) -> &[u8] {
        let numcols = self.c.u32le(pos) as usize;
        // skip the numcol and col sizes
        let mut cpos = pos + 4 + (numcols * 4);

        for n in 0..colidx {
            let colsize = self.c.u32le(pos + 4 + n * 4) as usize;
            cpos += colsize;
        }
        let size = self.c.u32le(pos + 4 + colidx * 4) as usize;
        self.c.varbytes(cpos, size)
    }

    pub fn next(&self, pos: usize) -> usize {
        let mut cpos = 0;
        let cols = self.cols(pos);
        for n in 0..cols {
            cpos += self.c.u32le(pos + 4 + n * 4) as usize;
        }
        pos + cpos + 4 + cols * 4
    }

    pub fn len(&self) -> usize {
        self.c.len()
    }
}
