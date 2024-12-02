use day6;


#[test]
fn part1_get_roots1() {
    assert_eq!((2, 5), day6::get_roots(7, 9));
}

#[test]
fn part1_get_roots2() {
    assert_eq!((4, 11), day6::get_roots(15, 40));
}

#[test]
fn part1_get_roots3() {
    assert_eq!((11, 19), day6::get_roots(30, 200));
}
