pub mod part_a {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Loc {
        pub x: usize,
        pub y: usize,
        pub z: usize,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Brick {
        pub lower: Loc,
        pub higher: Loc,
    }

    impl Ord for Brick {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.lower.z.cmp(&other.lower.z)
        }
    }

    impl PartialOrd for Brick {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.lower.z.cmp(&other.lower.z))
        }
    }

    impl Brick {
        pub fn get_all_locs(&self) -> Vec<Loc> {
            let mut locs: Vec<Loc> = vec![self.lower.clone()];

            for (lo, hi, dim) in [
                (self.lower.x + 1, self.higher.x, 'x'),
                (self.lower.y + 1, self.higher.y, 'y'),
                (self.lower.z + 1, self.higher.z, 'z'),
            ] {
                for i in lo..=hi {
                    match dim {
                        'x' => locs.push(Loc {
                            x: i,
                            y: self.lower.y,
                            z: self.lower.z,
                        }),
                        'y' => locs.push(Loc {
                            x: self.lower.x,
                            y: i,
                            z: self.lower.z,
                        }),
                        'z' => locs.push(Loc {
                            x: self.lower.x,
                            y: self.lower.y,
                            z: i,
                        }),
                        _ => panic!(),
                    }
                }
            }

            locs
        }

        pub fn conflicts(&self, other: &Self) -> bool {
            let other_locs = other.get_all_locs();
            for loc in self.get_all_locs() {
                if other_locs.contains(&loc) {
                    return true;
                }
            }
            false
        }

        // given a slice of Bricks starting at the one after self,
        // because ones resting on top can only be after, for each brick
        // if brick is horizontal, just check all bricks which have
        // lowest_z == low_z + 1, if it is vertical check all bricks which have
        // lowest_z == high_z + 1
        pub fn is_disintegratable(
            &self,
            descended_relevant: &[Self],
            all_descended: &[Self],
        ) -> bool {
            let relevant_z = (self.lower.z + 1).max(self.higher.z + 1);

            'outer: for brick in descended_relevant {
                if brick.lower.z == relevant_z {
                    // lower the brick.lower.z by 1 and then check if there
                    // is any overlap, if there would be overlap if it was
                    // one lower, then we can't disintegrate
                    let new_other = Brick {
                        lower: Loc {
                            x: brick.lower.x,
                            y: brick.lower.y,
                            z: brick.lower.z - 1,
                        },
                        higher: Loc {
                            x: brick.higher.x,
                            y: brick.higher.y,
                            z: brick.higher.z - 1,
                        },
                    };

                    if self.conflicts(&new_other) {
                        // if it conflicts with self, we need to check if it would conflict with
                        // any other current brick because a brick can be held up by more than one
                        // the other conflicting brick must not be self or brick
                        for b in all_descended {
                            if b != brick && b != self {
                                if new_other.conflicts(&b) {
                                    continue 'outer;
                                }
                            }
                        }

                        return false;
                    }
                }
            }
            true
        }
    }

    pub fn parse_input(input_str: &str) -> Vec<Brick> {
        input_str
            .lines()
            .map(|line| {
                let mut lower_higher = line.split("~");

                let mut lower_vec: Vec<usize> = lower_higher
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|num_str| num_str.parse::<usize>().unwrap())
                    .collect();
                let mut higher_vec: Vec<usize> = lower_higher
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|num_str| num_str.parse::<usize>().unwrap())
                    .collect();

                let lower = Loc {
                    z: lower_vec.pop().unwrap(),
                    y: lower_vec.pop().unwrap(),
                    x: lower_vec.pop().unwrap(),
                };
                let higher = Loc {
                    z: higher_vec.pop().unwrap(),
                    y: higher_vec.pop().unwrap(),
                    x: higher_vec.pop().unwrap(),
                };

                Brick { lower, higher }
            })
            .collect()
    }

    // descends the brick as far as possible until we hit a snag
    pub fn descend_brick(mut cur: Brick, landed_bricks: &mut Vec<Brick>) {
        'outer: loop {
            cur = Brick {
                lower: Loc {
                    x: cur.lower.x,
                    y: cur.lower.y,
                    z: cur.lower.z - 1,
                },
                higher: Loc {
                    x: cur.higher.x,
                    y: cur.higher.y,
                    z: cur.higher.z - 1,
                },
            };

            if cur.lower.z == 0 {
                break 'outer;
            }

            for landed in landed_bricks.iter() {
                if cur.conflicts(&landed) {
                    break 'outer;
                }
            }

        }

        landed_bricks.push(Brick {
            lower: Loc {
                x: cur.lower.x,
                y: cur.lower.y,
                z: cur.lower.z + 1,
            },
            higher: Loc {
                x: cur.higher.x,
                y: cur.higher.y,
                z: cur.higher.z + 1,
            },
        });
    }

    pub fn descend_all_bricks(mut bricks: Vec<Brick>) -> Vec<Brick> {
        let mut landed_bricks = Vec::new();

        // ascending order, so the lowest are first
        bricks.sort();

        for brick in bricks {
            descend_brick(brick, &mut landed_bricks);
        }

        landed_bricks
    }

    pub fn solve_part_a(input_str: &str) -> usize {
        let bricks = parse_input(input_str);
        let descended = descend_all_bricks(bricks);

        println!("got here");

        let mut count = 0;
        for (i, brick) in descended.iter().enumerate() {
            if brick.is_disintegratable(descended.get(i..).unwrap(), &descended) {
                count += 1;
            }
        }
        count
    }
}
