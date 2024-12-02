use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct SoilMap {
    pub destination: String,
    pub destination_range_start: i64,
    pub source_range_start: i64,
    pub range_length: i64
}


#[derive(PartialEq, Debug)]
pub struct Almanac {
    pub seeds: Vec<i64>,
    pub soil_maps: HashMap<String, Vec<SoilMap>>
}

impl Almanac {
    pub fn from_string(data: String) -> Self {
        /*
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
         */
        let (header, maps_string) = data.trim().split_once("\n").unwrap();
        
        assert_eq!("seeds:", &header[0..6]);

        let seeds: Vec<i64> = header[6..]
        .trim()
        .split_whitespace()
        .map(|val| val.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

        let maps = maps_string
            .split("\n\n").map(|val| read_map(val))
            .collect::<HashMap<String, Vec<SoilMap>>>();
        
        return Almanac {
            seeds: seeds,
            soil_maps: maps
        }
    }

    pub fn find_location(&self, seed: i64) -> i64 {
        let mut current_location: String = "seed".to_string();
        let mut current_seed = seed;

        while self.soil_maps.get(&current_location).is_some() {
            let map: Vec<&SoilMap> = self.soil_maps
            .get(&current_location).unwrap()
            .iter()
            .filter(|&map| current_seed >= map.source_range_start && current_seed < map.source_range_start + map.range_length)
            .collect();

            if !map.is_empty() {
                assert_eq!(1, map.len());
                current_seed = map[0].destination_range_start - map[0].source_range_start + current_seed;
            }

            current_location = self.soil_maps.get(&current_location).unwrap().get(0).unwrap().destination.to_owned();
        }
        
        return current_seed;
        
    }
    

}

pub fn read_file_string(filepath: &str) -> String {
    return fs::read_to_string(filepath).unwrap().trim().to_string();
}

pub fn read_map(map_section: &str) -> (String, Vec<SoilMap>) {
    /*
    temperature-to-humidity map:
    0 69 1
    1 0 69
    */

    let (header, maps) = map_section.trim().split_once("\n").unwrap();
    
    assert_eq!(" map:", &header[header.len() - 5..]);
    let (start, destination) = header[0..(header.len() - 5)].split_once("-to-").unwrap();

    let ranges = maps.trim()
        .split("\n")
        .map(|map| {
            let range = map.trim().split_whitespace().collect::<Vec<&str>>();
            assert_eq!(3, range.len());
            return SoilMap {
                destination: destination.to_string(),
                destination_range_start: range[0].parse::<i64>().unwrap(),
                source_range_start: range[1].parse::<i64>().unwrap(),
                range_length: range[2].parse::<i64>().unwrap()
            };
        })
        .collect::<Vec<SoilMap>>();



    return (start.to_string(), ranges);
}
