use std::collections::HashMap;
use crate::poker::cards::deck_card::DeckCard;
use crate::poker::cards::play_card::PlayCard;
use crate::poker::cards::suit::Suit;
use crate::enum_index::EnumIndex;
use crate::helper::collection;

#[derive(Debug)]
pub enum Combination {
    Kicker(DeckCard),
    Pair(DeckCard),
    TwoPairs { pair1: DeckCard, pair2: DeckCard },
    ThreeOfAKind(DeckCard),
    Straight(DeckCard),
    Flush(PlayCard),
    FullHouse { triple: DeckCard, double: DeckCard },
    FourOfAKind(DeckCard),
    StraightFlush(PlayCard),
    RoyalFlush(Suit)
}

impl Combination {
    fn check_flush(cards: &[PlayCard]) -> Option<PlayCard> {
        let is_flash = cards.len() == 5 &&
            cards.iter().all(
                |card|
                    &card.suit.to_string() == &cards[0].suit.to_string()
            );
        match is_flash {
            true => Some(PlayCard::calc_highest_card(&cards)),
            false => None
        }
    }

    fn check_straight(cards: &[PlayCard]) -> Option<DeckCard> {
        if cards.len() != 5 {
            return None
        }

        let mut values = cards
            .iter()
            .map(|&card| card.value)
            .collect::<Vec<DeckCard>>();
        values
            .sort_by(
                |value1, value2|
                    value1.enum_index().cmp(
                        &value2.enum_index()
                    )
            );

        let mut step = 0;
        let mut sequence = true;
        while step < values.len() - 1 && sequence {
            let current = values.get(step).unwrap();
            let next = values.get(step + 1).unwrap();
            sequence = sequence &&
                next.enum_index() - current.enum_index() == 1;
            step += 1;
        }
        let wheel = vec![
            DeckCard::Two,
            DeckCard::Three,
            DeckCard::Four,
            DeckCard::Five,
            DeckCard::Ace
        ];
        let is_wheel = wheel.eq(&values);
        match sequence && is_wheel {
            true => match is_wheel {
                true => Some(*values.get(wheel.len() - 2).unwrap()),
                false => Some(PlayCard::calc_highest_card(&cards).value),
            },
            false => None
        }
    }

    fn calc_pairs(cards: &[PlayCard]) -> HashMap<DeckCard, u8> {
        let mut pairs: HashMap<DeckCard, u8> = HashMap::new();
        for card in cards.iter() {
            let new_value = match pairs.get(&card.value) {
                None => 1,
                Some(old_value) => old_value + 1
            };
            pairs.insert(card.value, new_value);
        };
        pairs.retain(|_, v| *v > 1);
        pairs
    }

    pub(crate) fn from_vec_cards(cards: &[PlayCard]) -> Result<Combination, String> {
        let cards_len = cards.len();
        if cards_len < 3 || cards_len > 5 {
            return Err(String::from("Invalid number of cards passed"))
        }
        let pairs = Combination::calc_pairs(&cards);
        return match pairs.len() {
            0 => sequence_hand(cards),
            _ => pairs_hand(pairs, cards_len),

        }
    }
}

fn sequence_hand(cards: &[PlayCard]) -> Result<Combination, String> {
    let straight = Combination::check_straight(&cards);
    let flush = Combination::check_flush(&cards);
    match (straight, flush) {
        (Some(high_card), None) => Ok(Combination::Straight(high_card)),
        (None, Some(high_card)) => Ok(Combination::Flush(high_card)),
        (Some(_), Some(flush_high_card)) => match flush_high_card.value {
            DeckCard::Ace => Ok(Combination::RoyalFlush(flush_high_card.suit)),
            _ => Ok(Combination::StraightFlush(flush_high_card))
        },
        (_, _) => Ok(Combination::Kicker(PlayCard::calc_highest_card(cards).value))
    }
}

fn pairs_hand(pairs: HashMap<DeckCard, u8>, cards_len: usize) -> Result<Combination, String> {
    match pairs.len() {
        1 => single_card_pairs(pairs, cards_len),
        2 => multiple_cards_pairs(pairs, cards_len),
        _ => Err(String::from("Invalid number of pairs"))
    }
}

fn single_card_pairs(pairs: HashMap<DeckCard, u8>, cards_len: usize) -> Result<Combination, String> {
    let deck_card = pairs
        .keys()
        .nth(0)
        .unwrap();
    let counter = pairs
        .get(deck_card)
        .unwrap();
    match counter {
        2 => Ok(Combination::Pair(*deck_card)),
        3 => Ok(Combination::ThreeOfAKind(*deck_card)),
        4 => match cards_len {
            3 => Err(String::from("Found four of a kind on 3-card line")),
            5 => Ok(Combination::FourOfAKind(*deck_card)),
            _ => Err(String::from("Wrong number of cards"))
        }
        _ => Err(String::from("Invalid pair counter"))
    }
}

fn multiple_cards_pairs(pairs: HashMap<DeckCard, u8>, cards_len: usize) -> Result<Combination, String> {
    match cards_len {
        3 => Err(String::from("Found two pairs or full house on 3-card line")),
        5 => {
            let counter_sum = pairs
                .iter()
                .fold(0, |acc, (_, &counter)| acc + counter);
            match counter_sum {
                4 => {
                    let pair1 = collection::hashmap_key_max_by_value(&pairs);
                    let pair2 = collection::hashmap_key_min_by_value(&pairs);
                    Ok(Combination::TwoPairs { pair1: *pair1, pair2: *pair2 })
                },
                5 => {
                    let triple = collection::hashmap_get_key_by_value(
                        &pairs, 3
                    ).unwrap();
                    let double = collection::hashmap_get_key_by_value(
                        &pairs, 2
                    ).unwrap();
                    Ok(Combination::FullHouse {
                        triple,
                        double
                    })
                },
                _ => Err(String::from("Invalid pair counter sum"))
            }
        },
        _ => Err(String::from("Wrong number of cards"))
    }
}