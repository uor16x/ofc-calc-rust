use crate::poker::cards::deck_card::DeckCard::*;
use crate::poker::cards::suit::Suit::*;
use crate::poker::cards::play_card::PlayCard;
use crate::poker::combination::Combination;

#[test]
fn royal_flush() {
    let royal_flush = [
        PlayCard { value: Ace, suit: Spades },
        PlayCard { value: King, suit: Spades },
        PlayCard { value: Queen, suit: Spades },
        PlayCard { value: Jack, suit: Spades },
        PlayCard { value: Ten, suit: Spades },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &royal_flush
        ).unwrap(),
        Combination::RoyalFlush(Spades),
        "Royal flush"
    );
}

#[test]
fn straight_flush() {
    let straight_flush = [
        PlayCard { value: King, suit: Spades },
        PlayCard { value: Queen, suit: Spades },
        PlayCard { value: Jack, suit: Spades },
        PlayCard { value: Ten, suit: Spades },
        PlayCard { value: Nine, suit: Spades },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &straight_flush
        ).unwrap(),
        Combination::StraightFlush(
            PlayCard { value: King, suit: Spades }
        ),
        "Straight flush"
    );
}

#[test]
fn straight_flush_wheel() {
    let straight_flush_wheel = [
        PlayCard { value: Two, suit: Spades },
        PlayCard { value: Four, suit: Spades },
        PlayCard { value: Three, suit: Spades },
        PlayCard { value: Five, suit: Spades },
        PlayCard { value: Ace, suit: Spades },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &straight_flush_wheel
        ).unwrap(),
        Combination::StraightFlush(
            PlayCard { value: Five, suit: Spades }
        ),
        "Straight flush - wheel"
    );
}

#[test]
fn four_of_a_kind() {
    let four_of_a_kind = [
        PlayCard { value: King, suit: Hearts },
        PlayCard { value: King, suit: Diamonds },
        PlayCard { value: King, suit: Clubs },
        PlayCard { value: King, suit: Spades },
        PlayCard { value: Ace, suit: Spades },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &four_of_a_kind
        ).unwrap(),
        Combination::FourOfAKind(
            King
        ),
        "Four of a kind"
    );
}

#[test]
fn full_house() {
    let full_house = [
        PlayCard { value: King, suit: Hearts },
        PlayCard { value: King, suit: Diamonds },
        PlayCard { value: King, suit: Clubs },
        PlayCard { value: Ace, suit: Clubs },
        PlayCard { value: Ace, suit: Spades },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &full_house
        ).unwrap(),
        Combination::FullHouse {
            triple: King,
            double: Ace
        },
        "Full house"
    );
}

#[test]
fn flush() {
    let flush = [
        PlayCard { value: King, suit: Hearts },
        PlayCard { value: Jack, suit: Hearts },
        PlayCard { value: Ten, suit: Hearts },
        PlayCard { value: Eight, suit: Hearts },
        PlayCard { value: Five, suit: Hearts },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &flush
        ).unwrap(),
        Combination::Flush(PlayCard {
            value: King,
            suit: Hearts
        }),
        "Flush"
    );
}

#[test]
fn straight() {
    let straight = [
        PlayCard { value: King, suit: Hearts },
        PlayCard { value: Queen, suit: Spades },
        PlayCard { value: Jack, suit: Hearts },
        PlayCard { value: Ten, suit: Hearts },
        PlayCard { value: Nine, suit: Hearts },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &straight
        ).unwrap(),
        Combination::Straight(King),
        "Straight"
    );
}

#[test]
fn straight_wheel() {
    let straight_wheel = [
        PlayCard { value: Two, suit: Spades },
        PlayCard { value: Four, suit: Spades },
        PlayCard { value: Three, suit: Clubs },
        PlayCard { value: Five, suit: Spades },
        PlayCard { value: Ace, suit: Spades },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &straight_wheel
        ).unwrap(),
        Combination::Straight(Five),
        "Straight - wheel"
    );
}

#[test]
fn three_of_a_kind_5() {
    let three_of_a_kind = [
        PlayCard { value: Two, suit: Spades },
        PlayCard { value: Two, suit: Hearts },
        PlayCard { value: Two, suit: Clubs },
        PlayCard { value: Five, suit: Spades },
        PlayCard { value: Ace, suit: Spades },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &three_of_a_kind
        ).unwrap(),
        Combination::ThreeOfAKind(Two),
        "Three of a kind - 5"
    );
}

#[test]
fn three_of_a_kind_3() {
    let three_of_a_kind = [
        PlayCard { value: Four, suit: Spades },
        PlayCard { value: Four, suit: Hearts },
        PlayCard { value: Four, suit: Clubs },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &three_of_a_kind
        ).unwrap(),
        Combination::ThreeOfAKind(Four),
        "Three of a kind - 3"
    );
}

#[test]
fn two_pairs() {
    let two_pairs = [
        PlayCard { value: Seven, suit: Spades },
        PlayCard { value: Jack, suit: Hearts },
        PlayCard { value: Seven, suit: Clubs },
        PlayCard { value: Jack, suit: Spades },
        PlayCard { value: Ace, suit: Spades },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &two_pairs
        ).unwrap(),
        Combination::TwoPairs {
            pair1: Jack,
            pair2: Seven
        },
        "Two pairs"
    );
}

#[test]
fn pair_5() {
    let pair = [
        PlayCard { value: Seven, suit: Spades },
        PlayCard { value: Jack, suit: Hearts },
        PlayCard { value: Eight, suit: Clubs },
        PlayCard { value: Jack, suit: Spades },
        PlayCard { value: Ace, suit: Spades },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &pair
        ).unwrap(),
        Combination::Pair(Jack),
        "Pair - 5"
    );
}

#[test]
fn pair_3() {
    let two_pairs = [
        PlayCard { value: Eight, suit: Clubs },
        PlayCard { value: Ace, suit: Spades },
        PlayCard { value: Ace, suit: Hearts },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &two_pairs
        ).unwrap(),
        Combination::Pair(Ace),
        "Pair - 3"
    );
}

#[test]
fn kicker_5() {
    let kicker = [
        PlayCard { value: Eight, suit: Clubs },
        PlayCard { value: Three, suit: Spades },
        PlayCard { value: King, suit: Hearts },
        PlayCard { value: Two, suit: Hearts },
        PlayCard { value: Five, suit: Hearts },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &kicker
        ).unwrap(),
        Combination::Kicker(King),
        "Pair - 3"
    );
}

#[test]
fn kicker_3() {
    let kicker = [
        PlayCard { value: Eight, suit: Clubs },
        PlayCard { value: Three, suit: Spades },
        PlayCard { value: Five, suit: Hearts },
    ];
    assert_eq!(
        Combination::from_vec_cards(
            &kicker
        ).unwrap(),
        Combination::Kicker(Eight),
        "Pair - 3"
    );
}
