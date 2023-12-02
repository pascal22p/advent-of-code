use std::fs::read_to_string;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Rgb {
    pub red: i32,
    pub green: i32,
    pub blue: i32
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

        let red = *colour_map.clone()
        .into_iter()
        .filter(|(colour, _)| colour == &"red")
        .map(|(_, number)| number)
        .collect::<Vec<i32>>()
        .get(0).unwrap_or(&0);

        let green = *colour_map.clone()
        .into_iter()
        .filter(|(colour, _)| colour == &"green")
        .map(|(_, number)| number)
        .collect::<Vec<i32>>()
        .get(0).unwrap_or(&0);

        let blue = *colour_map.clone()
        .into_iter()
        .filter(|(colour, _)| colour == &"blue")
        .map(|(_, number)| number)
        .collect::<Vec<i32>>()
        .get(0).unwrap_or(&0);

        return Rgb {
            red,
            green,
            blue
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
    .filter(|rgb| rgb.red > 12 || rgb.green > 13 || rgb.blue > 14)
    .collect::<Vec<&Rgb>>()
    .is_empty();
    return (id, valid);
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

    