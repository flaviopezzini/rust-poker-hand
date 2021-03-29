/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::cmp::Ordering;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
struct Card {
  suit: char,
  numeric_value: u8
}

impl Card {
  const ACE: u8 = 14;
  const KING: u8 = 13;
  const QUEEN: u8 = 12;
  const JACK: u8 = 11;

  fn new(str_card: &str) -> Result<Self, String> {
    let size = str_card.len();
    if size < 2 || size > 3 {
      return Err("Invalid card length".into());
    }

    let value = match size {
        2 => str_card.chars().nth(0).unwrap(),
        3 => {
            let first_2: &str = &str_card[0..2];
            if &first_2 == &"10" {
                'T'
            } else {
                return Err(format!("Invalid card: {}", str_card));
            }
        },
        _ => panic!("Invalid card length"),
    };

    let suit = str_card.chars().nth(size - 1).unwrap();
    if suit != 'D' && suit != 'C' && suit != 'S' && suit != 'H' {
        return Err(format!("Invalid suit: {}", suit));
    }
    let numeric_value = match value {
        '2'..='9' => value.to_digit(10).unwrap() as u8, 
        'T' => 10,
        'J' => Card::JACK,
        'Q' => Card::QUEEN,
        'K' => Card::KING,
        'A' => Card::ACE,
        _ => return Err("Invalid value".into()),
    };

    Ok(Self {
        suit, numeric_value
    })
  }

}

#[derive(Eq, PartialEq, Ord, Debug)]
struct HighCardData {
    cards: Vec<u8>,
}

impl HighCardData {
    fn new(cards: Vec<u8>) -> Self {
        Self {
            cards: cards,
        }
    }
}

impl PartialOrd for HighCardData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        for i in (0..self.cards.len()).rev() {
            let self_value = self.cards[i];
            let other_value = other.cards[i];
      
            if self_value != other_value {
              return Some(other_value.cmp(&self_value));
            }
        }
        Some(Ordering::Equal)
    }
}

#[derive(Eq, PartialEq, Ord, Debug)]
struct OnePairData {
    pair_value: u8,
    remaining_cards: Vec<u8>,
}

impl OnePairData {
    fn new(pair_value: u8, remaining_cards: Vec<u8>) -> Self {
        Self {
            pair_value, remaining_cards,
        }
    }
}

impl PartialOrd for OnePairData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp_pair_value = self.pair_value.cmp(&other.pair_value);
        if cmp_pair_value != Ordering::Equal {
            return Some(cmp_pair_value);
        }
        Some(HighCardData::new(
            self.remaining_cards.clone()).cmp(&HighCardData::new(other.remaining_cards.clone())))
    }
}

#[derive(Eq, PartialEq, Debug)]
enum HandType {
  StraightFlush(HighCardData), FourOfAKind(u8,u8), FullHouse(u8, u8), Flush(HighCardData), Straight(HighCardData), ThreeOfAKind(u8, Vec<u8>), 
       TwoPairs(u8, u8, u8), OnePair(u8, Vec<u8>), HighCard(HighCardData),
}

impl HandType {
    fn rank(&self) -> u8 {
        match self {
            HandType::StraightFlush(_) => 9,
            HandType::FourOfAKind(_, _) => 8,
            HandType::FullHouse(_,_) => 7,
            HandType::Flush(_) => 6, 
            HandType::Straight(_) => 5,
            HandType::ThreeOfAKind(_,_) => 4,
            HandType::TwoPairs(_,_,_) => 3,
            HandType::OnePair(_,_) => 2,
            HandType::HighCard(_) => 1
        }
    }
}

#[derive(Eq, Debug)]
struct PokerHand<'a> {
    str_hand: &'a str,
    hand_type: HandType,
}

impl<'a> PokerHand<'a> {

  fn new_from_values(
      str_hand: &'a str, 
      hand_type: HandType) -> Self {
    Self {
      str_hand,
      hand_type,
    }
  }

