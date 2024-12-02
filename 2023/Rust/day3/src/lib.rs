use std::fs::read_to_string;
use std::cmp;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Symbol {
    pub symbol: char,
    pub row: usize,
    pub col: usize
}

impl Symbol {

}

pub fn part1(_input: &str) -> i32 {
    let numbers: [&str; 10] = ["0","1","2","3","4","5","6","7","8","9"];
    let mut start: &str = "";
    let mut my_buf: [u8; 4] = [0; 4];
    'outer: for c in _input.chars() {
        let my_str: &str = c.encode_utf8(&mut my_buf);
        for i in numbers {
            if my_str == i {
                start = i;
                break 'outer;
            }
        }
    }

    let mut finish: &str = "";
    'outer: for c in _input.chars().rev() {
        let my_str: &str = c.encode_utf8(&mut my_buf);
        for i in numbers {
            if my_str == i {
                finish = i;
                break 'outer;
            }
        }
    }
    let result: String = start.to_owned() + &finish;
    return result.parse::<i32>().unwrap();
}

pub fn read_table(filename: &str) -> Vec<Vec<char>> {
    return read_to_string(filename) 
    .unwrap()  // panic on possible file-reading errors
    .lines()
    .filter(|line| line.trim() != "")
    .map(| line | line.trim().chars().collect())
    .collect::<Vec<Vec<char>>>();
}

pub fn get_symbols_position(data: Vec<Vec<char>>) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    let not_a_symbol = vec!['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

    for (i, row) in data.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if !not_a_symbol.contains(cell) {
                symbols.push(
                    Symbol {
                        symbol: *cell,
                        row: i,
                        col: j
                    }
                );
            }
        }
    }

    return symbols;
}

pub fn get_neighbours(data: Vec<Vec<char>>, symbols: Vec<Symbol>) -> Vec<i32> {
    let mut temp_data = data.clone();
    let digits: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut neighbours: Vec<i32> = Vec::new();

    for symbol in symbols {
        let row_min = cmp::max(symbol.row - 1, 0);
        let row_max = cmp::min(symbol.row + 2, data.len());

        let col_min = cmp::max(symbol.col - 1, 0);
        let col_max = cmp::min(symbol.col + 2, data[0].len());

        for row in row_min..row_max {
            for col in col_min..col_max {
                if digits.contains(&temp_data[row][col]) {
                    let result = read_and_delete_number(temp_data, row, col);
                    temp_data = result.1;
                    neighbours.push(result.0);
                }
            }
        }

    }
    return neighbours;
}

pub fn read_and_delete_number(data: Vec<Vec<char>>, row: usize, col: usize) -> (i32, Vec<Vec<char>>) {
    let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut number = vec![data[row][col]];
    let mut result = data.clone();
    result[row][col] = '.';

    let mut left = true;
    let mut i: usize = 1;
    while left {
        if (col as i32 - i as i32) < 0 {
            left = false;
            continue;
        }
        if digits.contains(&result[row][col - i]) {
            number.insert(0, result[row][col - i]);
            result[row][col - i] = '.';
            i += 1;
        } else {
            left = false;
        }
    }

    let mut right = true;
    let mut i: usize = 1;
    while right  {
        if col + i >= result[0].len() {
            break;
        }
        if digits.contains(&result[row][col + i]) {
            number.push(result[row][col + i]);
            result[row][col + i] = '.';
            i += 1;
        } else {
            right = false;
        }
    }

    return (number.iter().collect::<String>().parse::<i32>().unwrap(), result);
}

pub fn get_gears(data: Vec<Vec<char>>, symbols: Vec<Symbol>) -> Vec<i32> {
    let mut temp_data = data.clone();
    let digits: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut gears: Vec<i32> = Vec::new();

    for symbol in symbols.iter().filter(|symbol| symbol.symbol == '*') {
        let mut neighbours: Vec<i32> = Vec::new();

        let row_min = cmp::max(symbol.row - 1, 0);
        let row_max = cmp::min(symbol.row + 2, data.len());

        let col_min = cmp::max(symbol.col - 1, 0);
        let col_max = cmp::min(symbol.col + 2, data[0].len());

        for row in row_min..row_max {
            for col in col_min..col_max {
                if digits.contains(&temp_data[row][col]) {
                    let result = read_and_delete_number(temp_data, row, col);
                    temp_data = result.1;
                    neighbours.push(result.0);
                }
            }
        }

        if neighbours.len() == 2 {
            gears.push(neighbours[0] * neighbours[1]);
        }

    }
    return gears;
}