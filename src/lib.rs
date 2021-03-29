/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;
use std::collections::HashMap;
use std::{cmp, convert::TryFrom};

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

#[derive(Debug)]
struct Card {
    suit: Suit,
    numeric_value: u8,
}

impl Card {
    const ACE: u8 = 14;
    const KING: u8 = 13;
    const QUEEN: u8 = 12;
    const JACK: u8 = 11;

    fn new(str_card: &str) -> Result<Self, String> {
        let size = str_card.len();
        if !(2..=3).contains(&size) {
            return Err("Invalid card length".into());
        }

        let value = match size {
            2 => str_card.chars().next().unwrap(),
            3 => {
                let first_2: &str = &str_card[0..2];
                if first_2 == "10" {
                    'T'
                } else {
                    return Err(format!("Invalid card: {}", str_card));
                }
            }
            _ => panic!("Invalid card length"),
        };

        let numeric_value = match value {
            '2'..='9' => value.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => Card::JACK,
            'Q' => Card::QUEEN,
            'K' => Card::KING,
            'A' => Card::ACE,
            _ => return Err("Invalid value".into()),
        };

        let suit = str_card.chars().nth(size - 1).unwrap();
        let suit = match Suit::try_from(suit) {
            Ok(suit) => suit,
            Err(err) => return Err(err),
        };

        Ok(Self {
            suit,
            numeric_value,
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
struct HighCardData {
    cards: Vec<u8>,
}

impl HighCardData {
    fn new(cards: Vec<u8>) -> Self {
        Self { cards }
    }
}

impl PartialOrd for HighCardData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for HighCardData {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..self.cards.len()).rev() {
            let self_value = self.cards[i];
            let other_value = other.cards[i];

            if self_value != other_value {
                return self_value.cmp(&other_value);
            }
        }
        Ordering::Equal
    }
}

#[derive(Eq, PartialEq, Debug)]
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

impl PartialOrd for OnePairData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for OnePairData {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp_pair_value = self.pair_value.cmp(&other.pair_value);
        if cmp_pair_value != Ordering::Equal {
            return cmp_pair_value;
        }
        HighCardData::new(self.remaining_cards.clone())
            .cmp(&HighCardData::new(other.remaining_cards.clone()))
    }
}

#[derive(Eq, PartialEq, Debug)]
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

impl PartialOrd for TwoPairsData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for TwoPairsData {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp_high_pair_value = self.high_pair_value.cmp(&other.high_pair_value);
        if cmp_high_pair_value != Ordering::Equal {
            return cmp_high_pair_value;
        }
        let cmp_low_pair_value = self.low_pair_value.cmp(&other.low_pair_value);
        if cmp_low_pair_value != Ordering::Equal {
            return cmp_low_pair_value;
        }
        self.remaining_card.cmp(&other.remaining_card)
    }
}

#[derive(Eq, PartialEq, Debug)]
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

impl PartialOrd for ThreeOfAKindData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ThreeOfAKindData {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp_value = self.three_of_a_kind_value.cmp(&other.three_of_a_kind_value);
        if cmp_value != Ordering::Equal {
            return cmp_value;
        }
        HighCardData::new(self.remaining_cards.clone())
            .cmp(&HighCardData::new(other.remaining_cards.clone()))
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
        if sv[0] == 2 && sv[sv.len() - 1] == Card::ACE {
            self_vec = vec![1, sv[0], sv[1], sv[2], sv[3]];
        } else {
            self_vec = sv.clone();
        }
        let other_vec: Vec<u8>;
        if ov[0] == 2 && ov[ov.len() - 1] == Card::ACE {
            other_vec = vec![1, ov[0], ov[1], ov[2], ov[3]];
        } else {
            other_vec = ov.clone();
        }

        HighCardData::new(self_vec).cmp(&HighCardData::new(other_vec))
    }
}

#[derive(Eq, PartialEq, Debug)]
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

