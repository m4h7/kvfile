use swriter::SimpleWriter;
use std::io::Result;

pub struct ColumnWriter {
    w: SimpleWriter,
    rowpos: usize,
}

impl ColumnWriter {
    pub fn create_file(path: &str) -> Result<ColumnWriter> {
        let sw = SimpleWriter::create_file(path);
        sw.map(|w| ColumnWriter { w: w, rowpos: 0 })
    }

    /**
     * Add column to row at position 'rowpos'
     */
    pub fn add_column(&mut self,
                      colidx: usize,
                      data: &[u8]) {
        let datalen = data.len() as u32;
        self.w.varbytes(data);
        self.w.le32_at(
            datalen,
            self.rowpos + 4 + colidx * 4);
    }

    /**
     * Start next row with <numcol> columns
     */
    pub fn next_row(&mut self, numcols: usize) {
        self.rowpos = self.w.tell();
        self.w.le32(numcols as u32);
        for _ in 0..numcols {
            self.w.le32(0);
        }
    }

    pub fn sync(&mut self) {
        self.w.sync();
    }
}
