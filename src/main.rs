mod day_1;
mod day_2;
mod day_3;
mod utilities;

use utilities::read_input_file;

fn main() {
    println!("Day 1a: {}", day_1::sum_first_and_last_digits(read_input_file("day_1.txt"), false));
    println!("Day 1b: {}", day_1::sum_first_and_last_digits(read_input_file("day_1.txt"), true));
    
    println!("Day 2a: {}", day_2::sum_valid_game_ids(read_input_file("day_2.txt"), (12, 13, 14)));
    println!("Day 2b: {}", day_2::sum_powers_of_minimum_grabs(read_input_file("day_2.txt")));

    println!("Day 3a: {}", day_3::sum_part_numbers(read_input_file("day_3.txt")));
}
