use std::collections::VecDeque;
use std::iter::zip;
use std::ops::RangeInclusive;
use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum Union {
    PartialLeft,
    PartialRight,
    Contains,
    IsContained,
}

#[derive(Debug, Clone, PartialEq)]
struct Cuboid {
    ranges: [RangeInclusive<i64>; 3],
}

impl Cuboid {
    fn from_str(s: &str) -> Self {
        let ranges: Vec<RangeInclusive<i64>> = s
            .split(",")
            .map(|r| {
                let (_coord, range) = r.split_once("=").unwrap();
                let (from, to) = range.split_once("..").unwrap();
                let from: i64 = from.parse().unwrap();
                let to: i64 = to.parse().unwrap();

                std::cmp::min(from, to)..=std::cmp::max(from, to)
            })
            .collect();
        Cuboid {
            ranges: ranges.try_into().unwrap(),
        }
    }

    fn count(&self) -> u64 {
        self.ranges
            .iter()
            .map(|r| r.start().abs_diff(r.end() + 1))
            .product()
    }

    #[inline]
    fn overlaps(&self, other: &Self) -> Option<[Union; 3]> {
        // Returns type of overlaps for each axis or None if cuboids don't overlap
        let mut bounds: Vec<Union> = vec![];
        for (sr, or) in zip(&self.ranges, &other.ranges) {
            let start_cmp = sr.start().cmp(or.start());
            let end_cmp = sr.end().cmp(or.end());

            let bound = match (start_cmp, end_cmp) {
                (Ordering::Greater, Ordering::Greater) => {
                    if sr.start() > or.end() {
                        return None;
                    }
                    Union::PartialRight                 
                },
                (Ordering::Less, Ordering::Less) => {
                    if sr.end() < or.start() {
                        return None;
                    }
                    Union::PartialLeft
                },
                (Ordering::Less, Ordering::Equal) => Union::Contains,
                (Ordering::Less, Ordering::Greater) => Union::Contains,
                (Ordering::Equal, Ordering::Less) => Union::IsContained,
                (Ordering::Equal, Ordering::Equal) => Union::IsContained,
                (Ordering::Equal, Ordering::Greater) => Union::Contains,
                (Ordering::Greater, Ordering::Less) => Union::IsContained,
                (Ordering::Greater, Ordering::Equal) => Union::IsContained,
                
            };
            bounds.push(bound);
        }
        Some(bounds.try_into().unwrap())
    }

    #[inline]
    fn get_cuts(&self, other: &Self, overlaps: [Union; 3]) -> Vec<[Option<i64>; 2]> {
        // Returns a list of cuts where to cut the cuboid based on edges of other cuboid
        zip(zip(&self.ranges, &other.ranges), overlaps)
            .map(|((sr, or), overlap)| {
                match overlap {
                    Union::IsContained => [None, None], // No splits
                    Union::PartialLeft => [Some(*or.start()), None],
                    Union::PartialRight => [None, Some(*or.end())],
                    Union::Contains => {
                        // Two splits
                        let lhs = if sr.start() == or.start() {
                            None
                        } else {
                            Some(*or.start())
                        };
                        let rhs = if sr.end() == or.end() {
                            None
                        } else {
                            Some(*or.end())
                        };
                        [lhs, rhs]
                    }
                }
            })
            .collect()
    }

