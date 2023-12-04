use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Game {
    pub game: i32,
    pub numbers_game: i32,
    pub numbers_have: HashSet<i32>,
    pub numbers_winning: HashSet<i32>
}

impl Game {
    pub fn from_string(game_string: &str) -> Self {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let split1: Vec<&str> = game_string.split(":").collect();
        let game_id: i32 = (&split1[0][5..]).trim().parse::<i32>().unwrap();

        let split2: Vec<&str> = split1[1].split("|").collect();
        let numbers_have: Vec<i32> = split2[0].trim().split(" ")
            .map(|val| val.trim())
            .filter(|&val| val != "")
            .map(|val| val.parse::<i32>().unwrap())
            .collect();

        let number_winning: Vec<i32> = split2[1].trim().split(" ")
            .map(|val| val.trim())
            .filter(|&val| val != "")
            .map(|val| val.parse::<i32>().unwrap())
            .collect();

        return Game {
            game: game_id,
            numbers_game: 1,
            numbers_have: HashSet::from_iter(numbers_have),
            numbers_winning: HashSet::from_iter(number_winning)
        }
    }

    pub fn get_score(&self) -> i32 {
        let intersect: Vec<&i32> = self.numbers_winning.intersection(&self.numbers_have).collect::<Vec<&i32>>();
        if intersect.is_empty() {
            return 0;
        } else {
            return 2_i32.pow((intersect.len() as u32) - 1_u32);
        }
    }

    pub fn get_extra_cards(&self) -> Vec<i32> {
        let extra_cards_numer: i32 = self.numbers_winning.intersection(&self.numbers_have).collect::<Vec<&i32>>().len() as i32;
        return ((self.game + 1)..(self.game + extra_cards_numer + 1)).map(i32::from).collect();
    }

}

pub fn read_table(filename: &str) -> HashMap<i32, Game> {
    return read_to_string(filename) 
    .unwrap()  // panic on possible file-reading errors
    .lines()
    .filter(|line| line.trim() != "")
    .map(|val| {
        let game = Game::from_string(val);
        return (game.game, game);
    })
    .collect::<HashMap<i32, Game>>();
}

pub fn part2(games: HashMap<i32, Game>) -> i32 {
    let mut extras: Vec<i32> = games.iter()
        .map(|(_, game)| game.get_extra_cards())
        .flatten()
        .collect();
    let mut total = games.len();

    while !extras.is_empty() {
        total += extras.len();
        extras = extras
        .iter()
        .map(|id|
            match games.get(id) {
                None => vec![],
                Some(game) => game.get_extra_cards()
            })
        .flatten().collect::<Vec<i32>>();
    }

    return total as i32;
}
