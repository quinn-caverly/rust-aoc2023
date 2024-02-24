use day01a::mylib;

fn main() {
    let lines = include_str!("input.txt").lines();

    let mut sum = 0;
    for line in lines {
        sum += mylib::calculate_line(line);
    }

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use day01a::mylib::{calculate_line, find_first_num, find_last_num};

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
