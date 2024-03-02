use day09::{part_a::solve_part_a, solve_part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day09::part_a::{extrapolate_up, parse_input, push_down};

    #[test]
    fn test_parse_input() {
        const SAMPLE_LINE: &str = "10 16 -31 69 144 270 461 731 1094 1564 2155 2881 3756 4794 6009 7415 9026 10856 12919 15229 17800";
        let res = parse_input(SAMPLE_LINE);

        assert_eq!(res.get(0).unwrap().get(0).unwrap(), &10);
        assert_eq!(res.get(0).unwrap().get(2).unwrap(), &-31);
    }

    #[test]
    fn test_push_down() {
        const SAMPLE: &str = "0 3 6 9 12 15";
        let res = parse_input(SAMPLE);
        let cur = res.get(0).unwrap();

        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];

        assert_eq!(push_down(cur.clone()), expected);
    }

    #[test]
    fn test_extrapolate_up() {
        const SAMPLE: &str = "0 3 6 9 12 15";
        let res = parse_input(SAMPLE);
        let downed = push_down(res.get(0).unwrap().clone());

        let expected = vec![0, 3, 18];
        assert_eq!(extrapolate_up(downed), expected);
    }
}

#[cfg(test)]
mod solve_part_b {
    use day09::{
        part_a::{parse_input, push_down},
        solve_part_b::extrapolate_forward,
    };

    #[test]
    fn test_extrapolate_forward() {
        const SAMPLE: &str = "10 13 16 21 30 45";
        let res = parse_input(SAMPLE);
        let downed = push_down(res.get(0).unwrap().clone());

        let expected = 5;

        assert_eq!(extrapolate_forward(downed), expected);
    }
}
