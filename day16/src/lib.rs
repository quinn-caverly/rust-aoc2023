pub mod part_a {
    use std::collections::HashSet;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Tile {
        Empty,
        FwdMirror,
        BackMirror,
        Horizi,
        Verti,
        Border,
    }

    // will add a border for ease of simulation
    pub fn parse_input(input_str: &str) -> Vec<Vec<Tile>> {
        let mut inter_grid: Vec<Vec<Tile>> = input_str
            .lines()
            .map(|line| {
                let mut tiles = vec![Tile::Border];

                tiles.append(
                    &mut line
                        .chars()
                        .map(|ch| match ch {
                            '\\' => Tile::BackMirror,
                            '/' => Tile::FwdMirror,
                            '.' => Tile::Empty,
                            '|' => Tile::Verti,
                            '-' => Tile::Horizi,
                            _ => panic!("Did not expect: {}", ch),
                        })
                        .collect(),
                );

                tiles.push(Tile::Border);

                tiles
            })
            .collect();

        let pole = vec![Tile::Border; inter_grid.get(0).unwrap().len()];
        inter_grid.push(pole.clone());

        let mut final_grid = vec![pole];
        final_grid.append(&mut inter_grid);
        final_grid
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    pub enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    pub struct Beam {
        pub row_idx: usize,
        pub col_idx: usize,
        pub dir: Dir,
    }

    // true if beam still exists, false if it is over
    pub fn take_step(beam: &mut Beam, grid: &Vec<Vec<Tile>>) -> (bool, Option<Beam>) {
        match grid.get(beam.row_idx).unwrap().get(beam.col_idx).unwrap() {
            Tile::Empty => match beam.dir {
                Dir::Up => beam.row_idx -= 1,
                Dir::Down => beam.row_idx += 1,
                Dir::Left => beam.col_idx -= 1,
                Dir::Right => beam.col_idx += 1,
            },
            Tile::FwdMirror => match beam.dir {
                Dir::Up => {
                    beam.dir = Dir::Right;
                    beam.col_idx += 1;
                }
                Dir::Down => {
                    beam.dir = Dir::Left;
                    beam.col_idx -= 1;
                }
                Dir::Left => {
                    beam.dir = Dir::Down;
                    beam.row_idx += 1;
                }
                Dir::Right => {
                    beam.dir = Dir::Up;
                    beam.row_idx -= 1;
                }
            },
            Tile::BackMirror => match beam.dir {
                Dir::Up => {
                    beam.dir = Dir::Left;
                    beam.col_idx -= 1;
                }
                Dir::Down => {
                    beam.dir = Dir::Right;
                    beam.col_idx += 1;
                }
                Dir::Left => {
                    beam.dir = Dir::Up;
                    beam.row_idx -= 1;
                }
                Dir::Right => {
                    beam.dir = Dir::Down;
                    beam.row_idx += 1;
                }
            },
            Tile::Horizi => match beam.dir {
                Dir::Up | Dir::Down => {
                    let new_beam = Beam {
                        row_idx: beam.row_idx,
                        col_idx: beam.col_idx + 1,
                        dir: Dir::Right,
                    };

                    beam.col_idx -= 1;
                    beam.dir = Dir::Left;

                    return (true, Some(new_beam));
                }
                Dir::Left => beam.col_idx -= 1,
                Dir::Right => beam.col_idx += 1,
            },
            Tile::Verti => match beam.dir {
                Dir::Left | Dir::Right => {
                    let new_beam = Beam {
                        row_idx: beam.row_idx - 1,
                        col_idx: beam.col_idx,
                        dir: Dir::Up,
                    };

                    beam.row_idx += 1;
                    beam.dir = Dir::Down;

                    return (true, Some(new_beam));
                }
                Dir::Up => beam.row_idx -= 1,
                Dir::Down => beam.row_idx += 1,
            },
            Tile::Border => return (false, None),
        }

        (true, None)
    }

    pub fn simulate(grid: &Vec<Vec<Tile>>) -> Vec<Vec<usize>> {
        let mut intensity = vec![vec![0; grid.get(0).unwrap().len()]; grid.len()];
        // 1, 1 because of the buffer / border
        let mut beams = vec![];

        let mut cur_beam_opt = Some(Beam {
            row_idx: 1,
            col_idx: 1,
            dir: Dir::Right,
        });

        let mut hit_beams: HashSet<Beam> = HashSet::new();

        'w: while beams.len() > 0 || cur_beam_opt.is_some() {
            if cur_beam_opt.is_none() {
                cur_beam_opt = Some(beams.pop().unwrap());
            }
            let mut cur_beam = cur_beam_opt.unwrap();

            if hit_beams.contains(&cur_beam) {
                cur_beam_opt = None;
                continue 'w;
            }
            hit_beams.insert(cur_beam.clone());

            if grid
                .get(cur_beam.row_idx)
                .unwrap()
                .get(cur_beam.col_idx)
                .unwrap()
                != &Tile::Border
            {
                *intensity
                    .get_mut(cur_beam.row_idx)
                    .unwrap()
                    .get_mut(cur_beam.col_idx)
                    .unwrap() += 1;
            }
            let (keep_beam, new_beam_opt) = take_step(&mut cur_beam, grid);

            if keep_beam {
                cur_beam_opt = Some(cur_beam);
            } else {
                cur_beam_opt = None;
            }

            if let Some(new_beam) = new_beam_opt {
                beams.push(new_beam);
            }
        }

        intensity
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let grid = parse_input(input_str);
        let intensity = simulate(&grid);

        let mut ans = 0;
        for row in intensity {
            for x in row {
                if x > 0 {
                    ans += 1;
                }
            }
        }
        ans
    }
}

