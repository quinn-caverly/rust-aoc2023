use lib::part_a::{is_game_possible, parse_game};
use lib::part_b::find_square_of_game;

fn main() {
    println!("part a: {}", solve_part_a());
    println!("part b: {}", solve_part_b());
}

fn solve_part_a() -> usize {
    let mut sum = 0;

    let mut cur_game = 1;
    for line in include_str!("input.txt").lines() {
        let game = parse_game(line);
        if is_game_possible(game) {
            sum += cur_game;
        }

        cur_game += 1;
    }

    sum
}

fn solve_part_b() -> usize {
    let mut sum = 0;

    for line in include_str!("input.txt").lines() {
        let game = parse_game(line);
        sum += find_square_of_game(game);
    }

    sum
}

#[cfg(test)]
mod test_part_b {

    use lib::part_a::Hand;
    use lib::part_b::find_square_of_game;

    #[test]
    fn test_find_square_of_game() {
        let mut game = Vec::new();
        game.push(Hand {
            blue_num: 3,
            red_num: 4,
            green_num: 0,
        });
        game.push(Hand {
            blue_num: 6,
            red_num: 1,
            green_num: 2,
        });

        assert_eq!(find_square_of_game(game), 48);
    }
}

#[cfg(test)]
mod test_part_a {

    use lib::part_a::{self, is_game_possible};

    #[test]
    fn test_parse_game() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue";
        let mut hands = Vec::new();

        hands.push(part_a::Hand {
            blue_num: 3,
            red_num: 4,
            green_num: 0,
        });

        hands.push(part_a::Hand {
            blue_num: 6,
            red_num: 1,
            green_num: 2,
        });

        assert_eq!(part_a::parse_game(line), hands);
    }

    #[test]
    fn test_is_game_possible() {
        let mut hands = Vec::new();
        hands.push(part_a::Hand {
            blue_num: 3,
            red_num: 4,
            green_num: 0,
        });
        hands.push(part_a::Hand {
            blue_num: 6,
            red_num: 1,
            green_num: 2,
        });
        assert!(is_game_possible(hands));

        let mut hands = Vec::new();
        hands.push(part_a::Hand {
            blue_num: 15,
            red_num: 4,
            green_num: 0,
        });
        hands.push(part_a::Hand {
            blue_num: 6,
            red_num: 1,
            green_num: 2,
        });
        assert!(!is_game_possible(hands));
    }
}