impl PartialOrd for FullHouseData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for FullHouseData {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp_three = self.three_of_a_kind_value.cmp(&other.three_of_a_kind_value);
        if cmp_three != Ordering::Equal {
            return cmp_three;
        }
        self.pair_value.cmp(&other.pair_value)
    }
}

#[derive(Eq, PartialEq, Debug)]
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

impl PartialOrd for FourOfAKindData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for FourOfAKindData {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.four_of_a_kind_value.cmp(&other.four_of_a_kind_value);
        if cmp != Ordering::Equal {
            return cmp;
        }
        self.remaining_card.cmp(&other.remaining_card)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum HandType {
    StraightFlush(HighCardData),
    FourOfAKind(FourOfAKindData),
    FullHouse(FullHouseData),
    Flush(HighCardData),
    Straight(StraightData),
    ThreeOfAKind(ThreeOfAKindData),
    TwoPairs(TwoPairsData),
    OnePair(OnePairData),
    HighCard(HighCardData),
}

impl HandType {
    fn rank(&self) -> u8 {
        match self {
            HandType::StraightFlush(_) => 9,
            HandType::FourOfAKind(_) => 8,
            HandType::FullHouse(_) => 7,
            HandType::Flush(_) => 6,
            HandType::Straight(_) => 5,
            HandType::ThreeOfAKind(_) => 4,
            HandType::TwoPairs(_) => 3,
            HandType::OnePair(_) => 2,
            HandType::HighCard(_) => 1,
        }
    }
}

#[derive(Eq, Debug)]
struct PokerHand<'a> {
    str_hand: &'a str,
    hand_type: HandType,
}

impl<'a> PokerHand<'a> {
    fn new_from_values(str_hand: &'a str, hand_type: HandType) -> Self {
        Self {
            str_hand,
            hand_type,
        }
    }

