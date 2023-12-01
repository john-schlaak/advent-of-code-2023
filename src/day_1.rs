pub fn sum_first_and_last_digits(text: String) -> u32 {
    let iter = text.split_whitespace();
    iter.map(
        |line| {
            let digit_chars: Vec<&str> = line.matches(char::is_numeric).collect();
            let digits: [u32; 2] = [digit_chars.first(), digit_chars.last()].map(
                |digit_char| if let Some(digit_char) = digit_char {
                    digit_char.parse::<u32>().unwrap_or(0)
                } else {
                    0
                }
            );
            digits[0] * 10 + digits[1]
        }
    ).sum()
}