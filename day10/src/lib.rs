pub mod part_a {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Pipe {
        Straight(Straight),
        Bend(Bend),
        Start,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Bend {
        NorthAndEast,
        NorthAndWest,
        SouthAndWest,
        SouthAndEast,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Straight {
        Vertical,
        Horizontal,
    }

    pub fn parse_input(input_str: &str) -> ((usize, usize), Vec<Vec<Option<Pipe>>>) {
        let (mut s_row, mut s_col) = (0, 0);

        let vec = input_str
            .lines()
            .enumerate()
            .map(|(row_idx, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col_idx, ch)| match ch {
                        '.' => None,
                        '|' => Some(Pipe::Straight(Straight::Vertical)),
                        '-' => Some(Pipe::Straight(Straight::Horizontal)),
                        'L' => Some(Pipe::Bend(Bend::NorthAndEast)),
                        'J' => Some(Pipe::Bend(Bend::NorthAndWest)),
                        '7' => Some(Pipe::Bend(Bend::SouthAndWest)),
                        'F' => Some(Pipe::Bend(Bend::SouthAndEast)),
                        'S' => Some({
                            s_row = row_idx;
                            s_col = col_idx;
                            Pipe::Start
                        }),
                        _ => panic!("Did not expect other char"),
                    })
                    .collect::<Vec<Option<Pipe>>>()
            })
            .collect();

        ((s_row, s_col), vec)
    }

    fn go_down(row_idx: usize, col_idx: usize) -> (usize, usize, Direction) {
        (row_idx + 1, col_idx, Direction::Down)
    }
    fn go_up(row_idx: usize, col_idx: usize) -> (usize, usize, Direction) {
        (row_idx - 1, col_idx, Direction::Up)
    }
    fn go_left(row_idx: usize, col_idx: usize) -> (usize, usize, Direction) {
        (row_idx, col_idx - 1, Direction::Left)
    }
    fn go_right(row_idx: usize, col_idx: usize) -> (usize, usize, Direction) {
        (row_idx, col_idx + 1, Direction::Right)
    }

    pub fn find_type_of_s() -> Pipe {
        Pipe::Bend(Bend::NorthAndWest)
    }

    pub fn traverse_pipe(
        row_idx: usize,
        col_idx: usize,
        cur_direction: Direction,
        layout: &Vec<Vec<Option<Pipe>>>,
    ) -> (usize, usize, Direction) {
        let cur_pipe_slot = layout.get(row_idx).unwrap().get(col_idx).unwrap().clone();

        match cur_pipe_slot {
            None => panic!("Non pipe location: ({}, {})", row_idx, col_idx),
            Some(pipe) => match pipe {
                Pipe::Straight(straight) => match straight {
                    Straight::Vertical => {
                        if cur_direction == Direction::Up {
                            return go_up(row_idx, col_idx);
                        }
                        go_down(row_idx, col_idx)
                    }
                    Straight::Horizontal => {
                        if cur_direction == Direction::Left {
                            return go_left(row_idx, col_idx);
                        }
                        go_right(row_idx, col_idx)
                    }
                },
                Pipe::Bend(bend) => match bend {
                    Bend::NorthAndEast => {
                        if cur_direction == Direction::Down {
                            return go_right(row_idx, col_idx);
                        }
                        go_up(row_idx, col_idx)
                    }
                    Bend::NorthAndWest => {
                        if cur_direction == Direction::Down {
                            return go_left(row_idx, col_idx);
                        }
                        go_up(row_idx, col_idx)
                    }
                    Bend::SouthAndWest => {
                        if cur_direction == Direction::Up {
                            return go_left(row_idx, col_idx);
                        }
                        go_down(row_idx, col_idx)
                    }
                    Bend::SouthAndEast => {
                        if cur_direction == Direction::Up {
                            return go_right(row_idx, col_idx);
                        }
                        go_down(row_idx, col_idx)
                    }
                },
                Pipe::Start => panic!("Did not actually expect to return to the Start position"),
            },
        }
    }

    // go in both directions at once and we go until we are on the same coordinate, at this point
    // we return the number of steps it took to reach this point
    pub fn solve_part_a(
        input_str: &str,
        path1_builder: (i64, i64, Direction),
        path2_builder: (i64, i64, Direction),
    ) -> usize {
        let ((s_row, s_col), layout) = parse_input(input_str);

        let mut path1 = (
            (s_row as i64 + path1_builder.0) as usize,
            (s_col as i64 + path1_builder.1) as usize,
            path1_builder.2,
        );

        let mut path2 = (
            (s_row as i64 + path2_builder.0) as usize,
            (s_col as i64 + path2_builder.1) as usize,
            path2_builder.2,
        );

        // we start 1 away from S so we start with a single step
        let mut steps = 1;

        while (path1.0, path1.1) != (path2.0, path2.1) {
            path1 = traverse_pipe(path1.0, path1.1, path1.2, &layout);
            path2 = traverse_pipe(path2.0, path2.1, path2.2, &layout);

            steps += 1;
        }

        steps
    }
}

