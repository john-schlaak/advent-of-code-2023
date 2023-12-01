const DIGIT_STRINGS: [&'static str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];


pub fn sum_first_and_last_digits(text: String, use_words: bool) -> u32 {
    let iter = text.split_whitespace();
    iter.map(
        |line| {
            let digit_indices = get_digit_indices(line, use_words);
            let digits: [u32; 2] = [digit_indices.first(), digit_indices.last()].map(
                |digit_index| if let Some((_, term)) = digit_index {
                    parse_digit(term)
                } else {
                    0
                }
            );
            digits[0] * 10 + digits[1]
        }
    ).sum()
}


fn get_digit_indices(line: &str, use_words: bool) -> Vec<(usize, &str)> {
    let mut digit_string_indices: Vec<(usize, &str)> = Vec::new();
    digit_string_indices.extend(line.match_indices(char::is_numeric));
    if use_words {
        digit_string_indices.extend(
            DIGIT_STRINGS.iter().map(
                |digit_string| line.match_indices(digit_string)
            ).flatten()
        )
    }
    digit_string_indices.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    return digit_string_indices
}


fn parse_digit(term: &str) -> u32 {
    term.parse::<u32>().unwrap_or(
        match term {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => 0
        }
    )
}