pub mod part_a {
    use core::panic;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Cat {
        X,
        S,
        M,
        A,
    }

    #[derive(Debug, PartialEq)]
    pub enum Dest {
        Reject,
        Accept,
        Tag(String),
    }

    #[derive(Debug, PartialEq)]
    pub enum Comp {
        LessThan(Cat, usize),
        GreaterThan(Cat, usize),
        Jump,
    }

    #[derive(Debug, PartialEq)]
    pub struct Condition {
        pub comp: Comp,
        pub dest: Dest,
    }

    #[derive(Debug, PartialEq)]
    pub struct Rule {
        pub tag: String,
        pub conditions: Vec<Condition>,
    }

    pub fn parse_rule(line: &str) -> Rule {
        let mut portions = line.split("{");

        let tag = portions.next().unwrap().to_string();
        let rest = portions.next().unwrap().strip_suffix("}").unwrap();

        let conditions = rest
            .split(",")
            .map(|cond_str| {
                if cond_str.contains(">") || cond_str.contains("<") {
                    let split_with = if cond_str.contains(">") { ">" } else { "<" };

                    let mut portions = cond_str.split(split_with);
                    let cats = match portions.next().unwrap() {
                        "x" => Cat::X,
                        "a" => Cat::A,
                        "m" => Cat::M,
                        "s" => Cat::S,
                        y => panic!("Did not expect category: {}", y),
                    };

                    let mut num_dest = portions.next().unwrap().split(":");
                    let num = num_dest.next().unwrap().parse::<usize>().unwrap();
                    let dest = match num_dest.next().unwrap() {
                        "A" => Dest::Accept,
                        "R" => Dest::Reject,
                        x => Dest::Tag(x.to_string()),
                    };

                    let comp = if split_with == ">" {
                        Comp::GreaterThan(cats, num)
                    } else {
                        Comp::LessThan(cats, num)
                    };

                    Condition { comp, dest }
                } else {
                    let dest = match cond_str {
                        "A" => Dest::Accept,
                        "R" => Dest::Reject,
                        x => Dest::Tag(x.to_string()),
                    };

                    Condition {
                        comp: Comp::Jump,
                        dest,
                    }
                }
            })
            .collect();

        Rule { tag, conditions }
    }

    #[derive(Debug, PartialEq)]
    pub struct Part {
        pub x: usize,
        pub m: usize,
        pub a: usize,
        pub s: usize,
    }

    pub fn parse_part(line: &str) -> Part {
        // x, m, a, s
        let split = line.strip_suffix("}").unwrap().strip_prefix("{").unwrap();
        let cats: Vec<usize> = split
            .split(",")
            .map(|eq| {
                let mut halves = eq.split("=");
                let num_str = halves.nth(1).unwrap();

                num_str.parse::<usize>().unwrap()
            })
            .collect();

        Part {
            x: *cats.get(0).unwrap(),
            m: *cats.get(1).unwrap(),
            a: *cats.get(2).unwrap(),
            s: *cats.get(3).unwrap(),
        }
    }

    pub fn parse_input(input_str: &str) -> (Vec<Rule>, Vec<Part>) {
        let lines = input_str.lines();

        let mut rules = vec![];
        let mut parts = vec![];

        let mut is_rules = true;
        for line in lines {
            if line == "" {
                is_rules = false;
                continue;
            }

            if is_rules {
                rules.push(parse_rule(line));
            } else {
                parts.push(parse_part(line));
            }
        }

        (rules, parts)
    }

    pub fn create_rules_map(rules: Vec<Rule>) -> HashMap<String, Vec<Condition>> {
        rules
            .into_iter()
            .map(|rule| (rule.tag, rule.conditions))
            .collect()
    }

    pub fn is_accepted(
        part: &Part,
        cur_dest: &Dest,
        rules_map: &HashMap<String, Vec<Condition>>,
    ) -> bool {
        let cur_rule_tag = match cur_dest {
            Dest::Reject => return false,
            Dest::Accept => return true,
            Dest::Tag(x) => x,
        };

        for condition in rules_map.get(cur_rule_tag).unwrap() {
            match &condition.comp {
                Comp::LessThan(cat, num) | Comp::GreaterThan(cat, num) => {
                    let num_from_part = match cat {
                        Cat::X => part.x,
                        Cat::S => part.s,
                        Cat::M => part.m,
                        Cat::A => part.a,
                    };

                    if condition.comp == Comp::LessThan(cat.clone(), *num) {
                        if num_from_part < *num {
                            return is_accepted(part, &condition.dest, rules_map);
                        }
                    } else {
                        if num_from_part > *num {
                            return is_accepted(part, &condition.dest, rules_map);
                        }
                    }
                }
                Comp::Jump => return is_accepted(part, &condition.dest, rules_map),
            }
        }

        panic!("Did not expect to fall through");
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let (rules, parts) = parse_input(input_str);
        let rules_map = create_rules_map(rules);

        let mut sum = 0;
        for part in parts {
            if is_accepted(&part, &Dest::Tag("in".to_string()), &rules_map) {
                sum += part.x + part.m + part.a + part.s;
            }
        }

        sum
    }
}

pub mod part_b {
    use std::collections::HashMap;

    use crate::part_a::{create_rules_map, parse_input, Cat, Comp, Condition, Dest, Part};

    // range is inclusive, start: 1, stop: 4, includes 1, 2, 3, 4
    #[derive(Debug, Clone, PartialEq, Copy)]
    pub struct PartRange {
        pub start: usize,
        pub stop: usize,
    }

