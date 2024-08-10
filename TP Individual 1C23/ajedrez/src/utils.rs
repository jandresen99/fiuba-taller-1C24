use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// READ_LINES: lee el archivo indicado y devuelve una lista con cada linea
pub fn read_lines(filename: String) -> io::Result<io::Lines<BufReader<File>>> {
    let file = File::open(filename).unwrap();
    return Ok(BufReader::new(file).lines());
}
