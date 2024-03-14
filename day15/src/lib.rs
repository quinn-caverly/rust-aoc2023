pub mod part_a {
    pub fn parse_input(input_str: &str) -> Vec<&str> {
        input_str.split(",").collect()
    }

    pub fn run_hash_algo(string: &str) -> usize {
        let mut val = 0;
        for ch in string.chars() {
            val += ch as usize;
            val *= 17;
            val = val % 256;
        }
        val
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let mut ans = 0;
        for string in parse_input(input_str) {
            ans += run_hash_algo(string);
        }
        ans
    }
}

pub mod part_b {
    use crate::part_a::{parse_input, run_hash_algo};

    pub fn grab_label(string: &str) -> String {
        string
            .chars()
            .take_while(|&c| c != '=' && c != '-')
            .collect()
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Lens {
        pub label: String,
        pub focal_length: usize,
    }

    // assumes that the string has an equal sign
    pub fn grab_focal_length(string: &str) -> usize {
        let mut portions = string.split("=");
        let mut num_str = portions.nth(1).unwrap();
        num_str = num_str.trim();

        num_str.parse().expect(num_str)
    }

    pub fn create_boxes(strings: Vec<&str>) -> Vec<Vec<Lens>> {
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

        'outer: for string in strings {
            let label = grab_label(string);
            let box_idx = run_hash_algo(&label);

            if string.contains("=") {
                let focal_length = grab_focal_length(string);

                // check if there is a lens with the label in this box,
                // if there is not just add it to the end
                // if there is, replace the lenses
                for lens in boxes.get_mut(box_idx).unwrap() {
                    if lens.label == label {
                        lens.label = label;
                        lens.focal_length = focal_length;
                        continue 'outer;
                    }
                }
                // this is only reachable if we havent found a match
                boxes.get_mut(box_idx).unwrap().push(Lens {
                    label,
                    focal_length,
                });
            } else {
                let mut idx_to_remove = 0;
                let mut remove = false;
                for (idx, lens) in boxes.get(box_idx).unwrap().iter().enumerate() {
                    if lens.label == label {
                        idx_to_remove = idx;
                        remove = true;
                        break;
                    }
                }

                if remove {
                    boxes.get_mut(box_idx).unwrap().remove(idx_to_remove);
                }
            }
        }

        boxes
    }

    pub fn calculate_focus_pow(boxes: &Vec<Vec<Lens>>) -> usize {
        let mut ans = 0;

        for (box_idx, bx) in boxes.iter().enumerate() {
            for (lens_idx, lens) in bx.iter().enumerate() {
                ans += (box_idx + 1) * (lens_idx + 1) * lens.focal_length;
            }
        }

        ans
    }

    pub fn solve_part_b(input_str: &str) -> usize {
        let boxes = create_boxes(parse_input(input_str));
        calculate_focus_pow(&boxes)
    }
}
