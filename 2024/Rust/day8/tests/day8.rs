use std::collections::{btree_map::VacantEntry, HashMap};

use day8::Operation;


#[test]
fn part1_test1() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(14, day8::run1(input.to_string()));
}

#[test]
fn part1_test2() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    assert_eq!(34, day8::run2(input.to_string()));
}

#[test]
fn part1_test_read_coordinates() {
    let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let map = day8::read_map(input.to_string());

    let expected: Vec<(char, (i32, i32))> = vec![
        ('0', (1, 8)), 
        ('0', (2, 5)), 
        ('0', (3, 7)), 
        ('0', (4, 4)), 
        ('A', (5, 6)), 
        ('A', (8, 8)), 
        ('A', (9, 9))
    ];

    assert_eq!(expected, day8::read_coordinates(map));
}

#[test]
fn part1_test_find_antennas() {
    let input: Vec<(char, (i32, i32))> = vec![
        ('0', (1, 8)), 
        ('0', (2, 5)), 
        ('0', (3, 7)), 
        ('0', (4, 4)), 
        ('A', (5, 6)), 
        ('A', (8, 8)), 
        ('A', (9, 9))
    ];

    let expected: HashMap<char, Vec<(i32, i32)>> = [
        ('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]),
        ('A', vec![(5, 6), (8, 8), (9, 9)])
    ].iter().cloned().collect();

    assert_eq!(expected, day8::find_antennas(input));
}

#[test]
fn part1_test_calculate_vector() {
    let coord1 = (1, 8);
    let coord2 = (2, 5);

    let expected = (1, -3);

    assert_eq!(expected, day8::calculate_vector(coord1, coord2));
}

#[test]
fn part1_test_calculate_antinodes() {
    let input: HashMap<char, Vec<(i32, i32)>> = [
        ('a', vec![(3, 4), (5, 5)])
    ].iter().cloned().collect();

    let expected: HashMap<char, Vec<(i32, i32)>> = [
        ('a', vec![(1, 3), (7, 6), (7, 6), (1, 3)])
    ].iter().cloned().collect();


    assert_eq!(expected, day8::calculate_antinodes(input));
}
