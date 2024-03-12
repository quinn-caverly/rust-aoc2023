use day12::part_a::solve_part_a;

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
}

#[cfg(test)]
mod test_part_a {

    use day12::part_a::{
        count_possibilities, create_all_binary_condition_sequences, parse_input,
        split_to_contig_groups, BinaryCondition,
    };
    use day12::part_a::{Condition, Elem};

    #[test]
    fn test_parse_input() {
        let input_str = "#.? 1,1,3";

        let expected = vec![Elem::new(
            vec![
                Condition::Damaged,
                Condition::Operational,
                Condition::Unknown,
            ],
            vec![1, 1, 3],
        )];

        assert_eq!(parse_input(input_str), expected);
    }

    #[test]
    fn test_split_to_contig_groups() {
        let conds = vec![
            BinaryCondition::Damaged,
            BinaryCondition::Damaged,
            BinaryCondition::Operational,
            BinaryCondition::Damaged,
        ];

        assert_eq!(split_to_contig_groups(&conds), vec![2, 1]);

        let conds = vec![BinaryCondition::Operational];
        assert_eq!(split_to_contig_groups(&conds), vec![]);

        let conds = vec![
            BinaryCondition::Damaged,
            BinaryCondition::Damaged,
            BinaryCondition::Operational,
            BinaryCondition::Damaged,
            BinaryCondition::Damaged,
            BinaryCondition::Operational,
            BinaryCondition::Operational,
            BinaryCondition::Damaged,
        ];
        assert_eq!(split_to_contig_groups(&conds), vec![2, 2, 1]);
    }

    #[test]
    fn test_create_all_binary_cond_seq() {
        let origi_conds = vec![
            Condition::Damaged,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Operational,
        ];

        let expected = vec![
            vec![
                BinaryCondition::Damaged,
                BinaryCondition::Operational,
                BinaryCondition::Operational,
                BinaryCondition::Operational,
            ],
            vec![
                BinaryCondition::Damaged,
                BinaryCondition::Damaged,
                BinaryCondition::Operational,
                BinaryCondition::Operational,
            ],
            vec![
                BinaryCondition::Damaged,
                BinaryCondition::Operational,
                BinaryCondition::Damaged,
                BinaryCondition::Operational,
            ],
            vec![
                BinaryCondition::Damaged,
                BinaryCondition::Damaged,
                BinaryCondition::Damaged,
                BinaryCondition::Operational,
            ],
        ];

        let result = create_all_binary_condition_sequences(&origi_conds);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_possibilities() {
        let input_str = ".??..??...?##. 1,1,3";
        let elems = parse_input(input_str);
        let elem = elems.get(0).unwrap();
        assert_eq!(count_possibilities(elem.clone()), 4);

        let input_str = "?###???????? 3,2,1";
        let elems = parse_input(input_str);
        let elem = elems.get(0).unwrap();
        assert_eq!(count_possibilities(elem.clone()), 10);

        let input_str = "????.#...#... 4,1,1";
        let elems = parse_input(input_str);
        let elem = elems.get(0).unwrap();
        assert_eq!(count_possibilities(elem.clone()), 1);
    }
}
