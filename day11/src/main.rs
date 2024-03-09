use day11::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day11::part_a::{
        calc_adjusted_bet_coords, get_cols_to_expand, get_galaxy_coords, get_rows_to_expand,
        parse_input, solve_part_a,
    };

    #[test]
    fn test_parse_input() {
        let input_str = "..#\n...\n#..";

        let expected = vec![
            vec![false, false, true],
            vec![false, false, false],
            vec![true, false, false],
        ];

        assert_eq!(parse_input(input_str), expected);
    }

    #[test]
    fn test_get_galaxy_coords() {
        let expected = vec![(0, 2), (2, 0)];
        let input_str = "..#\n...\n#..";

        assert_eq!(get_galaxy_coords(&parse_input(input_str)), expected);
    }

    #[test]
    fn test_get_rows_to_expand() {
        let expected: Vec<usize> = vec![1];
        let input_str = "..#\n...\n#..";

        assert_eq!(
            get_rows_to_expand(&parse_input(input_str)),
            expected.into_iter().collect()
        )
    }

    #[test]
    fn test_get_cols_to_expand() {
        let expected: Vec<usize> = vec![1];
        let input_str = "..#\n...\n#..";

        assert_eq!(
            get_cols_to_expand(&parse_input(input_str)),
            expected.into_iter().collect()
        )
    }

    #[test]
    fn test_calc_adjusted_bet_coords() {
        let mut input_str = "..#\n...\n#..";
        let mut grid = parse_input(input_str);

        let mut rows = get_rows_to_expand(&grid);
        let mut cols = get_rows_to_expand(&grid);

        assert_eq!(calc_adjusted_bet_coords(&(0, 2), &(2, 0), &rows, &cols), 6);

        input_str = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        grid = parse_input(input_str);

        rows = get_rows_to_expand(&grid);
        cols = get_cols_to_expand(&grid);

        assert_eq!(calc_adjusted_bet_coords(&(5, 1), &(9, 4), &rows, &cols), 9);
    }

    #[test]
    fn test_solve_part_a() {
        let input_str = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";

        assert_eq!(solve_part_a(input_str), 374);
    }
}
