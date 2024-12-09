use std::{collections::{HashMap, HashSet}, fs::read_to_string};
extern crate itertools; // 0.7.8
use itertools::Itertools;

#[derive(Clone, PartialEq, Debug)]
pub struct Operation {
    pub result: u64,
    pub elements: Vec<u64>
}

pub fn read_file(filename: &str) -> String {
    return read_to_string(filename).unwrap()
}

pub fn read_map(string: String) -> Vec<Vec<char>> {
    return string.trim().lines()
        .map(|l| {
            l.chars().collect()
        })
        .collect();
}

pub fn read_coordinates(map: Vec<Vec<char>>) -> Vec<(char, (i32, i32))> {
    let mut coordinates: Vec<(char, (i32, i32))> = vec![];

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                coordinates.push((map[i][j], (i as i32, j as i32)))
            }
        }
    }

    return coordinates
}

pub fn find_antennas(coordinates: Vec<(char, (i32, i32))>) -> HashMap<char, Vec<(i32, i32)>> {
    return coordinates.into_iter().into_group_map()
}

pub fn calculate_vector(coordinate1: (i32, i32), coordinate2: (i32, i32)) -> (i32, i32) {
    return (coordinate2.0 - coordinate1.0 , coordinate2.1 - coordinate1.1)
}

pub fn add_vector(a: (i32, i32), b: (i32, i32), mul: i32) -> (i32, i32) {
    return (a.0 + mul * b.0, a.1 + mul * b.1)
}

pub fn substract_vector(a: (i32, i32), b: (i32, i32), mul: i32) -> (i32, i32) {
    return (a.0 - mul * b.0, a.1 - mul * b.1)
}

pub fn calculate_antinodes(antennas: HashMap<char, Vec<(i32, i32)>>) -> HashMap<char, Vec<(i32, i32)>> {
    antennas.into_iter().map(|(group, values)| {
        let mut antinodes = vec![];
        for pair in values.iter().permutations(2).unique() {
            let vector: (i32, i32) = calculate_vector(*pair[0], *pair[1]);

            antinodes.push(substract_vector(*pair[0], vector, 1));
            antinodes.push(add_vector(*pair[1], vector, 1))
        };
        (group, antinodes)
    })
    .collect()
}

pub fn calculate_antinodes_with_harmonics(antennas: HashMap<char, Vec<(i32, i32)>>) -> HashMap<char, Vec<(i32, i32)>> {
    antennas.into_iter().map(|(group, values)| {
        let mut antinodes = vec![];
        for pair in values.iter().permutations(2).unique() {
            let vector: (i32, i32) = calculate_vector(*pair[0], *pair[1]);

            for i in 0..10000 {
                antinodes.push(substract_vector(*pair[0], vector, i));
                antinodes.push(add_vector(*pair[1], vector, i))
            }
        };
        (group, antinodes)
    })
    .collect()
}

pub fn reject_invalid_coordinates(coordinates: Vec<(i32, i32)>, max_size: (i32, i32)) -> Vec<(i32, i32)> {
    coordinates.into_iter().filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < max_size.0 && p.1 < max_size.1).collect()
}

pub fn run1(string: String) -> u64 {
    let map = read_map(string);
    let max_size = (map.len() as i32, map[0].len() as i32);
    let coords = read_coordinates(map);
    let antennas: HashMap<char, Vec<(i32, i32)>> = find_antennas(coords);
    let antinodes_by_antenna: HashMap<char, Vec<(i32, i32)>> = calculate_antinodes(antennas).into_iter().map(|(group, antinodes)| {
        (group, reject_invalid_coordinates(antinodes, max_size))
    })
    .collect();

    let flatten_unique_antinodes: HashSet<(i32, i32)> = antinodes_by_antenna.into_values().flatten().collect();
    return flatten_unique_antinodes.len() as u64;
}

pub fn run2(string: String) -> u64 {
    let map = read_map(string);
    let max_size = (map.len() as i32, map[0].len() as i32);
    let coords = read_coordinates(map);
    let antennas: HashMap<char, Vec<(i32, i32)>> = find_antennas(coords);
    let antinodes_by_antenna: HashMap<char, Vec<(i32, i32)>> = calculate_antinodes_with_harmonics(antennas).into_iter().map(|(group, antinodes)| {
        (group, reject_invalid_coordinates(antinodes, max_size))
    })
    .collect();

    let flatten_unique_antinodes: HashSet<(i32, i32)> = antinodes_by_antenna.into_values().flatten().collect();
    return flatten_unique_antinodes.len() as u64;
}

pub fn part1(filename: &str) -> u64 {
    let string = read_file(filename);

    return run1(string);

}

pub fn part2(filename: &str) -> u64 {
    let string = read_file(filename);

    return run2(string);
}

    