  fn new(str_hand: &'a str) -> Result<Self, String> {
    let mut cards : Vec<Card> = Vec::new();
    let split = str_hand.split(" ");
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
    let mut previous_suit = first_card.suit;
    let mut previous_value = 0;
    let mut suit_counter = 0;
    let mut straight_counter = 1;

    let mut kind_count_map : HashMap<u8, usize> = HashMap::new();

    let mut starts_at_two = false;

    for card in &cards {
        if card.suit == previous_suit {
            suit_counter += 1;
        } else {
            suit_counter = 0;
            previous_suit = card.suit;
        }
        if previous_value == 0 {
            if card.numeric_value == 2 {
                starts_at_two = true;
            } else {
                previous_value = card.numeric_value;
            }
        } else {
            if card.numeric_value == (previous_value + 1) {
                straight_counter += 1;
            }
        }
        if card.numeric_value != previous_value {
            previous_value = card.numeric_value;
        }
        *kind_count_map.entry(card.numeric_value).or_insert(0) += 1;
    }
    let is_flush = suit_counter == 5;
    let is_straight = PokerHand::is_straight(straight_counter, starts_at_two, cards[cards.len() - 1].numeric_value);

    let sorted_numeric_values : Vec<u8> = cards.iter().map(|c| c.numeric_value).collect();

    if is_flush {
        if is_straight {
            return Ok(PokerHand::new_from_values(
                str_hand, 
                HandType::StraightFlush(HighCardData::new(sorted_numeric_values))));
        } else {
            return Ok(PokerHand::new_from_values(
                str_hand, 
                HandType::Flush(HighCardData::new(sorted_numeric_values))));
        }
    }

    if is_straight {
        return Ok(PokerHand::new_from_values(
            str_hand, 
            HandType::Straight(HighCardData::new(sorted_numeric_values))));
    }

    let mut has_three = false;
    let mut pair_count = 0;

    let mut three_of_a_kind_value: u8 = 0;

    let mut pairs : Vec<u8> = Vec::new();

    for (key, value) in kind_count_map {
        if value == 4 {
            let the_other_card: Vec<&u8> = sorted_numeric_values.iter().filter(|v| *v != &key).collect();
            return Ok(PokerHand::new_from_values(str_hand, HandType::FourOfAKind(key, *the_other_card[0])));
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
                HandType::FullHouse(three_of_a_kind_value, pairs[0])));
        } else {
            let remaining_cards: Vec<u8> = sorted_numeric_values.into_iter().filter(|v| *v != three_of_a_kind_value).collect();
            return Ok(PokerHand::new_from_values(
                str_hand, 
                HandType::ThreeOfAKind(three_of_a_kind_value, remaining_cards)));
        }
    }

    if pair_count == 2 {
        let highest_pair = cmp::max(pairs[0], pairs[1]);
        let lowest_pair = cmp::min(pairs[0], pairs[1]);
        let the_other_card: Vec<&u8> = sorted_numeric_values.iter().filter(|v| *v != &highest_pair && *v != &lowest_pair).collect();
        return Ok(PokerHand::new_from_values(
            str_hand, 
            HandType::TwoPairs(highest_pair, lowest_pair, *the_other_card[0])));
    }

    if pair_count == 1 {
        let pair_value = pairs[0];
        let remaining_cards: Vec<u8> = sorted_numeric_values.into_iter().filter(|v| *v != pair_value).collect();
        return Ok(PokerHand::new_from_values(
            str_hand, 
            HandType::OnePair(pair_value, remaining_cards)));
    }

    Ok(PokerHand::new_from_values(
        str_hand, 
        HandType::HighCard(HighCardData::new(sorted_numeric_values))))
  }

  fn is_straight(straight_counter: usize, starts_at_two: bool, last_card_numeric_value: u8) -> bool {
    straight_counter == 5 || PokerHand::is_straight_starting_with_ace(straight_counter, starts_at_two, last_card_numeric_value)
  }

  fn is_straight_starting_with_ace(straight_counter: usize, starts_at_two: bool, last_card_numeric_value: u8) -> bool {
    straight_counter == 4 && starts_at_two && last_card_numeric_value == Card::ACE
  }

  fn compare_four_of_a_kind(self_kind_value: &u8, sv: &u8, other_kind_value: &u8, ov: &u8) -> Ordering {
    let cmp_kind_value = other_kind_value.cmp(&self_kind_value);
    if cmp_kind_value != Ordering::Equal {
        return cmp_kind_value;
    }
    ov.cmp(&sv)    
  }

  fn compare_full_house(self_three_value: &u8, self_pair_value: &u8, other_three_value: &u8, other_pair_value: &u8) -> Ordering {
    let cmp_three_value = other_three_value.cmp(&self_three_value);
    if cmp_three_value != Ordering::Equal {
        return cmp_three_value;
    }
    other_pair_value.cmp(&self_pair_value)
  }

  fn compare_three_of_a_kind(self_kind_value: &u8, sv: &Vec<u8>, other_kind_value: &u8, ov: &Vec<u8>) -> Ordering {
    let cmp_kind_value = other_kind_value.cmp(&self_kind_value);
    if cmp_kind_value != Ordering::Equal {
        return cmp_kind_value;
    }
    HighCardData::new(ov.clone()).cmp(&HighCardData::new(sv.clone()))
  }

  fn compare_two_pairs(self_high_pair: &u8, self_low_pair: &u8, sv: &u8, other_high_pair: &u8, other_low_pair: &u8, ov: &u8) -> Ordering {
    let cmp_high_pair = other_high_pair.cmp(&self_high_pair);
    if cmp_high_pair != Ordering::Equal {
        return cmp_high_pair;
    }
    let cmp_low_pair = other_low_pair.cmp(&self_low_pair);
    if cmp_low_pair != Ordering::Equal {
        return cmp_low_pair;
    }

    ov.cmp(&sv)
  }

}

