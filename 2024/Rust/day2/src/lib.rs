use std::fs::read_to_string;

pub fn read_matrix_from_string(string: String) -> Vec<Vec<u32>> {
    return string.trim().lines()
        .map(|l| {
            l.split_whitespace()
            .map(|number| {
                number.parse::<u32>().unwrap()
            })
            .collect()
        })
        .collect();
}

pub fn read_file(filename: &str) -> String {
    return read_to_string(filename).unwrap()
}

pub fn calculate_differences(vector: Vec<u32>) -> Vec<i32> {
    vector.windows(2).map(|pair| {
        i32::try_from(pair[0]).unwrap() - i32::try_from(pair[1]).unwrap()
    })
    .collect()
}

pub fn is_safe(diff_vector: Vec<i32>) -> bool {
    let result = if diff_vector.clone().into_iter().all(|x| x > 0) || diff_vector.clone().into_iter().all(|x| x < 0) {
        diff_vector.into_iter().all(|x| x.abs() <= 3)
    } else {
        false
    };
    return result
}

pub fn is_safe2(diff_vector: Vec<i32>, original_vector: Vec<u32>) -> bool {
    let result = if is_safe(diff_vector) {
        true
    } else {
        (0..original_vector.len()).map(|i| {
            let mut work_vector = original_vector.clone();
            work_vector.remove(i);
            let result = is_safe(calculate_differences(work_vector));
            result
        }).any(|x| x == true)
    };
    return result
}

pub fn run1(string: String) -> usize {
    let data = read_matrix_from_string(string);

    let result: Vec<bool> = data.into_iter().map(|line| {
        is_safe(calculate_differences(line))
    })
    .collect();
    return result.into_iter().filter(|x| *x == true).collect::<Vec<bool>>().len();
}

pub fn run2(string: String) -> usize {
    let data = read_matrix_from_string(string);

    let result: Vec<bool> = data.into_iter().map(|line| {
        let line_copy = line.clone();
        is_safe2(calculate_differences(line), line_copy)
    })
    .collect();
    return result.into_iter().filter(|x| *x == true).collect::<Vec<bool>>().len();
}


pub fn part1(filename: &str) -> usize {
    let string = read_file(filename);

    return run1(string);

}

pub fn part2(filename: &str) -> usize {
    let string = read_file(filename);

    return run2(string);
}

    