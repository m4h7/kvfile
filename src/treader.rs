use colreader::ColumnReader;
use std::str;
use std::io::Result;

pub struct TableReader {
    c: ColumnReader,
    names: Vec<String>,
    types: Vec<String>,
    datastart: usize,
}

pub struct TableReaderIterator<'a> {
  t: &'a TableReader,
  pos: usize,
}

impl<'a> TableReaderIterator<'a> {
    pub fn next(&mut self) {
        self.pos = self.t.next(self.pos);
    }

    pub fn eof(&self) -> bool {
        self.t.eof(self.pos)
    }

    pub fn numcols(&self) -> usize {
        self.t.numcols(self.pos)
    }

    pub fn raw(&self, colnum: usize) -> &'a [u8] {
        self.t.raw(self.pos, colnum)
    }
}

impl TableReader {

    pub fn open_file(path: &str) -> Result<TableReader> {
        let cr = ColumnReader::open_file(path);
        match cr {
            Err(why) => Err(why),
            Ok(reader) => {
                let mut names: Vec<String> = Vec::new();
                let namepos = 0;

                // number of name columns
                let namecols = reader.cols(namepos);
                for n in 0..namecols {
                    let v = reader.value(
                        namepos,
                        n);
                    let colname = str::from_utf8(v).unwrap();
                    names.push(colname.to_string());
                }

                // read list of types
                let mut types: Vec<String> = Vec::new();
                let typepos = reader.next(namepos);
                let typecols = reader.cols(typepos);
                for n in 0..typecols {
                    let v = reader.value(
                        typepos,
                        n);
                    let colname = str::from_utf8(v).unwrap();
                    types.push(colname.to_string());
                }

                let pos = reader.next(typepos);

                let tr = TableReader {
                    c: reader,
                    datastart: pos,
                    names: names,
                    types: types,
                };
                return Ok(tr);
            }
        }
    }

    pub fn iter(&self) -> TableReaderIterator {
        TableReaderIterator {
            t: self,
            pos: self.datastart,
        }
    }

    pub fn start(&self) -> usize {
        self.datastart
    }

    // look up column name
    pub fn nameidx(&self, name: &str) -> Option<usize> {
        for n in 0..self.names.len() {
            if self.names[n] == name {
                return Some(n)
            }
        }

        None
    }

    pub fn eof(&self, pos: usize) -> bool {
        pos >= self.c.len()
    }

    pub fn numcols(&self, pos: usize) -> usize {
        self.c.cols(pos)
    }

    pub fn raw(&self, pos: usize, colnum: usize) -> &[u8] {
        self.c.value(pos, colnum)
    }

    pub fn u32le(&self, pos: usize, colnum: usize) -> u32 {
        let value = self.c.value(pos, colnum);

        (value[0] as u32) |
        ((value[1] as u32) << 8) |
        ((value[2] as u32) << 16) |
        ((value[3] as u32) << 24)
    }

    pub fn string(&self, pos: usize, colnum: usize) -> &str {
        let value = self.c.value(pos, colnum);
        str::from_utf8(value).unwrap()
    }

    pub fn next(&self, pos: usize) -> usize {
        self.c.next(pos)
    }
}
