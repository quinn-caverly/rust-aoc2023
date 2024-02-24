pub mod part_a {
    pub fn find_first_num(line: &str) -> u32 {
        for ch in line.chars() {
            if ch.is_ascii_digit() {
                return ch.to_digit(10).unwrap();
            }
        }

        panic!("Did not expect for there to be no digits");
    }

    pub fn find_last_num(line: &str) -> u32 {
        for ch in line.chars().rev() {
            if ch.is_ascii_digit() {
                return ch.to_digit(10).unwrap();
            }
        }

        panic!("Did not expect there to be no digits");
    }

    pub fn calculate_line(line: &str) -> usize {
        let first_num = find_first_num(line);
        let last_num = find_last_num(line);

        first_num as usize * 10 + last_num as usize
    }
}

pub mod part_b {
    use core::panic;

    const NUM_STRS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    pub fn find_val_of_num_substr(x: &str) -> Option<usize> {
        for i in 0..NUM_STRS.len() {
            if x.contains(NUM_STRS[i]) {
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
