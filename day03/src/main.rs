use day03::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let string = include_str!("input.txt");
    println!("part a: {}", solve_part_a(string));
    println!("part b: {}", solve_part_b(string));
}

#[cfg(test)]
mod test_part_a {
    use std::collections::HashSet;

    use day03::part_a::{
        get_adjacent_coords_for_num, get_gear_locs, get_numbers, get_part_nums_based_on_gears,
        solve_part_a, Number,
    };

    #[test]
    fn test_get_gear_locs() {
        let string = "467..114..\n...$.*....\n.664.598./";
        let lines_vec = &string.lines().collect();

        let mut expected = HashSet::new();
        expected.insert((1, 3));
        expected.insert((1, 5));
        expected.insert((2, 9));

        assert_eq!(expected, get_gear_locs(lines_vec));
    }

    #[test]
    fn test_get_numbers() {
        let string = "467..114..\n.53$9*....\n.664...598";
        let lines_vec = &string.lines().collect();

        let first_val = Number {
            value: 467,
            row_idx: 0,
            col_start: 0,
            col_end: 2,
        };

        let third_val = Number {
            value: 53,
            row_idx: 1,
            col_start: 1,
            col_end: 2,
        };

        let fourth_val = Number {
            value: 9,
            row_idx: 1,
            col_start: 4,
            col_end: 4,
        };

        let last_val = Number {
            value: 598,
            row_idx: 2,
            col_start: 7,
            col_end: 9,
        };

        assert_eq!(first_val, *get_numbers(lines_vec).get(0).unwrap());
        assert_eq!(third_val, *get_numbers(lines_vec).get(2).unwrap());
        assert_eq!(fourth_val, *get_numbers(lines_vec).get(3).unwrap());
        assert_eq!(last_val, *get_numbers(lines_vec).get(5).unwrap());
    }

    #[test]
    fn test_get_adjacent_coords_for_num() {
        let num = Number {
            value: 46,
            row_idx: 0,
            col_start: 0,
            col_end: 1,
        };

        // ****
        // *46*
        // ****

        let mut adj_coords: Vec<(i64, i64)> = Vec::new();
        adj_coords.extend(vec![(-1, -1), (-1, 0), (-1, 1), (-1, 2)]);
        adj_coords.extend(vec![(1, -1), (1, 0), (1, 1), (1, 2)]);
        adj_coords.extend(vec![(0, -1), (0, 2)]);

        let mut result = get_adjacent_coords_for_num(&num);

        adj_coords.sort();
        result.sort();

        assert_eq!(adj_coords, result);
    }

    #[test]
    fn test_get_part_nums_based_on_gears() {
        let string = "467..114..\n...$.*....\n.664.598..";
        let lines_vec = &string.lines().collect();

        let mut gear_locs = get_gear_locs(lines_vec);
        let mut nums = get_numbers(lines_vec);

        assert_eq!(4, get_part_nums_based_on_gears(&nums, &gear_locs).len());

        let string2 = "467..114..\n..........\n.664.598..";
        let lines_vec2 = &string2.lines().collect();

        gear_locs = get_gear_locs(lines_vec2);
        nums = get_numbers(lines_vec2);

        assert_eq!(0, get_part_nums_based_on_gears(&nums, &gear_locs).len())
    }

    #[test]
    fn test_solve_part_a() {
        let string = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        assert_eq!(4361, solve_part_a(string));
    }
}

#[cfg(test)]
mod test_part_b {
    use day03::{
        part_a::get_numbers,
        part_b::{find_star_ratios, get_star_locs},
    };

    #[test]
    fn test_get_star_locs() {
        let string = "467..114..\n...$.*...*\n.664.598..";
        let lines_vec = &string.lines().collect();

        let expected = vec![(1, 5), (1, 9)];

        assert_eq!(get_star_locs(lines_vec), expected);
    }

    #[test]
    fn test_find_star_ratios() {
        let string = "467*.114..\n....9.....\n.664...598";
        let lines_vec = &string.lines().collect();

        let nums = get_numbers(lines_vec);
        let star_locs = get_star_locs(lines_vec);

        assert_eq!(
            *find_star_ratios(&star_locs, &nums).get(0).unwrap(),
            467 * 9
        );
    }
}
