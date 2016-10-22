use colwriter::ColumnWriter;
use colreader::ColumnReader;

use twriter::TableWriter;
use treader::TableReader;

#[test]
fn test_col_rw() {
    let col0 = "hello world".as_bytes();
    let col1 = "second column".as_bytes();
    let col2 = "third column".as_bytes();

    let mut cw = ColumnWriter::create_file("/tmp/col.data").unwrap();
    let maxn = 100;
    for n in 1..maxn {
        cw.next_row(n);
        for m in 0..n {
            let value = format!("{}:{}", n, m);
            cw.add_column(m, value.as_bytes());
        }
    }
    drop(cw);

    let mut rw = ColumnReader::open_file("/tmp/col.data").unwrap();
    let mut pos = 0;
    let mut expected_n = 1;

    while pos < rw.len() {
        assert!(rw.cols(pos) == expected_n);
        for m in 0..expected_n {
            let expected_value = format!("{}:{}", expected_n, m);
            let v = rw.value(pos, m);
            assert!(v == expected_value.as_bytes());
        }
        pos = rw.next(pos);
        expected_n += 1;
    }
    assert!(expected_n == maxn);

    drop(rw);
}

#[test]
fn test_table_rw() {
    let filename = "/tmp/table.dat";
    let names = ["id", "value", "value2"];
    let types = ["int", "string", "string" ];

    {
        let mut tw = TableWriter::create_file(filename, &names, &types).unwrap();
        for n in 1..100 {
            let s1 = format!("{}:{}", n, n + 1);
            let s2 = format!("{}:{}", n, n - 1);
            tw.build_row()
                .u32le(n as u32)
                .utf8(&s1)
                .utf8(&s2)
                .commit();
        }
    }

    {
        let mut tr = TableReader::open_file(filename).unwrap();
        for n in 1..100 {
            assert!(!tr.eof());
            assert!(tr.numcols() == 3);
            let u = tr.u32le(0) as usize;
            assert!(u == n);
            let es1 = format!("{}:{}", n, n + 1);
            let es2 = format!("{}:{}", n, n - 1);
            {
                let s1 = tr.string(1);
                assert!(s1 == es1);
            }
            {
                let s2 = tr.string(2);
                assert!(s2 == es2);
            }
            tr.next();
        }
        assert!(tr.eof());
    }

}

