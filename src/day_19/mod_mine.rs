// My implementation of day 19 using brute force; rotating
// each scanner untill 12 matching offsets are found.
// This code runs in ~ 1.0 second.

use std::fmt;
use std::ops::{Add, Sub, AddAssign};
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Sign {
    Pos,
    Neg
}

impl Sign {
    fn next(&self) -> Self {
        match self {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Axis {
    X, Y, Z
}

impl Axis {
    fn next(&self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash)]
struct Coord {
    x: isize, 
    y: isize,
    z: isize
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}


#[derive(Clone)]
struct Scanner {
    i: usize,
    c: Option<Coord>,
    beacons: Vec<Coord>,
    facing_axis: Axis,
    facing_sign: Sign,
    rotation_index: usize
}

impl Scanner {
    fn from_str(s: &str) -> Self {
        let mut it = s.lines();

        let head = it.next().unwrap();
        let i: usize = head.strip_prefix("--- scanner ").unwrap().strip_suffix(" ---").unwrap().parse().unwrap();

        let beacons: Vec<Coord> = it.map(|l| {
            let coords: Vec<isize> = l.split(",").map(|s| {
                let r = s.parse();
                if r.is_err() {
                    println!("S: {}, L: {}", s, l);

                }
                r.unwrap()
            }).collect();
            Coord {
                x: coords[0],
                y: coords[1],
                z: coords[2]
            }
        
        }).collect();
        Scanner::new( beacons, i )         
    }

    fn new(beacons: Vec<Coord>, i: usize) -> Self {
        Scanner { c: None, beacons, facing_axis: Axis::X, facing_sign: Sign::Pos, i, rotation_index: 0 }
    }

    fn change_axis(&mut self) {
        let next_axis = self.facing_axis.next();        
        match (self.facing_axis, next_axis) {
            (Axis::X, Axis::Y) => {
                self.tilt_rev(Axis::Z);
            },
            (Axis::Y, Axis::Z) => {
                self.tilt_rev(Axis::X);
            },
            (Axis::Z, Axis::X) => {
                self.tilt_rev(Axis::Y);
            },
            _ => panic!("Invalid next axis configuration. Wanted to go from {:?} to {:?}", self.facing_axis, next_axis),
        }
        self.facing_axis = next_axis;
    }

    fn change_sign(&mut self) {
        self.facing_sign = self.facing_sign.next();
        let tilt_over = match self.facing_axis {
            Axis::X => Axis::Z,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X            
        };
        self.tilt(tilt_over);
        self.tilt(tilt_over);
    }

    fn tilt(&mut self, over: Axis) {
        // println!("Tilted over: {:?}", over);
        for c in self.beacons.iter_mut() {
            match over {
                Axis::X => { 
                    let temp = c.y;
                    c.y = c.z;
                    c.z = temp * -1;
                },
                Axis::Y => {
                    let temp = c.z;
                    c.z = c.x;
                    c.x = temp * -1;
                },
                Axis::Z => { 
                    let temp = c.x;
                    c.x = c.y;
                    c.y = temp * -1;
                }
            }            
        }
    }

    fn tilt_rev(&mut self, over: Axis) {
        // println!("Tilted over: {:?}", over);
        for c in self.beacons.iter_mut() {
            match over {
                Axis::X => { 
                    let temp = c.z;
                    c.z = c.y;
                    c.y = temp * -1;
                },
                Axis::Y => {
                    let temp = c.x;
                    c.x = c.z;
                    c.z = temp * -1;
                },
                Axis::Z => { 
                    let temp = c.y;
                    c.y = c.x;
                    c.x = temp * -1;
                }
            }            
        }
    }

    fn rotate(&mut self) {
        if self.rotation_index  == 0 {
            // Already in correct position, just return
            self.rotation_index += 1;
            return;
        }        
        if self.rotation_index % 4 == 0 {                 
            // Tilt back to original position
            self.tilt(self.facing_axis.clone());
            self.change_sign();

            if self.rotation_index % 8 == 0 {
                self.change_axis();
                if self.rotation_index == 24 {
                    self.tilt(Axis::X);
                    self.rotation_index = 1;
                } else {
                    self.rotation_index += 1;
                }
                return;
            }

            self.rotation_index += 1;
            return;
        }
        self.tilt(self.facing_axis.clone());
        self.rotation_index += 1;
    }

    fn find_offset(&self, other: &mut Self) -> Option<Coord> {
        for _x in 0..24 {
            other.rotate();
            let mut offsets: HashMap<Coord, usize> = HashMap::new();
            for b1 in &self.beacons {
                for b2 in &other.beacons {
                    let diff = *b1 - *b2;
                    let offset_count = offsets.entry(diff).or_insert(1);
                    *offset_count += 1;
                    if *offset_count >= 12 {
                        return Some(diff)
                    }
                }
            }
            // println!("Max Offsets: {:?}", offsets.values().max());
            offsets.clear(); 
        }

        None
    }

    fn merge(&mut self, other: &mut Self, offset: Coord) {
        for beacon in other.beacons.iter_mut() {
            *beacon += offset;
            if !self.beacons.contains(beacon) {
                self.beacons.push(*beacon);
            }
        }
    }
    
}

impl fmt::Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Scanner {}:\n", self.i)?;
        for beacon in &self.beacons {
            write!(f, "\t{}\n", beacon)?;
        }
        write!(f, "\n")        
    }
}

pub struct BeaconScaner {
    scanners: Vec<Scanner>,
    offsets: Vec<Coord>
}

impl crate::Advent for BeaconScaner {
    fn new(data: &str) -> Self {
        let data = data.replace("\r", "");
        let scanners: Vec<Scanner> = data
            .split("\n\n")
            .map(|s| Scanner::from_str(s))
            .collect();
        BeaconScaner { scanners, offsets: vec![] }
    }

