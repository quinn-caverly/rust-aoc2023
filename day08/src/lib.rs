pub mod part_a {
    use core::panic;
    use std::collections::HashMap;

    pub fn parse_node_str(node_str: &str) -> (Vec<char>, (Vec<char>, Vec<char>)) {
        let mut key_and_val = node_str.split(" = ");
        let key_str = key_and_val.next().unwrap();

        let mut val_str = key_and_val.next().unwrap();
        val_str = val_str
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap();

        let mut left_and_right = val_str.split(", ");

        let (left_str, right_str) = (
            left_and_right.next().unwrap(),
            left_and_right.next().unwrap(),
        );

        (
            key_str.chars().collect(),
            (left_str.chars().collect(), right_str.chars().collect()),
        )
    }

    pub fn parse_input(input_str: &str) -> (Vec<char>, HashMap<Vec<char>, (Vec<char>, Vec<char>)>) {
        let mut portions = input_str.lines();

        let nav = portions.next().unwrap().chars().collect::<Vec<char>>();

        portions.next().unwrap();

        let nodes: HashMap<Vec<char>, (Vec<char>, Vec<char>)> =
            portions.map(|node_str| parse_node_str(node_str)).collect();

        (nav, nodes)
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let (nav, nodes) = parse_input(input_str);

        let mut cur_idx = 0;
        let mut cur_node = vec!['A', 'A', 'A'];

        let mut steps = 0;

        while cur_node != vec!['Z', 'Z', 'Z'] {
            let cur_nav = nav.get(cur_idx).unwrap();

            let cur_path = nodes.get(&cur_node).unwrap();

            match cur_nav {
                'L' => cur_node = cur_path.0.clone(),
                'R' => cur_node = cur_path.1.clone(),
                _ => panic!("Did not expect neither L or R"),
            }

            steps += 1;
            cur_idx += 1;
            if cur_idx == nav.len() {
                cur_idx = 0;
            }
        }

        steps
    }
}

pub mod part_b {
    use std::collections::{HashMap, HashSet};

    use crate::part_a::parse_input;

    pub fn find_circuit(
        start: Vec<char>,
        nav: &Vec<char>,
        nodes: &HashMap<Vec<char>, (Vec<char>, Vec<char>)>,
    ) -> (usize, Vec<usize>) {
        // we want to go until we repeat a configuration
        // once we have repeated a configuration, we will know what that loop for this start
        // node is, this can be used to synchronize all the nodes
        let mut steps = 0;
        let mut nav_idx = 0;
        let mut steps_on_ender: Vec<usize> = Vec::new();

        let mut cur_node = start;

        // configuration determined by the index of nav and cur node
        let mut configurations: HashSet<(usize, Vec<char>)> = HashSet::new();

        loop {
            let cur_nav = nav.get(nav_idx).unwrap();
            let cur_path = nodes.get(&cur_node).unwrap();
            match cur_nav {
                'L' => cur_node = cur_path.0.clone(),
                'R' => cur_node = cur_path.1.clone(),
                _ => panic!("Did not expect neither L or R"),
            }

            nav_idx += 1;
            steps += 1;
            if nav_idx == nav.len() {
                nav_idx = 0;
            }

            if cur_node.get(2).unwrap() == &'Z' {
                steps_on_ender.push(steps);
            }

            if configurations.contains(&(nav_idx, cur_node.clone())) {
                break;
            }
            configurations.insert((nav_idx, cur_node.clone()));
        }

        (steps, steps_on_ender)
    }

    pub struct EndIterator {
        steps_for_circuit: usize,
        steps_on_ender: Vec<usize>,
        cur_idx: usize,
        cur_loop: usize,
    }

    impl EndIterator {
        pub fn new(steps_for_circuit: usize, mut steps_on_ender: Vec<usize>) -> EndIterator {
            // make sure that they are sorted from smallest to largest nums
            steps_on_ender.sort();

            EndIterator {
                steps_for_circuit,
                steps_on_ender,
                cur_idx: 0,
                cur_loop: 0,
            }
        }
    }

