use core::fmt;
use std::collections::HashMap;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, PartialEq)]
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
    TwoPairs {pair1: DeckCard, pair2: DeckCard },
    ThreeOfAKind(DeckCard),
    Straight(DeckCard),
    Flush(PlayCard),
    FullHouse { triple: DeckCard, double: DeckCard },
    FourOfAKind(DeckCard),
    StraightFlush(PlayCard),
    RoyalFlush(Suit)
}

struct Line {
    cards: Vec<PlayCard>,
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

impl Line {
    fn check_flush(&self) -> bool {
        let first_suit = &self.cards[0].suit.to_string();
        self.cards.iter().all(|card| &card.suit.to_string() == first_suit)
    }

    fn get_combination(&self) -> Result<Combination, String> {
        Ok(Combination::RoyalFlush(Suit::Hearts))
        // let line_length = &self.cards.len();
        // let pairs = &self.count_pairs();
        // match pairs.keys().len() {
        //     1 => {
        //         match line_length {
        //             3 => {
        //                 Ok(Combination::ThreeOfAKind(DeckCard::Three))
        //             },
        //             _ => Err(String::from("sdf"))
        //         }
        //     }
        //     _ => Err(String::from("sdf"))
        // }
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

        let top = Line {
            cards: (&parsed_cards[..3]).iter().cloned().collect()
        };
        let middle = Line {
            cards: (&parsed_cards[3..8]).iter().cloned().collect()
        };
        let bottom = Line {
            cards: (&parsed_cards[3..8]).iter().cloned().collect(),
        };

        let top_combination = top.get_combination()?;
        let middle_combination = middle.get_combination()?;
        let bottom_combination = bottom.get_combination()?;

        let table = PlayerTable {
            top: top_combination,
            middle: middle_combination,
            bottom: bottom_combination
        };
        println!("Table calculated");
    }
    Ok(String::from("Ok"))
}