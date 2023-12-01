use std::fs;

fn get_digits(s: &str) -> Vec<char> {
    let res: Vec<char> = s.chars().filter(|&c| c.is_numeric()).collect();
    res
}

fn main() {
    let contents = fs::read_to_string("./bin/day1.input").expect("uhoh couldnt read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut sum = 0;
    for line in lines.iter() {
        let digits = get_digits(line);
        let x: u32 = format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse().unwrap();
        sum += x;
    }
    println!("{:?}", sum)
}
