use day5::read_file_string;
use day5::Almanac;
use itertools::Itertools;
use itertools::Tuples;
use std::cmp;

fn main() {

    let data1_test = read_file_string("test-input");
    let almanac1_test = Almanac::from_string(data1_test);
    let result1_test = almanac1_test.seeds.iter().map(|&seed| almanac1_test.find_location(seed));

    print!("{}", "The result for part 1 test is: ");
    println!("{}", result1_test.min().unwrap());

    let data1 = read_file_string("input");
    let almanac1 = Almanac::from_string(data1);
    let result1 = almanac1.seeds.iter().map(|&seed| almanac1.find_location(seed));

    print!("{}", "The result for part 1 test is: ");
    println!("{}", result1.min().unwrap());

    println!("{}", "=============");

    let data2_test = read_file_string("test-input");
    let almanac2_test = Almanac::from_string(data2_test);

    let mut iter = almanac2_test.seeds.iter();
    let mut current = iter.next();
    let mut minimum = i64::MAX;
    while current.is_some() {
        let start = current.unwrap();
        let range = iter.next().unwrap();

        for seed in *start..(start+range) {
            minimum = cmp::min(minimum, almanac2_test.find_location(seed));
        }

        current = iter.next();
    }

    print!("{}", "The result for part 2 test is: ");
    println!("{}", minimum);


    let data2 = read_file_string("input");
    let almanac2 = Almanac::from_string(data2);

    let mut iter = almanac2.seeds.iter();
    let mut current = iter.next();
    let mut minimum = i64::MAX;
    while current.is_some() {
        let start = current.unwrap();
        let range = iter.next().unwrap();
        println!("start {}, range {}", start, range);

        for seed in *start..(start+range) {
            if (seed - start) % 1000000 == 0 {
                println!("Doing seed {} out of {}", (seed - start), range);
            }
            minimum = cmp::min(minimum, almanac2.find_location(seed));
        }

        current = iter.next();
    }

    print!("{}", "The result for part 2 test is: ");
    println!("{}", minimum);


}


