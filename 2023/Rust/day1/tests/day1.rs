#[test]
fn part1_test1() {
    assert_eq!(12, day1::part1("1abc2"));
}

#[test]
fn part1_test2() {
    assert_eq!(38, day1::part1("pqr3stu8vwx"));
}

#[test]
fn part1_test3() {
    assert_eq!(15, day1::part1("a1b2c3d4e5f"));
}

#[test]
fn part1_test4() {
    assert_eq!(77, day1::part1("treb7uchet"));
}

//===================

#[test]
fn part2_test1() {
    assert_eq!(29, day1::part2("two1nine"));
}

#[test]
fn part2_test2() {
    assert_eq!(83, day1::part2("eightwothree"));
}

#[test]
fn part2_test3() {
    assert_eq!(13, day1::part2("abcone2threexyz"));
}

#[test]
fn part2_test4() {
    assert_eq!(24, day1::part2("xtwone3four"));
}

#[test]
fn part2_test5() {
    assert_eq!(42, day1::part2("4nineeightseven2"));
}

#[test]
fn part2_test6() {
    assert_eq!(14, day1::part2("zoneight234"));
}

#[test]
fn part2_test7() {
    assert_eq!(76, day1::part2("7pqrstsixteen"));
}

#[test]
fn part2_test_overlap   () {
    assert_eq!(82, day1::part2("jcb82eightwond"));
}