use std::fs::read_to_string;
use std::collections::HashMap;

pub fn part1(_input: &str) -> i32 {
    let numbers: [&str; 10] = ["0","1","2","3","4","5","6","7","8","9"];
    let mut start: &str = "";
    let mut my_buf: [u8; 4] = [0; 4];
    'outer: for c in _input.chars() {
        let my_str: &str = c.encode_utf8(&mut my_buf);
        for i in numbers {
            if my_str == i {
                start = i;
                break 'outer;
            }
        }
    }

    let mut finish: &str = "";
    'outer: for c in _input.chars().rev() {
        let my_str: &str = c.encode_utf8(&mut my_buf);
        for i in numbers {
            if my_str == i {
                finish = i;
                break 'outer;
            }
        }
    }
    let result: String = start.to_owned() + &finish;
    return result.parse::<i32>().unwrap();
}

pub fn day1_part1(filename: &str) -> Vec<i32> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(part1)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn day1_part2(filename: &str) -> Vec<i32> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(part2)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn part2(_input: &str) -> i32 {
    let number_as_string: HashMap<&str, &str> = HashMap::from([
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut tmp = String::from(_input);

    'outer: for i in 0..(tmp.chars().count()) {
        for (letters, number) in &number_as_string {
            if i+letters.chars().count() > tmp.chars().count() {
                continue;
            }
            if &tmp[i..i+letters.chars().count()] == *letters {
                tmp = tmp.replace(letters, number);
                break 'outer;
            }
        }
    }

    'outer: for i in (0..(tmp.chars().count())).rev() {
        for (letters, number) in &number_as_string {
            if i+letters.chars().count() > tmp.chars().count() {
                continue;
            }
            if &tmp[i..i+letters.chars().count()] == *letters {
                tmp = tmp.replace(letters, number);
                break 'outer;
            }
        }
    }

    return part1(&tmp);
}
    