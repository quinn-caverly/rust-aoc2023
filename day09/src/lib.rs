pub mod part_a {

    pub fn parse_input(input_str: &str) -> Vec<Vec<i64>> {
        input_str
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|num_str| num_str.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect()
    }

    pub fn push_down(seq: Vec<i64>) -> Vec<Vec<i64>> {
        let mut res = vec![seq];

        let mut i = 0;
        loop {
            let mut done = true;
            for val in res.get(i).unwrap() {
                if val != &0 {
                    done = false;
                    break;
                }
            }
            if done {
                return res;
            }

            let mut next_row: Vec<i64> = Vec::new();

            // we want to go to the second to last index
            for j in 0..res.get(i).unwrap().len() - 1 {
                next_row.push(
                    res.get(i).unwrap().get(j + 1).unwrap() - res.get(i).unwrap().get(j).unwrap(),
                );
            }

            res.push(next_row);
            i += 1;
        }
    }

    pub fn extrapolate_up(mut downed: Vec<Vec<i64>>) -> Vec<i64> {
        let mut extrapolated_values = vec![0];

        let mut i = downed.len() - 1;
        downed.get_mut(i).unwrap().push(0);
        let mut j = downed.get(i).unwrap().len() - 1;

        loop {
            let extrapolated_val =
                downed.get(i - 1).unwrap().get(j).unwrap() + downed.get(i).unwrap().get(j).unwrap();

            extrapolated_values.push(extrapolated_val);
            downed.get_mut(i - 1).unwrap().push(extrapolated_val);
            j += 1;

            i -= 1;
            if i == 0 {
                return extrapolated_values;
            }
        }
    }

    pub fn solve_part_a(input_str: &str) -> i64 {
        let sequences = parse_input(input_str);

        let mut sum = 0;

        for seq in sequences {
            let downed = push_down(seq);
            let extrapolated_vals = extrapolate_up(downed);

            sum += extrapolated_vals.get(extrapolated_vals.len() - 1).unwrap();
        }

        sum
    }
}

pub mod solve_part_b {
    use crate::part_a::{parse_input, push_down};

    pub fn extrapolate_forward(downed: Vec<Vec<i64>>) -> i64 {
        // instead of actually modifying the lists by pushing to the front
        // we are just going to maintain the last elem and necessary indices
        let mut last_val = 0;

        let mut i = downed.len() - 1;

        loop {
            last_val = downed.get(i).unwrap().get(0).unwrap() - last_val;

            if i == 0 {
                return last_val;
            }
            i -= 1;
        }
    }

    pub fn solve_part_b(input_str: &str) -> i64 {
        let sequences = parse_input(input_str);

        let mut sum = 0;

        for seq in sequences {
            let downed = push_down(seq);
            sum += extrapolate_forward(downed);
        }

        sum
    }
}
