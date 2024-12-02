use day6::test_data;
use day6::data;
use day6::data2;
use day6::get_roots;

fn main() {
    let table1_test = test_data();
    let result1_test = table1_test.iter()
    .map(|eq| {
        let (root1, root2) = get_roots(eq["time"], eq["distance"]);
        return root2 - root1 + 1;
    })
    .fold(1 as i64, |acc, current| acc * current);

    print!("{}", "The result for part 1 test is: ");
    println!("{}", result1_test);


    let table1 = data();
    let result1 = table1.iter()
    .map(|eq| {
        let (root1, root2) = get_roots(eq["time"], eq["distance"]);
        return root2 - root1 + 1;
    })
    .fold(1 as i64, |acc, current| acc * current);

    print!("{}", "The result for part 1 is: ");
    println!("{}", result1);

    println!("{}", "=============");

    let (time2, distance2) = data2();
    let (root1, root2) = get_roots(time2, distance2);
    let result2 = root2 - root1 + 1;

    print!("{}", "The result for part 2 is: ");
    println!("{}", result2);

}


