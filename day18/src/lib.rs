pub mod part_a {
    // use a HashSet of coords isize to build the initial loop
    // convert to a grid of usize with a buffer
    // use dfs or bfs from the corner to find all spots outside the loop
    // count the spots which were not reached, this is the area of the loop

    use core::panic;
    use std::collections::HashSet;

    #[derive(Debug, PartialEq)]
    pub enum Dir {
        Right,
        Left,
        Down,
        Up,
    }

    #[derive(Debug, PartialEq)]
    pub struct Step {
        pub dir: Dir,
        pub length: isize,
    }

    pub fn parse_input(input_str: &str) -> Vec<Step> {
        input_str
            .lines()
            .map(|line| {
                let mut portions = line.split(" ");
                let dir_str = portions.next().unwrap();
                let num_str = portions.next().unwrap();

                let dir = match dir_str {
                    "R" => Dir::Right,
                    "L" => Dir::Left,
                    "D" => Dir::Down,
                    "U" => Dir::Up,
                    _ => panic!("Did not expect: {}", dir_str),
                };

                Step {
                    dir,
                    length: num_str.parse::<isize>().unwrap(),
                }
            })
            .collect()
    }

    pub fn build_loop(steps: Vec<Step>) -> HashSet<(isize, isize)> {
        let mut set = HashSet::new();

        let mut cur_pos: (isize, isize) = (0, 0);
        set.insert(cur_pos);

        for step in steps {
            for _ in 1..=step.length {
                cur_pos = match step.dir {
                    Dir::Right => (cur_pos.0, cur_pos.1 + 1),
                    Dir::Left => (cur_pos.0, cur_pos.1 - 1),
                    Dir::Down => (cur_pos.0 + 1, cur_pos.1),
                    Dir::Up => (cur_pos.0 - 1, cur_pos.1),
                };

                set.insert(cur_pos);
            }
        }

        set
    }

    pub fn find_lowest_highest(set: &HashSet<(isize, isize)>) -> ((isize, isize), (isize, isize)) {
        let mut lowest_x = 0;
        let mut lowest_y = 0;

        let mut highest_x = 0;
        let mut highest_y = 0;

        for (a, b) in set {
            if a < &lowest_x {
                lowest_x = *a;
            }
            if b < &lowest_y {
                lowest_y = *b;
            }

            if a > &highest_x {
                highest_x = *a;
            }
            if b > &highest_y {
                highest_y = *b;
            }
        }

        ((lowest_x, lowest_y), (highest_x, highest_y))
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Node {
        Brd,
        Emp,
        Dug,
        Vis,
    }

    pub fn build_usize_graph(
        set: &HashSet<(isize, isize)>,
        lowest_row: isize,
        lowest_col: isize,
        highest_row: isize,
        highest_col: isize,
    ) -> Vec<Vec<Node>> {
        // becuase lowest_x, lowest_y will be negative or 0, add their inverse to
        // highest vals to get the top value, note this isnt the width/height but index

        let rows = (highest_row - lowest_row + 1) as usize;
        let cols = (highest_col - lowest_col + 1) as usize;

        let mut graph = Vec::new();
        let border_topper = vec![Node::Brd; cols + 4];

        let mut temp = vec![Node::Emp; cols + 2];
        let mut empty_topper = vec![Node::Brd];
        empty_topper.append(&mut temp);
        empty_topper.push(Node::Brd);

        graph.push(border_topper.clone());
        graph.push(empty_topper.clone());

        for row in 0..rows {
            let mut cur_row = Vec::new();
            cur_row.push(Node::Brd);
            cur_row.push(Node::Emp);

            for col in 0..cols {
                if set.contains(&((row as isize) + lowest_row, (col as isize) + lowest_col)) {
                    cur_row.push(Node::Dug);
                } else {
                    cur_row.push(Node::Emp);
                }
            }

            cur_row.push(Node::Emp);
            cur_row.push(Node::Brd);
            graph.push(cur_row);
        }

        graph.push(empty_topper);
        graph.push(border_topper);
        graph
    }

    pub fn run_search(graph: &mut Vec<Vec<Node>>) {
        let mut to_visit = vec![(1, 1)];
        'outer: while to_visit.len() > 0 {
            let (cur_row, cur_col) = to_visit.pop().unwrap();
            if *graph.get(cur_row).unwrap().get(cur_col).unwrap() != Node::Emp {
                continue 'outer;
            }
            *graph.get_mut(cur_row).unwrap().get_mut(cur_col).unwrap() = Node::Vis;

            let coords = vec![
                (cur_row - 1, cur_col),
                (cur_row + 1, cur_col),
                (cur_row, cur_col + 1),
                (cur_row, cur_col - 1),
            ];

            for coord in coords {
                if *graph.get(coord.0).unwrap().get(coord.1).unwrap() == Node::Emp {
                    to_visit.push(coord);
                }
            }
        }
    }

    pub fn count_area(graph: &Vec<Vec<Node>>) -> usize {
        // we count all Nodes which are Emp or Dug
        let mut ans = 0;
        for row in graph {
            for node in row {
                if *node == Node::Emp || *node == Node::Dug {
                    ans += 1;
                }
            }
        }
        ans
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let steps = parse_input(input_str);
        let set = build_loop(steps);
        let ((lowest_row, lowest_col), (highest_row, highest_col)) = find_lowest_highest(&set);
        let mut graph = build_usize_graph(&set, lowest_row, lowest_col, highest_row, highest_col);

        run_search(&mut graph);
        count_area(&graph)
    }
}

