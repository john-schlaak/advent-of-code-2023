struct Race {
    time: u64,
    distance: u64
}

impl Race {
    fn get_num_better_distances(&self) -> u64 {
        for i in 0..(self.time + 1) {
            if (self.time - i) * i > self.distance {
                return self.time - 2 * i + 1
            }
        }
        return 0
    }
}


pub fn get_product_of_num_possible_record_breaks(races_text: String) -> u64 {
    parse_races(races_text).iter().fold(
        1,
        |product, race| race.get_num_better_distances() * product
    )
}


pub fn get_single_race_num_possible_record_breaks(races_text: String) -> u64 {
    parse_races_bad_kerning(races_text).get_num_better_distances()
}


fn parse_races(races_text: String) -> Vec<Race> {
    if let Some([time_line, distance_line]) = races_text.split('\n').map(|line| line.trim()).collect::<Vec<&str>>().get(0..2) {
        time_line.split_whitespace().zip(distance_line.split_whitespace()).filter_map(
            |(time_str, distance_str)| if let (Ok(time), Ok(distance)) = (time_str.parse::<u64>(), distance_str.parse::<u64>()) {
                Some(Race { time, distance })
            } else {
                None
            }
        ).collect()
    } else {
        panic!("Could not parse races")
    }
}


fn parse_races_bad_kerning(races_text: String) -> Race {
    if let Some([time_line, distance_line]) = races_text.split('\n').map(|line| line.trim()).collect::<Vec<&str>>().get(0..2) {
        let [time, distance] = [time_line, distance_line].map(
            |line| 
            line.split_whitespace()
                .filter_map(
                    |num_str|
                    if let Ok(_) = num_str.parse::<u64>() {
                        Some(num_str) 
                    } else {
                        None 
                    }
                ).collect::<Vec<&str>>()
                .concat()
                .parse::<u64>()
                .ok()
                .unwrap_or(0)
        );
        Race { time, distance }
    } else {
        panic!("Could not parse races")
    }
}