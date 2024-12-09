use std::fs::read_to_string;
use itertools::{Itertools, MultiProduct};

#[derive(Clone, PartialEq, Debug)]
pub struct Operation {
    pub result: u64,
    pub elements: Vec<u64>
}

pub fn read_file(filename: &str) -> String {
    return read_to_string(filename).unwrap()
}

pub fn read_operations_from_string(string: String) -> Vec<Operation> {
    return string.trim().lines()
        .map(|l| {
            let operation: Vec<&str> = l.split(':').collect();
            Operation {
                result: operation[0].trim().parse().unwrap(),
                elements: operation[1].trim().split(' ').map(|x| x.parse().unwrap()).collect()
            }
        })
        .collect();
}

pub fn calculate(operation: Operation, operators: Vec<u64>) -> u64 {
    let mut running_result = operation.elements[0];

    for (value, operator) in (operation.elements[1..].to_vec()).into_iter().zip(operators) {
        match operator  {
            1 => { // +
                running_result += value;
            }

            2 => { // *
                running_result *= value;
            }

            3 => { // concatenate
                let merged = format!("{}{}", running_result, value);
                running_result = merged.parse().unwrap();
            }

            error => {
                panic!("Ooops bug. Operator `{}` not suported", error);
            }
        }
    }
    
    running_result
}

pub fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
  where
    I: Iterator + Clone,
    I::Item: Clone {
  std::iter::repeat(it)
    .take(repeat)
    .multi_cartesian_product()
}

pub fn search_valid_operations(operation: Operation, operator_list: Vec<u64>) -> Option<Vec<u64>> {
    product_repeat(operator_list.into_iter(), operation.elements.len() - 1).find(|operators| {
        calculate(operation.clone(), (*operators).clone()) == operation.result
    })
}


pub fn run1(string: String) -> u64 {
    let operation_list = vec![1, 2];
    let operations = read_operations_from_string(string);

    let sum = operations.into_iter().map(|operation| {
        let operators_option = search_valid_operations(operation.clone(), operation_list.clone());
        operators_option.map(|operators| calculate(operation.clone(), operators))
    })
    .filter_map(|p| p)
    .sum();
    return sum;
}

pub fn run2(string: String) -> u64 {
    let operation_list = vec![1, 2, 3];
    let operations = read_operations_from_string(string);

    let sum = operations.into_iter().map(|operation| {
        let operators_option = search_valid_operations(operation.clone(), operation_list.clone());
        operators_option.map(|operators| calculate(operation.clone(), operators))
    })
    .filter_map(|p| p)
    .sum();
    return sum;
}

pub fn part1(filename: &str) -> u64 {
    let string = read_file(filename);

    return run1(string);

}

pub fn part2(filename: &str) -> u64 {
    let string = read_file(filename);

    return run2(string);
}

    