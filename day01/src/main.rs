use lib;

fn main() {
    println!("part a: {}", solve_part_a());
    println!("part b: {}", solve_part_b());
}

fn solve_part_a() -> usize {
    let lines = include_str!("input.txt").lines();

    let mut sum = 0;
    for line in lines {
        sum += lib::part_a::calculate_line(line);
    }

    sum
}

fn solve_part_b() -> usize {
    let lines = include_str!("input.txt").lines();

    let mut sum = 0;
    for line in lines {
        sum += lib::part_b::find_val_of_line(line);
    }

    sum
}

#[cfg(test)]
mod test_part_b {
    use crate::lib::part_b;

    #[test]
    fn test_find_val_of_num_substr() {
        assert_eq!(part_b::find_val_of_num_substr("sasfdoneasdfasd"), Some(1));
        assert_eq!(part_b::find_val_of_num_substr("nineo"), Some(9));
    }

    #[test]
    fn test_find_first_num() {
        assert_eq!(part_b::find_first_num("one234"), 1);
        assert_eq!(part_b::find_first_num("2three"), 2);
    }

    #[test]
    fn test_find_last_num() {
        assert_eq!(part_b::find_last_num("one234"), 4);
        assert_eq!(part_b::find_last_num("2three"), 3);
    }

    #[test]
    fn test_find_val_of_line() {
        assert_eq!(part_b::find_val_of_line("one3434five"), 15);
    }
}

#[cfg(test)]
mod test_part_a {
    use lib::part_a::{calculate_line, find_first_num, find_last_num};

    #[test]
    fn test_find_first_num() {
        let line = "9asd8s7f";
        assert_eq!(find_first_num(line), 9);
    }

    #[test]
    fn test_find_last_num() {
        let line = "9asd8s7f";
        assert_eq!(find_last_num(line), 7);
    }

    #[test]
    fn test_calculate_line() {
        let line = "9asd8s7f";
        assert!(calculate_line(line) == 97);
    }
}
