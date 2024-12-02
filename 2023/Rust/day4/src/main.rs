use day4::{read_table, part2};
use day4::Game;
use std::collections::HashMap;

fn main() {
    let table1_test: HashMap<i32, Game> = read_table("test-input");
    let scores1_test = table1_test.iter().map(|(_, game)| game.get_score());
    let total1_test: i32 = scores1_test.sum();

    print!("{}", "The result for part 1 test is: ");
    println!("{}", total1_test);

    let table1: HashMap<i32, Game> = read_table("input");
    let scores1 = table1.iter().map(|(_, game)| game.get_score());
    let total1: i32 = scores1.sum();

    print!("{}", "The result for part 1 is: ");
    println!("{}", total1);

    println!("{}", "=============");

    let table2_test: HashMap<i32, Game> = read_table("test-input");
    let result2_test = part2(table2_test);

    print!("{}", "The result for part 2 test is: ");
    println!("{}", result2_test);

    let table2: HashMap<i32, Game> = read_table("input");
    let result2 = part2(table2);

    print!("{}", "The result for part 2 test is: ");
    println!("{}", result2);

}


