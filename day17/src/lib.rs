pub mod part_a {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
    };

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Pointer {
        pub row: usize,
        pub col: usize,
        pub dir: Dir,

        pub ticker: usize, // the steps we have been going in this dir
    }

    pub fn find_pos_locations(pointer: Pointer, rows: usize, cols: usize) -> Vec<Pointer> {
        // return is the list of potential vectors, can be a max of 3
        // max of 3 because we can't turn 180 degrees just 90 or straight
        let mut pos = vec![];

        // first try forward
        if pointer.ticker < 3 {
            let row_col_opt = match pointer.dir {
                Dir::Up => {
                    if pointer.row > 0 {
                        Some((pointer.row - 1, pointer.col))
                    } else {
                        None
                    }
                }
                Dir::Down => {
                    if pointer.row < rows - 1 {
                        Some((pointer.row + 1, pointer.col))
                    } else {
                        None
                    }
                }
                Dir::Left => {
                    if pointer.col > 0 {
                        Some((pointer.row, pointer.col - 1))
                    } else {
                        None
                    }
                }
                Dir::Right => {
                    if pointer.col < cols - 1 {
                        Some((pointer.row, pointer.col + 1))
                    } else {
                        None
                    }
                }
            };

            match row_col_opt {
                Some((new_row, new_col)) => pos.push(Pointer {
                    row: new_row,
                    col: new_col,
                    dir: pointer.dir,

                    ticker: pointer.ticker + 1,
                }),
                None => (),
            }
        }

        // each of this has a conditional based on the boundaries
        match pointer.dir {
            Dir::Up | Dir::Down => {
                if pointer.col > 0 {
                    pos.push(Pointer {
                        row: pointer.row,
                        col: pointer.col - 1,
                        dir: Dir::Left,
                        ticker: 1,
                    })
                }
                if pointer.col < cols - 1 {
                    pos.push(Pointer {
                        row: pointer.row,
                        col: pointer.col + 1,
                        dir: Dir::Right,
                        ticker: 1,
                    })
                }
            }
            Dir::Left | Dir::Right => {
                if pointer.row > 0 {
                    pos.push(Pointer {
                        row: pointer.row - 1,
                        col: pointer.col,
                        dir: Dir::Up,
                        ticker: 1,
                    })
                }
                if pointer.row < rows - 1 {
                    pos.push(Pointer {
                        row: pointer.row + 1,
                        col: pointer.col,
                        dir: Dir::Down,
                        ticker: 1,
                    })
                }
            }
        }

        pos
    }

    pub fn parse_input(input_str: &str) -> HashMap<(usize, usize), usize> {
        input_str
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col_idx, ch)| {
                        (
                            (row_idx, col_idx),
                            ch.to_digit(10).unwrap().try_into().unwrap(),
                        )
                    })
                    .collect::<Vec<((usize, usize), usize)>>()
            })
            .collect()
    }

    #[derive(Eq, PartialEq)]
    pub struct BinHeapElem {
        pointer: Pointer,
        heat: usize,
    }

    impl PartialOrd for BinHeapElem {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for BinHeapElem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let order = self.heat.cmp(&other.heat);
            match order {
                std::cmp::Ordering::Less => order,
                std::cmp::Ordering::Equal => self.pointer.ticker.cmp(&other.pointer.ticker),
                std::cmp::Ordering::Greater => order,
            }
        }
    }

    pub fn traverse_through(
        nodes: &HashMap<(usize, usize), usize>,
        rows: usize,
        cols: usize,
        traverse_function: fn(Pointer, usize, usize) -> Vec<Pointer>,
        is_ultra: bool,
    ) -> HashMap<Pointer, usize> {
        let right = BinHeapElem {
            pointer: Pointer {
                row: 0,
                col: 1,
                dir: Dir::Right,
                ticker: 1,
            },
            heat: *nodes.get(&(0, 1)).unwrap(),
        };
        let down = BinHeapElem {
            pointer: Pointer {
                row: 1,
                col: 0,
                dir: Dir::Down,
                ticker: 1,
            },
            heat: *nodes.get(&(1, 0)).unwrap(),
        };

        let mut bin_heap: BinaryHeap<Reverse<BinHeapElem>> =
            vec![Reverse(down), Reverse(right)].into_iter().collect();
        let mut distances: HashMap<Pointer, usize> = HashMap::new();

        'outer: while bin_heap.len() > 0 {
            let Reverse(bin_heap_elem) = bin_heap.pop().unwrap();
            let (cur_pointer, cur_heat) = (bin_heap_elem.pointer, bin_heap_elem.heat);

            // we also want to check if we have hit this point with this pointer but
            // also possibly with a lesser num of ticker

            // if ultra, we don't want to count down on the tickers
            if is_ultra {
                if let Some(&cur_lowest) = distances.get(&cur_pointer) {
                    if cur_heat >= cur_lowest {
                        continue 'outer;
                    }
                }
            } else {
                let mut cur_ticker = cur_pointer.ticker;
                while cur_ticker > 0 {
                    let query = Pointer {
                        row: cur_pointer.row,
                        col: cur_pointer.col,
                        dir: cur_pointer.dir,
                        ticker: cur_ticker,
                    };

                    if let Some(&cur_lowest) = distances.get(&query) {
                        if cur_heat >= cur_lowest {
                            continue 'outer;
                        }
                    }

                    cur_ticker -= 1;
                }
            }

            distances.insert(cur_pointer.clone(), cur_heat);

            // if we don't retire then we keep looking
            let new_pointers = traverse_function(cur_pointer, rows, cols);

            for pointer in new_pointers {
                bin_heap.push(Reverse(BinHeapElem {
                    pointer: pointer.clone(),
                    heat: cur_heat
                        + *nodes
                            .get(&(pointer.row, pointer.col))
                            .expect(&format!("{}, {}", pointer.row, pointer.col)),
                }));
            }
        }

        distances
    }

    pub fn find_rows_cols(nodes: &HashMap<(usize, usize), usize>) -> (usize, usize) {
        let (mut max_row, mut max_col) = (0, 0);
        for (row, col) in nodes.keys() {
            if row > &max_row {
                max_row = *row;
            }
            if col > &max_col {
                max_col = *col;
            }
        }
        (max_row + 1, max_col + 1)
    }

    pub fn find_shortest_dist_to_point(
        shortest_distances: &HashMap<Pointer, usize>,
        cur_row: usize,
        cur_col: usize,
    ) -> usize {
        // there could be various pointers which reached the end, we need to search all
        let mut shortest = usize::MAX;
        for (pointer, &dist) in shortest_distances.iter() {
            if pointer.row == cur_row && pointer.col == cur_col {
                if dist < shortest {
                    shortest = dist;
                }
            }
        }
        shortest
    }

    pub fn create_shortest_matrix(
        rows: usize,
        cols: usize,
        shortest_distances: &HashMap<Pointer, usize>,
    ) -> Vec<Vec<usize>> {
        let mut mat: Vec<Vec<usize>> = Vec::new();
        for i in 0..rows {
            let mut cur_row: Vec<usize> = vec![];
            for j in 0..cols {
                cur_row.push(find_shortest_dist_to_point(shortest_distances, i, j));
            }
            mat.push(cur_row);
        }
        mat
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let nodes = parse_input(input_str);
        let (rows, cols) = find_rows_cols(&nodes);

        let shortest_distances = traverse_through(&nodes, rows, cols, find_pos_locations, false);
        let shortest_matrix = create_shortest_matrix(rows, cols, &shortest_distances);

        // for row in &shortest_matrix {
        //     println!();
        //     for val in row {
        //         print!("{}, ", val);
        //     }
        // }
        // println!();

        *shortest_matrix
            .get(rows - 1)
            .unwrap()
            .get(cols - 1)
            .unwrap()
    }
}

