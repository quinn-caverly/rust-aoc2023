use day04::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input));
    println!("part b: {}", solve_part_b(input));
}

const TEST_STR: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[cfg(test)]
mod test_part_a {
    use day04::part_a::{parse_input, Card};

    use crate::TEST_STR;

    #[test]
    fn test_parse_input() {
        let cards = parse_input(TEST_STR);

        let card_one: Card = Card {
            winning_nums: vec![41, 48, 83, 86, 17],
            dealt_nums: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(card_one, *cards.get(0).unwrap());
    }

    #[test]
    fn test_calc_value() {
        let cards = parse_input(TEST_STR);

        assert_eq!(cards.get(0).unwrap().calc_value(), 8);
        assert_eq!(cards.get(1).unwrap().calc_value(), 2);
    }
}

#[cfg(test)]
mod test_part_b {
    use day04::part_b::solve_part_b;

    use crate::TEST_STR;

    #[test]
    fn test_solve_part_b() {
        assert_eq!(solve_part_b(TEST_STR), 30);
    }
}
