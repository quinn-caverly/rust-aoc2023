use day13::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day13::part_a::{find_horizi, find_vert, grab_column, parse_input};

    #[test]
    fn test_find_horizi() {
        let input_str =
            "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        let inp = parse_input(input_str);
        let elem = inp.get(0).unwrap();
        assert_eq!(find_horizi(&elem), Some(4));

        let input_str =
            "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        let inp = parse_input(input_str);
        let elem = inp.get(0).unwrap();
        assert_eq!(find_horizi(elem), None);
    }

    #[test]
    fn test_grab_column() {
        let input_str =
            "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";

        let inp = parse_input(input_str);
        let elem = inp.get(0).unwrap();

        let expected = vec![false, false, true, false, false, true, false];
        assert_eq!(expected, grab_column(elem, 0));
    }

    #[test]
    fn test_find_vert() {
        let input_str =
            "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        let inp = parse_input(input_str);
        let elem = inp.get(0).unwrap();
        assert_eq!(find_vert(elem), Some(5));

        let input_str =
            "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        let inp = parse_input(input_str);
        let elem = inp.get(0).unwrap();
        assert_eq!(find_vert(elem), None);
    }
}

#[cfg(test)]
pub mod test_part_b {
    use day13::{part_a::parse_input, part_b::find_horizi_smudge};

    #[test]
    fn test_find_horizi_smudge() {
        let input_str =
            "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        let inp = parse_input(input_str);
        let elem = inp.get(0).unwrap();
        assert_eq!(find_horizi_smudge(elem, None), Some(1));
    }
}
