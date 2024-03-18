use day19::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day19::part_a::{parse_input, Cat, Comp, Condition, Dest, Part, Rule};

    #[test]
    fn test_part_a() {
        let input_str = "px{a<2006:qkq,m>2090:A,rfg}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}";
        let (rules, parts) = parse_input(input_str);

        let part = Part {
            x: 2127,
            m: 1623,
            a: 2188,
            s: 1013,
        };
        assert_eq!(parts.get(1).unwrap(), &part);

        let (rules, parts) = parse_input(include_str!("input.txt"));
        let rule = Rule {
            tag: "hfm".to_string(),
            conditions: vec![
                Condition {
                    comp: Comp::LessThan(Cat::X, 661),
                    dest: Dest::Tag("gm".to_string()),
                },
                Condition {
                    comp: Comp::Jump,
                    dest: Dest::Tag("fck".to_string()),
                },
            ],
        };
        assert_eq!(rules.get(3).unwrap(), &rule);
    }
}

#[cfg(test)]
mod test_part_b {
    use day19::{
        part_a::Cat,
        part_b::{get_both_part_range, get_both_partb, split, PartB, PartRange},
    };

    #[test]
    fn test_split() {
        let part_range = PartRange { start: 1, stop: 10 };
        let new = split(&part_range, true, 5);
        assert_eq!(new, Some(PartRange { start: 6, stop: 10 }));

        let new = split(&part_range, false, 5);
        assert_eq!(new, Some(PartRange { start: 1, stop: 4 }));
    }

    #[test]
    fn test_get_both_part_range() {
        let part_range = PartRange { start: 1, stop: 10 };

        let (true_opt, false_opt) = get_both_part_range(&part_range, 5, true);
        assert_eq!(true_opt, Some(PartRange { start: 6, stop: 10 }));
        assert_eq!(false_opt, Some(PartRange { start: 1, stop: 5 }));

        let (true_opt, false_opt) = get_both_part_range(&part_range, 5, false);
        assert_eq!(true_opt, Some(PartRange { start: 1, stop: 4 }));
        assert_eq!(false_opt, Some(PartRange { start: 5, stop: 10 }));
    }

    #[test]
    fn test_get_both_partb() {
        let partb = PartB {
            x: PartRange { start: 1, stop: 10 },
            m: PartRange { start: 3, stop: 7 },
            s: PartRange { start: 1, stop: 3 },
            a: PartRange { start: 8, stop: 20 },
        };

        let expected_true = PartB {
            x: PartRange { start: 6, stop: 10 },
            m: PartRange { start: 3, stop: 7 },
            s: PartRange { start: 1, stop: 3 },
            a: PartRange { start: 8, stop: 20 },
        };

        let expected_false = PartB {
            x: PartRange { start: 1, stop: 5 },
            m: PartRange { start: 3, stop: 7 },
            s: PartRange { start: 1, stop: 3 },
            a: PartRange { start: 8, stop: 20 },
        };

        let (true_opt, false_opt) = get_both_partb(&partb, &Cat::X, 5, true);
        assert_eq!(true_opt, Some(expected_true));
        assert_eq!(false_opt, Some(expected_false));
    }
}
