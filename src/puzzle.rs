use std::{
    collections::HashSet,
    fmt::Display,
    hash::Hash,
    ops::{Add, Sub},
};

use crate::backtracking::Backtrackable;

#[derive(Clone, Debug, Eq, PartialOrd, Ord)]
pub struct PuzzleHash {
    pub forwards: String,
    pub backwards: String,
}

impl PuzzleHash {
    pub fn new(forwards: String) -> Self {
        let backwards = forwards.chars().rev().collect::<String>();
        Self {
            forwards,
            backwards,
        }
    }
}

impl PartialEq for PuzzleHash {
    fn eq(&self, other: &Self) -> bool {
        // to remove symmetric answers (if I rotate the puzzle 180deg)
        self.forwards == other.forwards || self.backwards == other.forwards
    }
}

/**
 * (0,0) - top-left
 * (width,height) - bottom-right
 */
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Puzzle {
    pub height: u8,
    pub width: u8,
    pub pieces: Vec<Piece>,
    pub spaces: Vec<Option<usize>>,
    pub hash: PuzzleHash,
}

impl Puzzle {
    pub fn new(height: u8, width: u8, pieces: Vec<Piece>) -> Self {
        let mut spaces = Vec::new();
        for _ in 0..height * width {
            spaces.push(None);
        }
        Self {
            height,
            width,
            pieces,
            spaces,
            hash: PuzzleHash::new(String::from("")),
        }
    }

    fn valid_piece_placement(&self, origin: Location, placement: Placement, piece: &Piece) -> bool {
        let locations = piece.get_potentially_occupied_locations(origin, placement);

        !locations.iter().any(|loc| {
            let idx = loc.to_index(self.width);
            self.spaces.get(idx as usize).unwrap_or(&None).is_some()
                || loc.x < 0
                || loc.x >= self.width as i8
                || loc.y < 0
                || loc.y >= self.height as i8
        })
    }

    pub fn place_piece(
        &mut self,
        origin: Location,
        placement: Placement,
        piece_index: usize,
    ) -> Result<(), ()> {
        let piece = self.pieces.get(piece_index).unwrap();
        if !self.valid_piece_placement(origin, placement, piece) {
            return Err(());
        }

        let piece = self.pieces.get_mut(piece_index).unwrap();
        piece.placement = Some((origin, placement));
        piece
            .get_occupied_locations()
            .iter()
            .map(|l| l.to_index(self.width))
            .for_each(|idx| self.spaces[idx] = Some(piece_index));
        self.hash = self.get_puzzle_hash();
        Ok(())
    }

    fn get_next_empty_space(&self) -> Option<usize> {
        for idx in 0..(self.width * self.height) as usize {
            if self.spaces.get(idx as usize).unwrap().is_none() {
                return Some(idx);
            }
        }
        None
    }

    pub fn get_puzzle_hash(&self) -> PuzzleHash {
        let mut acc = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = usize::try_from(x + self.width * y).unwrap();
                let space = self.spaces.get(idx).unwrap();
                match space {
                    None => acc.push('-'),
                    Some(s) => acc.push(self.pieces.get(*s).unwrap().shape),
                };
            }
        }

        PuzzleHash::new(acc)
    }
}

impl Backtrackable<Puzzle> for Puzzle {
    fn get_next_candidates(&self) -> Vec<Puzzle> {
        let unplaced_pieces = self
            .pieces
            .iter()
            .enumerate()
            .filter(|(_, pp)| pp.placement == None)
            .collect::<Vec<(usize, &Piece)>>();

        let empty_space_idx = self.get_next_empty_space();

        if empty_space_idx.is_none() {
            return vec![];
        }

        let mut candidates: Vec<Puzzle> = Vec::new();

        for (idx, unplaced_piece) in unplaced_pieces {
            for origin in unplaced_piece.locations.iter() {
                for orientation in get_all_orientations().iter() {
                    let mut next_candidate = self.clone();
                    let placement = Placement {
                        location: Location::from_index(self.width, empty_space_idx.unwrap()),
                        orientation: *orientation,
                    };
                    if next_candidate.place_piece(*origin, placement, idx).is_ok() {
                        candidates.push(next_candidate);
                    }
                }
            }
        }

        return candidates;
    }

    fn is_solution(&self) -> bool {
        self.get_next_empty_space().is_none()
    }

    fn insert_explorations(&self, hash_set: &mut HashSet<String>) {
        hash_set.insert(self.hash.forwards.clone());
        hash_set.insert(self.hash.backwards.clone());
    }

    fn is_candidate_explored(&self, hash_set: &HashSet<String>) -> bool {
        hash_set.contains(&self.hash.forwards) || hash_set.contains(&self.hash.backwards)
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.width * y + x;
                let ch = match self.spaces.get(idx as usize).unwrap() {
                    None => '-',
                    Some(c) => char::from_digit(*c as u32, 10).unwrap(),
                };
                ret.push(ch);
            }
            ret.push('\n');
        }
        f.write_str(ret.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Up(bool),    // 0deg rotation + horizontal flip flag
    Right(bool), // 90deg rotation + horizontal flip flag
    Down(bool),  // 180deg rotation + horizontal flip flag
    Left(bool),  // 270deg rotation + horizontal flip flag
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let get_flip_str = |flip: &bool| {
            if *flip {
                "flipped"
            } else {
                "normal"
            }
        };
        match self {
            Orientation::Up(flip) => f.write_str(format!("Up {}", get_flip_str(flip)).as_str()),
            Orientation::Right(flip) => {
                f.write_str(format!("Right {}", get_flip_str(flip)).as_str())
            }
            Orientation::Down(flip) => f.write_str(format!("Down {}", get_flip_str(flip)).as_str()),
            Orientation::Left(flip) => f.write_str(format!("Left {}", get_flip_str(flip)).as_str()),
        }
    }
}

