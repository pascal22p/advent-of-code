use std::fs::read_to_string;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Rgb {
    pub red: Option<i32>,
    pub green: Option<i32>,
    pub blue: Option<i32>
}

impl Rgb {
    pub fn from_game(_game: &str) -> Self {
        // 1 red, 2 green, 6 blue
        let colours = _game.split(",").map(|s| s.trim());

        let colour_map: Vec<(&str, i32)> = colours.map(|colour| {
            let list: Vec<&str> = colour.trim().split(" ").collect();
            let number = list[0].parse::<i32>().unwrap();
            let colour = list[1].trim();
            (colour, number)
        }).collect();

        let binding = colour_map.clone()
        .into_iter()
        .filter(|(colour, _)| colour == &"red")
        .map(|(_, number)| number)
        .collect::<Vec<i32>>();
        let red = binding
        .get(0);

        let binding = colour_map.clone()
        .into_iter()
        .filter(|(colour, _)| colour == &"green")
        .map(|(_, number)| number)
        .collect::<Vec<i32>>();
        let green = binding
        .get(0);

        let binding = colour_map.clone()
        .into_iter()
        .filter(|(colour, _)| colour == &"blue")
        .map(|(_, number)| number)
        .collect::<Vec<i32>>();
        let blue = binding
        .get(0);

        return Rgb {
            red: red.copied(),
            green: green.copied(),
            blue: blue.copied()
        }
    }
}

pub fn get_game_id(_input: &str) -> i32 {
    return _input
    .split(":")
    .collect::<Vec<&str>>()
    .get(0)
    .unwrap_or(&"")
    [5..]
    .parse::<i32>().unwrap();
}

pub fn get_games(_input: &str) -> Vec<Rgb> {
    return _input
    .split(":")
    .collect::<Vec<&str>>()
    .get(1)
    .unwrap_or(&"")
    .split(";")
    .map(get_game_colours)
    .collect();
}

pub fn get_game_colours(_input: &str) -> Rgb {
  return Rgb::from_game(_input)
}

pub fn part1(_input: &str) -> (i32, bool) {
    let id = get_game_id(_input);
    let games: Vec<Rgb> = get_games(_input);
    let valid = games.iter()
    .filter(|rgb| rgb.red.unwrap_or(0) > 12 || rgb.green.unwrap_or(0) > 13 || rgb.blue.unwrap_or(0) > 14)
    .collect::<Vec<&Rgb>>()
    .is_empty();
    return (id, valid);
}

pub fn part2(_input: &str) -> i32 {
    let games: Vec<Rgb> = get_games(_input);
    let min_red = games.iter()
    .map(|rgb| rgb.red.unwrap_or(1))
    .max().unwrap_or(1);
    let min_green = games.iter()
    .map(|rgb| rgb.green.unwrap_or(1))
    .max().unwrap_or(1);
    let min_blue = games.iter()
    .map(|rgb| rgb.blue.unwrap_or(1))
    .max().unwrap_or(1);

    return min_red * min_green * min_blue;
}

pub fn day2_part1(filename: &str) -> Vec<i32> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(part1)  // make each slice into a string
        .filter(|(_, is_valid)| *is_valid)
        .map(|(id, _)| id)
        .collect()
}

pub fn day2_part2(filename: &str) -> Vec<i32> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(part2)  // make each slice into a string
        .collect()
}
    