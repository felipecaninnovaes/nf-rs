use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file(file: &str) -> io::Result<String> {
    let mut result = String::new();

    let lines = read_lines(file)?;
    for line in lines {
        result.push_str(&line?);
    }

    Ok(result)
}

fn read_lines(file: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(file)?;
    Ok(io::BufReader::new(file).lines())
}
