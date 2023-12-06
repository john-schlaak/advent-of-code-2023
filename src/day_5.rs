use std::cmp::{min, max};
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

    fn map_input_range(&self, src_range: &Range<u64>) -> Option<(Range<u64>, Range<u64>)> {
        if src_range.start <= self.src_range.start {
            if src_range.end <= self.src_range.start {
                /* <src_range>................
                 * ...........<self.src_range>
                 */
                None
            } else {
                if src_range.end <= self.src_range.end {
                    /* ...<src_range>.........
                     * .......<self.src_range>
                     */
                    Some((
                        self.src_range.start..src_range.end,
                        self.dest_range.start..(src_range.end - self.src_range.start + self.dest_range.start)
                    ))
                } else {
                    /* ..<....src_range....>..
                     * ...<self.src_range>....
                     */
                    Some((
                        self.src_range.start..self.src_range.end,
                        self.dest_range.start..self.dest_range.end
                    ))
                }
            }
        } else {
            if src_range.start >= self.src_range.end {
                /* ................<src_range>
                 * <self.src_range>...........
                 */
                None
            } else {
                if src_range.end <= self.src_range.end {
                    /* ..<src_range>..........
                     * <self.src_range>.......
                     */
                    Some((
                        src_range.start..src_range.end,
                        (src_range.start - self.src_range.start + self.dest_range.start)..(src_range.end - self.src_range.start + self.dest_range.start)
                    ))
                } else {
                    /* .........<src_range>...
                     * <self.src_range>.......
                     */
                    Some((
                        src_range.start..self.src_range.end,
                        (src_range.start - self.src_range.start + self.dest_range.start)..self.dest_range.end
                    ))
                }
            }
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

    fn map_cat_to_cat_range(&self, src_cat: &String, src_range: &Range<u64>) -> Vec<Range<u64>> {
        if src_range.start == src_range.end {
            vec![]
        }
        else if let Some(cat_maps) = self.almanac_maps.get(src_cat) {
            let (remaining_src_ranges, dest_ranges) = cat_maps.iter()
                .fold(
                    (vec![src_range.clone()], vec![]), 
                    |(src_ranges, dest_ranges), cat_map| if let Some((matched_src_range, dest_range)) = cat_map.map_input_range(src_range) {
                        (
                            src_ranges.into_iter().flat_map(
                                |src_range| if src_range.start <= matched_src_range.start {
                                    if src_range.end <= matched_src_range.start {
                                        vec![Some(src_range)]
                                    } else {
                                        vec![
                                            if src_range.start < matched_src_range.start {
                                                Some(src_range.start..matched_src_range.start)
                                            } else {
                                                None
                                            },
                                            if src_range.end > matched_src_range.end {
                                                Some(matched_src_range.end..src_range.end)
                                            } else {
                                                None
                                            }
                                        ]
                                    }
                                } else {
                                    if src_range.start >= matched_src_range.end {
                                        vec![Some(src_range)]
                                    } else {
                                        if src_range.end <= matched_src_range.end {
                                            vec![None]
                                        } else {
                                            vec![Some(matched_src_range.end..src_range.end)]
                                        }
                                    }
                                }.into_iter().filter_map(|item| item)
                            ).collect(),
                            {
                                let mut dest_ranges = [dest_ranges, vec![dest_range]].concat();
                                dest_ranges.sort_by(|range, other_range| range.start.partial_cmp(&other_range.start).unwrap());
                                dest_ranges.iter().fold(
                                    vec![], 
                                    |dest_ranges, dest_range| if let Some(other_range) = dest_ranges.last() {
                                        Vec::from([
                                            Vec::from(dest_ranges.get(0..(dest_ranges.len() - 1)).unwrap()),
                                            if dest_range.start <= other_range.end && dest_range.end >= other_range.start {
                                                vec![min(dest_range.start, other_range.start)..max(dest_range.end, other_range.end)]
                                            } else {
                                                vec![other_range.clone(), dest_range.clone()]
                                            }
                                        ].concat())
                                    } else {
                                        vec![dest_range.clone()]
                                    }
                                )
                            }
                        )
                    } else {
                        (src_ranges, dest_ranges)
                    }
                );
            let dest_cat = &cat_maps.first().unwrap().dest_cat;
            [
                remaining_src_ranges.iter().map(
                    |src_range| self.map_cat_to_cat_range(&dest_cat, src_range).into_iter()
                ).flatten().collect::<Vec<Range<u64>>>(),
                dest_ranges.iter().map(
                    |dest_range| self.map_cat_to_cat_range(&dest_cat, dest_range).into_iter()
                ).flatten().collect::<Vec<Range<u64>>>()
            ].concat()
        } else {
            vec![src_range.clone()]
        }
    }

    fn map_seeds_to_outputs(&self) -> Vec<u64> {
        let seed_cat = String::from_str("seed").unwrap();
        self.seeds.iter().map(
            |&seed| self.map_cat_to_cat(&seed_cat, seed)
        ).collect()
    }

    fn map_seed_pairs_to_outputs(&self) -> Vec<Range<u64>> {
        let seed_cat = String::from_str("seed").unwrap();
        (0..(self.seeds.len() / 2)).flat_map(
            |i| self.map_cat_to_cat_range(
                &seed_cat,
                {
                    let (&src_start, &range) = (self.seeds.get(i * 2).unwrap(), self.seeds.get(i * 2 + 1).unwrap());
                    &(src_start..(src_start + range))
                }
            ).into_iter()
        ).collect()
    }
}


pub fn get_locations_for_seeds(almanac_text: String) -> u64 {
    parse_almanac(almanac_text)
        .map_seeds_to_outputs()
        .into_iter()
        .reduce(|min_loc, loc| min(min_loc, loc)).unwrap_or(0)
}


pub fn get_locations_for_seed_ranges(almanac_text: String) -> u64 {
    parse_almanac(almanac_text)
        .map_seed_pairs_to_outputs()
        .into_iter()
        .map(|range| range.start)
        .reduce(|min_loc, loc| min(min_loc, loc))
        .unwrap_or(0)
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