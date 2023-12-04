use day4::Game;
use std::collections::HashSet;
use std::collections::HashMap;

#[test]
fn part1_read_table1() {
    let expected = Game {
        game: 1,
        numbers_game: 1,
        numbers_have: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
        numbers_winning: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53])
    };

    assert_eq!(expected, Game::from_string("Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"));
}

#[test]
fn part1_read_table2() {
    let expected = Game {
        game: 5,
        numbers_game: 1,
        numbers_have: HashSet::from_iter(vec![26, 83, 32, 87, 28]),
        numbers_winning: HashSet::from_iter(vec![30, 12, 88, 70, 22, 93, 36, 82])
    };

    assert_eq!(expected, Game::from_string("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"));
}

#[test]
fn part1_read_table3() {
    let expected: HashMap<i32, Game> = HashMap::from([
        (1, Game { game: 1, numbers_game: 1, numbers_have: HashSet::from_iter(vec![41, 48, 86, 83, 17]), numbers_winning: HashSet::from_iter(vec![83, 86,  6, 31, 17,  9, 48, 53]) }),
        (2, Game { game: 2, numbers_game: 1, numbers_have: HashSet::from_iter(vec![13, 32, 20, 16, 61]), numbers_winning: HashSet::from_iter(vec![61, 30, 68, 82, 17, 32, 24, 19]) }),
        (3, Game { game: 3, numbers_game: 1, numbers_have: HashSet::from_iter(vec![1, 21, 53, 59, 44]), numbers_winning: HashSet::from_iter(vec![69, 82, 63, 72, 16, 21, 14,  1]) }),
        (4, Game { game: 4, numbers_game: 1, numbers_have: HashSet::from_iter(vec![41, 92, 73, 84, 69]), numbers_winning: HashSet::from_iter(vec![59, 84, 76, 51, 58,  5, 54, 83]) }),
        (5, Game { game: 5, numbers_game: 1, numbers_have: HashSet::from_iter(vec![87, 83, 26, 28, 32]), numbers_winning: HashSet::from_iter(vec![88, 30, 70, 12, 93, 22, 82, 36]) }),
        (6, Game { game: 6, numbers_game: 1, numbers_have: HashSet::from_iter(vec![31, 18, 13, 56, 72]), numbers_winning: HashSet::from_iter(vec![74, 77, 10, 23, 35, 67, 36, 11]) })
    ]);

    let result = day4::read_table("test-input");

    assert_eq!(expected.get(&1), result.get(&1));
    assert_eq!(expected.get(&2), result.get(&2));
    assert_eq!(expected.get(&3), result.get(&3));
    assert_eq!(expected.get(&4), result.get(&4));
    assert_eq!(expected.get(&5), result.get(&5));
    assert_eq!(expected.get(&6), result.get(&6));
}


#[test]
fn part1_get_score1() {
    let input = Game {
        game: 1,
        numbers_game: 1,
        numbers_have: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
        numbers_winning: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53])
    };

    let expected = 8;

    assert_eq!(expected, Game::get_score(&input));
}

#[test]
fn part1_get_score2() {
    let input = Game {
        game: 5,
        numbers_game: 1,
        numbers_have: HashSet::from_iter(vec![26, 83, 32, 87, 28]),
        numbers_winning: HashSet::from_iter(vec![30, 12, 88, 70, 22, 93, 36, 82])
    };

    let expected = 0;

    assert_eq!(expected, Game::get_score(&input));
}

#[test]
fn part1_get_extra_cards1() {
    let input = Game {
        game: 1,
        numbers_game: 1,
        numbers_have: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
        numbers_winning: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53])
    };

    let expected = vec![2, 3, 4, 5];

    assert_eq!(expected, Game::get_extra_cards(&input));
}

#[test]
fn part1_part2() {
    let input: HashMap<i32, Game> = HashMap::from([
        (1, Game { game: 1, numbers_game: 1, numbers_have: HashSet::from_iter(vec![41, 48, 86, 83, 17]), numbers_winning: HashSet::from_iter(vec![83, 86,  6, 31, 17,  9, 48, 53]) }),
        (2, Game { game: 2, numbers_game: 1, numbers_have: HashSet::from_iter(vec![13, 32, 20, 16, 61]), numbers_winning: HashSet::from_iter(vec![61, 30, 68, 82, 17, 32, 24, 1]) }),
        (3, Game { game: 3, numbers_game: 1, numbers_have: HashSet::from_iter(vec![1, 21, 53, 59, 44]), numbers_winning: HashSet::from_iter(vec![69, 82, 63, 72, 16, 21, 14,  1]) }),
        (4, Game { game: 4, numbers_game: 1, numbers_have: HashSet::from_iter(vec![41, 92, 73, 84, 69]), numbers_winning: HashSet::from_iter(vec![59, 84, 76, 51, 58,  5, 54, 83]) }),
        (5, Game { game: 5, numbers_game: 1, numbers_have: HashSet::from_iter(vec![87, 83, 26, 28, 32]), numbers_winning: HashSet::from_iter(vec![88, 30, 70, 12, 93, 22, 82, 3]) }),
        (6, Game { game: 6, numbers_game: 1, numbers_have: HashSet::from_iter(vec![31, 18, 13, 56, 72]), numbers_winning: HashSet::from_iter(vec![74, 77, 10, 23, 35, 67, 36, 11]) })
    ]);

    let expected = 30;

    assert_eq!(expected, day4::part2(input));
}

