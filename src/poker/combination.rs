use std::collections::HashMap;
use crate::poker::cards::deck_card::DeckCard;
use crate::poker::cards::play_card::PlayCard;
use crate::poker::cards::suit::Suit;
use crate::enum_index::EnumIndex;
use crate::poker::combination::Combination::Flush;

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
        // let wheel = vec![
        //     DeckCard::Two,
        //     DeckCard::Three,
        //     DeckCard::Four,
        //     DeckCard::Five,
        //     DeckCard::Ace
        // ];
        // TODO: add wheel
        match sequence {
            true => Some(PlayCard::calc_highest_card(&cards)),
            false => None
        }
    }

    fn get_pairs(cards: &[PlayCard]) -> HashMap<DeckCard, u8> {
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
        if cards.len() < 3 || cards.len() > 5 {
            return Err(String::from("Invalid number of cards passed"))
        }
        let pairs = Combination::get_pairs(&cards);
        return match pairs.len() {
            0 => {
                let straight = Combination::check_straight(&cards);
                let flush = Combination::check_flush(&cards);
                match (straight, flush) {
                    (_, _) => Ok(Flush(PlayCard{suit:Suit::Spades, value: DeckCard::Ace}))
                }

                // match Combination::check_flush(&cards) {
                //     true => {
                //         let card = cards
                //             .iter()
                //             .max_by(
                //                 |
                //                     &card1,
                //                     &card2
                //                 |
                //                     card1.value.enum_index().cmp(
                //                         &card2.value.enum_index()
                //                     )
                //             )
                //             .unwrap();
                //         Ok(Combination::Flush(*card))
                //     }
                //     false => {
                //         Ok(Combination::Straight(DeckCard::Ace))
                //     }
                // }
            },
            1 => {
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
                    4 => {
                        match cards.len() {
                            3 => Err(String::from("Found four of a kind on 3-card line")),
                            5 => Ok(Combination::FourOfAKind(*deck_card)),
                            _ => Err(String::from("Wrong number of cards"))
                        }
                    }
                    _ => Err(String::from("Invalid pair counter"))
                }
            },
            2 => {
                match cards.len() {
                    3 => Err(String::from("Found two pairs or full house on 3-card line")),
                    5 => {
                        let counter_sum = pairs
                            .iter()
                            .fold(
                                0,
                                |acc, (_, &counter)|
                                    acc + counter
                            );
                        match counter_sum {
                            4 => {
                                let pair1 = pairs
                                    .iter()
                                    .max_by(
                                        |
                                            &(_, a_value),
                                            &(_, b_value)
                                        |
                                            a_value.cmp(b_value)
                                    )
                                    .map(|(key, _)| key)
                                    .unwrap();
                                let pair2 = pairs
                                    .iter()
                                    .min_by(
                                        |
                                            &(_, a_value),
                                            &(_, b_value)
                                        |
                                            a_value.cmp(b_value)
                                    )
                                    .map(|(key, _)| key)
                                    .unwrap();
                                Ok(Combination::TwoPairs {
                                    pair1: *pair1,
                                    pair2: *pair2,
                                })
                            },
                            5 => {
                                let double_deck_card = pairs
                                    .iter()
                                    .filter(|&(_, &y)| y == 2)
                                    .map(|(key, _)| key)
                                    .next()
                                    .unwrap();
                                let triple_deck_card = pairs
                                    .iter()
                                    .filter(|&(_, &y)| y == 3)
                                    .map(|(key, _)| key)
                                    .next()
                                    .unwrap();
                                Ok(Combination::FullHouse {
                                    triple: *triple_deck_card,
                                    double: *double_deck_card
                                })
                            },
                            _ => Err(String::from("Invalid pair counter sum"))
                        }
                    },
                    _ => Err(String::from("Wrong number of cards"))
                }
            },
            _ => Err(String::from("Invalid number of pairs"))
        }
    }
}