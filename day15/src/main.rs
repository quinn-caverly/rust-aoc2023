use day15::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day15::part_a::solve_part_a;

    #[test]
    fn test_solve_part_a() {
        let input_str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(solve_part_a(input_str), 1320);
    }
}

#[cfg(test)]
mod test_part_b {
    use day15::{
        part_a::parse_input,
        part_b::{create_boxes, grab_label, solve_part_b, Lens},
    };

    #[test]
    fn test_grab_label() {
        let sample = "rn=1";
        assert_eq!(grab_label(sample), "rn".to_string());

        let sample = "cm-";
        assert_eq!(grab_label(sample), "cm".to_string());
    }

    #[test]
    fn test_create_boxes() {
        let input_str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        let expected_box_0 = vec![
            Lens {
                label: "rn".to_string(),
                focal_length: 1,
            },
            Lens {
                label: "cm".to_string(),
                focal_length: 2,
            },
        ];

        let actual = create_boxes(parse_input(input_str));
        assert_eq!(actual.get(0).unwrap(), &expected_box_0);
    }

    #[test]
    fn test_solve_part_b() {
        let input_str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(solve_part_b(input_str), 145);
    }
}
