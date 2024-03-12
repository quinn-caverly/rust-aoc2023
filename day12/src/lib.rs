pub mod part_a {
    #[derive(Debug, PartialEq, Clone)]
    pub enum Condition {
        Operational,
        Damaged,
        Unknown,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum BinaryCondition {
        Operational,
        Damaged,
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Elem {
        conditions: Vec<Condition>,
        nums: Vec<usize>,
    }

    impl Elem {
        pub fn new(conditions: Vec<Condition>, nums: Vec<usize>) -> Elem {
            Elem { conditions, nums }
        }

        pub fn get_nums(&self) -> &Vec<usize> {
            &self.nums
        }
    }

    pub fn parse_input(input_str: &str) -> Vec<Elem> {
        input_str
            .lines()
            .map(|line| {
                let mut portions = line.split(" ");

                let conditions = portions
                    .next()
                    .unwrap()
                    .chars()
                    .map(|ch| match ch {
                        '.' => Condition::Operational,
                        '?' => Condition::Unknown,
                        '#' => Condition::Damaged,
                        _ => panic!("Did not expect neither ., ?, nor #"),
                    })
                    .collect::<Vec<Condition>>();

                let nums = portions
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|num_str| num_str.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                Elem { conditions, nums }
            })
            .collect()
    }

    pub fn split_to_contig_groups(binary_conditions: &Vec<BinaryCondition>) -> Vec<usize> {
        let mut contig_groups = Vec::new();

        let mut cur_size = 0;
        let mut is_building = false;

        for cond in binary_conditions {
            match cond {
                BinaryCondition::Operational => {
                    if is_building {
                        is_building = false;
                        contig_groups.push(cur_size);
                    }
                }
                BinaryCondition::Damaged => {
                    if is_building {
                        cur_size += 1;
                    } else {
                        is_building = true;
                        cur_size = 1;
                    }
                }
            }
        }

        if is_building {
            contig_groups.push(cur_size);
        }

        contig_groups
    }

    pub fn is_valid(binary_conditions: &Vec<BinaryCondition>, contig_groups: &Vec<usize>) -> bool {
        // we simply divide binary conditions into contig groups and then
        // check if it matches the actual contig_groups
        &split_to_contig_groups(&binary_conditions) == contig_groups
    }

    pub fn create_all_binary_condition_sequences(
        conditions: &Vec<Condition>,
    ) -> Vec<Vec<BinaryCondition>> {
        let mut new_sequences = vec![vec![]];

        // we start out with just a single vector, as long as there are no unknowns,
        // we just add the cond to the vectors in new_sequences,
        // if there is an unknown, for each vector in the sequence, there now needs
        // to be two sequences for both damaged and operational at index
        for cond in conditions {
            if cond == &Condition::Unknown {
                let mut to_concat = Vec::new();
                for seq in new_sequences.iter_mut() {
                    let mut a = seq.clone();
                    a.push(BinaryCondition::Damaged);
                    to_concat.push(a);

                    seq.push(BinaryCondition::Operational);
                }

                new_sequences.append(&mut to_concat);
            } else {
                for seq in new_sequences.iter_mut() {
                    if cond == &Condition::Damaged {
                        seq.push(BinaryCondition::Damaged);
                    } else {
                        seq.push(BinaryCondition::Operational);
                    }
                }
            }
        }

        new_sequences
    }

    pub fn count_possibilities(elem: Elem) -> usize {
        let all = create_all_binary_condition_sequences(&elem.conditions);

        let mut sum = 0;
        for binary_conditions in all {
            if split_to_contig_groups(&binary_conditions) == elem.nums {
                sum += 1;
            }
        }

        sum
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let mut total = 0;

        for elem in parse_input(input_str) {
            total += count_possibilities(elem);
        }

        total
    }
}