impl<'a> Ord for PokerHand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let value_cmp = other.hand_type.rank().cmp(&self.hand_type.rank());
        if value_cmp != Ordering::Equal {
            return value_cmp;
        }

        // break ties
        let cmp_result = match (&self.hand_type, &other.hand_type) {
            (HandType::StraightFlush(sv), HandType::StraightFlush(ov)) |
            (HandType::Flush(sv), HandType::Flush(ov)) |
            (HandType::HighCard(sv), HandType::HighCard(ov)) => ov.cmp(&sv),
            (HandType::Straight(high_card_sv), HandType::Straight(high_card_ov)) => {
                let sv = &high_card_sv.cards;
                let ov = &high_card_ov.cards;
                let self_vec: Vec<u8>;
                let sv_last = sv.len() - 1;
                if sv[0] == 2 && sv[sv_last] == Card::ACE {
                    self_vec = vec![1, sv[0], sv[1], sv[2], sv[3]];
                } else {
                    self_vec = sv.clone();
                }
                let other_vec: Vec<u8>;
                let ov_last = ov.len() - 1;
                if ov[0] == 2 && ov[sv_last] == Card::ACE {
                    other_vec = vec![ov[ov_last], ov[0], ov[1], ov[2], ov[3]];
                } else {
                    other_vec = ov.clone();
                }

                HighCardData::new(other_vec).cmp(&HighCardData::new(self_vec))
            },
            (HandType::FourOfAKind(self_kind_value, sv), HandType::FourOfAKind(other_kind_value, ov)) => 
                PokerHand::compare_four_of_a_kind(self_kind_value, sv, other_kind_value, ov),
            (HandType::FullHouse(self_three_value, self_pair_value), HandType::FullHouse(other_three_value, other_pair_value)) => 
                PokerHand::compare_full_house(self_three_value, self_pair_value, other_three_value, other_pair_value),
            (HandType::ThreeOfAKind(self_kind_value, sv), HandType::ThreeOfAKind(other_kind_value, ov)) => 
                PokerHand::compare_three_of_a_kind(self_kind_value, sv, other_kind_value, ov),
            (HandType::TwoPairs(self_high_pair, self_low_pair, sv), HandType::TwoPairs(other_high_pair, other_low_pair, ov)) => 
                PokerHand::compare_two_pairs(self_high_pair, self_low_pair, sv, other_high_pair, other_low_pair, ov),
            (HandType::OnePair(self_pair_value, sv), HandType::OnePair(other_pair_value, ov)) => 
                OnePairData::new(*other_pair_value, ov.clone()).cmp(&OnePairData::new(*self_pair_value, sv.clone())),
            (_,_) => panic!("Unexpected match!")
        };

        cmp_result
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

    let mut processed_hands : Vec<PokerHand> = Vec::new();

    for hand in hands {
        let hand_type = match PokerHand::new(hand) {
            Ok(hand_type) => hand_type,
            Err(_) => return None,
        };
        
        processed_hands.push(hand_type);
    }

    processed_hands.sort();

    let mut result = Vec::new();
    let highest_hand = &processed_hands[0];
    result.push(highest_hand.str_hand);

    for i in 1..processed_hands.len() {
        let current = &processed_hands[i];
        
        if current.cmp(highest_hand) == Ordering::Equal {
            result.push(current.str_hand);
        } else {
            break;
        }
    }
        
    Some(result)
}