    #[derive(Debug, PartialEq)]
    pub struct PartB {
        pub x: PartRange,
        pub m: PartRange,
        pub a: PartRange,
        pub s: PartRange,
    }

    // the num of possibilities for x * m * a * s
    pub fn calculate_final_possiblities(partb: &PartB) -> usize {
        (partb.x.stop - partb.x.start + 1)
            * (partb.m.stop - partb.m.start + 1)
            * (partb.a.stop - partb.a.start + 1)
            * (partb.s.stop - partb.s.start + 1)
    }

    // could be 0 or 1
    pub fn split(part_range: &PartRange, take_above: bool, num: usize) -> Option<PartRange> {
        if take_above {
            if part_range.start > num {
                Some(part_range.clone())
            } else if part_range.start <= num && part_range.stop > num {
                Some(PartRange {
                    start: num + 1,
                    stop: part_range.stop,
                })
            } else {
                // if both are less than or equal, then we return None
                None
            }
        } else {
            if part_range.stop < num {
                Some(part_range.clone())
            } else if part_range.stop >= num && part_range.start < num {
                Some(PartRange {
                    start: part_range.start,
                    stop: num - 1,
                })
            } else {
                None
            }
        }
    }

    // the first is the one which makes the boolean true
    pub fn get_both_part_range(
        part_range: &PartRange,
        num: usize,
        take_above: bool,
    ) -> (Option<PartRange>, Option<PartRange>) {
        match take_above {
            true => (
                split(part_range, take_above, num),
                split(part_range, !take_above, num + 1),
            ),
            false => (
                split(part_range, take_above, num),
                split(part_range, !take_above, num - 1),
            ),
        }
    }

    pub fn get_both_partb(
        partb: &PartB,
        cat: &Cat,
        num: usize,
        take_above: bool,
    ) -> (Option<PartB>, Option<PartB>) {
        match cat {
            Cat::X => {
                let (true_range, false_range) = get_both_part_range(&partb.x, num, take_above);

                let true_part = match true_range {
                    Some(x) => Some(PartB {
                        x,
                        m: partb.m,
                        s: partb.s,
                        a: partb.a,
                    }),
                    None => None,
                };

                let false_part = match false_range {
                    Some(x) => Some(PartB {
                        x,
                        m: partb.m,
                        s: partb.s,
                        a: partb.a,
                    }),
                    None => None,
                };

                (true_part, false_part)
            }
            Cat::S => {
                let (true_range, false_range) = get_both_part_range(&partb.s, num, take_above);

                let true_part = match true_range {
                    Some(s) => Some(PartB {
                        x: partb.x,
                        m: partb.m,
                        s,
                        a: partb.a,
                    }),
                    None => None,
                };

                let false_part = match false_range {
                    Some(s) => Some(PartB {
                        x: partb.x,
                        m: partb.m,
                        s,
                        a: partb.a,
                    }),
                    None => None,
                };

                (true_part, false_part)
            }
            Cat::M => {
                let (true_range, false_range) = get_both_part_range(&partb.m, num, take_above);

                let true_part = match true_range {
                    Some(m) => Some(PartB {
                        x: partb.x,
                        m,
                        s: partb.s,
                        a: partb.a,
                    }),
                    None => None,
                };

                let false_part = match false_range {
                    Some(m) => Some(PartB {
                        x: partb.x,
                        m,
                        s: partb.s,
                        a: partb.a,
                    }),
                    None => None,
                };

                (true_part, false_part)
            }
            Cat::A => {
                let (true_range, false_range) = get_both_part_range(&partb.a, num, take_above);

                let true_part = match true_range {
                    Some(a) => Some(PartB {
                        x: partb.x,
                        m: partb.m,
                        s: partb.s,
                        a,
                    }),
                    None => None,
                };

                let false_part = match false_range {
                    Some(a) => Some(PartB {
                        x: partb.x,
                        m: partb.m,
                        s: partb.s,
                        a,
                    }),
                    None => None,
                };

                (true_part, false_part)
            }
        }
    }

    // at each step, if > or <, split part b and return sum of both
    // if it is a jump just return recursive
    // not tail recursive. Less efficient? whatever
    pub fn count_accepted(
        mut partb: PartB,
        cur_dest: &Dest,
        rules_map: &HashMap<String, Vec<Condition>>,
    ) -> usize {
        let cur_tag = match cur_dest {
            Dest::Reject => return 0,
            Dest::Accept => return calculate_final_possiblities(&partb),
            Dest::Tag(x) => x,
        };

        let mut sum = 0;
        for condition in rules_map.get(cur_tag).unwrap() {
            match &condition.comp {
                Comp::LessThan(cat, num) | Comp::GreaterThan(cat, num) => {
                    let take_above = condition.comp == Comp::GreaterThan(cat.clone(), *num);
                    let (true_opt, false_opt) = get_both_partb(&partb, cat, *num, take_above);

                    if let Some(x) = true_opt {
                        sum += count_accepted(x, &condition.dest, rules_map);
                    }
                    if let Some(y) = false_opt {
                        partb = y;
                    }
                }
                Comp::Jump => return sum + count_accepted(partb, &condition.dest, rules_map),
            }
        }

        panic!("Did not expect to fall through");
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let (rules, _) = parse_input(input_str);
        let rules_map = create_rules_map(rules);

        let default_range = PartRange {
            start: 1,
            stop: 4000,
        };
        let initial_part_b = PartB {
            x: default_range.clone(),
            m: default_range.clone(),
            s: default_range.clone(),
            a: default_range.clone(),
        };

        count_accepted(initial_part_b, &Dest::Tag("in".to_string()), &rules_map)
    }
}
