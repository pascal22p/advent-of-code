#[test]
fn part1_test1() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(11, day1::run1(input.to_string()));
}

#[test]
fn part1_test2() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let expected = vec![
        vec![1, 2, 3, 3, 3, 4],
        vec![3, 3, 3, 4, 5, 9]
    ];

    let data = day1::read_matrix_from_string(input.to_string());

    assert_eq!(expected, day1::build_series(data));
}

#[test]
fn part1_test3() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    let expected = vec![9, 4, 0, 0, 9, 9];

    let data = day1::read_matrix_from_string(input.to_string());

    assert_eq!(expected, day1::collect_values(data));
}
