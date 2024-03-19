pub mod part_a {
    use core::panic;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq)]
    pub enum Mod<'a> {
        Flop(bool),                        // bool for is_on
        Conjunction(Vec<(&'a str, bool)>), // (tag of input, last_pulse_was_on)
        Broadcaster,
    }

    pub fn parse_input(input_str: &str) -> HashMap<&str, (Mod, Vec<&str>)> {
        input_str
            .lines()
            .map(|line| {
                let mut portions = line.split(" -> ");
                let tag_and_type = portions.next().unwrap();

                let outputs: Vec<&str> = portions.next().unwrap().split(", ").collect();

                match tag_and_type.chars().nth(0).unwrap() {
                    '%' => {
                        let tag = tag_and_type.strip_prefix("%").unwrap();
                        (tag, (Mod::Flop(false), outputs))
                    }
                    '&' => {
                        let tag = tag_and_type.strip_prefix("&").unwrap();
                        (tag, (Mod::Conjunction(vec![]), outputs))
                    }
                    _ => ("broadcaster", (Mod::Broadcaster, outputs)),
                }
            })
            .collect()
    }

    pub fn populate_conjuctions_inputs<'a>(
        tag_map: &mut HashMap<&'a str, (Mod<'a>, Vec<&'a str>)>,
    ) {
        // first is the tag of the conjuction, second is the tag of the element
        // which serves as an input to this conjuction
        let mut to_add: Vec<(&str, &str)> = Vec::new();

        for (tag, (_, outputs)) in tag_map.iter() {
            for &output in outputs {
                if let Some((out_mod, _)) = tag_map.get(output) {
                    match out_mod {
                        Mod::Conjunction(_) => {
                            to_add.push((output, tag));
                        }
                        _ => (),
                    }
                }
            }
        }

        for (conj_tag, inp_tag) in to_add {
            if let Some((x, _)) = tag_map.get_mut(conj_tag) {
                match x {
                    Mod::Conjunction(inputs_vec) => inputs_vec.push((inp_tag, false)),
                    _ => panic!(),
                }
            }
        }
    }

    pub fn get_pulses<'a>(
        is_high: bool,
        sender_tag: &str,
        receiver_tag: &'a str,
        tag_map: &mut HashMap<&'a str, (Mod<'a>, Vec<&'a str>)>,
    ) -> Vec<(bool, &'a str, &'a str)> {
        let mut new_pulses = vec![];

        if let Some((mod_type, outs)) = tag_map.get_mut(receiver_tag) {
            match mod_type {
                Mod::Flop(is_on) => {
                    // if flop receives high, nothing happens
                    if !is_high {
                        let send_highs = if *is_on {
                            *is_on = false;
                            false
                        } else {
                            *is_on = true;
                            true
                        };

                        for out in outs {
                            new_pulses.push((send_highs, receiver_tag, *out));
                        }
                    }
                }
                Mod::Conjunction(vec) => {
                    // first update memory for cur_input
                    // now check if it remembers high for all inputs
                    let mut remembers_high_for_all = true;
                    for (inp, mem) in vec {
                        if *inp == sender_tag {
                            *mem = is_high;
                        }

                        if *mem != true {
                            remembers_high_for_all = false;
                        }
                    }

                    let send_highs = if remembers_high_for_all { false } else { true };

                    for out in outs {
                        new_pulses.push((send_highs, receiver_tag, *out));
                    }
                }
                Mod::Broadcaster => {
                    // sends same pulse to all outs
                    for out in outs {
                        new_pulses.push((is_high, receiver_tag, *out));
                    }
                }
            }
        }

        new_pulses
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let mut tag_map = parse_input(input_str);
        populate_conjuctions_inputs(&mut tag_map);

        let mut total_highs = 0;
        let mut total_lows = 0;
        for _ in 0..1000 {
            let mut mut_rem_pulses = vec![(false, "button", "broadcaster")];

            while !mut_rem_pulses.is_empty() {
                let cur = mut_rem_pulses.pop().unwrap();
                if cur.0 {
                    total_highs += 1;
                } else {
                    total_lows += 1;
                }

                let mut new_pulses = get_pulses(cur.0, cur.1, cur.2, &mut tag_map);
                mut_rem_pulses.append(&mut new_pulses);
            }
        }

        total_highs * total_lows
    }
}

pub mod part_b {
    use crate::part_a::{get_pulses, parse_input, populate_conjuctions_inputs};

    // I am not solving part 2 in earnest because it requires making
    // assumptions about the input string, this implementation would
    // run for essentially forever
    pub fn solve_part_b(input_str: &str) -> usize {
        let mut tag_map = parse_input(input_str);
        populate_conjuctions_inputs(&mut tag_map);

        let mut step = 0;
        loop {
            step += 1;
            let mut mut_rem_pulses = vec![(false, "button", "broadcaster")];

            while !mut_rem_pulses.is_empty() {
                let cur = mut_rem_pulses.pop().unwrap();

                if cur.0 == false && cur.2 == "rx" {
                    return step;
                }

                let mut new_pulses = get_pulses(cur.0, cur.1, cur.2, &mut tag_map);
                mut_rem_pulses.append(&mut new_pulses);
            }
        }
    }
}
