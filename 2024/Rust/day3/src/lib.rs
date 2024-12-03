use std::fs::read_to_string;
use regex::Regex;

pub fn read_file(filename: &str) -> String {
    return read_to_string(filename).unwrap()
}

pub fn mach_all_mul(memory: String) -> Vec<Vec<u32>> {
    let mul_match: Regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    return mul_match.captures_iter(&memory).map(|capture| {
        let mul1: u32 = capture.get(1).unwrap().as_str().parse().unwrap();
        let mul2: u32 = capture.get(2).unwrap().as_str().parse().unwrap();
        vec![mul1, mul2]
    })
    .collect();
}

pub fn mul_and_sum(operations: Vec<Vec<u32>>) -> u32 {
    operations.into_iter().map(|values| {
        values[0] * values[1]
    })
    .sum()
}

pub fn keep_do_amd_remove_dont(memory: String) -> Vec<String> {
    let sections = memory.split("do()");
    return sections.map(|section| {
        section.split("don't()").next().unwrap().to_string()
    })
    .collect();
}

pub fn run1(string: String) -> u32 {
    let operations = mach_all_mul(string);
    return mul_and_sum(operations);
}

pub fn run2(string: String) -> u32 {
    let valid_instructions = keep_do_amd_remove_dont(string).join("");
    let operations = mach_all_mul(valid_instructions);
    return mul_and_sum(operations);
}

pub fn part1(filename: &str) -> u32 {
    let string = read_file(filename);

    return run1(string);

}

pub fn part2(filename: &str) -> u32 {
    let string = read_file(filename);

    return run2(string);
}

    