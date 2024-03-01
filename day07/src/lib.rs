pub mod part_a {
    use core::panic;
    use std::{collections::HashMap, iter::zip};

    use crate::part_b::HandJoker;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Hand {
        value: usize,
        cards: [char; 5],
    }

    impl Hand {
        pub fn new(value: usize, cards: [char; 5]) -> Hand {
            Hand { value, cards }
        }

        pub fn get_value(&self) -> usize {
            self.value
        }

        pub fn get_cards(&self) -> [char; 5] {
            self.cards
        }

        pub fn from_hand_joker(hand_joker: &HandJoker) -> Hand {
            Hand {
                value: hand_joker.get_value(),
                cards: hand_joker.get_cards(),
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let self_sig = get_sig(&self);
            let other_sig = get_sig(&other);

            if self_sig > other_sig {
                std::cmp::Ordering::Greater
            } else if self_sig < other_sig {
                std::cmp::Ordering::Less
            } else {
                // if they have the same sig, we compare starting with the first ch
                let mut key_val_pairs = vec![('A', 13), ('K', 12), ('Q', 11), ('J', 10), ('T', 9)];
                for i in (2..=9).rev() {
                    key_val_pairs.push((char::from_digit(i, 10).unwrap(), i - 1));
                }

                let ch_map: HashMap<char, u32> = key_val_pairs.iter().cloned().collect();

                for (self_ch, other_ch) in zip(self.cards, other.cards) {
                    let self_val = ch_map.get(&self_ch).unwrap();
                    let other_val = ch_map.get(&other_ch).unwrap();

                    if self_val > other_val {
                        return std::cmp::Ordering::Greater;
                    }
                    if self_val < other_val {
                        return std::cmp::Ordering::Less;
                    }
                }

                panic!("Did not expect cards to be equal");
            }
        }
    }

    pub fn parse_input(input_str: &str) -> Vec<Hand> {
        input_str
            .lines()
            .map(|line| {
                let mut portions = line.split(" ");

                let cards_str = portions.next().unwrap();
                let value = portions.next().unwrap().parse::<usize>().unwrap();

                let cards_vec = cards_str.chars().collect::<Vec<char>>();
                let cards: [char; 5] = cards_vec.try_into().unwrap();

                Hand { cards, value }
            })
            .collect::<Vec<Hand>>()
    }

    pub fn is_five_of_a_kind(hand: &Hand) -> bool {
        let first_char = hand.cards[0];

        for i in 1..5 {
            if first_char != hand.cards[i] {
                return false;
            }
        }

        true
    }

    pub fn is_four_of_a_kind(hand: &Hand) -> bool {
        for i in 0..2 {
            let comparison_char = hand.cards[i];
            let mut differences = 0;

            for j in 0..5 {
                if hand.cards[j] != comparison_char {
                    differences += 1;
                }
            }

            if differences == 1 {
                return true;
            }
        }

        false
    }

    pub fn is_full_house(hand: &Hand) -> bool {
        let mut sorted = hand.cards;
        sorted.sort();

        if (sorted[0] == sorted[1] && sorted[1] == sorted[2]) && (sorted[3] == sorted[4]) {
            return true;
        }
        if (sorted[0] == sorted[1]) && (sorted[2] == sorted[3] && sorted[3] == sorted[4]) {
            return true;
        }

        false
    }

    pub fn is_three_of_a_kind(hand: &Hand) -> bool {
        let mut sorted = hand.cards;
        sorted.sort();

        if sorted[0] == sorted[1] && sorted[1] == sorted[2] {
            return true;
        }
        if sorted[2] == sorted[3] && sorted[3] == sorted[4] {
            return true;
        }
        if sorted[1] == sorted[2] && sorted[2] == sorted[3] {
            return true;
        }

        false
    }

    pub fn is_two_pair(hand: &Hand) -> bool {
        // AABCC
        // AABBC
        // ABBCC
        let mut sorted = hand.cards;
        sorted.sort();

        if sorted[0] == sorted[1] && sorted[2] == sorted[3] {
            return true;
        }
        if sorted[0] == sorted[1] && sorted[3] == sorted[4] {
            return true;
        }
        if sorted[1] == sorted[2] && sorted[3] == sorted[4] {
            return true;
        }

        false
    }

    pub fn is_one_pair(hand: &Hand) -> bool {
        // AAXXX
        // XAAXX
        // XXAAX
        // XXXAA
        let mut sorted = hand.cards;
        sorted.sort();

        for i in 0..4 {
            if sorted[i] == sorted[i + 1] {
                return true;
            }
        }

        false
    }

    pub fn get_sig(hand: &Hand) -> usize {
        // can be from 6 to 0
        if is_five_of_a_kind(hand) {
            return 6;
        }
        if is_four_of_a_kind(hand) {
            return 5;
        }
        if is_full_house(hand) {
            return 4;
        }
        if is_three_of_a_kind(hand) {
            return 3;
        }
        if is_two_pair(hand) {
            return 2;
        }
        if is_one_pair(hand) {
            return 1;
        }
        0
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let mut hands = parse_input(input_str);
        hands.sort();

        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.value * (idx + 1))
            .sum()
    }
}

