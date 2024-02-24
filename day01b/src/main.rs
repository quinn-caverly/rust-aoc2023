use lib::mylib::find_val_of_line;

mod lib;

fn main() {
    let lines = include_str!("input.txt").lines();

    let mut sum = 0;
    for line in lines {
        sum += find_val_of_line(line);
    }

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use crate::lib::mylib;

    #[test]
    fn test_find_val_of_num_substr() {
        assert_eq!(mylib::find_val_of_num_substr("sasfdoneasdfasd"), Some(1));
        assert_eq!(mylib::find_val_of_num_substr("nineo"), Some(9));
    }

    #[test]
    fn test_find_first_num() {
        assert_eq!(mylib::find_first_num("one234"), 1);
        assert_eq!(mylib::find_first_num("2three"), 2);
    }

    #[test]
    fn test_find_last_num() {
        assert_eq!(mylib::find_last_num("one234"), 4);
        assert_eq!(mylib::find_last_num("2three"), 3);
    }

    #[test]
    fn test_find_val_of_line() {
        assert_eq!(mylib::find_val_of_line("one3434five"), 15);
    }
}
