mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod utilities;

use utilities::read_input_file;

fn main() {
    println!("Day 1a: {}", day_1::sum_first_and_last_digits(read_input_file("day_1.txt"), false));
    println!("Day 1b: {}", day_1::sum_first_and_last_digits(read_input_file("day_1.txt"), true));
    
    println!("Day 2a: {}", day_2::sum_valid_game_ids(read_input_file("day_2.txt"), (12, 13, 14)));
    println!("Day 2b: {}", day_2::sum_powers_of_minimum_grabs(read_input_file("day_2.txt")));

    println!("Day 3a: {}", day_3::sum_part_numbers(read_input_file("day_3.txt")));
    println!("Day 3b: {}", day_3::sum_gear_ratios(read_input_file("day_3.txt")));

    println!("Day 4a: {}", day_4::sum_cards(read_input_file("day_4.txt")));
    println!("Day 4b: {}", day_4::count_cards(read_input_file("day_4.txt")));

    println!("Day 5a: {}", day_5::get_locations_for_seeds(read_input_file("day_5.txt")));
    println!("Day 5b: {}", day_5::get_locations_for_seed_ranges(read_input_file("day_5.txt")));

    println!("Day 6a: {}", day_6::get_product_of_num_possible_record_breaks(read_input_file("day_6.txt")));
}