pub mod part_b {
    use std::{collections::HashMap, iter::zip};

    use crate::part_a::{
        is_four_of_a_kind, is_full_house, is_one_pair, is_three_of_a_kind, is_two_pair, Hand,
    };

    // we need to make a new struct here because we are
    // changing the implementation of Ord and PartialOrd
    #[derive(Debug, Eq, PartialEq)]
    pub struct HandJoker {
        value: usize,
        cards: [char; 5],
    }

    impl PartialOrd for HandJoker {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for HandJoker {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let self_sig = get_sig_joker(&self);
            let other_sig = get_sig_joker(&other);

            if self_sig > other_sig {
                std::cmp::Ordering::Greater
            } else if self_sig < other_sig {
                std::cmp::Ordering::Less
            } else {
                // if they have the same sig, we compare starting with the first ch
                let mut key_val_pairs = vec![('A', 13), ('K', 12), ('Q', 11), ('J', 0), ('T', 9)];
                for i in (2..=9).rev() {
                    key_val_pairs.push((char::from_digit(i, 10).unwrap(), i - 1));
                }

                let ch_map: HashMap<char, u32> = key_val_pairs.iter().cloned().collect();

                for (self_ch, other_ch) in zip(self.cards, other.cards) {
                    let self_val = ch_map.get(&self_ch).unwrap();
                    let other_val = ch_map.get(&other_ch).unwrap();

                    if self_val > other_val {
                        return std::cmp::Ordering::Greater;
                    }
                    if self_val < other_val {
                        return std::cmp::Ordering::Less;
                    }
                }

                panic!("Did not expect cards to be equal");
            }
        }
    }

    impl HandJoker {
        pub fn new(value: usize, cards: [char; 5]) -> HandJoker {
            HandJoker { value, cards }
        }

        pub fn get_value(&self) -> usize {
            self.value
        }

        pub fn get_cards(&self) -> [char; 5] {
            self.cards
        }
    }

    pub fn is_five_of_a_kind_joker(hand_joker: &HandJoker) -> bool {
        // first we need to grab one of the chars which is not a joker
        let mut non_joker = 'J';
        for ch in hand_joker.get_cards() {
            if ch != 'J' {
                non_joker = ch;
                break;
            }
        }

        // if all the cards are jokers, then return true
        if non_joker == 'J' {
            return true;
        }

        for ch in hand_joker.get_cards() {
            if ch != 'J' && ch != non_joker {
                return false;
            }
        }

        true
    }

