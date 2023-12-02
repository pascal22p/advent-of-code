use day2::Rgb;

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
    assert_eq!(Rgb {red: 1,green: 2, blue: 6}, Rgb::from_game(" 1 red, 2 green, 6 blue"));
}

#[test]
fn part1_from_game2() {
    assert_eq!(Rgb {red: 0,green: 2, blue: 6}, Rgb::from_game("  2 green, 6 blue"));
}
