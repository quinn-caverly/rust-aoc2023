use day22::part_a::solve_part_a;

fn main() {
    let input_str = include_str!("input.txt");
    println!("part a: {}", solve_part_a(input_str));
}

#[cfg(test)]
mod test_part_a {
    use day22::part_a::{descend_all_bricks, parse_input, solve_part_a, Brick, Loc};

    #[test]
    fn test_sort_bricks() {
        let input_str = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9";
        let mut bricks = parse_input(input_str);

        let exp_lowest_brick = Brick {
            lower: Loc { x: 1, y: 0, z: 1 },
            higher: Loc { x: 1, y: 2, z: 1 },
        };

        bricks.sort();

        assert_eq!(exp_lowest_brick, *bricks.get(0).unwrap());
    }

    #[test]
    fn test_get_all_locs() {
        let brick = Brick {
            lower: Loc { x: 1, y: 0, z: 1 },
            higher: Loc { x: 1, y: 2, z: 1 },
        };

        assert_eq!(
            brick.get_all_locs(),
            vec![
                Loc { x: 1, y: 0, z: 1 },
                Loc { x: 1, y: 1, z: 1 },
                Loc { x: 1, y: 2, z: 1 }
            ]
        );

        let brick = Brick {
            lower: Loc { x: 1, y: 0, z: 1 },
            higher: Loc { x: 1, y: 0, z: 3 },
        };

        assert_eq!(
            brick.get_all_locs(),
            vec![
                Loc { x: 1, y: 0, z: 1 },
                Loc { x: 1, y: 0, z: 2 },
                Loc { x: 1, y: 0, z: 3 }
            ]
        );

        let brick = Brick {
            lower: Loc { x: 1, y: 0, z: 1 },
            higher: Loc { x: 1, y: 0, z: 1 },
        };

        assert_eq!(brick.get_all_locs(), vec![Loc { x: 1, y: 0, z: 1 },]);
    }

    #[test]
    fn test_do_bricks_conflict() {
        let brick0 = Brick {
            lower: Loc { x: 1, y: 0, z: 1 },
            higher: Loc { x: 1, y: 0, z: 1 },
        };

        let brick1 = Brick {
            lower: Loc { x: 1, y: 0, z: 1 },
            higher: Loc { x: 1, y: 0, z: 3 },
        };

        let brick2 = Brick {
            lower: Loc { x: 1, y: 0, z: 2 },
            higher: Loc { x: 1, y: 0, z: 3 },
        };

        assert!(brick0.conflicts(&brick1));
        assert!(!brick0.conflicts(&brick2));
    }

    #[test]
    fn test_descend_all_bricks() {
        let input_str = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9";
        let bricks = parse_input(input_str);

        let descended = descend_all_bricks(bricks);

        for brick in descended {
            println!("{:?}", brick);
        }
    }

    #[test]
    fn test_solve_part_a() {
        let input_str = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9";
        assert_eq!(solve_part_a(input_str), 5);
    }
}
