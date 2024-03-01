pub mod part_a {

    #[derive(Debug, PartialEq)]
    pub struct Entry {
        pub dest_range_start: usize,
        pub src_range_start: usize,
        pub range_length: usize,
    }

    pub fn parse_seeds_line(seeds_line: &str) -> Vec<usize> {
        let trimmed = seeds_line.strip_prefix("seeds: ").unwrap();

        trimmed
            .split(" ")
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect()
    }

    pub fn parse_entry(entry_line: &str) -> Entry {
        let mut nums: Vec<usize> = entry_line
            .split(" ")
            .map(|num_str| num_str.parse::<usize>().unwrap())
            .collect();

        Entry {
            range_length: nums.pop().unwrap(),
            src_range_start: nums.pop().unwrap(),
            dest_range_start: nums.pop().unwrap(),
        }
    }

    // has seeds portion and maps portion
    pub fn parse_input(input_str: &str) -> (Vec<usize>, Vec<Vec<Entry>>) {
        let mut portions = input_str.lines();

        let seeds_line = parse_seeds_line(portions.next().unwrap());

        portions.next();
        let mut maps = Vec::new();

        let mut building = false;
        let mut cur_entries = Vec::new();
        for line in portions {
            if line == "" {
                building = false;
                maps.push(cur_entries);
                cur_entries = Vec::new();
            } else {
                if !building {
                    // if not building then we are cur on title line
                    building = true;
                } else {
                    cur_entries.push(parse_entry(line));
                }
            }
        }
        maps.push(cur_entries);

        (seeds_line, maps)
    }

    pub fn move_through_map(map: &Vec<Entry>, inputs: Vec<usize>) -> Vec<usize> {
        let mut outputs = Vec::new();

        'outer: for input in inputs {
            for entry in map {
                if input >= entry.src_range_start
                    && input < entry.src_range_start + entry.range_length
                {
                    outputs.push(entry.dest_range_start + (input - entry.src_range_start));
                    continue 'outer;
                }
            }

            // if still at this point, the input is mapped to itself
            outputs.push(input);
        }
        outputs
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let (mut inputs, maps) = parse_input(input_str);

        for map in maps {
            inputs = move_through_map(&map, inputs);
        }

        *inputs.iter().min().unwrap()
    }
}

pub mod part_b {

    use crate::part_a::Entry;

    // switching to inclusive ranges for simplicity
    pub struct Mapper {
        src_start: usize,
        dest_start: usize,

        nums_included: usize,
    }

    #[derive(PartialEq, PartialOrd, Debug, Clone, Ord, Eq)]
    pub struct Range {
        pub start: usize,
        pub stop: usize,
    }

    impl Mapper {
        pub fn get_source(&self) -> (usize, usize) {
            (self.src_start, self.src_start + self.nums_included - 1)
        }

        pub fn get_dest(&self) -> (usize, usize) {
            (self.dest_start, self.dest_start + self.nums_included - 1)
        }

        pub fn get_nums_included(&self) -> usize {
            self.nums_included
        }

        pub fn new(entry: Entry) -> Mapper {
            Mapper {
                src_start: entry.src_range_start,
                dest_start: entry.dest_range_start,
                nums_included: entry.range_length,
            }
        }

        pub fn new_by_params(src_start: usize, dest_start: usize, nums_included: usize) -> Mapper {
            Mapper {
                src_start,
                dest_start,
                nums_included,
            }
        }
    }

    // portion removed must be part of the overlap, can't remove nonexisting portion
    pub fn subtract_ranges(original_range: &Range, portion_removed: &Range) -> Vec<Range> {
        let mut resulting_ranges = Vec::new();

        // if it is all covered
        if original_range == portion_removed {
            return resulting_ranges;
        }

        if original_range.start == portion_removed.start
            && original_range.stop > portion_removed.stop
        {
            // we remove the beginning portion
            resulting_ranges.push(Range {
                start: portion_removed.stop + 1,
                stop: original_range.stop,
            });
        } else if original_range.stop == portion_removed.stop
            && original_range.start < portion_removed.start
        {
            // we remove the end portion
            resulting_ranges.push(Range {
                start: original_range.start,
                stop: portion_removed.start - 1,
            });
        } else {
            // we remove some middle portion, creating two new ranges
            resulting_ranges.push(Range {
                start: original_range.start,
                stop: portion_removed.start - 1,
            });
            resulting_ranges.push(Range {
                start: portion_removed.stop + 1,
                stop: original_range.stop,
            });
        }

        resulting_ranges
    }

