use crate::poker::cards::deck_card::DeckCard;
use crate::poker::cards::suit::Suit;

#[derive(Debug, Clone, Copy)]
pub struct PlayCard {
    pub value: DeckCard,
    pub suit: Suit
}

impl PlayCard {
    pub(crate) fn from_string(input: &str) -> Result<PlayCard, String> {
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