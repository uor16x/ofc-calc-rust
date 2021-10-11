use crate::poker::cards::deck_card::DeckCard;
use crate::poker::cards::suit::Suit;
use enum_index::{EnumIndex, IndexEnum};

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

        let value = DeckCard::from_string(
            input.chars().nth(0).unwrap()
        )?;
        let suit = Suit::from_string(
            input.chars().nth(1).unwrap()
        )?;
        let result = PlayCard {
            value,
            suit,
        };
        Ok(result)
    }

    pub fn calc_highest_card(cards: &[PlayCard]) -> PlayCard {
        cards
            .iter()
            .max_by(
                |
                    &card1,
                    &card2
                |
                    card1.value.enum_index().cmp(
                        &card2.value.enum_index()
                    )
            )
            .map(|&card| card)
            .unwrap()
    }
}