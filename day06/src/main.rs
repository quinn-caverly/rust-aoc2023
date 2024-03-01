use day06::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
pub mod test_part_a {
    use day06::part_a::{calculate_ways_to_win_race, parse_input, Race};

    #[test]
    pub fn test_parse_input() {
        let input_str = include_str!("input.txt");

        let expected_first = Race::new(63, 411);
        let expected_last = Race::new(68, 1035);

        let resulting_vec = parse_input(input_str);

        assert_eq!(resulting_vec.get(0).unwrap(), &expected_first);
        assert_eq!(resulting_vec.get(3).unwrap(), &expected_last);
    }

    #[test]
    pub fn test_calc_ways_to_win() {
        let race = Race::new(7, 9);
        assert_eq!(calculate_ways_to_win_race(&race), 4);
    }
}

#[cfg(test)]
pub mod test_part_b {
    use day06::part_a::Race;
    use day06::part_b::parse_input;

    #[test]
    pub fn test_parse_input() {
        let input_str = include_str!("input.txt");
        let expected = Race::new(63789468, 411127420471035);

        assert_eq!(expected, parse_input(input_str));
    }
}