pub mod part_b {
    use crate::part_a::{parse_input, take_step, Beam, Dir, Tile};
    use std::collections::HashSet;

    pub fn simulate(grid: &Vec<Vec<Tile>>, start_beam: Beam) -> Vec<Vec<usize>> {
        let mut intensity = vec![vec![0; grid.get(0).unwrap().len()]; grid.len()];
        // 1, 1 because of the buffer / border
        let mut beams = vec![];

        let mut cur_beam_opt = Some(start_beam);

        let mut hit_beams: HashSet<Beam> = HashSet::new();

        'w: while beams.len() > 0 || cur_beam_opt.is_some() {
            if cur_beam_opt.is_none() {
                cur_beam_opt = Some(beams.pop().unwrap());
            }
            let mut cur_beam = cur_beam_opt.unwrap();

            if hit_beams.contains(&cur_beam) {
                cur_beam_opt = None;
                continue 'w;
            }
            hit_beams.insert(cur_beam.clone());

            if grid
                .get(cur_beam.row_idx)
                .unwrap()
                .get(cur_beam.col_idx)
                .unwrap()
                != &Tile::Border
            {
                *intensity
                    .get_mut(cur_beam.row_idx)
                    .unwrap()
                    .get_mut(cur_beam.col_idx)
                    .unwrap() += 1;
            }
            let (keep_beam, new_beam_opt) = take_step(&mut cur_beam, grid);

            if keep_beam {
                cur_beam_opt = Some(cur_beam);
            } else {
                cur_beam_opt = None;
            }

            if let Some(new_beam) = new_beam_opt {
                beams.push(new_beam);
            }
        }

        intensity
    }

    pub fn calculate_val(grid: &Vec<Vec<Tile>>, start_beam: Beam) -> usize {
        let intensity = simulate(&grid, start_beam);
        let mut ans = 0;
        for row in intensity {
            for x in row {
                if x > 0 {
                    ans += 1;
                }
            }
        }
        ans
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let grid = parse_input(input_str);

        let mut starting_beams = vec![];
        // the top and the bottom
        for i in 1..grid.get(0).unwrap().len() - 1 {
            starting_beams.push(Beam {
                row_idx: 1,
                col_idx: i,
                dir: Dir::Down,
            });
            starting_beams.push(Beam {
                row_idx: grid.len() - 2,
                col_idx: i,
                dir: Dir::Up,
            });
        }
        // left and right
        for i in 1..grid.len() - 1 {
            starting_beams.push(Beam {
                row_idx: i,
                col_idx: 1,
                dir: Dir::Right,
            });
            starting_beams.push(Beam {
                row_idx: i,
                col_idx: grid.get(0).unwrap().len() - 2,
                dir: Dir::Left,
            });
        }

        let mut max = 0;
        for start_beam in starting_beams {
            let cur = calculate_val(&grid, start_beam);
            if cur > max {
                max = cur;
            }
        }

        max
    }
}
