use day2::{Rgb, part2};

#[test]
fn part1_game_id_test1() {
    assert_eq!(1, day2::get_game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
}

#[test]
fn part1_game_id_test2() {
    assert_eq!(134, day2::get_game_id("Game 134: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
}

#[test]
fn part1_from_game1() {
    assert_eq!(Rgb {red: Some(1),green: Some(2), blue: Some(6)}, Rgb::from_game(" 1 red, 2 green, 6 blue"));
}

#[test]
fn part1_from_game2() {
    assert_eq!(Rgb {red: None, green: Some(2), blue: Some(6)}, Rgb::from_game("  2 green, 6 blue"));
}

#[test]
fn part2_test1() {
    assert_eq!(630, part2("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"));
}
