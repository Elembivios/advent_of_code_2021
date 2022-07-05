use std::fmt;
use std::ops::{Add, Sub, AddAssign};
use std::cmp;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash)]
struct Coord {
    x: isize, 
    y: isize,
    z: isize
}

impl Coord {
    fn normalize(&mut self) {
        // Orders x, y and z from smallest to largest and makes x and y positive
        // eg.: (-68, -1246, -43) => (43, -1246. -68) => (43, 68, -1246)
        let valsort = |lhs: &mut isize, rhs: &mut isize| {
            if lhs.abs() > rhs.abs() {
                let (olhs, orhs) = (*lhs, *rhs);
                if *rhs <= 0 {
                    *rhs = olhs;
                    *lhs = -orhs;
                } else {
                    *rhs = -olhs;
                    *lhs = orhs;
                }
            }
        };

        valsort(&mut self.x, &mut self.z);
        valsort(&mut self.x, &mut self.y);
        valsort(&mut self.y, &mut self.z);

        if self.x < 0 {
            self.x = -self.x;
            self.y = -self.y;
        }
        if self.y < 0 {
            self.y = -self.y;
            self.z = -self.z;
        }
    }
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

impl cmp::PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Coord {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let x_ord = self.x.cmp(&other.x);
        match x_ord {
            cmp::Ordering::Equal => {
                let y_ord = self.y.cmp(&other.y);
                match y_ord {
                    cmp::Ordering::Equal => {
                        return self.z.cmp(&other.z);
                    },
                    _ => return y_ord
                }
            },
            _ => return x_ord
        }
    }
}

