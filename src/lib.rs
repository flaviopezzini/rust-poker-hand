use std::collections::HashMap;
use std::{cmp, convert::TryFrom};
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl TryFrom<char> for Suit {
    type Error = String;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'C' => Ok(Suit::Clubs),
            'D' => Ok(Suit::Diamonds),
            'H' => Ok(Suit::Hearts),
            'S' => Ok(Suit::Spades),
            _ => Err("Invalid Suit".into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "10" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err("Invalid rank".into()),
        }
    }
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl FromStr for Card {
    type Err = String;
    fn from_str(str_card: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let size = str_card.chars().count();
        if !(2..=3).contains(&size) {
            return Err("Invalid card length".into());
        }

        let value = if size == 2 {
            &str_card[0..1]
        } else {
            &str_card[0..2]
        };

        let rank = Rank::from_str(value)?;

        let suit = str_card.chars().last().unwrap();
        let suit = Suit::try_from(suit)?;

        Ok(Card::new(suit, rank))
    }
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
struct HighCardData {
    cards: Vec<u8>,
}

impl HighCardData {
    fn new(cards: Vec<u8>) -> Self {
        Self { cards }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
struct OnePairData {
    pair_value: u8,
    remaining_cards: Vec<u8>,
}

impl OnePairData {
    fn new(pair_value: u8, remaining_cards: Vec<u8>) -> Self {
        Self {
            pair_value,
            remaining_cards,
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
struct TwoPairsData {
    high_pair_value: u8,
    low_pair_value: u8,
    remaining_card: u8,
}

impl TwoPairsData {
    fn new(high_pair_value: u8, low_pair_value: u8, remaining_card: u8) -> Self {
        Self {
            high_pair_value,
            low_pair_value,
            remaining_card,
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct ThreeOfAKindData {
    three_of_a_kind_value: u8,
    remaining_cards: Vec<u8>,
}

impl ThreeOfAKindData {
    fn new(three_of_a_kind_value: u8, remaining_cards: Vec<u8>) -> Self {
        Self {
            three_of_a_kind_value,
            remaining_cards,
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct StraightData {
    cards: Vec<u8>,
}

impl StraightData {
    fn new(cards: Vec<u8>) -> Self {
        Self { cards }
    }
}

impl PartialOrd for StraightData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for StraightData {
    fn cmp(&self, other: &Self) -> Ordering {
        let sv = &self.cards;
        let ov = &other.cards;
        let self_vec: Vec<u8>;
        if sv[0] == Rank::Two as u8 && sv.iter().last().unwrap() == &(Rank::Ace as u8) {
            self_vec = vec![1, sv[0], sv[1], sv[2], sv[3]];
        } else {
            self_vec = sv.clone();
        }
        let other_vec: Vec<u8>;
        if ov[0] == Rank::Two as u8 && ov.iter().last().unwrap() == &(Rank::Ace as u8) {
            other_vec = vec![1, ov[0], ov[1], ov[2], ov[3]];
        } else {
            other_vec = ov.clone();
        }

        HighCardData::new(self_vec).cmp(&HighCardData::new(other_vec))
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
struct FullHouseData {
    three_of_a_kind_value: u8,
    pair_value: u8,
}

impl FullHouseData {
    fn new(three_of_a_kind_value: u8, pair_value: u8) -> Self {
        Self {
            three_of_a_kind_value,
            pair_value,
        }
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
struct FourOfAKindData {
    four_of_a_kind_value: u8,
    remaining_card: u8,
}

impl FourOfAKindData {
    fn new(four_of_a_kind_value: u8, remaining_card: u8) -> Self {
        Self {
            four_of_a_kind_value,
            remaining_card,
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum HandType {
    HighCard(HighCardData),
    OnePair(OnePairData),
    TwoPairs(TwoPairsData),
    ThreeOfAKind(ThreeOfAKindData),
    Straight(StraightData),
    Flush(HighCardData),
    FullHouse(FullHouseData),
    FourOfAKind(FourOfAKindData),
    StraightFlush(StraightData),
}

#[derive(Eq, Debug, Ord, PartialOrd, PartialEq)]
struct PokerHand {
    hand_type: HandType,
}

impl FromStr for PokerHand {
    type Err = String;
    fn from_str(str_hand: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut cards: Vec<Card> = Vec::new();
        let split = str_hand.split_whitespace();
        for s in split {
            let possible_new_card = Card::from_str(s);
            let new_card = match possible_new_card {
                Ok(card) => card,
                Err(_) => return Err("Cannot create card".into()),
            };
            cards.push(new_card);
        }

        cards.sort_unstable_by(|a, b| a.rank.cmp(&b.rank));

        let is_flush = PokerHand::is_flush(&cards);
        let is_straight = PokerHand::is_straight(&cards);

        let sorted_numeric_values: Vec<u8> = cards.iter().map(|c| c.rank as u8).collect();

        if is_flush && is_straight {
            return Ok(PokerHand::new(HandType::StraightFlush(StraightData::new(
                sorted_numeric_values,
            ))));
        } else if is_flush {
            return Ok(PokerHand::new(HandType::Flush(HighCardData::new(
                sorted_numeric_values,
            ))));
        } else if is_straight {
            return Ok(PokerHand::new(HandType::Straight(StraightData::new(
                sorted_numeric_values,
            ))));
        }

        let kind_count_map: HashMap<u8, usize> = cards.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c.rank as u8).or_insert(0) += 1;
            acc
        });

        let mut pairs: Vec<u8> = Vec::new();

        let four_of_a_kind_value = PokerHand::get_many_cards_with_same_value(&kind_count_map, 4);
        if let Some(four_value) = four_of_a_kind_value {
            let the_other_card: Vec<&u8> = sorted_numeric_values
                .iter()
                .filter(|v| **v != four_value)
                .collect();
            return Ok(PokerHand::new(HandType::FourOfAKind(FourOfAKindData::new(
                four_value,
                *the_other_card[0],
            ))));
        }

        for (key, value) in &kind_count_map {
            if *value == 2 {
                pairs.push(*key);
            }
        }

        let three_of_a_kind_value = PokerHand::get_many_cards_with_same_value(&kind_count_map, 3);
        if let Some(three_value) = three_of_a_kind_value {
            if pairs.len() == 1 {
                return Ok(PokerHand::new(HandType::FullHouse(FullHouseData::new(
                    three_value,
                    pairs[0],
                ))));
            } else {
                let remaining_cards: Vec<u8> = sorted_numeric_values
                    .into_iter()
                    .filter(|v| *v != three_value)
                    .collect();
                return Ok(PokerHand::new(HandType::ThreeOfAKind(
                    ThreeOfAKindData::new(three_value, remaining_cards),
                )));
            }
        }

        if pairs.len() == 2 {
            let highest_pair = cmp::max(pairs[0], pairs[1]);
            let lowest_pair = cmp::min(pairs[0], pairs[1]);
            let the_other_card: Vec<&u8> = sorted_numeric_values
                .iter()
                .filter(|v| *v != &highest_pair && *v != &lowest_pair)
                .collect();
            return Ok(PokerHand::new(HandType::TwoPairs(TwoPairsData::new(
                highest_pair,
                lowest_pair,
                *the_other_card[0],
            ))));
        }

        if pairs.len() == 1 {
            let pair_value = pairs[0];
            let remaining_cards: Vec<u8> = sorted_numeric_values
                .into_iter()
                .filter(|v| *v != pair_value)
                .collect();
            return Ok(PokerHand::new(HandType::OnePair(OnePairData::new(
                pair_value,
                remaining_cards,
            ))));
        }

        Ok(PokerHand::new(HandType::HighCard(HighCardData::new(
            sorted_numeric_values,
        ))))
    }
}

impl PokerHand {
    fn new(hand_type: HandType) -> Self {
        Self { hand_type }
    }

    fn is_flush(cards: &Vec<Card>) -> bool {
        let first_card = &cards[0];
        cards.iter().all(|item| item.suit == first_card.suit)
    }

    fn is_straight(cards: &Vec<Card>) -> bool {
        cards
            .windows(2)
            .all(|w| w[1].rank as u8 == w[0].rank as u8 + 1)
            || PokerHand::is_straight_from_ace(cards)
    }

    fn is_straight_from_ace(cards: &Vec<Card>) -> bool {
        cards[0].rank == Rank::Two
            && cards[1].rank == Rank::Three
            && cards[2].rank == Rank::Four
            && cards[3].rank == Rank::Five
            && cards[4].rank == Rank::Ace
    }

    fn get_many_cards_with_same_value(
        kind_count_map: &HashMap<u8, usize>,
        how_many: usize,
    ) -> Option<u8> {
        return kind_count_map
            .iter()
            .fold(None, |mut three_of_a_kind, (k, v)| {
                if *v == how_many {
                    three_of_a_kind = Some(*k);
                }
                three_of_a_kind
            });
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let size = hands.len();
    if size == 0 {
        return None;
    }
    if size == 1 {
        return Some(vec![hands[0]]);
    }

    let mut processed_hands: Vec<(&str, PokerHand)> = Vec::new();

    for hand in hands {
        let processed_hand = match PokerHand::from_str(hand) {
            Ok(processed_hand) => processed_hand,
            Err(_) => return None,
        };

        processed_hands.push((hand, processed_hand));
    }

    processed_hands.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut result = Vec::new();
    let highest_hand = &processed_hands[0];
    result.push(highest_hand.0);

    for processed_hand in processed_hands.iter().skip(1) {
        if processed_hand.1.cmp(&highest_hand.1) == Ordering::Equal {
            result.push(processed_hand.0);
        } else {
            break;
        }
    }

    Some(result)
}
