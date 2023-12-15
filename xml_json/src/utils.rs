pub use std::fs::File;
pub use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file(file: String) -> String {
    let mut result = String::new();

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(line_value) = line {
                result.push_str(&line_value);
            }
        }
    }
    return result;
}
