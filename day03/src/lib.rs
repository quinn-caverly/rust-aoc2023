pub mod part_a {
    use std::collections::HashSet;

    pub fn get_gear_locs(lines: &Vec<&str>) -> HashSet<(i64, i64)> {
        let mut gear_locs = HashSet::new();

        for (row_idx, &row) in lines.iter().enumerate() {
            for (col_idx, ch) in row.chars().enumerate() {
                if !ch.is_digit(10) && ch != '.' {
                    gear_locs.insert((row_idx.try_into().unwrap(), col_idx.try_into().unwrap()));
                }
            }
        }

        gear_locs
    }

    // each Number has an integer value, a row number, a column start, and a column end
    #[derive(Debug, PartialEq)]
    pub struct Number {
        pub value: i64,
        pub row_idx: i64,
        pub col_start: i64,
        pub col_end: i64,
    }

    pub fn get_numbers(lines: &Vec<&str>) -> Vec<Number> {
        let mut part_numbers: Vec<Number> = Vec::new();

        for (row_idx, &row) in lines.iter().enumerate() {
            let mut building = false;
            let mut col_start = 0;
            let mut col_end = 0;
            let mut digits: Vec<i64> = Vec::new();

            for (col_idx, ch) in row.chars().enumerate() {
                match building {
                    true => {
                        if let Some(digit) = ch.to_digit(10) {
                            col_end = col_idx;
                            digits.push(digit.try_into().unwrap());
                        } else {
                            // if we were building but this is not a digit, then we need to close
                            let mut value = 0;
                            let mut mult = 1;
                            for dig in digits.iter().rev() {
                                value += dig * mult;
                                mult *= 10;
                            }

                            part_numbers.push(Number {
                                value,
                                row_idx: row_idx.try_into().unwrap(),
                                col_start,
                                col_end: col_end.try_into().unwrap(),
                            });

                            building = false;
                        }
                    }
                    false => {
                        if let Some(digit) = ch.to_digit(10) {
                            building = true;
                            col_start = col_idx.try_into().unwrap();
                            col_end = col_idx;
                            digits = Vec::new();
                            digits.push(digit.try_into().unwrap());
                        }
                    }
                }
            }
            // we may have been building a num when the line ended
            if building {
                let mut value = 0;
                let mut mult = 1;
                for dig in digits.iter().rev() {
                    value += dig * mult;
                    mult *= 10;
                }

                part_numbers.push(Number {
                    value,
                    row_idx: row_idx.try_into().unwrap(),
                    col_start,
                    col_end: col_end.try_into().unwrap(),
                });
            }
        }

        part_numbers
    }

    pub fn get_adjacent_coords_for_num(num: &Number) -> Vec<(i64, i64)> {
        let mut coords: Vec<(i64, i64)> = Vec::new();

        // edges
        (-1..=1).for_each(|offset| {
            coords.push((num.row_idx + offset, num.col_start - 1));
            coords.push((num.row_idx + offset, num.col_end + 1));
        });

        // top and bottom
        (num.col_start..=num.col_end).for_each(|col| {
            coords.push((num.row_idx - 1, col));
            coords.push((num.row_idx + 1, col));
        });

        coords
    }

    pub fn get_part_nums_based_on_gears<'a>(
        nums: &'a Vec<Number>,
        gear_locs: &HashSet<(i64, i64)>,
    ) -> Vec<&'a Number> {
        let mut part_numbers: Vec<&Number> = Vec::new();

        // we can create a discrete list of all possible coordinates for a gear
        // to be in which would make this a part number
        'outer: for num in nums {
            let adj_coords = get_adjacent_coords_for_num(num);

            for adj_coord in adj_coords {
                if gear_locs.contains(&adj_coord) {
                    part_numbers.push(num);
                    continue 'outer;
                }
            }
        }

        part_numbers
    }

    pub fn solve_part_a(string: &str) -> i64 {
        let lines = &string.lines().collect();

        let gear_locs = get_gear_locs(lines);
        let nums = get_numbers(lines);

        let part_nums = get_part_nums_based_on_gears(&nums, &gear_locs);

        let mut sum = 0;
        for part_num in part_nums {
            sum += part_num.value;
        }

        sum
    }
}

pub mod part_b {
    use crate::part_a::{get_adjacent_coords_for_num, get_numbers, Number};

    pub fn get_star_locs(lines: &Vec<&str>) -> Vec<(i64, i64)> {
        let mut star_locs = Vec::new();

        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, ch) in line.chars().enumerate() {
                if ch == '*' {
                    star_locs.push((row_idx.try_into().unwrap(), col_idx.try_into().unwrap()));
                }
            }
        }

        star_locs
    }

    pub fn find_star_ratios(star_locs: &Vec<(i64, i64)>, nums: &Vec<Number>) -> Vec<i64> {
        let mut star_ratios = Vec::new();

        for star_loc in star_locs {
            // there needs to be exactly two adjacent numbers
            let mut adjacents: Vec<&Number> = Vec::new();

            'inner: for num in nums {
                let adjs = get_adjacent_coords_for_num(num);
                if adjs.contains(star_loc) {
                    adjacents.push(num);

                    // break on 3 not 2 because if there are 3 or more,
                    // then at this point is is not a proper star
                    // a star must have exactly 2 adj nums
                    if adjs.len() == 3 {
                        break 'inner;
                    }
                }
            }

            if adjacents.len() == 2 {
                star_ratios.push(adjacents.get(0).unwrap().value * adjacents.get(1).unwrap().value);
            }
        }

        star_ratios
    }

    pub fn solve_part_b(string: &str) -> i64 {
        let lines_vec = &string.lines().collect();

        let nums = get_numbers(lines_vec);
        let star_locs = get_star_locs(lines_vec);

        find_star_ratios(&star_locs, &nums).iter().sum()
    }
}
