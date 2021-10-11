use core::fmt;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

impl Suit {
    pub fn from_string(symbol: &str) -> Result<Suit, String> {
        match symbol {
            "h" => Ok(Suit::Hearts),
            "d" => Ok(Suit::Diamonds),
            "c" => Ok(Suit::Clubs),
            "s" => Ok(Suit::Spades),
            _ => Err(String::from(" -- no such suit"))
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}