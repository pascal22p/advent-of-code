#[test]
fn part1_test1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(161, day3::run1(input.to_string()));
}

#[test]
fn part1_test2() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(48, day3::run2(input.to_string()));
}

#[test]
fn part1_test3() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let expected = vec![
        vec![2,4],
        vec![5,5],
        vec![11,8],
        vec![8,5],
    ];

    let result = day3::mach_all_mul(input.to_string());

    assert_eq!(expected, result);
}

#[test]
fn part1_test4() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    let expected = vec![
        "xmul(2,4)&mul[3,7]!^",
        "?mul(8,5))"
    ];

    let result = day3::keep_do_amd_remove_dont(input.to_string());

    assert_eq!(expected, result);
}