#[derive(Clone)]
struct Scanner {
    i: usize,
    beacons: Vec<Coord>,
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
        Scanner { beacons, i }
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

fn find_valid_transformer(from: Coord, to: Coord) -> (usize, &'static dyn Fn(Coord) -> Coord) {
    let transformation: [&dyn Fn(Coord) -> Coord; 24] = [
        &|c| { let c = Coord { x: c.x, y: c.y, z: c.z}; Coord { x: c.x, y: c.y, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: c.y, z: c.z}; Coord { x: -c.y, y: c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: c.y, z: c.z}; Coord { x: c.y, y: -c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: c.y, z: c.z}; Coord { x: -c.x, y: -c.y, z: c.z}}.into(),

        &|c| { let c = Coord { x: c.x, y: -c.z, z: c.y}; Coord { x: c.x, y: c.y, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: -c.z, z: c.y}; Coord { x: -c.y, y: c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: -c.z, z: c.y}; Coord { x: c.y, y: -c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: -c.z, z: c.y}; Coord { x: -c.x, y: -c.y, z: c.z}}.into(),

        &|c| { let c = Coord { x: c.x, y: c.z, z: -c.y}; Coord { x: c.x, y: c.y, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: c.z, z: -c.y}; Coord { x: -c.y, y: c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: c.z, z: -c.y}; Coord { x: c.y, y: -c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: c.z, z: -c.y}; Coord { x: -c.x, y: -c.y, z: c.z}}.into(),     

        &|c| { let c = Coord { x: c.x, y: -c.y, z: -c.z}; Coord { x: c.x, y: c.y, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: -c.y, z: -c.z}; Coord { x: -c.y, y: c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: -c.y, z: -c.z}; Coord { x: c.y, y: -c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.x, y: -c.y, z: -c.z}; Coord { x: -c.x, y: -c.y, z: c.z}}.into(),

        &|c| { let c = Coord { x: c.z, y: c.y, z: -c.x}; Coord { x: c.x, y: c.y, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.z, y: c.y, z: -c.x}; Coord { x: -c.y, y: c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.z, y: c.y, z: -c.x}; Coord { x: c.y, y: -c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: c.z, y: c.y, z: -c.x}; Coord { x: -c.x, y: -c.y, z: c.z}}.into(),

        &|c| { let c = Coord { x: -c.z, y: c.y, z: c.x}; Coord { x: c.x, y: c.y, z: c.z}}.into(),
        &|c| { let c = Coord { x: -c.z, y: c.y, z: c.x}; Coord { x: -c.y, y: c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: -c.z, y: c.y, z: c.x}; Coord { x: c.y, y: -c.x, z: c.z}}.into(),
        &|c| { let c = Coord { x: -c.z, y: c.y, z: c.x}; Coord { x: -c.x, y: -c.y, z: c.z}}.into(),
    ];
    for (idx, tran) in transformation.iter().copied().enumerate() {
        if tran(from) == to {
            return (idx, tran);
        }
    }

    unreachable!()
}

fn solve(scanners: &[Scanner]) -> (Vec<(usize, Coord)>, Vec<Coord>) {
    let mut normalized: Vec<(usize, Vec<(Coord, Coord, Coord)>)> = scanners
        .iter()
        .enumerate()
        .map(|(sidx, s)| {
            let beacons = s.beacons.as_slice();
            let mut norm: Vec<_> = beacons.iter().permutations(2).map(|p| {
                    (p[0], p[1])
                }).map(|(&lhs, &rhs)| {
                    let mut diff = lhs - rhs;
                    diff.normalize();
                    (diff, lhs, rhs)
                }).collect();

            norm.sort_unstable();
            (sidx, norm)
        }).collect();
    let (psidx, mut active_normalized) = normalized.remove(0);
    let mut full_normalized = active_normalized.clone();
    let mut scanner_location = vec![(0, Coord { x: 0, y: 0, z: 0})];
    let mut known_beacon = scanners[psidx].beacons.clone();

    let mut candidate_count = vec![];

    while !normalized.is_empty() {
        // Rebuild the list of norms from the full set
        active_normalized.clear();
        active_normalized.extend(full_normalized.iter().copied());
        active_normalized.sort_unstable();

        normalized.drain_filter(|(nidx, n)|{
            candidate_count.clear();
            let mut lhsiter = active_normalized.group_by(|lhs, rhs| {lhs.0 == rhs.0}).peekable();
            let mut rhsiter = n.group_by(|lhs, rhs| {lhs.0 == rhs.0}).peekable();            
            while lhsiter.peek().is_some() && rhsiter.peek().is_some() {
                match (lhsiter.peek(), rhsiter.peek()) {
                    (Some(lhs), Some(rhs)) => {
                        match lhs[0].0.cmp(&rhs[0].0) {
                            std::cmp::Ordering::Equal => {
                                for (lhs, rhs) in lhs.iter().map(|lhs| rhs.iter().map(move |rhs| (lhs, rhs))).flatten() {
                                    let left = lhs.2 - lhs.1;
                                    let right = rhs.2 - rhs.1;
                                    let (tranid, tran) = find_valid_transformer(right, left);
                                    let ls = lhs.1;
                                    let rs = tran(rhs.1);
                                    let offset = ls - rs;
                                    candidate_count.push((offset, tranid, tran));                                    
                                }
                                lhsiter.next();
                                rhsiter.next();
                            },
                            std::cmp::Ordering::Less => {
                                lhsiter.next();
                            },
                            std::cmp::Ordering::Greater => {
                                rhsiter.next();
                            }
                        }
                    },
                    _ => unreachable!()
                }                
            }

            candidate_count.sort_unstable_by_key(|(offset, tranid, _tran)| (*offset, *tranid));
            let transinfo = candidate_count.group_by(|lhs, rhs| (lhs.0, lhs.1) == (rhs.0, rhs.1))
                .filter(|candidate| candidate.len() >= 6 )
                .max_by_key(|candidate| candidate.len());
            let candidate = match transinfo {
                Some(candidate) => candidate,
                None => return false
            };
            let (pos, _tranid, tran) = candidate[0];
            full_normalized.extend(n.iter().map(|(norm, lhs, rhs)| {
                let t = tran(*lhs);
                let lhs = t + pos;
                let t = tran(*rhs);
                let rhs = t + pos;
                (*norm, lhs, rhs)
            }));

            known_beacon.extend(scanners[*nidx].beacons.iter().map(|b| {
                let t = tran(*b);
                t + pos
            }));
            scanner_location.push((*nidx, (pos)));
            true
        }).for_each(|_| ());
    }

    (scanner_location, known_beacon)
}

pub struct BeaconScaner {
    scanners: Vec<Scanner>,
}

impl crate::Advent for BeaconScaner {
    fn new(data: &str) -> Self {
        let data = data.replace("\r", "");
        let scanners: Vec<Scanner> = data
            .split("\n\n")
            .map(|s| Scanner::from_str(s))
            .collect();
        BeaconScaner { scanners }
    }

    fn part1(&mut self) -> usize {        
        let (_, mut beacons) = solve(&self.scanners);
        beacons.sort_unstable();
        beacons.dedup();
        beacons.len()
    }

    fn part2(&mut self) -> usize {
        let (scanners, _) = solve(&self.scanners);
        scanners.iter()
            .map(|lhs| {
                scanners.iter().map(move |rhs| (lhs, rhs))
            }).flatten()
            .map(|((_, lhs), (_, rhs))| {
                let diff = *lhs - *rhs;
                (diff.x.abs() + diff.y.abs() + diff.z.abs()) as usize
            })
            .max().unwrap()
    }
}