pub mod part_b {
    // plan is to traverse the entire loop, we want to keep track of the spaces
    // in between the nodes rather than the nodes themselves. Then, we will use DFS or BFS
    // starting outside of the loop or inside the loop and see what points we can reach. In this
    // exploration we can travserse the half slots. Then, convert from half slots back to full
    // slots by rounding either up or down and checking whether it is a pipe or slot piece.

    // first, in the parse input we will need to add a "buffer" of periods around the edge so that,
    // we can make sure that we have a reference outside of the loop. We don't need to worry about
    // coordinates for this section.

    use core::num;
    use std::collections::HashSet;

    use crate::part_a::{parse_input, traverse_pipe, Bend, Direction, Pipe, Straight};

    pub fn mark_pipe(
        s_row: usize,
        s_col: usize,
        layout: &Vec<Vec<Option<Pipe>>>,
        path_builder: (i64, i64, Direction),
    ) -> HashSet<(usize, usize)> {
        // we need to traverse the pipe, what is important is the connection between
        // tiles. Specifically, mark when we transfer from tile to tile, which makes an impassable
        // point for the depth first search later

        // in order to represent these half points in memory, it'll simply be a set of points that
        // can't be crossed. For example:
        // ## If this is row 0, 1, 2 and cols 0, 1 then clearly the pipe
        // -- goes from (1, 0) to (1, 1). to the hashset we will add
        // ## (1, 0.5)

        let mut animal_pipe_tiles: HashSet<(usize, usize)> = HashSet::new();

        animal_pipe_tiles.insert((s_row, s_col));

        let mut path = (
            (s_row as i64 + path_builder.0) as usize,
            (s_col as i64 + path_builder.1) as usize,
            path_builder.2,
        );

        animal_pipe_tiles.insert((path.0, path.1));

        while layout.get(path.0).unwrap().get(path.1).unwrap() != &Some(Pipe::Start) {
            let new_path = traverse_pipe(path.0, path.1, path.2, layout);
            path = new_path;

            animal_pipe_tiles.insert((path.0, path.1));
        }

        animal_pipe_tiles
    }

    // ray tracing for checking of a None point is inside or outside the polygon
    pub fn get_inside_tiles(
        layout: &Vec<Vec<Option<Pipe>>>,
        animal_pipe_tiles: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let mut inside_tiles = Vec::new();

        for row_idx in 0..layout.len() {
            let mut intersections = 0.0;

            for col_idx in 0..layout.get(0).unwrap().len() {
                let cur = layout.get(row_idx).unwrap().get(col_idx).unwrap();

                if cur == &Some(Pipe::Straight(Straight::Vertical))
                    && animal_pipe_tiles.contains(&(row_idx, col_idx))
                {
                    intersections += 1.0;
                }

                if cur != &Some(Pipe::Straight(Straight::Vertical))
                    && animal_pipe_tiles.contains(&(row_idx, col_idx))
                {
                    intersections += 0.5;
                }

                if cur == &None && intersections % 2.0 == 1.0 {
                    inside_tiles.push((row_idx, col_idx));
                }
            }
        }

        inside_tiles
    }

    pub fn solve_part_b(input_str: &str, path: (i64, i64, Direction)) -> usize {
        let ((s_row, s_col), layout) = parse_input(input_str);
        let animal_pipe_tiles = mark_pipe(s_row, s_col, &layout, path);
        let inside_tiles = get_inside_tiles(&layout, &animal_pipe_tiles);

        inside_tiles.len()
    }
}
