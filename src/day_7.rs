use std::{collections::HashMap, cmp::Ordering};


#[derive(PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}


impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Tried to map invalid symbol to card")
        }
    }

    fn repr(&self) -> char {
        match &self {
            Card::Ace => 'A',
            Card::King => 'K',
            Card::Queen => 'Q',
            Card::Jack => 'J',
            Card::Ten => 'T',
            Card::Nine => '9',
            Card::Eight => '8',
            Card::Seven => '7',
            Card::Six => '6',
            Card::Five => '5',
            Card::Four => '4',
            Card::Three => '3',
            Card::Two => '2',
        }
    }

    fn get_ordered_list() -> [Self; 13] {
        [
            Self::Ace,
            Self::King,
            Self::Queen,
            Self::Jack,
            Self::Ten,
            Self::Nine,
            Self::Eight,
            Self::Seven,
            Self::Six,
            Self::Five,
            Self::Four,
            Self::Three,
            Self::Two,
        ]
    }
}


struct Hand {
    cards: [Card; 5],
    bid: u32
}


impl Hand {
    fn count_cards(&self) -> [Option<(Card, usize)>; 5] {
        let mut card_counts: HashMap<Card, usize> = HashMap::new();
        for card in self.cards {
            if let Some(count) = card_counts.get_mut(&card) {
                *count += 1;
            } else {
                card_counts.insert(card, 1);
            }
        }
        let mut card_count_list = Card::get_ordered_list().into_iter().filter_map(
            |card| if let Some(&count) = card_counts.get(&card) {
                Some((card, count))
            } else {
                None
            }
        ).collect::<Vec<(Card, usize)>>();
        card_count_list.sort_by(|(_, count_a), (_, count_b)| count_b.cmp(count_a));
        [
            if let Some(pair) = card_count_list.get(0) { Some(*pair) } else { None },
            if let Some(pair) = card_count_list.get(1) { Some(*pair) } else { None },
            if let Some(pair) = card_count_list.get(2) { Some(*pair) } else { None },
            if let Some(pair) = card_count_list.get(3) { Some(*pair) } else { None },
            if let Some(pair) = card_count_list.get(4) { Some(*pair) } else { None },
        ]
    }

    fn get_hand_type(&self) -> usize {
        match self.count_cards() {
            [Some((_, 5)), ..] => 6,                    // Five of a kind
            [Some((_, 4)), ..] => 5,                    // Four of a kind
            [Some((_, 3)), Some((_, 2)), ..] => 4,      // Full house
            [Some((_, 3)), ..] => 3,                    // Three of a kind
            [Some((_, 2)), Some((_, 2)), ..] => 2,      // Two pair
            [Some((_, 2)), ..] => 1,                    // One pair
            _ => 0                                      // High card
        }
    }
}


impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}


impl Eq for Hand {

}


impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (self_hand_type, other_hand_type) = (self.get_hand_type(), other.get_hand_type());
        if self_hand_type > other_hand_type {
            Some(Ordering::Greater)
        } else if self_hand_type < other_hand_type {
            Some(Ordering::Less)
        } else {
            Some(
                self.cards.iter().zip(other.cards.iter()).find_map(
                    |(card_a, card_b)| if card_a != card_b {
                        card_b.partial_cmp(&card_a)
                    } else {
                        None
                    }
                ).unwrap_or(Ordering::Equal)
            )
        }
    }
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


pub fn calculate_total_winnings(hands_text: String) -> u32 {
    let mut hands = parse_hands(hands_text);
    hands.sort();
    hands.iter()
        .enumerate()
        .fold(
            0,
            |total, (i, hand)| total + (i + 1) as u32 * hand.bid
        )
}


fn parse_hands(hands_text: String) -> Vec<Hand> {
    hands_text.split('\n').map(
        |line_text| if let Some([cards_text, bid_text]) = line_text.trim().split_whitespace().collect::<Vec<&str>>().get(0..2) {
            let cards = if let Some(&[c1, c2, c3, c4, c5]) = cards_text.chars().collect::<Vec<char>>().get(0..5) {
                [c1, c2, c3, c4, c5].map(|c| Card::from_char(c))
            } else {
                panic!("Could not parse cards from '{}'", line_text)
            };
            let bid = if let Ok(bid) = bid_text.parse::<u32>() {
                bid
            } else {
                panic!("Could not parse bid from '{}'", bid_text)
            };
            Hand { cards, bid }
        } else {
            panic!("Could not parse cards_text and bid_text from '{}'", line_text)
        }
    ).collect()
}
