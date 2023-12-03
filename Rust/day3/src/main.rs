use day3::read_table;
use day3::get_symbols_position;
use day3::get_neighbours;

fn main() {
    let table1_test = read_table("test-input");
    let symbols1_test = get_symbols_position(table1_test.clone());
    let neighbours1_test = get_neighbours(table1_test, symbols1_test);

    let sum1_test: i32 = neighbours1_test.iter().sum();
    print!("{}", "The result for part 1 test is: ");
    println!("{}", sum1_test);


    let table1 = read_table("input");
    let symbols1 = get_symbols_position(table1.clone());
    let neighbours1 = get_neighbours(table1, symbols1);

    let sum1: i32 = neighbours1.iter().sum();
    print!("{}", "The result for part 1 is: ");
    println!("{}", sum1);
}