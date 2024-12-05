use std::fs::read_to_string;
use hashbrown::HashSet;

pub struct Input{
    orderings: Vec<Vec<u32>>,
    updates: Vec<Vec<u32>>
}

pub fn read_file(filename: &str) -> String {
    return read_to_string(filename).unwrap()
}

pub fn read_input(string: String) -> Input {
    let sections = string.split("\n\n").collect::<Vec<&str>>();

    let orderings: Vec<Vec<u32>> = sections[0].lines().map(|line| {
        line.split("|")
        .map(|value| value.parse::<u32>().unwrap())
        .collect()
    })
    .collect();

    let updates: Vec<Vec<u32>> = sections[1].lines().map(|line| {
        line.split(",")
        .map(|value| value.parse::<u32>().unwrap())
        .collect()
    })
    .collect();

    return Input {
        orderings: orderings, 
        updates
    };
}

pub fn is_update_valid(update: Vec<u32>, orderings: Vec<Vec<u32>>) -> bool {

    (0..update.len()).map(|i| {
        let rules: HashSet<u32> = orderings.clone().into_iter().filter(|p| p[1] == update[i]).map(|p| p[0]).collect();
        let other_updates: HashSet<u32> = update[i+1..].into_iter().map(|x| *x).collect::<HashSet<u32>>();
        let intersect = rules.intersection(&other_updates).cloned().collect::<Vec<u32>>();
        
        intersect.len() == 0
    })
    .all(|x| x == true)
}

pub fn filter_valid_updates(updates: Vec<Vec<u32>>, orderings: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    return updates.into_iter().filter(|update| {
        is_update_valid(update.clone(), orderings.clone())
    })
    .collect();
}

pub fn filter_not_valid_updates(updates: Vec<Vec<u32>>, orderings: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    return updates.into_iter().filter(|update| {
        !is_update_valid(update.clone(), orderings.clone())
    })
    .collect();
}

pub fn return_middle_pages(valid_updates: Vec<Vec<u32>>) -> Vec<u32> {
    valid_updates.into_iter().map(|update| {
        let i = update.len()/2;
        update[i]
    })
    .collect()
}

fn compare(a: &u32, b: &u32, orderings: Vec<Vec<u32>>) ->std::cmp::Ordering{
    let wrong = orderings.clone().into_iter().find(|p| p[0] == *b && p[1] == *a);
    if wrong.is_none() {
        return std::cmp::Ordering::Less;
    } else {
        return  std::cmp::Ordering::Greater;
    }
}

pub fn run1(string: String) -> u32 {
    let input = read_input(string);
    let valid_updates = filter_valid_updates(input.updates.clone(), input.orderings.clone());
    return return_middle_pages(valid_updates).into_iter().sum();
}

pub fn run2(string: String) -> u32 {
    let input = read_input(string);
    let invalid_updates = filter_not_valid_updates(input.updates.clone(), input.orderings.clone());

    let valid_updates= invalid_updates.into_iter().map(|update| {
        let mut new = update.clone();
        new.sort_by(|a, b| compare(a, b, input.orderings.clone()));
        new
    })
    .collect();

    return return_middle_pages(valid_updates).into_iter().sum();
}

pub fn part1(filename: &str) -> u32 {
    let string = read_file(filename);

    return run1(string);

}

pub fn part2(filename: &str) -> u32 {
    let string = read_file(filename);

    return run2(string);
}

    