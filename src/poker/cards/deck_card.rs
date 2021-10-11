use core::fmt;
use std::hash::Hash;

#[derive(
Debug,
Clone,
Copy,
PartialEq,
Eq,
Hash,
EnumIndex,
IndexEnum
)]
pub enum DeckCard {
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

impl DeckCard {
    pub fn from_string(symbol: &str) -> Result<DeckCard, String> {
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

impl fmt::Display for DeckCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

