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

pub fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

pub fn read_file(filename: &str) -> String {
    return read_to_string(filename).unwrap()
}

pub fn differences(x: Vec<u32>, y: Vec<u32>) -> Vec<u32> {
    return x.iter().zip(y.iter())
    .map(|(x,y)| {
        if y > x {
            y - x 
        } else {
            x - y
        }
    })
    .collect();
}

pub fn build_series(data: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    return transpose(data)
    .into_iter()
    .map(|serie| {
        let mut clone: Vec<u32> = serie.to_vec();
        clone.sort();
        clone
    })
    .collect();
}

pub fn run1(string: String) -> u32 {
    let data = read_matrix_from_string(string);
    let series: Vec<Vec<u32>> = build_series(data);
    let diff: Vec<u32> = differences(series[0].clone(), series[1].clone());
    
    return diff.into_iter().sum();
}

pub fn collect_values(data: Vec<Vec<u32>>) -> Vec<u32> {
    let transposed = transpose(data);

    let result: Vec<u32> = transposed[0].clone().into_iter().map( |index| {
        let count: usize = transposed[1].clone().into_iter().filter(|v| *v == index).collect::<Vec<u32>>().len();
        index * u32::try_from(count).unwrap()
    })
    .collect();

    return result;
}

pub fn run2(string: String) -> u32 {
    let data = read_matrix_from_string(string);
    let counts: Vec<u32> = collect_values(data);
    
    return counts.into_iter().sum();
}

pub fn part1(filename: &str) -> u32 {
    let string = read_file(filename);
    return run1(string);

}

pub fn part2(filename: &str) -> u32 {
    let string = read_file(filename);
    return run2(string);
}

    