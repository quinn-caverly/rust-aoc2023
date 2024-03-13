pub mod part_a {
    use core::panic;

    pub fn parse_input(input_str: &str) -> Vec<Vec<Vec<bool>>> {
        input_str
            .split("\n\n")
            .map(|section| {
                section
                    .lines()
                    .map(|line| {
                        line.chars()
                            .map(|ch| if ch == '.' { true } else { false })
                            .collect()
                    })
                    .collect()
            })
            .collect()
    }

    pub fn grab_column(grid: &Vec<Vec<bool>>, idx: usize) -> Vec<bool> {
        let mut col = Vec::new();
        for row in grid {
            col.push(*row.get(idx).unwrap());
        }
        col
    }

    // the vertical line is between usize, returns the leftmost one
    // because the left columns are counted to find the ans
    pub fn find_vert(grid: &Vec<Vec<bool>>) -> Option<usize> {
        'outer: for i in 1..grid.get(0).unwrap().len() {
            let mut lower: i64 = i as i64 - 1;
            let mut higher = i;

            while lower >= 0 && higher < grid.get(0).unwrap().len() {
                if grab_column(grid, lower as usize) != grab_column(grid, higher) {
                    continue 'outer;
                }
                lower -= 1;
                higher += 1;
            }

            return Some(i);
        }

        None
    }

    pub fn find_horizi(grid: &Vec<Vec<bool>>) -> Option<usize> {
        // at each i we take the grid line between i - 1 and i
        'outer: for i in 1..grid.len() {
            let mut lower: i64 = i as i64 - 1;
            let mut higher = i;

            while lower >= 0 && higher < grid.len() {
                if grid.get(lower as usize).unwrap() != grid.get(higher).unwrap() {
                    continue 'outer;
                }
                lower -= 1;
                higher += 1;
            }

            return Some(i);
        }

        None
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let grids = parse_input(input_str);

        let mut total = 0;
        for grid in grids {
            if let Some(x) = find_vert(&grid) {
                total += x;
                continue;
            }

            if let Some(x) = find_horizi(&grid) {
                total += x * 100;
                continue;
            }

            panic!("Did not expect neither verti or horizi");
        }

        total
    }
}

pub mod part_b {
    use crate::part_a::{find_horizi, find_vert, grab_column, parse_input};

    pub fn count_diffs(a: &Vec<bool>, b: &Vec<bool>) -> usize {
        let mut diffs = 0;
        for (a1, b1) in a.iter().zip(b.iter()) {
            if a1 != b1 {
                diffs += 1;
            }
        }
        diffs
    }

    // there can be one mistake while searching through columns
    pub fn find_vert_smudge(grid: &Vec<Vec<bool>>, original_i: Option<usize>) -> Option<usize> {
        'outer: for i in 1..grid.get(0).unwrap().len() {
            if let Some(origi_i) = original_i {
                if origi_i == i {
                    continue 'outer;
                }
            }

            let mut lower: i64 = i as i64 - 1;
            let mut higher = i;

            let mut total_diffs = 0;
            while lower >= 0 && higher < grid.get(0).unwrap().len() {
                total_diffs += count_diffs(
                    &grab_column(grid, lower as usize),
                    &grab_column(grid, higher),
                );

                if total_diffs > 1 {
                    continue 'outer;
                }

                lower -= 1;
                higher += 1;
            }

            return Some(i);
        }

        None
    }

    pub fn find_horizi_smudge(grid: &Vec<Vec<bool>>, original_i: Option<usize>) -> Option<usize> {
        'outer: for i in 1..grid.len() {
            if let Some(origi_i) = original_i {
                if origi_i == i {
                    continue 'outer;
                }
            }

            let mut lower: i64 = i as i64 - 1;
            let mut higher = i;

            let mut total_diffs = 0;
            while lower >= 0 && higher < grid.len() {
                total_diffs +=
                    count_diffs(grid.get(lower as usize).unwrap(), grid.get(higher).unwrap());

                if total_diffs > 1 {
                    continue 'outer;
                }

                lower -= 1;
                higher += 1;
            }

            return Some(i);
        }

        None
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let grids = parse_input(input_str);

        let mut total = 0;
        for grid in grids {
            if let Some(x) = find_vert_smudge(&grid, find_vert(&grid)) {
                total += x;
                continue;
            }

            if let Some(x) = find_horizi_smudge(&grid, find_horizi(&grid)) {
                total += x * 100;
                continue;
            }

            panic!("Did not expect neither verti or horizi");
        }

        total
    }
}
