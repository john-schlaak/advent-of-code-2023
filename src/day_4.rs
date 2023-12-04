struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    held_numbers: Vec<u32>
}


impl Card {
    fn get_value(&self) -> u32 {
        let power = self.held_numbers.iter().map(|number| if self.winning_numbers.contains(number) { 1 } else { 0 }).sum::<u32>();
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



fn parse_card(card_text: &str) -> Card {
    if let Some([id_term, numbers_term]) = card_text.split(':').map(|term| term.trim()).collect::<Vec<&str>>().get(0..2) {
        let id = if let Some(id_str) = id_term.split_whitespace().last() {
            if let Ok(id) = id_str.parse::<u32>() {
                id
            } else {
                panic!("Could not parse id as number for card '{}'", card_text)
            }
        } else {
            panic!("Could not find id_str for card '{}'", card_text)
        };
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
                id,
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