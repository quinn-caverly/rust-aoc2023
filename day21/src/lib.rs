pub mod part_a {
    use std::collections::HashSet;

    // location of start, num rows and num columns, set of impassable locations
    pub fn parse_input(
        input_str: &str,
    ) -> ((isize, isize), (isize, isize), HashSet<(isize, isize)>) {
        let (mut s_row, mut s_col) = (0, 0);
        let mut max_row_idx = 0;
        let mut max_col_idx = 0;

        let set = input_str
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                if row_idx > max_row_idx {
                    max_row_idx = row_idx;
                }

                line.chars()
                    .enumerate()
                    .filter_map(|(col_idx, ch)| {
                        if col_idx > max_col_idx {
                            max_col_idx = col_idx;
                        }

                        if ch == '#' {
                            return Some((row_idx as isize, col_idx as isize));
                        }
                        if ch == 'S' {
                            (s_row, s_col) = (row_idx, col_idx);
                        }
                        None
                    })
                    .collect::<Vec<(isize, isize)>>()
            })
            .collect();

        (
            (s_row as isize, s_col as isize),
            (max_row_idx as isize + 1, max_col_idx as isize + 1),
            set,
        )
    }

    pub fn take_steps(
        (rows, cols): (isize, isize),
        impassable_locs: &HashSet<(isize, isize)>,
        curs: HashSet<(isize, isize)>,
    ) -> HashSet<(isize, isize)> {
        let mut new_curs = HashSet::new();

        for cur in curs {
            // try up, down, left, right for cur
            if cur.0 > 0 && !impassable_locs.contains(&(cur.0 - 1, cur.1)) {
                new_curs.insert((cur.0 - 1, cur.1));
            }
            if cur.0 < rows - 1 && !impassable_locs.contains(&(cur.0 + 1, cur.1)) {
                new_curs.insert((cur.0 + 1, cur.1));
            }
            if cur.1 > 0 && !impassable_locs.contains(&(cur.0, cur.1 - 1)) {
                new_curs.insert((cur.0, cur.1 - 1));
            }
            if cur.1 < cols - 1 && !impassable_locs.contains(&(cur.0, cur.1 + 1)) {
                new_curs.insert((cur.0, cur.1 + 1));
            }
        }

        new_curs
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let (s_loc, (rows, cols), impassable_locs) = parse_input(input_str);
        let mut curs: HashSet<(isize, isize)> = vec![s_loc].into_iter().collect();

        for _ in 0..64 {
            curs = take_steps((rows, cols), &impassable_locs, curs);
        }

        curs.len()
    }
}

pub mod solve_part_b {
    // in the original grid, whether a tile is reachable or not is really
    // just related to the parity of its distance from the starting point
    // for an infinite grid, this is true as well. Just generate more grids

    // things become more difficult because we could enter the new grid
    // from any position so the distances don't easily translate



}
