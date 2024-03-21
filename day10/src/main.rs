use day10::{
    part_a::{solve_part_a, Direction},
    part_b::solve_part_b,
};

fn main() {
    // hardcoding the replacement for S to avoid tedium
    // where the args0 and args1 are offsets from S
    let input_str = include_str!("input.txt");
    let path1_builder = (-1, 0, Direction::Up);
    let path2_builder = (0, -1, Direction::Left);
    println!(
        "part a: {}",
        solve_part_a(input_str, path1_builder.clone(), path2_builder)
    );

    println!("part b: {}", solve_part_b(input_str, path1_builder));
}

#[cfg(test)]
mod test_part_a {
    use day10::part_a::parse_input;
    use day10::part_a::solve_part_a;
    use day10::part_a::traverse_pipe;
    use day10::part_a::Direction;

    #[test]
    fn test_traverse_pipe() {
        let sample_input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        let (mut row_idx, mut col_idx, mut cur_direction) = (1, 2, Direction::Right);

        let (_, layout) = parse_input(sample_input);
        (row_idx, col_idx, cur_direction) = traverse_pipe(row_idx, col_idx, cur_direction, &layout);

        assert_eq!(cur_direction, Direction::Right);
        assert_eq!(row_idx, 1);
        assert_eq!(col_idx, 3);

        (row_idx, col_idx, cur_direction) = traverse_pipe(row_idx, col_idx, cur_direction, &layout);

        assert_eq!(row_idx, 2);
        assert_eq!(col_idx, 3);
        assert_eq!(cur_direction, Direction::Down);

        (row_idx, col_idx, cur_direction) = traverse_pipe(row_idx, col_idx, cur_direction, &layout);

        assert_eq!(row_idx, 3);
        assert_eq!(col_idx, 3);
        assert_eq!(cur_direction, Direction::Down);
    }

    #[test]
    fn test_solve_part_a() {
        let sample_input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        let path1 = (0, 1, Direction::Right);
        let path2 = (1, 0, Direction::Down);

        let steps = solve_part_a(sample_input, path1, path2);

        assert_eq!(steps, 4);
    }
}

#[cfg(test)]
mod test_part_b {
    use day10::{
        part_a::{parse_input, Direction},
        part_b::{mark_pipe, solve_part_b},
    };

    #[test]
    fn test_mark_tiles() {
        let sample_input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        // #####
        // #S-7#
        // #|#|#
        // #L-J#
        // #####
        let path = (0, 1, Direction::Right);

        let ((s_row, s_col), layout) = parse_input(sample_input);
        let animal_pipe_tiles = mark_pipe(s_row, s_col, &layout, path);

        for animal_pipe_tile in &animal_pipe_tiles {
            println!("{:?}", animal_pipe_tile)
        }

        assert_eq!(animal_pipe_tiles.len(), 8);
    }

    #[test]
    fn test_solve_part_b() {
        let mut sample_input = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        // #####
        // #S-7#
        // #|#|#
        // #L-J#
        // #####
        let mut path = (0, 1, Direction::Right);
        let mut val = solve_part_b(sample_input, path);
        assert_eq!(1, val);

        sample_input = "...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n...........";
        path = (0, 1, Direction::Right);
        val = solve_part_b(sample_input, path);
        assert_eq!(4, val);

        sample_input = ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...";
        path = (0, 1, Direction::Right);
        val = solve_part_b(sample_input, path);
        assert_eq!(8, val);
    }
}
