mod day_1;
mod utilities;

use utilities::read_input_file;

fn main() {
    println!("Day 1a: {}", day_1::sum_first_and_last_digits(read_input_file("day_1.txt"), false));
    println!("Day 1b: {}", day_1::sum_first_and_last_digits(read_input_file("day_1.txt"), true));
}
