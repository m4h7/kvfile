use colwriter::ColumnWriter;
use std::io::Result;

pub struct TableWriter {
    c: ColumnWriter,
    names: Vec<String>,
    types: Vec<String>,
}

pub struct TableRowBuilder<'a> {
    tw: &'a mut TableWriter,
    columns: Vec<Vec<u8>>,
}

impl<'a> TableRowBuilder<'a> {
    fn new(tw: &'a mut TableWriter) -> TableRowBuilder<'a> {
        TableRowBuilder {
            tw: tw,
            columns: Vec::new(),
        }
    }

    pub fn u32le(&mut self, v: u32) -> &'a mut TableRowBuilder {
        let buf = [v as u8,
                   (v >> 8) as u8,
                   (v >> 16) as u8,
                   (v >> 24) as u8];
        self.columns.push(buf.to_vec());
        self
    }

    pub fn utf8(&mut self, v: &str) -> &'a mut TableRowBuilder {
        let buf = v.to_string().as_bytes().to_vec();
        self.columns.push(buf);
        self
    }

    pub fn commit(&mut self) {
        self.tw.add_row(&self.columns);
        self.tw.sync();
    }
}

impl TableWriter {
    pub fn create_file(path: &str,
                       names: &[&str],
                       types: &[&str]) -> Result<TableWriter> {

        let cw = ColumnWriter::create_file(path);

        match cw {
            Err(why) => Err(why),
            Ok(mut c) => {
                // first row contains the column names
                c.next_row(names.len());

                for n in 0..names.len() {
                    c.add_column(n, names[n].as_bytes());
                }

                // second row contains the types
                c.next_row(types.len());

                for n in 0..types.len() {
                    c.add_column(n, types[n].as_bytes());
                }

                let tw = TableWriter {
                    c: c,
                    names: names.iter()
                        .map(|s| s.to_string()).collect(),
                    types: types.iter()
                        .map(|s| s.to_string()).collect(),
                };

                Ok(tw)
            }
        }
    }

    pub fn build_row(&mut self) -> TableRowBuilder {
        TableRowBuilder::new(self)
    }

    pub fn add_row(&mut self, args: &Vec<Vec<u8>>) {
        self.c.next_row(args.len());
        for colidx in 0..args.len() {
            self.c.add_column(
                colidx,
                &args[colidx]
            )
        }
    }

    pub fn sync(&mut self) {
        self.c.sync();
    }
}
