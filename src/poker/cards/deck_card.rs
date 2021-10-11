use core::fmt;
use enum_index::{EnumIndex, IndexEnum};

const CARD_NOTATION: [char; 13] = [
    '2', '3', '4', '5',
    '6', '7', '8', '9',
    'T', 'J', 'Q', 'K', 'A'
];

#[derive(
    Debug, Clone, Copy,
    PartialEq, Eq, Hash,
    EnumIndex, IndexEnum,
    Ord, PartialOrd
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
    pub fn from_string(symbol: char) -> Result<DeckCard, String> {
        let index = CARD_NOTATION
            .iter()
            .position(|&x| x == symbol);
        match index {
            None => Err(String::from(" -- no such deck card")),
            Some(found_index) => Ok(DeckCard::index_enum(found_index).unwrap())
        }
    }
}

impl fmt::Display for DeckCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