    fn new(str_hand: &'a str) -> Result<Self, String> {
        let mut cards: Vec<Card> = Vec::new();
        let split = str_hand.split(' ');
        for s in split {
            let possible_new_card = Card::new(s);
            let new_card = match possible_new_card {
                Ok(card) => card,
                Err(_) => return Err("Cannot create card".into()),
            };
            cards.push(new_card);
        }

        cards.sort_by(|a, b| a.numeric_value.cmp(&b.numeric_value));

        let first_card = &cards[0];
        let mut previous_suit = &first_card.suit;
        let mut previous_value = 0;
        let mut suit_counter = 0;
        let mut straight_counter = 1;

        let mut kind_count_map: HashMap<u8, usize> = HashMap::new();

        let mut starts_at_two = false;

        for card in &cards {
            if &card.suit == previous_suit {
                suit_counter += 1;
            } else {
                suit_counter = 0;
                previous_suit = &card.suit;
            }
            if previous_value == 0 {
                if card.numeric_value == 2 {
                    starts_at_two = true;
                } else {
                    previous_value = card.numeric_value;
                }
            } else if card.numeric_value == (previous_value + 1) {
                straight_counter += 1;
            }
            if card.numeric_value != previous_value {
                previous_value = card.numeric_value;
            }
            *kind_count_map.entry(card.numeric_value).or_insert(0) += 1;
        }
        let is_flush = suit_counter == 5;
        let is_straight = PokerHand::is_straight(
            straight_counter,
            starts_at_two,
            cards[cards.len() - 1].numeric_value,
        );

        let sorted_numeric_values: Vec<u8> = cards.iter().map(|c| c.numeric_value).collect();

        if is_flush {
            if is_straight {
                return Ok(PokerHand::new_from_values(
                    str_hand,
                    HandType::StraightFlush(HighCardData::new(sorted_numeric_values)),
                ));
            } else {
                return Ok(PokerHand::new_from_values(
                    str_hand,
                    HandType::Flush(HighCardData::new(sorted_numeric_values)),
                ));
            }
        }

        if is_straight {
            return Ok(PokerHand::new_from_values(
                str_hand,
                HandType::Straight(StraightData::new(sorted_numeric_values)),
            ));
        }

        let mut has_three = false;
        let mut pair_count = 0;

        let mut three_of_a_kind_value: u8 = 0;

        let mut pairs: Vec<u8> = Vec::new();

        for (key, value) in kind_count_map {
            if value == 4 {
                let the_other_card: Vec<&u8> = sorted_numeric_values
                    .iter()
                    .filter(|v| *v != &key)
                    .collect();
                return Ok(PokerHand::new_from_values(
                    str_hand,
                    HandType::FourOfAKind(FourOfAKindData::new(key, *the_other_card[0])),
                ));
            } else if value == 3 {
                has_three = true;
                three_of_a_kind_value = key;
            } else if value == 2 {
                pairs.push(key);
                pair_count += 1;
            }
        }

        if has_three {
            if pair_count == 1 {
                return Ok(PokerHand::new_from_values(
                    str_hand,
                    HandType::FullHouse(FullHouseData::new(three_of_a_kind_value, pairs[0])),
                ));
            } else {
                let remaining_cards: Vec<u8> = sorted_numeric_values
                    .into_iter()
                    .filter(|v| *v != three_of_a_kind_value)
                    .collect();
                return Ok(PokerHand::new_from_values(
                    str_hand,
                    HandType::ThreeOfAKind(ThreeOfAKindData::new(
                        three_of_a_kind_value,
                        remaining_cards,
                    )),
                ));
            }
        }

        if pair_count == 2 {
            let highest_pair = cmp::max(pairs[0], pairs[1]);
            let lowest_pair = cmp::min(pairs[0], pairs[1]);
            let the_other_card: Vec<&u8> = sorted_numeric_values
                .iter()
                .filter(|v| *v != &highest_pair && *v != &lowest_pair)
                .collect();
            return Ok(PokerHand::new_from_values(
                str_hand,
                HandType::TwoPairs(TwoPairsData::new(
                    highest_pair,
                    lowest_pair,
                    *the_other_card[0],
                )),
            ));
        }

        if pair_count == 1 {
            let pair_value = pairs[0];
            let remaining_cards: Vec<u8> = sorted_numeric_values
                .into_iter()
                .filter(|v| *v != pair_value)
                .collect();
            return Ok(PokerHand::new_from_values(
                str_hand,
                HandType::OnePair(OnePairData::new(pair_value, remaining_cards)),
            ));
        }

        Ok(PokerHand::new_from_values(
            str_hand,
            HandType::HighCard(HighCardData::new(sorted_numeric_values)),
        ))
    }

    fn is_straight(
        straight_counter: usize,
        starts_at_two: bool,
        last_card_numeric_value: u8,
    ) -> bool {
        straight_counter == 5
            || PokerHand::is_straight_starting_with_ace(
                straight_counter,
                starts_at_two,
                last_card_numeric_value,
            )
    }

    fn is_straight_starting_with_ace(
        straight_counter: usize,
        starts_at_two: bool,
        last_card_numeric_value: u8,
    ) -> bool {
        straight_counter == 4 && starts_at_two && last_card_numeric_value == Card::ACE
    }
}

impl<'a> Ord for PokerHand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_cmp = self.hand_type.rank().cmp(&other.hand_type.rank());
        if rank_cmp != Ordering::Equal {
            return rank_cmp;
        }
        self.hand_type.cmp(&other.hand_type)
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
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

    let mut processed_hands: Vec<PokerHand> = Vec::new();

    for hand in hands {
        let processed_hand = match PokerHand::new(hand) {
            Ok(processed_hand) => processed_hand,
            Err(_) => return None,
        };

        processed_hands.push(processed_hand);
    }

    processed_hands.sort();
    processed_hands.reverse();

    let mut result = Vec::new();
    let highest_hand = &processed_hands[0];
    result.push(highest_hand.str_hand);

    for processed_hand in processed_hands.iter().skip(1) {
        if processed_hand.cmp(highest_hand) == Ordering::Equal {
            result.push(processed_hand.str_hand);
        } else {
            break;
        }
    }

    Some(result)
}