    pub fn is_four_of_a_kind_joker(hand_joker: &HandJoker, joker_count: usize) -> bool {
        // if 3 jokers, can always make 4 of a kind
        if joker_count == 3 {
            return true;
        }

        // if 2 jokers, check if there is two pair no joker
        if joker_count == 2 && is_two_pair(&Hand::from_hand_joker(&hand_joker)) {
            return true;
        }

        // if 1 joker, then there needs to be 3 of a kind no joker
        if joker_count == 1 && is_three_of_a_kind(&Hand::from_hand_joker(&hand_joker)) {
            return true;
        }

        // if there is 4 of a kind normally...
        if is_four_of_a_kind(&Hand::from_hand_joker(&hand_joker)) {
            return true;
        }

        false
    }

    pub fn is_full_house_joker(hand_joker: &HandJoker, joker_count: usize) -> bool {
        // if joker count is > 2 we can have 4 of a kind
        // if joker count is 2, we need two pair no joker
        if joker_count == 2 && is_two_pair(&Hand::from_hand_joker(&hand_joker)) {
            return true;
        }

        // if joker count is 1, we need two pair no joker
        if joker_count == 1 && is_two_pair(&Hand::from_hand_joker(&hand_joker)) {
            return true;
        }

        // we can get full house normally
        if is_full_house(&Hand::from_hand_joker(&hand_joker)) {
            return true;
        }

        false
    }

    pub fn is_three_of_a_kind_joker(hand_joker: &HandJoker, joker_count: usize) -> bool {
        // if 2 jokers, can always make 3 of a kind
        if joker_count == 2 {
            return true;
        }

        // if 1 joker, there needs to be one pair no joker
        if joker_count == 1 && is_one_pair(&Hand::from_hand_joker(&hand_joker)) {
            return true;
        }

        // if no jokers, there can be no joker
        if is_three_of_a_kind(&Hand::from_hand_joker(&hand_joker)) {
            return true;
        }

        false
    }

    pub fn is_two_pair_joker(hand_joker: &HandJoker, joker_count: usize) -> bool {
        // only need to consider joker count = 1 because if there are 2 jokers
        // we can always make 3 of a kind

        // if 1 joker, we just need 1 pair
        if joker_count == 1 && is_one_pair(&Hand::from_hand_joker(&hand_joker)) {
            return false;
        }

        // if no jokers, we can get it normally
        is_two_pair(&Hand::from_hand_joker(&hand_joker))
    }

    pub fn is_one_pair_joker(hand_joker: &HandJoker, joker_count: usize) -> bool {
        // again, only need to consider case of 1 joker
        if joker_count == 1 {
            return true;
        }

        is_one_pair(&Hand::from_hand_joker(&hand_joker))
    }

    pub fn get_sig_joker(hand_joker: &HandJoker) -> usize {
        // first we need to count the jokers
        let mut joker_count = 0;
        for ch in hand_joker.get_cards() {
            if ch == 'J' {
                joker_count += 1;
            }
        }
        if is_five_of_a_kind_joker(hand_joker) {
            return 6;
        }
        if is_four_of_a_kind_joker(hand_joker, joker_count) {
            return 5;
        }
        if is_full_house_joker(hand_joker, joker_count) {
            return 4;
        }
        if is_three_of_a_kind_joker(hand_joker, joker_count) {
            return 3;
        }
        if is_two_pair_joker(hand_joker, joker_count) {
            return 2;
        }
        if is_one_pair_joker(hand_joker, joker_count) {
            return 1;
        }
        0
    }

    pub fn parse_input(input_str: &str) -> Vec<HandJoker> {
        input_str
            .lines()
            .map(|line| {
                let mut portions = line.split(" ");

                let cards_str = portions.next().unwrap();
                let value = portions.next().unwrap().parse::<usize>().unwrap();

                let cards_vec = cards_str.chars().collect::<Vec<char>>();
                let cards: [char; 5] = cards_vec.try_into().unwrap();

                HandJoker { cards, value }
            })
            .collect::<Vec<HandJoker>>()
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let mut hands = parse_input(input_str);
        hands.sort();

        hands
            .iter()
            .enumerate()
            .map(|(idx, hand)| hand.value * (idx + 1))
            .sum()
    }
}
