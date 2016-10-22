use colreader::ColumnReader;
use std::str;
use std::io::Result;

pub struct TableReader {
    c: ColumnReader,
    pos: usize,
    names: Vec<String>,
    types: Vec<String>,
}

impl TableReader {

    pub fn open_file(path: &str) -> Result<TableReader> {
        let mut cr = ColumnReader::open_file(path);
        match cr {
            Err(why) => Err(why),
            Ok(reader) => {
                let names: Vec<String> = Vec::new();
                let mut namepos = 0;
                let mut namecols = reader.cols(namepos);
                for n in 0..namecols {
                }

                let types: Vec<String> = Vec::new();
                let mut typepos = reader.next(namepos);
                let mut typecols = reader.cols(typepos);
                for n in 0..typecols {
                }
                let mut pos = reader.next(typepos);

                let tr = TableReader {
                    c: reader,
                    pos: pos,
                    names: names,
                    types: types,
                };
                return Ok(tr);
            }
        }
    }

    pub fn next(&mut self) {
        self.pos = self.c.next(self.pos);
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.c.len()
    }

    pub fn numcols(&self) -> usize {
      self.c.cols(self.pos)
    }

    pub fn u32le(&self, colnum: usize) -> u32 {
        let value = self.c.value(self.pos, colnum);

        (value[0] as u32) |
        ((value[1] as u32) << 8) |
        ((value[2] as u32) << 16) |
        ((value[3] as u32) << 24)
    }

    pub fn string(&self, colnum: usize) -> &str {
        let value = self.c.value(self.pos, colnum);
        str::from_utf8(value).unwrap()
    }
}
