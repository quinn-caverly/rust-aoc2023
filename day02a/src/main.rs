use lib::{is_game_possible, parse_game};

mod lib;

fn main() {
    let mut sum = 0;

    let mut cur_game = 1;
    for line in include_str!("input.txt").lines() {
        let game = parse_game(line);
        if is_game_possible(game) {
            sum += cur_game;
        }

        cur_game += 1;
    }

    println!("{}", sum);
}

#[cfg(test)]
mod tests {

    use crate::lib::{self, is_game_possible};

    #[test]
    fn test_parse_game() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue";
        let mut hands = Vec::new();

        hands.push(lib::Hand {
            blue_num: 3,
            red_num: 4,
            green_num: 0,
        });

        hands.push(lib::Hand {
            blue_num: 6,
            red_num: 1,
            green_num: 2,
        });

        assert_eq!(lib::parse_game(line), hands);
    }

    #[test]
    fn test_is_game_possible() {
        let mut hands = Vec::new();
        hands.push(lib::Hand {
            blue_num: 3,
            red_num: 4,
            green_num: 0,
        });
        hands.push(lib::Hand {
            blue_num: 6,
            red_num: 1,
            green_num: 2,
        });
        assert!(is_game_possible(hands));

        let mut hands = Vec::new();
        hands.push(lib::Hand {
            blue_num: 15,
            red_num: 4,
            green_num: 0,
        });
        hands.push(lib::Hand {
            blue_num: 6,
            red_num: 1,
            green_num: 2,
        });
        assert!(!is_game_possible(hands));
    }
}