    // infinite iterator
    impl Iterator for EndIterator {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            let to_return = self.steps_on_ender.get(self.cur_idx).unwrap()
                + (self.steps_for_circuit - 1) * self.cur_loop;

            self.cur_idx += 1;
            if self.cur_idx == self.steps_on_ender.len() {
                self.cur_idx = 0;
                self.cur_loop += 1;
            }

            Some(to_return)
        }
    }

    pub fn gen_all_end_iters(info: &Vec<(usize, Vec<usize>)>) -> Vec<EndIterator> {
        let mut end_iters = Vec::new();
        for (steps, steps_on_ender) in info {
            end_iters.push(EndIterator::new(steps.clone(), steps_on_ender.clone()));
        }
        end_iters
    }

    pub fn find_all_start_nodes(
        nodes: &HashMap<Vec<char>, (Vec<char>, Vec<char>)>,
    ) -> Vec<Vec<char>> {
        let mut start_nodes = Vec::new();

        for node in nodes.keys() {
            if node.get(2).unwrap() == &'A' {
                start_nodes.push(node.clone());
            }
        }

        start_nodes
    }

    pub fn gen_info_from_start_nodes(
        start_nodes: Vec<Vec<char>>,
        nav: &Vec<char>,
        nodes: &HashMap<Vec<char>, (Vec<char>, Vec<char>)>,
    ) -> Vec<(usize, Vec<usize>)> {
        let mut info = Vec::new();
        for start in start_nodes {
            info.push(find_circuit(start, nav, nodes));
        }
        info
    }

    fn helper_func(
        vec_of_last_gened_vals: &mut Vec<usize>,
        all_end_iters: &mut Vec<EndIterator>,
    ) -> Option<usize> {
        // now, we go through repeatedly finding the index of the lowest value
        // and then update it, we check if all indices have the same value,
        // if they have the same value we are done

        println!("{:?}", vec_of_last_gened_vals);

        // check if all are the same
        let first_val = vec_of_last_gened_vals.get(0).unwrap();
        let mut failed = false;
        for i in 1..vec_of_last_gened_vals.len() {
            if vec_of_last_gened_vals.get(i).unwrap() != first_val {
                failed = true;
                break;
            }
        }

        if failed == false {
            return Some(*first_val);
        }

        let mut smallest_val = vec_of_last_gened_vals.get(0).unwrap();
        let mut cor_idx = 0;
        for i in 1..vec_of_last_gened_vals.len() {
            if vec_of_last_gened_vals.get(i).unwrap() < smallest_val {
                smallest_val = vec_of_last_gened_vals.get(i).unwrap();
                cor_idx = i;
            }
        }

        // update the smallest value and return None so that the process is repeated
        *vec_of_last_gened_vals.get_mut(cor_idx).unwrap() =
            all_end_iters.get_mut(cor_idx).unwrap().next().unwrap();

        None
    }

    pub fn find_lowest_common_denom(info: &Vec<(usize, Vec<usize>)>) -> usize {
        // if it takes 3 steps to complete the circuit and we will be on Z at
        // step 2, then we will be on Z at 2, 5, 8, etc. 2 + (3*n)

        // however, for any given start node, it can be on a Z at multiple different points
        // in the circuit. This means that it is not as simple as each start node gets a
        // linear equation.

        // perhaps we can create iterators which will always give the next value
        // for the current node, we iterate the lowest value each time until
        // each start node has the same value, which indicates the step # where
        // they will all be synchronized

        let mut all_end_iters = gen_all_end_iters(info);
        let mut vec_of_last_gened_vals = Vec::new();
        for i in 0..all_end_iters.len() {
            vec_of_last_gened_vals.push(all_end_iters.get_mut(i).unwrap().next().unwrap());
        }

        loop {
            let val = helper_func(&mut vec_of_last_gened_vals, &mut all_end_iters);
            if let Some(x) = val {
                return x;
            }
        }
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let (nav, nodes) = parse_input(input_str);
        let start_nodes = find_all_start_nodes(&nodes);
        let info = gen_info_from_start_nodes(start_nodes, &nav, &nodes);

        find_lowest_common_denom(&info)
    }
}
