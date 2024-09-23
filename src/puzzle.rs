use std::{collections::HashMap, fmt::Display, hash::Hash, ops::Add};

/**
 * (0,0) - top-left
 * (width,height) - bottom-right
 */
pub struct Puzzle {
    pub width: u8,
    pub height: u8,
    pub pieces: HashMap<Option<Placement>, Piece>,
}

impl Puzzle {
    pub fn new(width: u8, height: u8, available_pieces: Vec<Piece>) -> Self {
        let mut pieces_map: HashMap<Option<Placement>, Piece> = HashMap::new();
        for available_piece in available_pieces {
            pieces_map.insert(None, available_piece);
        }
        Self {
            width,
            height,
            pieces: pieces_map,
        }
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
                "Location: {}\tOrientation: {})",
                self.location, self.orientation
            )
            .as_str(),
        )
    }
}

#[derive(Debug)]
pub struct OccupiedSpace {
    pub placement: Placement,
    pub locations: Vec<Location>,
}

impl Display for OccupiedSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let locations_str = self.locations.iter().fold(String::new(), |acc, location| {
            acc + "\n\t- " + &location.to_string()
        });
        f.write_str(
            format!(
                "Placement: {}\nLocations:\n {}",
                self.placement, locations_str
            )
            .as_str(),
        )
    }
}

#[derive(Clone)]
pub struct Piece {
    pub orientations: Vec<Orientation>,
    pub locations: Vec<Location>,
}

impl Piece {
    fn new(orientations: Vec<Orientation>, locations: Vec<Location>) -> Piece {
        Piece {
            orientations,
            locations,
        }
    }

    pub fn get_potentially_occupied_space(&self, placement: Placement) -> OccupiedSpace {
        let locations = self
            .locations
            .iter()
            .map(|location| location.orient(placement.orientation) + placement.location)
            .collect();

        OccupiedSpace {
            placement,
            locations,
        }
    }

    pub fn get_potentially_occupied_spaces(&self, location: Location) -> Vec<OccupiedSpace> {
        self.orientations
            .iter()
            .map(|orientation| {
                self.get_potentially_occupied_space(Placement {
                    location,
                    orientation: *orientation,
                })
            })
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
        vec![
            Orientation::Up(false),
            Orientation::Up(true),
            Orientation::Right(false),
            Orientation::Right(true),
            Orientation::Down(false),
            Orientation::Down(true),
            Orientation::Left(false),
            Orientation::Left(true),
        ],
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
        vec![
            Orientation::Up(false),
            Orientation::Up(true),
            Orientation::Right(false),
            Orientation::Left(false),
        ],
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
        vec![Orientation::Up(false)],
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
        vec![
            Orientation::Up(false),
            Orientation::Up(true),
            Orientation::Right(false),
            Orientation::Right(true),
            Orientation::Down(false),
            Orientation::Down(true),
            Orientation::Left(false),
            Orientation::Left(true),
        ],
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
        vec![Orientation::Up(false), Orientation::Right(false)],
        vec![
            Location { x: 0, y: 0 },
            Location { x: 0, y: 1 },
            Location { x: 0, y: 2 },
            Location { x: 0, y: 3 },
        ],
    )
}
