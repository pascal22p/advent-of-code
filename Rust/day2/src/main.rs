use day2::day2_part1;
use day2::day2_part2;

fn main() {
    let result1 = day2_part1("test_input");
    let sum1: i32 = result1.iter().sum();
    print!("{}", "The result for part 1 test is: ");
    println!("{}", sum1);

    let result1 = day2_part1("input");
    let sum1: i32 = result1.iter().sum();
    print!("{}", "The result for part 1 is: ");
    println!("{}", sum1);

    println!("");

    let result2 = day2_part2("test_input");
    let sum2: i32 = result2.iter().sum();
    print!("{}", "The result for part 2 is: ");
    println!("{}", sum2);

    let result2 = day2_part2("input");
    let sum2: i32 = result2.iter().sum();
    print!("{}", "The result for part 2 is: ");
    println!("{}", sum2);

}