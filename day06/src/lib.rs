pub mod part_a {
    use std::iter::zip;

    #[derive(Debug, PartialEq)]
    pub struct Race {
        time: usize,
        distance: usize,
    }

    impl Race {
        pub fn new(time: usize, distance: usize) -> Race {
            Race { time, distance }
        }

        pub fn get(&self) -> (usize, usize) {
            (self.time, self.distance)
        }
    }

    pub fn parse_input(input_str: &str) -> Vec<Race> {
        let mut portions = input_str.lines();

        let time_line = portions.next().unwrap();
        let times_strs = time_line.strip_prefix("Time:").unwrap().split_whitespace();

        let distance_line = portions.next().unwrap();
        let distances_strs = distance_line
            .strip_prefix("Distance:")
            .unwrap()
            .split_whitespace();

        zip(times_strs, distances_strs)
            .map(|(time_str, distance_str)| {
                let (time, distance) = (
                    time_str.parse::<usize>().unwrap(),
                    distance_str.parse::<usize>().unwrap(),
                );

                Race { time, distance }
            })
            .collect::<Vec<Race>>()
    }

    pub fn calculate_ways_to_win_race(race: &Race) -> usize {
        // distance_achieved = seconds_held_down * (time - seconds_held_down)
        // distance_achieved = seconds_held_down * time - seconds_held_down ** 2
        let mut ways = 0;

        for seconds_held_down in 1..race.time - 1 {
            if seconds_held_down * (race.time - seconds_held_down) > race.distance {
                ways += 1;
            }
        }

        ways
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let races = parse_input(input_str);

        races
            .iter()
            .map(|race| calculate_ways_to_win_race(race))
            .fold(1, |acc, x| acc * x)
    }
}

pub mod part_b {
    use core::panic;

    use crate::part_a::Race;

    fn convert_line_to_num(line: &str) -> usize {
        let mut time_digits: Vec<usize> = Vec::new();
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                time_digits.push(ch.to_digit(10).unwrap() as usize);
            }
        }

        let mut val = 0;
        let mut mult = 1;
        for dig in time_digits.iter().rev() {
            val += dig * mult;
            mult *= 10;
        }

        val
    }

    pub fn parse_input(input_str: &str) -> Race {
        let mut portions = input_str.lines();

        let time = convert_line_to_num(portions.next().unwrap());
        let distance = convert_line_to_num(portions.next().unwrap());

        Race::new(time, distance)
    }

    pub fn find_least_pressed_to_win_race(race: &Race) -> usize {
        for time_pressed in 1..=race.get().0 - 1 {
            if time_pressed * (race.get().0 - time_pressed) > race.get().1 {
                return time_pressed;
            }
        }

        panic!("Did not expect to not beat the score at all");
    }

    pub fn find_most_pressed_to_win_race(race: &Race) -> usize {
        for time_pressed in (1..=race.get().0 - 1).rev() {
            if time_pressed * (race.get().0 - time_pressed) > race.get().1 {
                return time_pressed;
            }
        }

        panic!("Did not expect to not beat the score at all");
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let race = parse_input(input_str);

        let least = find_least_pressed_to_win_race(&race);
        let most = find_most_pressed_to_win_race(&race);

        // if least = 3, and most = 4, then there are two times = (4-3) + 1
        most - least + 1
    }
}
