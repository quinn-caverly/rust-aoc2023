use day07::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day07::part_a::{
        is_five_of_a_kind, is_four_of_a_kind, is_full_house, is_one_pair, is_three_of_a_kind,
        is_two_pair, parse_input, solve_part_a, Hand,
    };

    #[test]
    fn test_parse_input() {
        let sample_input = "3Q373 470";

        let expected_first = Hand::new(470, ['3', 'Q', '3', '7', '3']);

        assert_eq!(parse_input(sample_input).get(0).unwrap(), &expected_first);
    }

    #[test]
    fn test_is_five_of_a_kind() {
        let mut hand = Hand::new(0, ['A', 'A', 'A', 'A', 'A']);
        assert_eq!(is_five_of_a_kind(&hand), true);

        hand = Hand::new(0, ['A', 'A', 'A', 'A', 'B']);
        assert_eq!(is_five_of_a_kind(&hand), false);
    }

    #[test]
    fn test_is_four_of_a_kind() {
        let mut hand = Hand::new(0, ['A', 'A', 'A', 'A', 'A']);
        assert_eq!(is_four_of_a_kind(&hand), false);

        hand = Hand::new(0, ['A', 'A', 'A', 'A', 'B']);
        assert_eq!(is_four_of_a_kind(&hand), true);

        hand = Hand::new(0, ['B', 'A', 'A', 'A', 'B']);
        assert_eq!(is_four_of_a_kind(&hand), false);

        hand = Hand::new(0, ['B', 'A', 'A', 'A', 'A']);
        assert_eq!(is_four_of_a_kind(&hand), true);
    }

    #[test]
    fn test_is_full_house() {
        let mut hand = Hand::new(0, ['A', 'A', 'A', 'B', 'B']);
        assert_eq!(is_full_house(&hand), true);

        hand = Hand::new(0, ['A', 'A', 'B', 'B', 'B']);
        assert_eq!(is_full_house(&hand), true);

        hand = Hand::new(0, ['A', 'A', 'C', 'B', 'B']);
        assert_eq!(is_full_house(&hand), false);
    }

    #[test]
    fn test_is_three_of_a_kind() {
        let mut hand = Hand::new(0, ['A', 'A', 'A', 'B', 'B']);
        assert_eq!(is_three_of_a_kind(&hand), true);

        hand = Hand::new(0, ['A', 'A', 'B', 'B', 'B']);
        assert_eq!(is_three_of_a_kind(&hand), true);

        hand = Hand::new(0, ['A', 'A', 'C', 'B', 'B']);
        assert_eq!(is_three_of_a_kind(&hand), false);

        hand = Hand::new(0, ['B', 'A', 'B', 'C', 'B']);
        assert_eq!(is_three_of_a_kind(&hand), true);
    }

    #[test]
    fn test_is_two_pair() {
        let mut hand = Hand::new(0, ['A', 'A', 'B', 'B', 'C']);
        assert_eq!(is_two_pair(&hand), true);

        hand = Hand::new(0, ['A', 'A', 'C', 'B', 'B']);
        assert_eq!(is_two_pair(&hand), true);

        hand = Hand::new(0, ['A', 'B', 'B', 'C', 'C']);
        assert_eq!(is_two_pair(&hand), true);

        hand = Hand::new(0, ['A', 'B', 'B', 'C', 'D']);
        assert_eq!(is_two_pair(&hand), false);
    }

    #[test]
    fn test_is_one_pair() {
        let mut hand = Hand::new(0, ['A', 'A', 'B', 'D', 'C']);
        assert_eq!(is_one_pair(&hand), true);

        hand = Hand::new(0, ['C', 'A', 'A', 'X', 'B']);
        assert_eq!(is_one_pair(&hand), true);

        hand = Hand::new(0, ['A', 'Y', 'B', 'B', 'X']);
        assert_eq!(is_one_pair(&hand), true);

        hand = Hand::new(0, ['A', 'B', 'X', 'C', 'C']);
        assert_eq!(is_one_pair(&hand), true);

        hand = Hand::new(0, ['A', 'B', 'X', 'Y', 'C']);
        assert_eq!(is_one_pair(&hand), false);
    }

    #[test]
    fn test_hand_sort() {
        let hand1 = Hand::new(0, ['A', 'A', 'A', 'D', 'C']);
        let hand2 = Hand::new(0, ['C', 'A', 'A', 'X', 'B']);

        let mut unsorted = vec![hand2.clone(), hand1.clone()];
        unsorted.sort();

        let expected = vec![hand2, hand1];
        assert_eq!(expected, unsorted);
    }

    #[test]
    fn test_solve_part_a() {
        const SAMPLE_INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

        let mut hands = parse_input(SAMPLE_INPUT);
        hands.sort();

        assert_eq!(solve_part_a(SAMPLE_INPUT), 6440);
    }
}

#[cfg(test)]
mod test_part_b {
    use std::fmt::write;

    use day07::part_b::{
        is_five_of_a_kind_joker, is_four_of_a_kind_joker, is_full_house_joker,
        is_three_of_a_kind_joker, parse_input, solve_part_b, HandJoker,
    };

    #[test]
    fn test_is_five_of_a_kind_joker() {
        let hand = HandJoker::new(0, ['A', 'A', 'A', 'J', 'A']);

        assert_eq!(is_five_of_a_kind_joker(&hand), true);
    }

    #[test]
    fn test_is_four_of_a_kind_joker() {
        let mut hand = HandJoker::new(0, ['A', 'A', 'B', 'J', 'A']);
        assert_eq!(is_four_of_a_kind_joker(&hand, 1), true);

        hand = HandJoker::new(0, ['A', 'C', 'J', 'J', 'A']);
        assert_eq!(is_four_of_a_kind_joker(&hand, 2), true);

        hand = HandJoker::new(0, ['C', 'J', 'J', 'J', 'A']);
        assert_eq!(is_four_of_a_kind_joker(&hand, 3), true);

        hand = HandJoker::new(0, ['C', 'C', 'C', 'C', 'A']);
        assert_eq!(is_four_of_a_kind_joker(&hand, 0), true);
    }

    #[test]
    fn test_is_full_house_joker() {
        let mut hand = HandJoker::new(0, ['A', 'A', 'B', 'J', 'J']);
        assert_eq!(is_full_house_joker(&hand, 2), true);

        hand = HandJoker::new(0, ['A', 'A', 'J', 'B', 'B']);
        assert_eq!(is_full_house_joker(&hand, 1), true);
    }

    #[test]
    fn test_is_three_of_a_kind_joker() {
        let mut hand = HandJoker::new(0, ['A', 'C', 'B', 'J', 'J']);
        assert_eq!(is_three_of_a_kind_joker(&hand, 2), true);

        hand = HandJoker::new(0, ['A', 'C', 'J', 'B', 'A']);
        assert_eq!(is_three_of_a_kind_joker(&hand, 1), true);

        hand = HandJoker::new(0, ['C', 'J', 'J', 'J', 'A']);
        assert_eq!(is_three_of_a_kind_joker(&hand, 3), true);
    }

    #[test]
    fn test_solve_part_b() {
        const SAMPLE_INPUT: &str = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

        let mut hands = parse_input(SAMPLE_INPUT);
        hands.sort();

        assert_eq!(solve_part_b(SAMPLE_INPUT), 5905);
    }
}
