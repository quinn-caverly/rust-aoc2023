pub mod mylib {
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

        return first_num as usize * 10 + last_num as usize;
    }
}