    #[inline]
    fn outer_cut(&self, cuts: Vec<[Option<i64>; 2]>) -> Vec<Cuboid> {
        let mut cutted_cuboids: Vec<Cuboid> = vec![];
        let mut added_sides: Vec<[Option<i64>; 2]> = vec![];
        // let mut added_extra = false;
        for (i, [cut_l, cut_r]) in cuts.iter().enumerate() {
            if let Some(cut_l) = cut_l {
                let ranges: Vec<RangeInclusive<i64>> = self
                    .ranges
                    .iter()
                    .enumerate()
                    .map(|(j, r)| {
                        if j == i {
                            *r.start()..=cut_l - 1
                        } else {
                            if let Some(cut) = added_sides.get(j) {
                                let final_range = match cut {
                                    [Some(lhs), Some(rhs)] => *lhs..=*rhs,
                                    [Some(lhs), None] => *lhs..=*r.end(),
                                    [None, Some(rhs)] => *r.start()..=*rhs,
                                    [None, None] => r.clone(),
                                };
                                return final_range;
                            }
                            r.clone()
                        }
                    })
                    .collect();
                cutted_cuboids.push(Cuboid {
                    ranges: ranges.try_into().unwrap(),
                })
            }
            if let Some(cut_r) = cut_r {
                let ranges: Vec<RangeInclusive<i64>> = self
                    .ranges
                    .iter()
                    .enumerate()
                    .map(|(j, r)| {
                        if j == i {
                            cut_r + 1..=*r.end()
                        } else {
                            if let Some(cut) = added_sides.get(j) {
                                let final_range = match cut {
                                    [Some(lhs), Some(rhs)] => *lhs..=*rhs,
                                    [Some(lhs), None] => *lhs..=*r.end(),
                                    [None, Some(rhs)] => *r.start()..=*rhs,
                                    [None, None] => r.clone(),
                                };
                                return final_range;
                            }
                            r.clone()
                        }
                    })
                    .collect();
                cutted_cuboids.push(Cuboid {
                    ranges: ranges.try_into().unwrap(),
                })
            }
            added_sides.push([cut_l.clone(), cut_r.clone()]);
        }
        cutted_cuboids
    }

    #[inline]
    fn substract(&self, rhs: &Self) -> Option<Vec<Cuboid>> {
        let overlaps = self.overlaps(rhs);
        if overlaps.is_none() {
            return None;
        }
        let overlaps = overlaps.unwrap();
        let first_overlap = &overlaps[0];
        let all_equal = overlaps.iter().skip(1).all(|o| o == first_overlap);
        match (all_equal, first_overlap) {
            (true, Union::IsContained) => return Some(vec![]),
            _ => {
                let cuts = self.get_cuts(rhs, overlaps);
                let remainder = self.outer_cut(cuts);
                Some(remainder)
            }
        }
    }
}

type Command = (bool, Cuboid);
pub struct ReactorReboot {
    commands: Vec<Command>,
}

impl crate::Advent for ReactorReboot {
    fn new(data: &str) -> ReactorReboot {
        let commands: Vec<Command> = data
            .lines()
            .map(|l| {
                let (switch, ranges) = l.split_once(" ").unwrap();
                let switch = match switch {
                    "on" => true,
                    "off" => false,
                    _ => panic!("Invalid switch statement: {}", switch),
                };
                let cuboid = Cuboid::from_str(ranges);

                (switch, cuboid)
            })
            .collect();
        ReactorReboot { commands }
    }

    fn part1(&mut self) -> usize {        
        let to_insert_cuboids: VecDeque<(bool, Cuboid)> =
            VecDeque::from_iter(self.commands.clone().into_iter().filter(|c| {
                c.1.ranges
                    .iter()
                    .all(|r| *r.start() >= -50 && *r.end() <= 50)
            }));
        reset_reactor(to_insert_cuboids)
    }

    fn part2(&mut self) -> usize {
        reset_reactor(VecDeque::from(self.commands.clone()))
    }
}

