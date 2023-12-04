use std::collections::HashMap;

struct Card {
    winning_numbers: Vec<u32>,
    held_numbers: Vec<u32>
}


impl Card {
    fn get_num_matches(&self) -> u32 {
        self.held_numbers.iter().map(|number| if self.winning_numbers.contains(number) { 1 } else { 0 }).sum::<u32>()
    }

    fn get_value(&self) -> u32 {
        let power = self.get_num_matches();
        if power > 0 {
            2_u32.pow(power - 1)
        } else {
            0
        }
    }
}


pub fn sum_cards(cards_text: String) -> u32 {
    cards_text.split('\n').map(|card_text| parse_card(card_text.trim()).get_value()).sum()
}


pub fn count_cards(cards_text: String) -> u32 {
    let cards = cards_text.split('\n').map(|card_text| parse_card(card_text.trim())).collect::<Vec<Card>>();
    let mut cache = HashMap::new();
    (0..cards.len()).map(|i| count_cards_from_card(&cards, i, &mut cache)).sum()
}


fn count_cards_from_card(cards: &Vec<Card>, start_index: usize, cache: &mut HashMap<usize, u32>) -> u32 {
    if let Some(&count) = cache.get(&start_index) {
        count
    }
     else if let Some(card) = cards.get(start_index) {
        let count = 1 + (0..(card.get_num_matches() as usize)).map(|i| count_cards_from_card(cards, start_index + i + 1, cache)).sum::<u32>();
        cache.insert(start_index, count);
        count
    } else {
        0
    }
}



fn parse_card(card_text: &str) -> Card {
    if let Some(numbers_term) = card_text.split(':').map(|term| term.trim()).last() {
        if let Some([winning_numbers_term, held_numbers_term]) = numbers_term.split('|').map(|term| term.trim()).collect::<Vec<&str>>().get(0..2) {
            let [winning_numbers, held_numbers] = [
                winning_numbers_term, held_numbers_term
            ].map(
                |numbers_term| numbers_term.split_whitespace().map(
                    |number_term| if let Ok(number) = number_term.parse::<u32>() {
                        number
                    } else {
                        panic!("Encountered a term that could not be parsed as a number for card '{}'", card_text)
                    }
                ).collect::<Vec<u32>>()
            );
            Card {
                winning_numbers,
                held_numbers
            }
        } else {
            panic!("Could not find winning_numbers_term and held_numbers_term for card '{}'", card_text)
        }
    } else {
        panic!("Could not find id_term and numbers_term for card '{}'", card_text)
    }
}