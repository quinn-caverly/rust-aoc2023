pub mod part_a {
    use std::{
        cmp::{max, min},
        collections::HashSet,
    };

    pub fn parse_input(input_str: &str) -> Vec<Vec<bool>> {
        input_str
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Did not expect neither . nor #"),
                    })
                    .collect()
            })
            .collect()
    }

    pub fn get_galaxy_coords(plot: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();

        for (row_idx, row) in plot.iter().enumerate() {
            for (col_idx, is_galaxy) in row.iter().enumerate() {
                if *is_galaxy {
                    galaxies.push((row_idx, col_idx));
                }
            }
        }

        galaxies
    }

    pub fn get_rows_to_expand(plot: &Vec<Vec<bool>>) -> HashSet<usize> {
        let mut rows = HashSet::new();

        'outer: for (row_idx, row) in plot.iter().enumerate() {
            for is_galaxy in row {
                if *is_galaxy {
                    continue 'outer;
                }
            }
            rows.insert(row_idx);
        }

        rows
    }

    pub fn get_cols_to_expand(plot: &Vec<Vec<bool>>) -> HashSet<usize> {
        let mut cols = HashSet::new();

        'outer: for col_idx in 0..plot.get(0).unwrap().len() {
            for row_idx in 0..plot.len() {
                if *plot.get(row_idx).unwrap().get(col_idx).unwrap() {
                    continue 'outer;
                }
            }

            cols.insert(col_idx);
        }

        cols
    }

    pub fn calc_adjusted_bet_coords(
        a: &(usize, usize),
        b: &(usize, usize),
        rows_to_expand: &HashSet<usize>,
        cols_to_expand: &HashSet<usize>,
    ) -> usize {
        let mut dist = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);

        for i in min(a.0, b.0)..max(a.0, b.0) {
            if rows_to_expand.contains(&i) {
                dist += 1;
            }
        }

        for i in min(a.1, b.1)..max(a.1, b.1) {
            if cols_to_expand.contains(&i) {
                dist += 1;
            }
        }

        dist
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let grid = parse_input(input_str);

        let coords = get_galaxy_coords(&grid);

        let rows = get_rows_to_expand(&grid);
        let cols = get_cols_to_expand(&grid);

        let mut ans = 0;
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                ans += calc_adjusted_bet_coords(
                    coords.get(i).unwrap(),
                    coords.get(j).unwrap(),
                    &rows,
                    &cols,
                );
            }
        }

        ans
    }
}

pub mod part_b {
    use std::cmp::max;
    use std::cmp::min;
    use std::collections::HashSet;

    use crate::part_a::get_cols_to_expand;
    use crate::part_a::get_galaxy_coords;
    use crate::part_a::get_rows_to_expand;
    use crate::part_a::parse_input;

    pub fn calc_adjusted_bet_coords_b(
        a: &(usize, usize),
        b: &(usize, usize),
        rows_to_expand: &HashSet<usize>,
        cols_to_expand: &HashSet<usize>,
    ) -> usize {
        let mut dist = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);

        for i in min(a.0, b.0)..max(a.0, b.0) {
            if rows_to_expand.contains(&i) {
                dist += 1000000 - 1;
            }
        }

        for i in min(a.1, b.1)..max(a.1, b.1) {
            if cols_to_expand.contains(&i) {
                dist += 1000000 - 1;
            }
        }

        dist
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let grid = parse_input(input_str);

        let coords = get_galaxy_coords(&grid);

        let rows = get_rows_to_expand(&grid);
        let cols = get_cols_to_expand(&grid);

        let mut ans = 0;
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                ans += calc_adjusted_bet_coords_b(
                    coords.get(i).unwrap(),
                    coords.get(j).unwrap(),
                    &rows,
                    &cols,
                );
            }
        }

        ans
    }
}
