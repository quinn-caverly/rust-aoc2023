use day14::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day14::part_a::{count_circles, shift_left, Slot};

    #[test]
    fn test_count_circles() {
        let sequence = vec![Slot::Emp, Slot::Emp, Slot::Rnd, Slot::Rnd, Slot::Cube];
        assert_eq!(count_circles(&sequence, 0), 2);
    }

    #[test]
    fn test_shift_left() {
        let sample_grid = vec![
            vec![Slot::Emp, Slot::Rnd, Slot::Rnd],
            vec![Slot::Emp, Slot::Cube, Slot::Rnd],
            vec![Slot::Emp, Slot::Rnd, Slot::Emp],
        ];

        let expected = vec![
            vec![Slot::Rnd, Slot::Rnd, Slot::Emp],
            vec![Slot::Emp, Slot::Cube, Slot::Rnd],
            vec![Slot::Rnd, Slot::Emp, Slot::Emp],
        ];

        for (idx, row) in shift_left(&sample_grid).iter().enumerate() {
            assert_eq!(row, expected.get(idx).unwrap());
        }
    }
}

#[cfg(test)]
mod test_part_b {
    use day14::{part_a::parse_input, part_b::{run_cycle, solve_part_b}};

    #[test]
    fn test_run_cycle() {
        let input_str =  "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";
        let mut grid = parse_input(input_str);

        let cycled = run_cycle(&mut grid);

        for row in cycled {
            println!("{:?}", row);
        }
    }

    #[test]
    fn test_solve_part_b() {
        let input_str = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";
        let val = solve_part_b(input_str);

        assert_eq!(64, val);
    }
}
