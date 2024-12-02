#[test]
fn part1_test1() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(2, day2::run1(input.to_string()));
}

#[test]
fn part1_test2() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    let expected = vec![
        vec![1, 2, 2, 1],
        vec![-1, -5, -1, -1],
        vec![2, 1, 4, 1],
        vec![-2, 1, -2, -1],
        vec![2, 2, 0, 3],
        vec![-2, -3, -1, -2],
    ];

    let data = day2::read_matrix_from_string(input.to_string());

    for (line, expected_line) in data.into_iter().zip(expected) {
        assert_eq!(expected_line, day2::calculate_differences(line));
    }
}

#[test]
fn part1_test3() {
    let input = vec![
        vec![1, 2, 2, 1],
        vec![-1, -5, -1, -1],
        vec![2, 1, 4, 1],
        vec![-2, 1, -2, -1],
        vec![2, 2, 0, 3],
        vec![-2, -3, -1, -2],
    ];

    let expected = vec![true, false, false, false, false, true];

    for (line, expected_line) in input.into_iter().zip(expected) {
        println!("{:?}", line);
        assert_eq!(expected_line, day2::is_safe(line));
    }
}

#[test]
fn part1_test4() {
    let original_string = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";
    let original = day2::read_matrix_from_string(original_string.to_string());

    
    let input = vec![
        vec![1, 2, 2, 1],
        vec![-1, -5, -1, -1],
        vec![2, 1, 4, 1],
        vec![-2, 1, -2, -1],
        vec![2, 2, 0, 3],
        vec![-2, -3, -1, -2],
    ];

    let expected = vec![true, false, false, true, true, true];

    for ((line, original_line), expected_line) in input.into_iter().zip(original).zip(expected) {
        println!("{:?}", line);
        assert_eq!(expected_line, day2::is_safe2(line, original_line));
    }
}

#[test]
fn part1_test5() {
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(4, day2::run2(input.to_string()));
}