pub mod part_b {
    use crate::part_a::{
        build_loop, build_usize_graph, count_area, find_lowest_highest, run_search, Dir, Step,
    };

    pub fn parse_input_b(input_str: &str) -> Vec<Step> {
        input_str
            .lines()
            .map(|line| {
                let mut portions = line.split(" ");
                let mut hexa = portions.nth(2).unwrap();
                hexa = hexa.strip_prefix("(#").unwrap().strip_suffix(")").unwrap();

                let hexa_distance = &hexa[0..5];
                let encoded_dir = hexa.chars().nth(5).unwrap();

                let dir = match encoded_dir {
                    '0' => Dir::Right,
                    '1' => Dir::Down,
                    '2' => Dir::Left,
                    '3' => Dir::Up,
                    _ => panic!(),
                };

                Step {
                    length: isize::from_str_radix(hexa_distance, 16).unwrap(),
                    dir,
                }
            })
            .collect()
    }

    pub fn shoelace_formula(points: &Vec<(isize, isize)>) -> isize {
        let len = points.len();

        let (area, perimeter) =
            points
                .iter()
                .enumerate()
                .fold((0isize, 0isize), |(sum, perimeter), (i, p1)| {
                    let l = (i + 1) % len;
                    let p2 = points[l];

                    let manhat_distance = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();

                    let new_perimeter = perimeter + manhat_distance;
                    let new_area = sum + (p1.1 * p2.0) - (p1.0 * p2.1);

                    (new_area, new_perimeter)
                });

        (area.abs() + perimeter) / 2 + 1
    }

    pub fn get_vertices_from_steps(steps: &Vec<Step>) -> Vec<(isize, isize)> {
        // we make it back to the beginning so we don't need to add (0, 0)
        let mut vertices = vec![];
        let mut cur = (0, 0);

        for step in steps {
            cur = match step.dir {
                Dir::Right => (cur.0, cur.1 + step.length),
                Dir::Left => (cur.0, cur.1 - step.length),
                Dir::Down => (cur.0 + step.length, cur.1),
                Dir::Up => (cur.0 - step.length, cur.1),
            };
            vertices.push(cur);
        }

        vertices
    }

    pub fn solve_part_b(input_str: &str) -> isize {
        let vertices = get_vertices_from_steps(&parse_input_b(input_str));
        shoelace_formula(&vertices)
    }
}
