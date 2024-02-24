pub mod part_a {

    #[derive(Debug, PartialEq)]
    pub struct Hand {
        pub blue_num: usize,
        pub green_num: usize,
        pub red_num: usize,
    }

    const TOTAL_REDS: usize = 12;
    const TOTAL_BLUES: usize = 14;
    const TOTAL_GREENS: usize = 13;

    pub fn is_game_possible(game: Vec<Hand>) -> bool {
        for hand in game {
            if hand.blue_num > TOTAL_BLUES {
                return false;
            }
            if hand.red_num > TOTAL_REDS {
                return false;
            }
            if hand.green_num > TOTAL_GREENS {
                return false;
            }
        }
        true
    }

    pub fn parse_game(line: &str) -> Vec<Hand> {
        let mut portions = line.split(": ");

        let mut hands = Vec::new();

        let hands_strs = portions.nth(1).unwrap().split("; ");
        for hand_str in hands_strs {
            let mut blue_num = 0;
            let mut green_num = 0;
            let mut red_num = 0;

            let colors = hand_str.split(", ");
            for color in colors {
                let mut parts = color.split(" ");
                let (num_str, color_str) = (parts.next().unwrap(), parts.next().unwrap());

                match color_str {
                    "blue" => blue_num = num_str.parse::<usize>().unwrap(),
                    "green" => green_num = num_str.parse::<usize>().unwrap(),
                    "red" => red_num = num_str.parse::<usize>().unwrap(),
                    _ => panic!("Did not expect color other than r, g, b"),
                }
            }

            hands.push(Hand {
                blue_num,
                green_num,
                red_num,
            });
        }
        hands
    }
}

pub mod part_b {
    use super::part_a::Hand;

    pub fn find_square_of_game(game: Vec<Hand>) -> usize {
        let mut min_blue = 0;
        let mut min_green = 0;
        let mut min_red = 0;

        for hand in game {
            if hand.blue_num > min_blue {
                min_blue = hand.blue_num;
            }
            if hand.red_num > min_red {
                min_red = hand.red_num;
            }
            if hand.green_num > min_green {
                min_green = hand.green_num;
            }
        }

        min_blue * min_green * min_red
    }
}
