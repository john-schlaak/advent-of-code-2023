struct Race {
    time: u32,
    distance: u32
}

impl Race {
    fn get_num_better_distances(&self) -> u32 {
        for i in 0..(self.time + 1) {
            if (self.time - i) * i > self.distance {
                return self.time - 2 * i + 1
            }
        }
        return 0
    }
}


pub fn get_product_of_num_possible_record_breaks(races_text: String) -> u32 {
    parse_races(races_text).iter().fold(
        1,
        |product, race| race.get_num_better_distances() * product
    )
}


fn parse_races(races_text: String) -> Vec<Race> {
    if let Some([time_line, distance_line]) = races_text.split('\n').map(|line| line.trim()).collect::<Vec<&str>>().get(0..2) {
        time_line.split_whitespace().zip(distance_line.split_whitespace()).filter_map(
            |(time_str, distance_str)| if let (Ok(time), Ok(distance)) = (time_str.parse::<u32>(), distance_str.parse::<u32>()) {
                Some(Race { time, distance })
            } else {
                None
            }
        ).collect()
    } else {
        panic!("Could not parse races")
    }
}