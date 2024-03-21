pub mod part_a {

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Slot {
        Rnd,
        Cube,
        Emp,
    }

    pub fn parse_input(input_str: &str) -> Vec<Vec<Slot>> {
        input_str
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        'O' => Slot::Rnd,
                        '#' => Slot::Cube,
                        '.' => Slot::Emp,
                        _ => panic!("Did not expect neither O, #, ."),
                    })
                    .collect()
            })
            .collect()
    }

    pub fn transpose(grid: &Vec<Vec<Slot>>) -> Vec<Vec<Slot>> {
        let mut transposed = Vec::new();

        for i in 0..grid.get(0).unwrap().len() {
            let mut cur = Vec::new();
            for row in grid {
                cur.push(row.get(i).unwrap().clone());
            }
            transposed.push(cur);
        }

        transposed
    }

    pub fn count_circles(row: &Vec<Slot>, mut idx: usize) -> usize {
        let mut rounds = 0;
        'outer: while idx < row.len() {
            match row.get(idx).unwrap() {
                Slot::Rnd => rounds += 1,
                Slot::Cube => break 'outer,
                Slot::Emp => (),
            }
            idx += 1;
        }
        rounds
    }

    pub fn add_circles(row: &mut Vec<Slot>, circles: usize) {
        for _ in 0..circles {
            row.push(Slot::Rnd);
        }
    }

    pub fn shift_left(grid: &Vec<Vec<Slot>>) -> Vec<Vec<Slot>> {
        let mut new_grid = Vec::new();

        for row in grid {
            let mut new_row = Vec::new();
            // go forward and take all the circles for this section
            // then, plug in periods until we hit the # then repeat
            let mut i = 0;

            let circles = count_circles(&row, 0);
            add_circles(&mut new_row, circles);
            i += circles;

            while i < row.len() {
                match row.get(i).unwrap() {
                    Slot::Rnd => {
                        new_row.push(Slot::Emp);
                        i += 1;
                    }
                    Slot::Cube => {
                        new_row.push(Slot::Cube);
                        i += 1;

                        let circles = count_circles(&row, i);
                        add_circles(&mut new_row, circles);
                        i += circles;
                    }
                    Slot::Emp => {
                        new_row.push(Slot::Emp);
                        i += 1;
                    }
                }
            }

            new_grid.push(new_row);
        }

        new_grid
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let grid = parse_input(input_str);
        let transposed = transpose(&grid);
        let shifted = shift_left(&transposed);

        let mut total = 0;
        for row in shifted {
            for (idx, elem) in row.iter().enumerate() {
                if elem == &Slot::Rnd {
                    total += row.len() - idx;
                }
            }
        }
        total
    }
}

pub mod part_b {
    use std::collections::HashMap;

    use crate::part_a::{parse_input, shift_left, Slot};

    pub fn transpose_in_place(grid: &mut Vec<Vec<Slot>>) {
        let rows = grid.len();
        let cols = grid.get(0).unwrap().len();

        for i in 0..rows {
            for j in i + 1..cols {
                let temp = grid.get(i).unwrap().get(j).unwrap().clone();

                *grid.get_mut(i).unwrap().get_mut(j).unwrap() =
                    grid.get(j).unwrap().get(i).unwrap().clone();

                *grid.get_mut(j).unwrap().get_mut(i).unwrap() = temp;
            }
        }
    }

    pub fn reverse_transpose_in_place(grid: &mut Vec<Vec<Slot>>) {
        transpose_in_place(grid);
        reverse_each_row(grid);
    }

    // starts off as North to the right
    pub fn other_axis_transpose_in_place(grid: &mut Vec<Vec<Slot>>) {
        // undos North to the right
        reverse_each_row(grid);
        transpose_in_place(grid);

        // now make North to the south
        grid.reverse();
        reverse_each_row(grid);
    }

    pub fn reverse_each_row(grid: &mut Vec<Vec<Slot>>) {
        for row in grid.iter_mut() {
            row.reverse();
        }
    }

    pub fn run_cycle(grid: &mut Vec<Vec<Slot>>) -> Vec<Vec<Slot>> {
        // one transpose creates north to the left
        transpose_in_place(grid);
        let mut grid = shift_left(&grid);

        // transpose again makes north up, west left
        transpose_in_place(&mut grid);
        grid = shift_left(&grid);

        // reverse transposed makes north to the right
        reverse_transpose_in_place(&mut grid);
        grid = shift_left(&grid);

        // from reverse transpose, we directly get east as left
        other_axis_transpose_in_place(&mut grid);
        grid = shift_left(&grid);

        // now because we still have east as left, we just need to reverse
        // each row in order to get north as up again
        grid.reverse();
        reverse_each_row(&mut grid);
        grid
    }

    pub fn calc_total_load_on_north(grid: &Vec<Vec<Slot>>) -> usize {
        let mut total = 0;
        for row in grid {
            for (idx, elem) in row.iter().enumerate() {
                if elem == &Slot::Rnd {
                    total += row.len() - idx;
                }
            }
        }
        total
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let mut grid = parse_input(input_str);
        let mut seen_formations: HashMap<Vec<Vec<Slot>>, usize> = HashMap::new();

        const TOTAL_CYCLES: usize = 1000000000;

        let mut i = 1;
        let mut done_skipping = false;
        'outer: while i <= TOTAL_CYCLES {
            grid = run_cycle(&mut grid);

            if !done_skipping {
                if let Some(&x) = seen_formations.get(&grid) {
                    // this means that we hit this formation after step x:
                    // so we have just run step i, meaning at i + (i-x) we will
                    // hit is again, so while (i + (i-x)) is less than total cycles, just add
                    // cycle_length

                    println!("origi i: {}, {}", i, calc_total_load_on_north(&grid));
                    println!("{}", i-x);

                    let cycle_length = i - x;
                    let full_cycles = (TOTAL_CYCLES - i) / cycle_length;
                    i += full_cycles * cycle_length;

                    println!("special {}: {}", i, calc_total_load_on_north(&grid));

                    done_skipping = true;
                    continue 'outer;
                }
                seen_formations.insert(grid.clone(), i);
            }

            println!("{}: {}", i, calc_total_load_on_north(&grid));

            // println!("After: {} cycles", i);
            // for row in &grid {
            //     for slot in row {
            //         match slot {
            //             Slot::Rnd => print!("O"),
            //             Slot::Cube => print!("#"),
            //             Slot::Emp => print!("."),
            //         }
            //     }
            //     print!("\n")
            // }

            i += 1;
        }

        calc_total_load_on_north(&grid)
    }
}
