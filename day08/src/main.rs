use day08::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day08::part_a::parse_node_str;

    #[test]
    fn test_parse_node_str() {
        const SAMPLE_INPUT: &str = "AAA = (BBB, CCC)";

        let expected = (
            vec!['A', 'A', 'A'],
            (vec!['B', 'B', 'B'], vec!['C', 'C', 'C']),
        );

        assert_eq!(expected, parse_node_str(SAMPLE_INPUT));
    }
}

#[cfg(test)]
mod test_part_b {
    use day08::{
        part_a::parse_input,
        part_b::{find_all_start_nodes, find_circuit, solve_part_b, EndIterator},
    };

    #[test]
    fn test_find_circuit() {
        const SAMPLE_INPUT: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let (nav, nodes) = parse_input(SAMPLE_INPUT);
        let mut start = vec!['1', '1', 'A'];
        let (mut steps, mut steps_on_ender) = find_circuit(start, &nav, &nodes);
        assert_eq!(steps, 3);
        assert_eq!(steps_on_ender, vec![2]);

        start = vec!['2', '2', 'A'];
        (steps, steps_on_ender) = find_circuit(start, &nav, &nodes);
        assert_eq!(steps, 7);
        assert_eq!(steps_on_ender, vec![3, 6]);
    }

    #[test]
    fn test_end_iterator() {
        const SAMPLE_INPUT: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let (nav, nodes) = parse_input(SAMPLE_INPUT);
        let mut start = vec!['1', '1', 'A'];
        let (mut steps, mut steps_on_ender) = find_circuit(start, &nav, &nodes);

        let mut end_iterator = EndIterator::new(steps, steps_on_ender);

        assert_eq!(end_iterator.next().unwrap(), 2);
        assert_eq!(end_iterator.next().unwrap(), 4);
        assert_eq!(end_iterator.next().unwrap(), 6);

        start = vec!['2', '2', 'A'];
        (steps, steps_on_ender) = find_circuit(start, &nav, &nodes);

        let mut end_iterator = EndIterator::new(steps, steps_on_ender);

        assert_eq!(end_iterator.next().unwrap(), 3);
        assert_eq!(end_iterator.next().unwrap(), 6);
    }

    #[test]
    fn test_find_all_start_nodes() {
        const SAMPLE_INPUT: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let (_, nodes) = parse_input(SAMPLE_INPUT);

        let mut start_nodes = find_all_start_nodes(&nodes);
        start_nodes.sort();

        let mut expected = vec![vec!['2', '2', 'A'], vec!['1', '1', 'A']];
        expected.sort();

        assert_eq!(start_nodes, expected);
    }

    #[test]
    fn test_solve_part_b() {
        const SAMPLE_INPUT: &str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";

        assert_eq!(solve_part_b(SAMPLE_INPUT), 6);
    }
}
