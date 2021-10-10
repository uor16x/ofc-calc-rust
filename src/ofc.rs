extern crate strum;
use core::fmt;
use std::collections::HashMap;
use strum_macros::{ EnumIter, EnumString };
use std::str::FromStr;

#[derive(Debug, EnumIter, Clone, PartialEq, EnumString)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

#[derive(Debug, EnumIter, Clone, PartialEq)]
enum DeckCard {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone)]
struct PlayCard {
    value: DeckCard,
    suit: Suit
}

enum Combination {
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

struct PlayerTable {
    top: Combination,
    middle: Combination,
    bottom: Combination,
}

impl DeckCard {
    fn from_string(symbol: &str) -> Result<DeckCard, String> {
        match symbol {
            "2" => Ok(DeckCard::Two),
            "3" => Ok(DeckCard::Three),
            "4" => Ok(DeckCard::Four),
            "5" => Ok(DeckCard::Five),
            "6" => Ok(DeckCard::Six),
            "7" => Ok(DeckCard::Seven),
            "8" => Ok(DeckCard::Eight),
            "9" => Ok(DeckCard::Nine),
            "T" => Ok(DeckCard::Ten),
            "J" => Ok(DeckCard::Jack),
            "Q" => Ok(DeckCard::Queen),
            "K" => Ok(DeckCard::King),
            "A" => Ok(DeckCard::Ace),
            _ => Err(String::from(" -- no such deck card"))
        }
    }
}

impl Suit {
    fn from_string(symbol: &str) -> Result<Suit, String> {
        match symbol {
            "h" => Ok(Suit::Hearts),
            "d" => Ok(Suit::Diamonds),
            "c" => Ok(Suit::Clubs),
            "s" => Ok(Suit::Spades),
            _ => Err(String::from(" -- no such suit"))
        }
    }
}

impl Combination {
    fn check_flush(cards: &[PlayCard]) -> bool {
        let first_suit = &cards[0].suit.to_string();
        cards.iter().all(|card| &card.suit.to_string() == first_suit)
    }

    fn check_pairs(cards: &[PlayCard]) -> HashMap<String, Vec<&PlayCard>> {
        let mut pairs: HashMap<String, Vec<&PlayCard>> = HashMap::new();
        for card in cards {
            let value = card.value.to_string();
            let mut new_pair_vec = vec![card];
            let found = match pairs.get(&value) {
                None => {}
                Some(old_pair_vec) => {
                    new_pair_vec = old_pair_vec
                        .iter()
                        .cloned()
                        .chain(
                            new_pair_vec
                                .iter()
                                .cloned()
                        ).collect();
                }
            };
            pairs.insert(value, new_pair_vec);
        };

        pairs
    }

    fn from_vec_cards(cards: &[PlayCard]) -> Result<Combination, String> {
        let pairs = Combination::check_pairs(&cards);
        Ok(Combination::RoyalFlush(Suit::Hearts))
    }
}


impl PlayCard {
    fn from_string(input: &str) -> Result<PlayCard, String> {
        let err_message = String::from("Failed to parse ") + &input;
        if input.len() != 2 {
            return Err(err_message + "length should be 2")
        }

        let value = DeckCard::from_string(&input[..1])?;
        let suit = Suit::from_string(&input[1..2])?;
        let result = PlayCard {
            value,
            suit,
        };
        Ok(result)
    }
}

impl fmt::Display for DeckCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn parse_input(player_input: [[&str; 13]; 3]) -> Result<String, String>{
    let mut player_tables: Vec<PlayerTable> = Vec::with_capacity(3);
    for player_data in player_input {
        let mut parsed_cards: Vec<PlayCard> = Vec::with_capacity(13);
        for card_value in player_data {
            let card = PlayCard::from_string(&card_value)?;
            parsed_cards.push(card);
        }

        let top = Combination::from_vec_cards(&parsed_cards[..3])?;
        let middle = Combination::from_vec_cards(&parsed_cards[3..8])?;
        let bottom = Combination::from_vec_cards(&parsed_cards[3..8])?;

        let table = PlayerTable {
            top,
            middle,
            bottom
        };
        println!("Table calculated");
    }
    Ok(String::from("Ok"))
}