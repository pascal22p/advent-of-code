use day5::Almanac;
use day5::SoilMap;
use std::collections::HashMap;

#[test]
fn part1_almanac_from_string() {
    let expected = Almanac { 
        seeds: vec![79, 14, 55, 13], 
        soil_maps: HashMap::from([
            ("seed".to_string(), vec![
                SoilMap { destination: "soil".to_string(), destination_range_start: 50, source_range_start: 98, range_length: 2 }, 
                SoilMap { destination: "soil".to_string(), destination_range_start: 52, source_range_start: 50, range_length: 48 }
            ]), 
            ("soil".to_string(), vec![
                SoilMap { destination: "fertilizer".to_string(), destination_range_start: 0, source_range_start: 15, range_length: 37 }, 
                SoilMap { destination: "fertilizer".to_string(), destination_range_start: 37, source_range_start: 52, range_length: 2 }, 
                SoilMap { destination: "fertilizer".to_string(), destination_range_start: 39, source_range_start: 0, range_length: 15 }
            ])
        ]) 
    };

    let input = "
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

    ".to_string();

    assert_eq!(expected, Almanac::from_string(input));
}


#[test]
fn part1_find_location1() {
    let data = day5::read_file_string("test-input");
    let almanac = Almanac::from_string(data);
    let seed = 79;

    assert_eq!(82, Almanac::find_location(&almanac, seed));
}

#[test]
fn part1_find_location2() {
    let data = day5::read_file_string("test-input");
    let almanac = Almanac::from_string(data);
    let seed = 14;

    assert_eq!(43, Almanac::find_location(&almanac, seed));
}

#[test]
fn part1_find_location3() {
    let data = day5::read_file_string("test-input");
    let almanac = Almanac::from_string(data);
    let seed = 55;

    assert_eq!(86, Almanac::find_location(&almanac, seed));
}

#[test]
fn part1_find_location4() {
    let data = day5::read_file_string("test-input");
    let almanac = Almanac::from_string(data);
    let seed = 13;

    assert_eq!(35, Almanac::find_location(&almanac, seed));
}
