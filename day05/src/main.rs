use day05::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day05::part_a::{move_through_map, parse_entry, parse_input, parse_seeds_line, Entry};

    #[test]
    fn test_parse_seeds_line() {
        const SEEDS_LINE: &str = "seeds: 79 14 55 13";

        assert_eq!(parse_seeds_line(SEEDS_LINE), vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parse_entry() {
        const SAMPLE_ENTRY: &str = "3547471595 1239929038 174680800";

        assert_eq!(
            parse_entry(SAMPLE_ENTRY),
            Entry {
                dest_range_start: 3547471595,
                src_range_start: 1239929038,
                range_length: 174680800
            }
        )
    }

    #[test]
    fn test_parse_input() {
        const SAMPLE_INP: &str = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48";
        let (_, maps) = parse_input(SAMPLE_INP);

        let expected_map = vec![
            Entry {
                dest_range_start: 50,
                src_range_start: 98,
                range_length: 2,
            },
            Entry {
                dest_range_start: 52,
                src_range_start: 50,
                range_length: 48,
            },
        ];

        assert_eq!(maps.get(0).unwrap(), &expected_map);
    }

    #[test]
    fn test_move_through_map() {
        const SAMPLE_INP: &str = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48";
        let (seeds, maps) = parse_input(SAMPLE_INP);

        let outputs = move_through_map(maps.get(0).unwrap(), seeds);
        let expected = vec![81, 14, 57, 13];

        assert_eq!(outputs, expected);
    }
}

#[cfg(test)]
mod test_part_b {
    use day05::{
        part_a::Entry,
        part_b::{
            get_mapped_range, get_source_overlap, parse_ranges_line, subtract_ranges, traverse_map,
            Mapper, Range,
        },
    };

    #[test]
    fn test_mapper() {
        let mapper = Mapper::new(Entry {
            dest_range_start: 50,
            src_range_start: 98,
            range_length: 2,
        });

        assert_eq!(mapper.get_source(), (98, 99));
        assert_eq!(mapper.get_dest(), (50, 51));
    }

    #[test]
    fn test_get_mapped_range() {
        let range = Range { start: 0, stop: 10 };
        let mapper = Mapper::new_by_params(0, 20, 11);

        assert_eq!(
            get_mapped_range(&range, &mapper),
            Range {
                start: 20,
                stop: 30
            }
        );
    }

    #[test]
    fn test_get_source_overlap() {
        // equal range
        let mut range = Range { start: 0, stop: 10 };
        let mut mapper = Mapper::new_by_params(0, 20, 11);
        assert_eq!(
            get_source_overlap(&range, &mapper),
            Some(Range { start: 0, stop: 10 })
        );

        // covers beginning
        range = Range { start: 0, stop: 20 };
        mapper = Mapper::new_by_params(0, 30, 11);
        assert_eq!(
            get_source_overlap(&range, &mapper),
            Some(Range { start: 0, stop: 10 })
        );

        // covers end
        range = Range { start: 0, stop: 10 };
        mapper = Mapper::new_by_params(5, 20, 11);
        assert_eq!(
            get_source_overlap(&range, &mapper),
            Some(Range { start: 5, stop: 10 })
        );

        // if some middle portion is covered
        range = Range { start: 0, stop: 10 };
        mapper = Mapper::new_by_params(1, 100, 9);
        assert_eq!(
            get_source_overlap(&range, &mapper),
            Some(Range { start: 1, stop: 9 })
        );
    }

    #[test]
    fn test_substract_ranges() {
        // entire range
        let mut original_range = Range { start: 0, stop: 10 };
        let mut portion_removed = Range { start: 0, stop: 10 };
        assert_eq!(subtract_ranges(&original_range, &portion_removed), vec![]);

        // starting from beginning
        original_range = Range { start: 0, stop: 10 };
        portion_removed = Range { start: 0, stop: 5 };
        assert_eq!(
            subtract_ranges(&original_range, &portion_removed),
            vec![Range { start: 6, stop: 10 }]
        );

        // starting from end
        original_range = Range { start: 0, stop: 10 };
        portion_removed = Range { start: 5, stop: 10 };
        assert_eq!(
            subtract_ranges(&original_range, &portion_removed),
            vec![Range { start: 0, stop: 4 }]
        );

        // remove some middle portion
        original_range = Range { start: 0, stop: 10 };
        portion_removed = Range { start: 5, stop: 6 };
        assert_eq!(
            subtract_ranges(&original_range, &portion_removed),
            vec![Range { start: 0, stop: 4 }, Range { start: 7, stop: 10 }]
        );
    }

    #[test]
    fn test_traverse_map() {
        let mapper = Mapper::new_by_params(1, 10, 5);
        let input = Range { start: 0, stop: 7 };

        let mut expected_ranges = vec![
            Range { start: 0, stop: 0 },
            Range {
                start: 10,
                stop: 14,
            },
            Range { start: 6, stop: 7 },
        ];
        expected_ranges.sort();

        let mut res = traverse_map(vec![input], vec![mapper]);
        res.sort();

        assert_eq!(res, expected_ranges);
    }

    #[test]
    fn test_parse_ranges_line() {
        const RANGES_LINE: &str = "seeds: 79 14 55 13";

        let expected = vec![
            Range {
                start: 79,
                stop: 92,
            },
            Range {
                start: 55,
                stop: 67,
            },
        ];

        assert_eq!(parse_ranges_line(RANGES_LINE), expected);
    }
}
