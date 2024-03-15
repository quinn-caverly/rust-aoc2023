use day17::{part_a::solve_part_a, part_b::solve_part_b};

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
    println!("part b: {}", solve_part_b(input_str));
}

#[cfg(test)]
mod test_part_a {

    // #[test]
    // fn test_parse_input() {
    //     let input_str = "241\n321\n325";
    //     let first = Node {
    //         row: 0,
    //         col: 0,
    //         heat_loss: 2,
    //     };
    //     let last = Node {
    //         row: 2,
    //         col: 2,
    //         heat_loss: 5,
    //     };
    //
    //     let ans = parse_input(input_str);
    //     assert_eq!(ans.get(0).unwrap(), &first);
    //     assert_eq!(ans.get(8).unwrap(), &last);
    // }

    use day17::part_a::solve_part_a;

    #[test]
    fn test_solve_part_a() {
        let input_str = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";
        assert_eq!(solve_part_a(input_str), 102);
    }
}

#[cfg(test)]
mod test_part_b {
    use day17::part_b::solve_part_b;

    #[test]
    fn test_solve_part_b() {
        let input_str = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";
        assert_eq!(solve_part_b(input_str), 94);
    }
}
