pub mod mylib {
    use core::panic;

    const num_strs: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    pub fn find_val_of_num_substr(x: &str) -> Option<usize> {
        for i in 0..num_strs.len() {
            if x.contains(num_strs[i]) {
                return Some(i + 1);
            }
        }
        None
    }

    pub fn find_first_num(line: &str) -> usize {
        for (idx, ch) in line.chars().enumerate() {
            match find_val_of_num_substr(&line[0..idx + 1]) {
                Some(x) => return x,
                None => match ch.to_digit(10) {
                    Some(x) => return x as usize,
                    None => continue,
                },
            }
        }

        panic!("Did not expect there to be no nums");
    }

    pub fn find_last_num(line: &str) -> usize {
        let num_chars = line.chars().count();

        for (idx, ch) in line.chars().rev().enumerate() {
            match find_val_of_num_substr(&line[num_chars - idx..]) {
                Some(x) => return x,
                None => match ch.to_digit(10) {
                    Some(x) => return x as usize,
                    None => continue,
                },
            }
        }

        panic!("Did not expect there to be no nums");
    }

    pub fn find_val_of_line(line: &str) -> usize {
        let first_num = find_first_num(line);
        let last_num = find_last_num(line);

        first_num * 10 + last_num
    }
}