fn get_all_orientations() -> Vec<Orientation> {
    vec![
        Orientation::Up(false),
        Orientation::Up(true),
        Orientation::Right(false),
        Orientation::Right(true),
        Orientation::Down(false),
        Orientation::Down(true),
        Orientation::Left(false),
        Orientation::Left(true),
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Location {
    pub x: i8,
    pub y: i8,
}

impl Location {
    fn orient(&self, orientation: Orientation) -> Location {
        match orientation {
            Orientation::Up(false) => *self,
            Orientation::Up(true) => Location {
                x: -self.x,
                y: self.y,
            },
            Orientation::Right(false) => Location {
                x: -self.y,
                y: self.x,
            },
            Orientation::Right(true) => Location {
                x: self.y,
                y: self.x,
            },
            Orientation::Down(false) => Location {
                x: -self.x,
                y: -self.y,
            },
            Orientation::Down(true) => Location {
                x: self.x,
                y: -self.y,
            },
            Orientation::Left(false) => Location {
                x: self.y,
                y: -self.x,
            },
            Orientation::Left(true) => Location {
                x: -self.y,
                y: -self.x,
            },
        }
    }

    fn from_index(width: u8, idx: usize) -> Location {
        Location {
            x: (idx as u8 % width) as i8,
            y: (idx as u8 / width) as i8,
        }
    }

    fn to_index(&self, width: u8) -> usize {
        (self.y * width as i8 + self.x) as usize
    }
}

impl Add for Location {
    type Output = Location;

    fn add(self, rhs: Self) -> Self::Output {
        Location {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Location {
    type Output = Location;

    fn sub(self, rhs: Self) -> Self::Output {
        Location {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("(x: {}, y: {})", self.x, self.y).as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Placement {
    pub location: Location,
    pub orientation: Orientation,
}

impl Display for Placement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "Location: {} + Orientation: {}",
                self.location, self.orientation
            )
            .as_str(),
        )
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Piece {
    pub shape: char,
    pub locations: Vec<Location>,
    pub placement: Option<(Location, Placement)>,
}

impl Piece {
    fn new(shape: char, locations: Vec<Location>) -> Piece {
        Piece {
            shape,
            locations,
            placement: None,
        }
    }

    pub fn get_occupied_locations(&self) -> Vec<Location> {
        match self.placement {
            None => vec![],
            Some((origin, placement)) => self
                .locations
                .iter()
                .map(|l| *l - origin)
                .map(|l| l.orient(placement.orientation) + placement.location)
                .collect(),
        }
    }

    pub fn get_potentially_occupied_locations(
        &self,
        origin: Location,
        placement: Placement,
    ) -> Vec<Location> {
        self.locations
            .iter()
            .map(|l| *l - origin)
            .map(|l| l.orient(placement.orientation) + placement.location)
            .collect()
    }
}

/**
 * U  | Uf | L   | Lf  | D  | Df | R   | Rf
 * X  |  X |     |     | XX | XX | XXX | XXX
 * X  |  X |   X | X   |  X | X  | X   |   X
 * XX | XX | XXX | XXX |  X | X  |     |
 */
pub fn make_l_shaped_piece() -> Piece {
    Piece::new(
        'L',
        vec![
            Location { x: 0, y: 0 },
            Location { x: 0, y: 1 },
            Location { x: 0, y: 2 },
            Location { x: 1, y: 2 },
        ],
    )
}

/**
 * X
 * XX
 * X
 */
pub fn make_t_shaped_piece() -> Piece {
    Piece::new(
        'T',
        vec![
            Location { x: 0, y: 0 },
            Location { x: 0, y: 1 },
            Location { x: 1, y: 1 },
            Location { x: 0, y: 2 },
        ],
    )
}

/**
 * XX
 * XX
 */
pub fn make_square_piece() -> Piece {
    Piece::new(
        'Q',
        vec![
            Location { x: 0, y: 0 },
            Location { x: 1, y: 0 },
            Location { x: 0, y: 1 },
            Location { x: 1, y: 1 },
        ],
    )
}

/**
 * X
 * XX
 *  X
 */
pub fn make_s_shaped_piece() -> Piece {
    Piece::new(
        'S',
        vec![
            Location { x: 0, y: 0 },
            Location { x: 0, y: 1 },
            Location { x: 1, y: 1 },
            Location { x: 1, y: 2 },
        ],
    )
}

/**
 * X
 * X
 * X
 * X
 */
pub fn make_rectangle_piece() -> Piece {
    Piece::new(
        'I',
        vec![
            Location { x: 0, y: 0 },
            Location { x: 0, y: 1 },
            Location { x: 0, y: 2 },
            Location { x: 0, y: 3 },
        ],
    )
}
