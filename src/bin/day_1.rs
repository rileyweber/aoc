use std::fs;
use std::collections::HashMap;


// part 1: first and last digit in the string
// part 2: strings can have _string_ numbers that need to be accounted for

fn create_map() -> HashMap<&'static str, i8> {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map
}

fn get_digit_from_pos(word: &str, map: &HashMap<&str, i8>) -> Option<i8> {
    let c = word.chars().next().unwrap();
    if c.is_numeric() {
        let dig: i8 = i8::try_from(c.to_digit(10).unwrap()).unwrap();
        return Some(dig);
    }

    if word.len() < 3 {
        return None;
    }


    for (key, value) in map.iter() {
        if word.starts_with(key) {
            return Some(i8::try_from(*value).unwrap());
        }
    }
    None
}

fn main() {
    let contents = fs::read_to_string("./bin/day1.input").expect("uhoh couldnt read the file");
    let lines: Vec<&str> = contents.lines().collect();

    let map = create_map();

    let mut sum: u32 = 0;
    for line in lines.iter() {
        let mut nums: Vec<i8> = vec![]; 
        for (i, _) in line.char_indices() {
            match get_digit_from_pos(&line[i..], &map) {
                Some(x) => nums.push(x),
                None => continue,
            }
        }
        let x: u32 = format!("{}{}", nums.first().unwrap(), nums.last().unwrap()).parse().unwrap();
        sum += x;
    }
    println!("{:?}", sum)
}