    fn part1(&mut self) -> usize {        
        let mut scanners = VecDeque::from(self.scanners.clone());

        let mut origin = scanners.pop_front().unwrap();
        origin.c = Some(Coord{ x: 0, y: 0, z: 0});
        while scanners.len() != 0 {
            let s2 = scanners.pop_front().unwrap();
            let mut s2 = s2.clone();
            let offset = origin.find_offset(&mut s2);
            if let Some(offset) = offset {
                // println!("Matches {}", s2.i);
                origin.merge(&mut s2, offset);
                self.offsets.push(offset);
            } else {
                scanners.push_back(s2);
            }
        }

        origin.beacons.len()    
    }

    fn part2(&mut self) -> usize {
        let max_dist = self.offsets.iter().permutations(2).map(|v| {
            let (a, b) = (v[0], v[1]);
            let diff = *a - *b;
            diff.x.abs() + diff.y.abs() + diff.z.abs()
        }).max();

        max_dist.unwrap() as usize
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_small_scanner() -> Scanner {
        Scanner::from_str(
            "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7")
    }

    fn get_scanner_0() -> Scanner {        
        Scanner::from_str(
            "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401"
        )
    }
    fn get_scanner_1() -> Scanner {
        Scanner::from_str(
            "--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390"
        )
    }
    // fn get_scanner_2() -> Scanner {
    //     Scanner::from_str(
    //         "--- scanner 2 ---
    // 649,640,665
    // 682,-795,504
    // -784,533,-524
    // -644,584,-595
    // -588,-843,648
    // -30,6,44
    // -674,560,763
    // 500,723,-460
    // 609,671,-379
    // -555,-800,653
    // -675,-892,-343
    // 697,-426,-610
    // 578,704,681
    // 493,664,-388
    // -671,-858,530
    // -667,343,800
    // 571,-461,-707
    // -138,-166,112
    // -889,563,-600
    // 646,-828,498
    // 640,759,510
    // -630,509,768
    // -681,-892,-333
    // 673,-379,-804
    // -742,-814,-386
    // 577,-820,562"
    //     )
    // }
    // fn get_scanner_3() -> Scanner {
    //     Scanner::from_str(
    //         "--- scanner 3 ---
    // -589,542,597
    // 605,-692,669
    // -500,565,-823
    // -660,373,557
    // -458,-679,-417
    // -488,449,543
    // -626,468,-788
    // 338,-750,-386
    // 528,-832,-391
    // 562,-778,733
    // -938,-730,414
    // 543,643,-506
    // -524,371,-870
    // 407,773,750
    // -104,29,83
    // 378,-903,-323
    // -778,-728,485
    // 426,699,580
    // -438,-605,-362
    // -469,-447,-387
    // 509,732,623
    // 647,635,-688
    // -868,-804,481
    // 614,-800,639
    // 595,780,-596"
    //     )
    // }
    fn get_scanner_4() -> Scanner {
    Scanner::from_str(
        "--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"
    )
}

    #[test]
    fn test_tilt_comes_around() {
        let mut scanner = get_small_scanner();
        let original = scanner.clone();
        for _ in 0..4 { scanner.tilt(Axis::X) }
        assert_eq!(scanner.beacons, original.beacons);

        for _ in 0..4 { scanner.tilt(Axis::Y) }
        assert_eq!(scanner.beacons, original.beacons);

        for _ in 0..4 { scanner.tilt(Axis::Z) }
        assert_eq!(scanner.beacons, original.beacons);

        for _ in 0..4 { 
            for _ in 0..4 {
                scanner.tilt(Axis::Y);
            }
            scanner.tilt(Axis::X);            
        }
        assert_eq!(scanner.beacons, original.beacons);
    }

    #[test]
    fn test_axis_comes_around() {
        let mut scanner = get_small_scanner();
        let original = scanner.clone();
        for x in 0..3 {
            scanner.change_axis();            
        }
        scanner.tilt(Axis::X);
        assert_eq!(scanner.beacons, original.beacons);
    }

    #[test] 
    fn test_whole_rotation_comes_around() {
        let mut scanner = get_small_scanner();
        let original = scanner.clone();
        for _ in 0..25 {
            scanner.rotate();
        }
        assert_eq!(original.beacons, scanner.beacons);
    }

    #[test]
    fn test_tilt_rev() {
        let mut scanner = get_small_scanner();
        let original = scanner.clone();

        scanner.tilt(Axis::X);
        scanner.tilt_rev(Axis::X);

        assert_eq!(scanner.beacons, original.beacons);
    }

    #[test]
    fn test_tilt_same_as_change_sign() {
        let mut scanner01 = get_small_scanner();
        let mut scanner02 = scanner01.clone();

        scanner01.tilt(Axis::Z);
        scanner01.tilt(Axis::Z);
        // scanner01.tilt(Axis::X);
        // scanner01.tilt(Axis::X);

        scanner02.change_sign();

        assert_eq!(scanner01.beacons, scanner02.beacons);
    }

    #[test]
    fn test_offset() {
        let scanner_0 = get_scanner_0();            
        let mut scanner_1 = get_scanner_1();         
        let offset = scanner_0.find_offset(&mut scanner_1);        
        assert_eq!(offset.unwrap(), Coord { x: 68, y: -1246, z: -43});

        let offset = offset.unwrap();
        for beacon in scanner_1.beacons.iter_mut() {
            *beacon += offset;
        }

        let mut scanner_4 = get_scanner_4();
        let offset_02 = scanner_1.find_offset(&mut scanner_4);

        assert_eq!(offset_02.unwrap(), Coord { x: -20, y: -1133, z: 1061 });
    }

}
