use crate::poker::cards::play_card::PlayCard;
use crate::poker::combination::Combination;

pub fn parse_input(player_input: [[&str; 13]; 3]) -> Result<String, String>{
    let mut overall = String::new();
    for (player_index, player_data) in player_input.iter().enumerate() {
        let mut parsed_cards: Vec<PlayCard> = Vec::with_capacity(13);
        for card_value in player_data {
            let card = PlayCard::from_string(&card_value)?;
            parsed_cards.push(card);
        }

        let top = Combination::from_vec_cards(&parsed_cards[..3])?;
        let middle = Combination::from_vec_cards(&parsed_cards[3..8])?;
        let bottom = Combination::from_vec_cards(&parsed_cards[8..])?;

        overall = overall.to_owned() + "\nPlayer #"
            + &(player_index + 1).to_string()[..]
            + " :\n"
            + "Top: "
            + &top.to_string()[..]
            + "\nMiddle: "
            + &middle.to_string()[..]
            + "\nBottom: "
            + &bottom.to_string()[..]
            + "\n---";
    }
    return Ok(overall);
}