    pub fn get_source_overlap(range: &Range, mapper: &Mapper) -> Option<Range> {
        if range.stop < mapper.get_source().0 || range.start > mapper.get_source().1 {
            return None;
        }

        // it either maps the initial part, the end part, or some middle part
        // or the entire range
        if range.start >= mapper.get_source().0 && range.stop <= mapper.get_source().1 {
            Some(Range {
                start: range.start,
                stop: range.stop,
            })
        } else if range.start >= mapper.get_source().0 && range.stop > mapper.get_source().1 {
            // if the whole beginning is covered
            Some(Range {
                start: range.start,
                stop: mapper.get_source().1,
            })
        } else if range.start < mapper.get_source().0 && range.stop <= mapper.get_source().1 {
            // if the whole end is covered
            Some(Range {
                start: mapper.get_source().0,
                stop: range.stop,
            })
        } else {
            // if some middle portion is covered
            Some(Range {
                start: mapper.get_source().0,
                stop: mapper.get_source().1,
            })
        }
    }

    // takes a portion which is contained in Mapper's source and returns
    // the mapped Range
    pub fn get_mapped_range(overlap: &Range, mapper: &Mapper) -> Range {
        let source_dest_dist = mapper.get_source().0.abs_diff(mapper.get_dest().0);

        if mapper.get_source().0 <= mapper.get_dest().0 {
            Range {
                start: overlap.start + source_dest_dist,
                stop: overlap.stop + source_dest_dist,
            }
        } else {
            Range {
                start: overlap.start - source_dest_dist,
                stop: overlap.stop - source_dest_dist,
            }
        }
    }

    // the second return is a potentially mapped range,
    // the vec is ranges which haven't been mapped
    pub fn attempt_map_range(range: &Range, mapper: &Mapper) -> (Vec<Range>, Option<Range>) {
        let overlap = get_source_overlap(&range, &mapper);
        match overlap {
            None => (vec![range.clone()], None),
            Some(overlap) => {
                let mapped_range = get_mapped_range(&overlap, &mapper);
                let leftover_ranges = subtract_ranges(range, &overlap);

                (leftover_ranges, Some(mapped_range))
            }
        }
    }

    pub fn traverse_map(mut input_ranges: Vec<Range>, mappers: Vec<Mapper>) -> Vec<Range> {
        let mut mapped_ranges = Vec::new();

        for mapper in mappers {
            let mut new_input_ranges = Vec::new();

            for input_range in input_ranges {
                let (mut inps, mapped_opt) = attempt_map_range(&input_range, &mapper);

                new_input_ranges.append(&mut inps);
                if let Some(x) = mapped_opt {
                    mapped_ranges.push(x);
                }
            }

            input_ranges = new_input_ranges;
        }

        // if we still have input_ranges which aren't mapped, just move them over
        mapped_ranges.append(&mut input_ranges);
        mapped_ranges
    }

    pub fn parse_ranges_line(ranges_line: &str) -> Vec<Range> {
        let trimmed = ranges_line.strip_prefix("seeds: ").unwrap();

        trimmed
            .split(" ")
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|pair| match pair {
                &[start_str, nums_included_str] => {
                    let (start, nums_included) = (
                        start_str.parse::<usize>().unwrap(),
                        nums_included_str.parse::<usize>().unwrap(),
                    );

                    Range {
                        start,
                        stop: start + nums_included - 1,
                    }
                }
                _ => panic!("Did not expect uneven num"),
            })
            .collect::<Vec<Range>>()
    }

    pub fn parse_input(input_str: &str) -> (Vec<Range>, Vec<Vec<Mapper>>) {
        let mut portions = input_str.lines();

        let ranges = parse_ranges_line(portions.next().unwrap());

        portions.next();
        let mut maps = Vec::new();

        let mut building = false;
        let mut cur_entries = Vec::new();
        for line in portions {
            if line == "" {
                building = false;
                maps.push(cur_entries);
                cur_entries = Vec::new();
            } else {
                if !building {
                    // if not building then we are cur on title line
                    building = true;
                } else {
                    cur_entries.push(Mapper::new(crate::part_a::parse_entry(line)));
                }
            }
        }
        maps.push(cur_entries);

        (ranges, maps)
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let (mut ranges, maps) = parse_input(input_str);

        for mapper in maps {
            ranges = traverse_map(ranges, mapper);
        }

        let mut lowest_num = ranges.get(0).unwrap().start;
        for range in ranges {
            if range.start < lowest_num {
                lowest_num = range.start;
            }
        }

        lowest_num
    }
}
