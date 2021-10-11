use core::fmt;
use enum_index::{EnumIndex, IndexEnum};

const SUIT_NOTATION: [char; 4] = [
    'h', 'd', 'c', 's',
];

#[derive(
    Debug,Clone,PartialEq,
    Copy, EnumIndex, IndexEnum
)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

impl Suit {
    pub fn from_string(symbol: char) -> Result<Suit, String> {
        let index = SUIT_NOTATION
            .iter()
            .position(|&x| x == symbol);
        match index {
            None => Err(String::from(" -- no such deck card")),
            Some(found_index) => Ok(Suit::index_enum(found_index).unwrap())
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}