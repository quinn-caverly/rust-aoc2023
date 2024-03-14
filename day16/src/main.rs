use day16::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day16::part_a::{parse_input, solve_part_a, Tile};

    #[test]
    fn test_parse_input() {
        let input_str = ".|.\n|.-\n...";
        let grid = parse_input(input_str);
        let expected = vec![
            vec![Tile::Border; 5],
            vec![
                Tile::Border,
                Tile::Empty,
                Tile::Verti,
                Tile::Empty,
                Tile::Border,
            ],
            vec![
                Tile::Border,
                Tile::Verti,
                Tile::Empty,
                Tile::Horizi,
                Tile::Border,
            ],
            vec![
                Tile::Border,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Border,
            ],
            vec![Tile::Border; 5],
        ];
        assert_eq!(grid, expected);

        let input_str = ".|/\n|.-\n\\..";
        let grid = parse_input(input_str);
        let expected = vec![
            vec![Tile::Border; 5],
            vec![
                Tile::Border,
                Tile::Empty,
                Tile::Verti,
                Tile::FwdMirror,
                Tile::Border,
            ],
            vec![
                Tile::Border,
                Tile::Verti,
                Tile::Empty,
                Tile::Horizi,
                Tile::Border,
            ],
            vec![
                Tile::Border,
                Tile::BackMirror,
                Tile::Empty,
                Tile::Empty,
                Tile::Border,
            ],
            vec![Tile::Border; 5],
        ];
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_solve_part_a() {
        let input_str = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";
        assert_eq!(solve_part_a(input_str), 46);
    }
}

#[cfg(test)]
mod test_part_b {
    use day16::part_b::solve_part_b;

    #[test]
    fn test_solve_part_b() {
        let input_str = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";
        assert_eq!(solve_part_b(input_str), 51);
    }
}
