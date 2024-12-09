use std::collections::btree_map::VacantEntry;

use day7::Operation;


#[test]
fn part1_test1() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!(3749, day7::run1(input.to_string()));
}

#[test]
fn part1_test2() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    assert_eq!(11387, day7::run2(input.to_string()));
}

#[test]
fn part1_test3() {
    let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    let expected = vec![
        Operation { result: 190, elements: vec![10, 19] }, 
        Operation { result: 3267, elements: vec![81, 40, 27] }, 
        Operation { result: 83, elements: vec![17, 5] }, 
        Operation { result: 156, elements: vec![15, 6] }, 
        Operation { result: 7290, elements: vec![6, 8, 6, 15] }, 
        Operation { result: 161011, elements:vec![16, 10, 13] }, 
        Operation { result: 192, elements: vec![17, 8, 14] }, 
        Operation { result: 21037, elements: vec![9, 7, 18, 13] }, 
        Operation { result: 292, elements: vec![11, 6, 16, 20] }
    ];

    assert_eq!(expected, day7::read_operations_from_string(input.to_string()));
}

#[test]
fn part1_test4() {
    let input = Operation {
        result: 20,
        elements: vec![2,3,4]
    };

    let expected: u64 = input.result;

    assert_eq!(expected, day7::calculate(input, vec![1,2]));
}

#[test]
fn part1_test5() {
    let input = Operation {
        result: 292,
        elements: vec![11, 6, 16, 20]
    };

    let expected: Option<Vec<u64>> = Some(vec![1,2,1]);

    assert_eq!(expected, day7::search_valid_operations(input, vec![1, 2]));
}

#[test]
fn part1_test6() {
    let input = Operation {
        result: 21037,
        elements: vec![9, 7, 18, 13]
    };

    let expected: Option<Vec<u64>> = None;

    assert_eq!(expected, day7::search_valid_operations(input, vec![1, 2]));
}
