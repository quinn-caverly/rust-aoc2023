use day18::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day18::part_a::{
        build_loop, build_usize_graph, find_lowest_highest, parse_input, solve_part_a, Dir, Step,
    };

    #[test]
    fn test_parse_input() {
        let input_str = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)";
        let steps = parse_input(input_str);

        assert_eq!(
            steps.get(0).unwrap(),
            &Step {
                dir: Dir::Right,
                length: 6
            }
        );
        assert_eq!(
            steps.get(3).unwrap(),
            &Step {
                dir: Dir::Down,
                length: 2
            }
        );
    }

    #[test]
    fn test_build_loop() {
        let input_str = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)";
        let steps = parse_input(input_str);
        let set = build_loop(steps);

        assert!(set.contains(&(0, 6)));
        assert!(set.contains(&(1, 6)));
    }

    #[test]
    fn test_find_lowest() {
        let input_str = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)";
        let steps = parse_input(input_str);
        let set = build_loop(steps);
        let ((lowest_x, lowest_y), (highest_x, highest_y)) = find_lowest_highest(&set);

        assert_eq!(lowest_y, 0);
        assert_eq!(lowest_x, 0);
    }

    #[test]
    fn test_build_usize_graph() {
        let input_str = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)";
        let steps = parse_input(input_str);
        let set = build_loop(steps);
        let ((lowest_row, lowest_col), (highest_row, highest_col)) = find_lowest_highest(&set);
        let graph = build_usize_graph(&set, lowest_row, lowest_col, highest_row, highest_col);

        for row in graph {
            println!("{:?}", row);
        }

        // assert_eq!(0, 1);
    }

    #[test]
    fn test_solve_part_a() {
        let input_str = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";
        assert_eq!(solve_part_a(input_str), 62);
    }
}

#[cfg(test)]
mod test_part_b {
    use day18::part_b::{get_vertices_from_steps, parse_input_b, shoelace_formula, solve_part_b};

    #[test]
    fn test_get_vertices_from_steps() {
        let input_str = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";
        let vertices = get_vertices_from_steps(&parse_input_b(input_str));

        let first = (0, 461937);
        let second = (56407, 461937);

        assert_eq!(vertices.get(0).unwrap(), &first);
        assert_eq!(vertices.get(1).unwrap(), &second);
    }

    #[test]
    fn test_solve_part_b() {
        let input_str = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";

        assert_eq!(solve_part_b(input_str), 952408144115);
    }
}
