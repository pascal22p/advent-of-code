use day3::get_gears;
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

    println!("{}", "=============");

    let table2_test = read_table("test-input");
    let symbols2_test = get_symbols_position(table2_test.clone());
    let neighbours2_test = get_gears(table2_test, symbols2_test);

    let sum2_test: i32 = neighbours2_test.iter().sum();
    print!("{}", "The result for part 2 test is: ");
    println!("{}", sum2_test);


    let table2 = read_table("input");
    let symbols2 = get_symbols_position(table2.clone());
    let neighbours2 = get_gears(table2, symbols2);

    let sum2: i32 = neighbours2.iter().sum();
    print!("{}", "The result for part 2 is: ");
    println!("{}", sum2);
}


