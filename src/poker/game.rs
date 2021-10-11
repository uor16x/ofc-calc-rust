use crate::poker::cards::play_card::PlayCard;
use crate::poker::combination::Combination;

pub fn parse_input(player_input: [[&str; 13]; 3]) -> Result<String, String>{
    // let mut player_tables: Vec<PlayerTable> = Vec::with_capacity(3);
    for player_data in player_input {
        let mut parsed_cards: Vec<PlayCard> = Vec::with_capacity(13);
        for card_value in player_data {
            let card = PlayCard::from_string(&card_value)?;
            parsed_cards.push(card);
        }

        let top = Combination::from_vec_cards(&parsed_cards[..3])?;
        let middle = Combination::from_vec_cards(&parsed_cards[3..8])?;
        let bottom = Combination::from_vec_cards(&parsed_cards[8..])?;

        let table = vec![top, middle, bottom];
        for combination in table {
            println!("Line: ${:?}", combination);
        }
    }
    return Ok(String::from("zxc"));
}