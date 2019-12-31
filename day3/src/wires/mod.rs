use std::error::Error;
use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

const COORD_ORIGIN: Coordinate = Coordinate { x: 0, y: 0 };

#[derive(Clone, Hash, PartialOrd, Ord, Debug, Eq, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> u32 {
        (i32::abs(self.x - other.x) + i32::abs(self.y - other.y)) as u32
    }
}

pub struct Grid {
    wires: Vec<Wire>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            wires: Vec::new(),
        }
    }

    pub fn parse_file(path: &str) -> Result<Grid, String> {
        let mut grid = Grid::new();
        match fs::read_to_string(path) {
            Ok(data) => {
                for line in data.lines() {
                    let wire = Wire::parse(line)?;
                    grid.add_wire(wire);
                }
                Ok(grid)
            }
            Err(e) => Err(String::from(e.description()))
        }
    }

    pub fn add_wire(&mut self, wire: Wire) {
        self.wires.push(wire)
    }

    pub fn intersections(&self) -> Vec<Coordinate> {
        let mut intersects = HashSet::new();
        // Brute forcing this. it could start skipping pairs it's already seen to speed it up
        for (idx, wire) in self.wires.iter().enumerate() {
            for (comp_idx, comp_wire) in self.wires.iter().enumerate() {
                if idx == comp_idx {
                    continue;
                }

                for coord in wire.hash_coords.intersection(&comp_wire.hash_coords) {
                    // We know this intersects at (0, 0) so ignore those.
                    if *coord != COORD_ORIGIN {
                        intersects.insert(coord.clone());
                    }
                }
            }
        }
        let mut result = intersects.into_iter().collect::<Vec<Coordinate>>();
        result.sort();
        result
    }

    pub fn closest_distance(&self) -> Option<u32> {
        if self.wires.len() < 2 {
            return None;
        }

        let mut lowest: u32 = std::u32::MAX;
        for coord in self.intersections() {
            let distance = coord.distance(&COORD_ORIGIN);
            if distance < lowest {
                lowest = distance;
            }
        }

        Some(lowest)
    }

    pub fn shortest_steps(&self) -> Option<u32> {
        if self.wires.len() < 2 {
            return None;
        }

        let mut steps = std::u32::MAX;
        for intersect in self.intersections() {
            let mut intersect_steps = 0;
            for wire in &self.wires {
                match wire.steps(&intersect) {
                    Some(steps) => intersect_steps += steps,
                    None => continue,
                }
            }

            if intersect_steps < steps {
                steps = intersect_steps;
            }
        }

        match steps {
            std::u32::MAX => None,
            v => Some(v)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Movement {
    fn parse(mov: &str) -> Result<Movement, String> {
        if mov.len() < 2 {
            return Err(format!("invalid movement length"));
        }

        let amount = match mov[1..].parse::<u32>() {
            Ok(v) => v,
            Err(e) => return Err(String::from(e.description())),
        };

        let dir = mov.to_uppercase().as_bytes()[0] as char;
        match dir {
            'U' => Ok(Movement::Up(amount)),
            'D' => Ok(Movement::Down(amount)),
            'L' => Ok(Movement::Left(amount)),
            'R' => Ok(Movement::Right(amount)),
            _ => return Err(format!("unknown movement direction '{}'", dir))
        }
    }

    pub fn find_path(&self, start: &Coordinate) -> Vec<Coordinate> {
        match self {
            Movement::Up(v) => (start.y..=start.y + *v as i32)
                .map(|y| Coordinate { x: start.x, y })
                .collect(),
            Movement::Down(v) => (start.y - *v as i32..=start.y).rev()
                .map(|y| { Coordinate { x: start.x, y } })
                .collect(),
            Movement::Left(v) => (start.x - *v as i32..=start.x).rev()
                .map(|x| Coordinate { x, y: start.y })
                .collect(),
            Movement::Right(v) => (start.x..=start.x + *v as i32)
                .map(|x| Coordinate { x, y: start.y })
                .collect(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Wire {
    moves: Vec<Movement>,
    coords: Vec<Coordinate>,
    hash_coords: HashSet<Coordinate>,
}

impl Wire {
    pub fn parse(path: &str) -> Result<Wire, String> {
        let mut moves = Vec::new();
        for raw_move in path.split(',') {
            if raw_move.trim().len() == 0 {
                continue;
            }
            moves.push(Movement::parse(raw_move)?);
        }

        let mut coords = Vec::new();
        let mut cur_coord = COORD_ORIGIN;
        for mov in &moves {
            let mut path = mov.find_path(&cur_coord);

            // Drop the first coord because it's the origin or already included
            path = path[1..].to_vec();

            match path.last() {
                Some(last) => cur_coord = last.clone(),
                None => continue,
            }

            coords.append(&mut path);
        }

        Ok(Wire {
            moves,
            hash_coords: HashSet::from_iter(coords.iter().cloned()),
            coords,
        })
    }

    pub fn steps(&self, coord: &Coordinate) -> Option<u32> {
        for (idx, test_coord) in self.coords.iter().enumerate() {
            if coord == test_coord {
                return Some(idx as u32 + 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    mod coordinate {
        use super::super::*;

        #[test]
        fn distance() {
            assert_eq!(6, COORD_ORIGIN.distance(&Coordinate { x: 3, y: 3 }));
            assert_eq!(9, COORD_ORIGIN.distance(&Coordinate { x: 3, y: 6 }));
            assert_eq!(9, COORD_ORIGIN.distance(&Coordinate { x: -3, y: 6 }));
        }
    }

    mod movement {
        use super::super::*;

        #[test]
        fn parse_up() {
            assert_eq!(Ok(Movement::Up(7)), Movement::parse("U7"));
            assert_eq!(Ok(Movement::Up(7)), Movement::parse("u7"));
            assert_eq!(Ok(Movement::Up(14)), Movement::parse("U14"));
        }

        #[test]
        fn parse_down() {
            assert_eq!(Ok(Movement::Down(7)), Movement::parse("D7"));
            assert_eq!(Ok(Movement::Down(7)), Movement::parse("d7"));
            assert_eq!(Ok(Movement::Down(14)), Movement::parse("D14"));
        }

        #[test]
        fn parse_left() {
            assert_eq!(Ok(Movement::Left(7)), Movement::parse("L7"));
            assert_eq!(Ok(Movement::Left(7)), Movement::parse("l7"));
            assert_eq!(Ok(Movement::Left(14)), Movement::parse("L14"));
        }

        #[test]
        fn parse_right() {
            assert_eq!(Ok(Movement::Right(7)), Movement::parse("R7"));
            assert_eq!(Ok(Movement::Right(7)), Movement::parse("r7"));
            assert_eq!(Ok(Movement::Right(14)), Movement::parse("R14"));
        }

        #[test]
        fn find_path_up() {
            assert_eq!(
                vec![
                    Coordinate { x: 0, y: 2 },
                    Coordinate { x: 0, y: 3 },
                    Coordinate { x: 0, y: 4 },
                    Coordinate { x: 0, y: 5 },
                ],
                Movement::Up(3).find_path(&Coordinate { x: 0, y: 2 }),
            )
        }

        #[test]
        fn find_path_down() {
            assert_eq!(
                vec![
                    Coordinate { x: 0, y: 2 },
                    Coordinate { x: 0, y: 1 },
                    Coordinate { x: 0, y: 0 },
                    Coordinate { x: 0, y: -1 },
                ],
                Movement::Down(3).find_path(&Coordinate { x: 0, y: 2 }),
            )
        }

        #[test]
        fn find_path_left() {
            assert_eq!(
                vec![
                    Coordinate { x: 2, y: 0 },
                    Coordinate { x: 1, y: 0 },
                    Coordinate { x: 0, y: 0 },
                    Coordinate { x: -1, y: 0 },
                ],
                Movement::Left(3).find_path(&Coordinate { x: 2, y: 0 }),
            )
        }

        #[test]
        fn find_path_right() {
            assert_eq!(
                vec![
                    Coordinate { x: 2, y: 0 },
                    Coordinate { x: 3, y: 0 },
                    Coordinate { x: 4, y: 0 },
                    Coordinate { x: 5, y: 0 },
                ],
                Movement::Right(3).find_path(&Coordinate { x: 2, y: 0 }),
            )
        }
    }

    mod wire {
        use super::super::*;

        #[test]
        fn parse_one() {
            assert_eq!(
                Ok(Wire {
                    moves: vec![Movement::Up(7)],
                    coords: vec![
                        Coordinate { x: 0, y: 1 },
                        Coordinate { x: 0, y: 2 },
                        Coordinate { x: 0, y: 3 },
                        Coordinate { x: 0, y: 4 },
                        Coordinate { x: 0, y: 5 },
                        Coordinate { x: 0, y: 6 },
                        Coordinate { x: 0, y: 7 },
                    ],
                    hash_coords: vec![
                        Coordinate { x: 0, y: 1 },
                        Coordinate { x: 0, y: 2 },
                        Coordinate { x: 0, y: 3 },
                        Coordinate { x: 0, y: 4 },
                        Coordinate { x: 0, y: 5 },
                        Coordinate { x: 0, y: 6 },
                        Coordinate { x: 0, y: 7 },
                    ].into_iter().collect(),
                }),
                Wire::parse("U7"),
            )
        }

        #[test]
        fn parse_two() {
            assert_eq!(
                Ok(Wire {
                    moves: vec![Movement::Up(7), Movement::Right(2)],
                    coords: vec![
                        Coordinate { x: 0, y: 1 },
                        Coordinate { x: 0, y: 2 },
                        Coordinate { x: 0, y: 3 },
                        Coordinate { x: 0, y: 4 },
                        Coordinate { x: 0, y: 5 },
                        Coordinate { x: 0, y: 6 },
                        Coordinate { x: 0, y: 7 },
                        Coordinate { x: 1, y: 7 },
                        Coordinate { x: 2, y: 7 },
                    ],
                    hash_coords: vec![
                        Coordinate { x: 0, y: 1 },
                        Coordinate { x: 0, y: 2 },
                        Coordinate { x: 0, y: 3 },
                        Coordinate { x: 0, y: 4 },
                        Coordinate { x: 0, y: 5 },
                        Coordinate { x: 0, y: 6 },
                        Coordinate { x: 0, y: 7 },
                        Coordinate { x: 1, y: 7 },
                        Coordinate { x: 2, y: 7 },
                    ].into_iter().collect(),
                }),
                Wire::parse("u7,R2"),
            )
        }

        #[test]
        fn parse_ignore_empty() {
            assert_eq!(
                Ok(Wire {
                    moves: vec![Movement::Up(7), Movement::Right(2)],
                    coords: vec![
                        Coordinate { x: 0, y: 1 },
                        Coordinate { x: 0, y: 2 },
                        Coordinate { x: 0, y: 3 },
                        Coordinate { x: 0, y: 4 },
                        Coordinate { x: 0, y: 5 },
                        Coordinate { x: 0, y: 6 },
                        Coordinate { x: 0, y: 7 },
                        Coordinate { x: 1, y: 7 },
                        Coordinate { x: 2, y: 7 },
                    ],
                    hash_coords: vec![
                        Coordinate { x: 0, y: 1 },
                        Coordinate { x: 0, y: 2 },
                        Coordinate { x: 0, y: 3 },
                        Coordinate { x: 0, y: 4 },
                        Coordinate { x: 0, y: 5 },
                        Coordinate { x: 0, y: 6 },
                        Coordinate { x: 0, y: 7 },
                        Coordinate { x: 1, y: 7 },
                        Coordinate { x: 2, y: 7 },
                    ].into_iter().collect(),
                }),
                Wire::parse("u7,,,R2"),
            )
        }

        #[test]
        fn steps() {
            assert_eq!(
                Some(5),
                Wire::parse("R3,U3,L3")
                    .unwrap()
                    .steps(&Coordinate { x: 3, y: 2 }),
            )
        }

        #[test]
        fn steps_coord_not_found() {
            assert_eq!(
                None,
                Wire::parse("R3,U3,L3")
                    .unwrap()
                    .steps(&Coordinate { x: 1, y: 1 }),
            )
        }
    }

    mod grid {
        use super::super::*;

        #[test]
        fn add_wire() {
            let mut g = Grid::new();
            g.add_wire(Wire::parse("U1,R1").unwrap());

            assert_eq!(
                vec![
                    Wire {
                        moves: vec![Movement::Up(1), Movement::Right(1)],
                        coords: vec![
                            Coordinate { x: 0, y: 1 },
                            Coordinate { x: 1, y: 1 },
                        ],
                        hash_coords: vec![
                            Coordinate { x: 0, y: 1 },
                            Coordinate { x: 1, y: 1 },
                        ].into_iter().collect(),
                    },
                ],
                g.wires,
            )
        }

        #[test]
        fn intersection_one() {
            let mut g = Grid::new();
            g.add_wire(Wire::parse("U3,R3,D3").unwrap());
            assert_eq!(
                Vec::<Coordinate>::new(),
                g.intersections(),
            )
        }

        #[test]
        fn intersection_two() {
            let mut g = Grid::new();
            g.add_wire(Wire::parse("U3,R3,D3").unwrap());
            g.add_wire(Wire::parse("D1,R2,U4").unwrap());
            assert_eq!(
                vec![Coordinate { x: 2, y: 3 }],
                g.intersections(),
            )
        }

        #[test]
        fn intersection_example() {
            let mut g = Grid::new();
            g.add_wire(Wire::parse("R8,U5,L5,D3").unwrap());
            g.add_wire(Wire::parse("U7,R6,D4,L4").unwrap());
            assert_eq!(
                vec![
                    Coordinate { x: 3, y: 3 },
                    Coordinate { x: 6, y: 5 },
                ],
                g.intersections(),
            )
        }

        #[test]
        fn closest_distance_no_wire() {
            let g = Grid::new();
            assert_eq!(None, g.closest_distance())
        }

        #[test]
        fn closest_distance_one_wire() {
            let mut g = Grid::new();
            g.add_wire(Wire::parse("R8,U5,L5,D3").unwrap());
            assert_eq!(None, g.closest_distance())
        }

        #[test]
        fn closest_distance_example1() {
            let mut g = Grid::new();
            g.add_wire(Wire::parse("R8,U5,L5,D3").unwrap());
            g.add_wire(Wire::parse("U7,R6,D4,L4").unwrap());
            assert_eq!(Some(6), g.closest_distance())
        }

        #[test]
        fn closest_distance_example2() {
            let mut g = Grid::new();
            g.add_wire(Wire::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap());
            g.add_wire(Wire::parse("U62,R66,U55,R34,D71,R55,D58,R83").unwrap());
            assert_eq!(Some(159), g.closest_distance())
        }

        #[test]
        fn closest_distance_example3() {
            let mut g = Grid::new();
            g.add_wire(Wire::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap());
            g.add_wire(Wire::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap());
            assert_eq!(Some(135), g.closest_distance())
        }

        #[test]
        fn shortest_steps_example1() {
           let mut g = Grid::new();
            g.add_wire(Wire::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap());
            g.add_wire(Wire::parse("U62,R66,U55,R34,D71,R55,D58,R83").unwrap());
            assert_eq!(Some(610), g.shortest_steps())
        }

        #[test]
        fn shortest_steps_example2() {
            let mut g = Grid::new();
            g.add_wire(Wire::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap());
            g.add_wire(Wire::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap());
            assert_eq!(Some(410), g.shortest_steps())
        }
    }
}