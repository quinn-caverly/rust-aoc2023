pub mod part_a {

    #[derive(Debug, PartialEq)]
    pub struct Card {
        pub winning_nums: Vec<usize>,
        pub dealt_nums: Vec<usize>,
    }

    impl Card {
        pub fn calc_value(&self) -> usize {
            let mut matches = 0;
            for num in &self.dealt_nums {
                if self.winning_nums.contains(num) {
                    matches += 1;
                }
            }

            if matches > 0 {
                2usize.pow(matches - 1)
            } else {
                0
            }
        }

        pub fn calc_matches(&self) -> usize {
            let mut matches = 0;
            for num in &self.dealt_nums {
                if self.winning_nums.contains(num) {
                    matches += 1;
                }
            }
            matches
        }
    }

    pub fn parse_input(string: &str) -> Vec<Card> {
        string
            .lines()
            .map(|line| {
                let mut scraps = line.split(": ");
                let stripped_line = scraps.nth(1).unwrap();

                let mut two_results: Vec<Vec<usize>> = Vec::new();
                for portion in stripped_line.split(" | ") {
                    two_results.push(
                        portion
                            .split(" ")
                            .filter_map(|num_str| match num_str.parse::<usize>() {
                                Ok(x) => Some(x),
                                Err(_) => None,
                            })
                            .collect::<Vec<usize>>(),
                    );
                }

                Card {
                    dealt_nums: two_results.pop().unwrap(),
                    winning_nums: two_results.pop().unwrap(),
                }
            })
            .collect::<Vec<Card>>()
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let cards = parse_input(input_str);

        cards.iter().map(|card| card.calc_value()).sum()
    }
}

pub mod part_b {
    use crate::part_a::parse_input;

    pub fn solve_part_b(input_str: &str) -> usize {
        let cards = parse_input(input_str);

        let mut copies = vec![1; cards.len()];
        let mut sum_total = 0;

        for (idx, card) in cards.iter().enumerate() {
            let cur_matches = card.calc_matches();

            let total_for_cur_card = *copies.get(idx).unwrap();

            for i in idx + 1..=idx + cur_matches {
                *copies.get_mut(i).unwrap() += total_for_cur_card;
            }

            sum_total += total_for_cur_card;
        }

        sum_total
    }
}
