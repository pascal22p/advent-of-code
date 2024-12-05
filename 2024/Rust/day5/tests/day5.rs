#[test]
fn part1_test1() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(143, day5::run1(input.to_string()));
}

#[test]
fn part1_test2() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    assert_eq!(123, day5::run2(input.to_string()));
}

#[test]
fn part1_test3() {
    let orderings = vec![
        vec![47,53],
        vec![97,13],
        vec![97,61],
        vec![97,47],
        vec![75,29],
        vec![61,13],
        vec![75,53],
        vec![29,13],
        vec![97,29],
        vec![53,29],
        vec![61,53],
        vec![97,53],
        vec![61,29],
        vec![47,13],
        vec![75,47],
        vec![97,75],
        vec![47,61],
        vec![75,61],
        vec![47,29],
        vec![75,13],
        vec![53,13]
    ];

    let operations = vec![
        vec![75,47,61,53,29],
        vec![97,61,53,29,13],
        vec![75,29,13],
        vec![75,97,47,61,53],
        vec![61,13,29],
        vec![97,13,75,29,47]
    ];

    let expected = vec![true, true, true, false, false, false];

    let result: Vec<bool> = operations.into_iter().map(|operation| {
        day5::is_update_valid(operation, orderings.clone())
    })
    .collect();
 
    assert_eq!(expected, result);
}
