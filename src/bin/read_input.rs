// use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// pub fn file(filepath: &str) -> Vec<&str> {
//     let contents = fs::read_to_string(filepath).expect("we could not read the file");
//     return &contents.lines().collect()
// }
//
//

pub fn file_to_lines<P>(filepath: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>{
    let file = File::open(filepath)?;
    Ok(io::BufReader::new(file).lines())
}
