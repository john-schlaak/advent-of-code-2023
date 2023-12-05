use std::cmp::min;
use std::ops::Range;
use std::collections::HashMap;
use std::str::FromStr;

struct AlmanacMap {
    dest_cat: String,
    src_range: Range<u64>,
    dest_range: Range<u64>
}

impl AlmanacMap {
    fn map_input(&self, src_num: u64) -> Option<u64> {
        if self.src_range.contains(&src_num) {
            Some(src_num - self.src_range.start + self.dest_range.start)
        } else {
            None
        }
    }
}


struct Almanac {
    seeds: Vec<u64>,
    almanac_maps: HashMap<String, Vec<AlmanacMap>>
}

impl Almanac {
    fn map_cat_to_cat(&self, src_cat: &String, src_num: u64) -> u64 {
        if let Some(cat_maps) = self.almanac_maps.get(src_cat) {
            self.map_cat_to_cat(
                &cat_maps.first().unwrap().dest_cat,
                cat_maps.iter()
                    .find_map(|cat_map| cat_map.map_input(src_num))
                    .unwrap_or(src_num)
            )
        } else {
            src_num
        }
    }

    fn map_seeds_to_outputs(&self) -> Vec<u64> {
        let seed_cat = String::from_str("seed").unwrap();
        self.seeds.iter().map(
            |&seed| self.map_cat_to_cat(&seed_cat, seed)
        ).collect()
    }
}


pub fn get_locations_for_seeds(almanac_text: String) -> u64 {
    *parse_almanac(almanac_text)
        .map_seeds_to_outputs()
        .iter()
        .reduce(|min_loc, loc| min(min_loc, loc)).unwrap_or(&0)
}


fn parse_almanac(almanac_text: String) -> Almanac {
    let mut almanac = Almanac {
        seeds: Vec::new(),
        almanac_maps: HashMap::new()
    };
    let (mut current_src_cat, mut current_dest_cat) = (String::new(), String::new());
    for line in almanac_text.split('\n').map(|line| line.trim()) {
        if line.contains(':') {
            let parts: Vec<&str> = line.split_terminator(':').map(|term| term.trim()).collect();
            if let Some(["seeds", nums_term]) = parts.get(0..2) {
                almanac.seeds.extend(
                    nums_term.split_whitespace().map(
                        |num_str| if let Ok(num) = num_str.parse::<u64>() {
                            num
                        } else {
                            panic!("Could not parse term as number in seeds list");
                        }
                    )
                );
            }
            else {
                let parts = parts.first().unwrap().split(['-', ' ']).collect::<Vec<&str>>();
                (current_src_cat, current_dest_cat) = (
                    String::from_str(parts.get(0).unwrap()).unwrap(),
                    String::from_str(parts.get(2).unwrap()).unwrap()
                );
            }
        }
        else {
            let nums: Vec<u64> = line.split_whitespace().filter_map(
                |num_str| if let Ok(num) = num_str.parse::<u64>() {
                    Some(num)
                } else {
                    None
                }
            ).collect();
            if let Some(&[dest_start, src_start, range]) = nums.get(0..3) {
                let almanac_map = AlmanacMap {
                    dest_cat: current_dest_cat.clone(),
                    src_range: src_start..(src_start + range),
                    dest_range: dest_start..(dest_start + range)
                };
                if let Some(almanac_maps) = almanac.almanac_maps.get_mut(&current_src_cat) {
                    almanac_maps.push(almanac_map);
                } else {
                    almanac.almanac_maps.insert(current_src_cat.clone(), vec![almanac_map]);
                }
            }
        }
    }
    return almanac;
}