pub mod part_b {
    use std::collections::HashMap;

    use crate::part_a::{
        create_shortest_matrix, find_rows_cols, parse_input, traverse_through, Dir, Pointer,
    };

    pub fn find_ultra_pos_locations(pointer: Pointer, rows: usize, cols: usize) -> Vec<Pointer> {
        // return is the list of potential vectors, can be a max of 3
        // max of 3 because we can't turn 180 degrees just 90 or straight
        let mut pos = vec![];

        // first try forward
        if pointer.ticker < 10 {
            let row_col_opt = match pointer.dir {
                Dir::Up => {
                    if pointer.row > 0 {
                        Some((pointer.row - 1, pointer.col))
                    } else {
                        None
                    }
                }
                Dir::Down => {
                    if pointer.row < rows - 1 {
                        Some((pointer.row + 1, pointer.col))
                    } else {
                        None
                    }
                }
                Dir::Left => {
                    if pointer.col > 0 {
                        Some((pointer.row, pointer.col - 1))
                    } else {
                        None
                    }
                }
                Dir::Right => {
                    if pointer.col < cols - 1 {
                        Some((pointer.row, pointer.col + 1))
                    } else {
                        None
                    }
                }
            };

            match row_col_opt {
                Some((new_row, new_col)) => pos.push(Pointer {
                    row: new_row,
                    col: new_col,
                    dir: pointer.dir,

                    ticker: pointer.ticker + 1,
                }),
                None => (),
            }
        }

        // each of this has a conditional based on the boundaries
        // now, we can only turn if ticker >= 4

        if pointer.ticker >= 4 {
            match pointer.dir {
                Dir::Up | Dir::Down => {
                    if pointer.col > 0 {
                        pos.push(Pointer {
                            row: pointer.row,
                            col: pointer.col - 1,
                            dir: Dir::Left,
                            ticker: 1,
                        })
                    }
                    if pointer.col < cols - 1 {
                        pos.push(Pointer {
                            row: pointer.row,
                            col: pointer.col + 1,
                            dir: Dir::Right,
                            ticker: 1,
                        })
                    }
                }
                Dir::Left | Dir::Right => {
                    if pointer.row > 0 {
                        pos.push(Pointer {
                            row: pointer.row - 1,
                            col: pointer.col,
                            dir: Dir::Up,
                            ticker: 1,
                        })
                    }
                    if pointer.row < rows - 1 {
                        pos.push(Pointer {
                            row: pointer.row + 1,
                            col: pointer.col,
                            dir: Dir::Down,
                            ticker: 1,
                        })
                    }
                }
            }
        }

        pos
    }

    pub fn find_ans_ultra(
        shortest_distances: &HashMap<Pointer, usize>,
        cur_row: usize,
        cur_col: usize,
    ) -> usize {
        // we can only choose a pointer which has reached the end in >= 4 ticker
        let mut shortest = usize::MAX;
        for (pointer, &dist) in shortest_distances.iter() {
            if pointer.row == cur_row && pointer.col == cur_col && pointer.ticker >= 4 {
                if dist < shortest {
                    shortest = dist;
                }
            }
        }
        shortest
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let nodes = parse_input(input_str);
        let (rows, cols) = find_rows_cols(&nodes);

        let shortest_distances =
            traverse_through(&nodes, rows, cols, find_ultra_pos_locations, true);

        // let shortest_matrix = create_shortest_matrix(rows, cols, &shortest_distances);
        // for row in &shortest_matrix {
        //     println!();
        //     for val in row {
        //         print!("{}, ", val);
        //     }
        // }
        // println!();

        find_ans_ultra(&shortest_distances, rows-1, cols-1)
    }
}