fn reset_reactor(mut to_insert_cuboids: VecDeque<(bool, Cuboid)>) -> usize {
    let mut current_cuboids: Vec<Cuboid> = vec![];
    'inserting: while let Some((switch, cuboid)) = to_insert_cuboids.pop_front() {
        if switch == true {
            for present_cuboid in &current_cuboids {
                let overlap = cuboid.substract(present_cuboid);
                if let Some(remainder) = overlap {
                    for r_cuboid in remainder {
                        to_insert_cuboids.push_front((switch, r_cuboid));
                    }
                    continue 'inserting;
                }
            }
            // Current cuboid doesn't interact with any of present cuboids - insert it
            current_cuboids.push(cuboid);
        } else {
            let mut new_cuboids: Vec<Cuboid> = vec![];
            current_cuboids.retain(|present_cuboid| {
                let mut leftover = present_cuboid.substract(&cuboid);
                if let Some(cuboids) = leftover.take() {
                    new_cuboids.extend(cuboids);
                    false
                } else {
                    true
                }
            });
            current_cuboids.extend(new_cuboids);
        }
    }
    current_cuboids.iter().map(|c| c.count()).sum::<u64>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn coord_sub() {
        assert_eq!(2 < 3, true);
        assert_eq!(-1 < 2, true);
        assert_eq!(-2 < -1, true);
        assert_eq!(2 > -2, true);
    }

    #[test]
    fn max_with_minus() {
        assert_eq!(std::cmp::max(1, 2), 2);
        assert_eq!(std::cmp::min(1, 2), 1);
        assert_eq!(std::cmp::max(-1, 2), 2);
        assert_eq!(std::cmp::min(-1, 2), -1);
        assert_eq!(std::cmp::max(-3, 2), 2);
    }

    #[test]
    fn test_overlap() {
        let c1 = Cuboid::from_str("x=10..12,y=10..12,z=10..12");
        let c2 = Cuboid::from_str("x=11..13,y=11..13,z=11..13");
        let mut res = c1.overlaps(&c2).unwrap().into_iter();
        assert_eq!(res.next(), Some(Union::PartialLeft));
        assert_eq!(res.next(), Some(Union::PartialLeft));
        assert_eq!(res.next(), Some(Union::PartialLeft));
        assert_eq!(res.next(), None);

        
    }

    #[test]
    fn test_cuts() {
        let c1 = Cuboid::from_str("x=10..12,y=10..12,z=10..12");
        let c2 = Cuboid::from_str("x=11..13,y=11..13,z=11..13");
        let c3 = Cuboid::from_str("x=9..11,y=9..11,z=9..11");
        let c4 = Cuboid::from_str("x=9..13,y=9..13,z=9..13");

        let overlaps = c2.overlaps(&c1).unwrap();
        let cuts = c2.get_cuts(&c1, overlaps);
        let mut it = cuts.into_iter();
        assert_eq!(it.next(), Some([None, Some(12)]));
        assert_eq!(it.next(), Some([None, Some(12)]));
        assert_eq!(it.next(), Some([None, Some(12)]));
        assert_eq!(it.next(), None);

        let overlaps = c3.overlaps(&c1).unwrap();
        let cuts = c3.get_cuts(&c1, overlaps);
        let mut it = cuts.into_iter();
        assert_eq!(it.next(), Some([Some(10), None]));
        assert_eq!(it.next(), Some([Some(10), None]));
        assert_eq!(it.next(), Some([Some(10), None]));
        assert_eq!(it.next(), None);

        let overlaps = c4.overlaps(&c1).unwrap();
        let cuts = c4.get_cuts(&c1, overlaps);
        let mut it = cuts.into_iter();
        assert_eq!(it.next(), Some([Some(10), Some(12)]));
        assert_eq!(it.next(), Some([Some(10), Some(12)]));
        assert_eq!(it.next(), Some([Some(10), Some(12)]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_substracting_overlapping_cuboids() {
        let c1 = Cuboid::from_str("x=10..12,y=10..12,z=10..12");
        let c2 = Cuboid::from_str("x=11..13,y=11..13,z=11..13");
        let c3 = Cuboid::from_str("x=9..11,y=9..11,z=9..11");
        let c4 = Cuboid::from_str("x=9..13,y=9..13,z=9..13");

        // Right side
        let result = c2.substract(&c1).unwrap();        
        let mut it = result.into_iter();
        assert_eq!(it.next(), Some(Cuboid { ranges: [13..=13, 11..=13, 11..=13]}));
        assert_eq!(it.next(), Some(Cuboid { ranges: [11..=12, 13..=13, 11..=13]}));
        assert_eq!(it.next(),Some(Cuboid { ranges: [11..=12, 11..=12, 13..=13]}));
        assert_eq!(it.next(), None);

        // Left side
        let result = c3.substract(&c1).unwrap();
        let mut it = result.into_iter();
        assert_eq!(it.next(), Some(Cuboid { ranges: [9..=9, 9..=11, 9..=11]}));
        assert_eq!(it.next(), Some(Cuboid { ranges: [10..=11, 9..=9, 9..=11]}));
        assert_eq!(it.next(),Some(Cuboid { ranges: [10..=11, 10..=11, 9..=9]}));
        assert_eq!(it.next(), None);

        // Test contains
        let result = c4.substract(&c1).unwrap();
        let mut it = result.into_iter();
        assert_eq!(it.next(), Some(Cuboid { ranges: [9..=9, 9..=13, 9..=13]}));
        assert_eq!(it.next(), Some(Cuboid { ranges: [13..=13, 9..=13, 9..=13]}));
        assert_eq!(it.next(),Some(Cuboid { ranges: [10..=12, 9..=9, 9..=13]}));
        assert_eq!(it.next(), Some(Cuboid { ranges: [10..=12, 13..=13, 9..=13]}));
        assert_eq!(it.next(), Some(Cuboid { ranges: [10..=12, 10..=12, 9..=9]}));
        assert_eq!(it.next(),Some(Cuboid { ranges: [10..=12, 10..=12, 13..=13]}));
        
        assert_eq!(it.next(), None);

        // Test is contained
        let result = c1.substract(&c4).unwrap();
        let mut it = result.into_iter();
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_remainder_not_overlaping() {
        let c1 = Cuboid::from_str("x=-10..10,y=-10..10,z=-10..10");
        let c2 = Cuboid::from_str("x=-12..12,y=-12..12,z=-12..12");
        let remainder = c2.substract(&c1).unwrap();
        remainder.iter().permutations(2).for_each(|p| {
            assert_eq!(p[0].overlaps(p[1]), None);
        });
        remainder.iter().for_each(|r| {
            assert_eq!(c1.overlaps(r), None);
        });
    }

    #[test]
    fn test_side_laying_cuboids() {
        let c1 = Cuboid::from_str("x=-10..10,y=-10..10,z=-10..10");
        let c2 = Cuboid::from_str("x=-20..-10,y=-20..-10,z=-20..-10"); // Touching in corner point
        let c3 = Cuboid::from_str("x=-20..-11,y=-10..10,z=-10..10"); // Lays side by side without overlaping
        let c4 = Cuboid::from_str("x=-20..-10,y=-10..10,z=-10..10"); // Overlaps on one side
        let overlaps = c1.overlaps(&c2);
        assert_eq!(overlaps, Some([Union::PartialRight, Union::PartialRight, Union::PartialRight]));        
        let remainder = c1.substract(&c2).unwrap();
        let mut it = remainder.into_iter();
        assert_eq!(it.next(), Some(Cuboid { ranges: [-9..=10, -10..=10, -10..=10 ]}));        
        assert_eq!(it.next(), Some(Cuboid { ranges: [-10..=-10, -9..=10, -10..=10 ]}));
        assert_eq!(it.next(), Some(Cuboid { ranges: [-10..=-10, -10..=-10, -9..=10 ]}));
        assert_eq!(it.next(), None);

        let overlaps = c1.overlaps(&c3);
        assert_eq!(overlaps, None);

        let overlaps = c1.overlaps(&c4);
        assert_eq!(overlaps, Some([Union::PartialRight, Union::IsContained, Union::IsContained]));
        let remainder = c1.substract(&c4).unwrap();
        assert_eq!(remainder, [Cuboid { ranges: [-9..=10, -10..=10, -10..=10]}]);            
    }

    #[test]
    fn test_fucking_error_shit() {
        let c1 = Cuboid::from_str("x=34..34,y=24..24,z=16..17");
        let c2 = Cuboid::from_str("x=34..34,y=24..24,z=16..16");        
        let c3 = Cuboid::from_str("x=34..34,y=24..24,z=17..17");

        let r1 = c2.substract(&c1).unwrap();
        assert_eq!(r1, []);
        let r2 = c1.substract(&c2).unwrap();        
        assert_eq!(r2, [Cuboid { ranges: [34..=34, 24..=24, 17..=17]}]);
        let r3 = c3.substract(&c1).unwrap();
        assert_eq!(r3, []);
        let r4 = c1.substract(&c3).unwrap();
        assert_eq!(r4, [Cuboid { ranges: [34..=34, 24..=24, 16..=16]}]);
    }
}

