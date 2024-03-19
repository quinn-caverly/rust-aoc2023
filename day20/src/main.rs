use day20::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day20::part_a::{parse_input, populate_conjuctions_inputs, Mod};

    #[test]
    fn test_parse_input() {
        let input_str = "%hb -> mj\n%mx -> mt, xz\n%xh -> qc\n%tg -> cq";
        let tag_map = parse_input(input_str);

        assert_eq!(tag_map.get("hb").unwrap(), &(Mod::Flop(false), vec!["mj"]));
    }

    #[test]
    fn test_populate_conjuctions_inputs() {
        let input_str = "%hb -> ab\n%mx -> ab, xz\n%xh -> ac\n&ab -> cq";
        let mut tag_map = parse_input(input_str);
        populate_conjuctions_inputs(&mut tag_map);

        for x in tag_map.iter() {
            println!("{:?}", x);
        }

        assert_eq!(
            tag_map.get("ab").unwrap().0,
            Mod::Conjunction(vec![("hb", false), ("mx", false)]),
        )
    }
}
