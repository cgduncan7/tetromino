use std::{
    fmt::Display,
    ops::{Add, Sub},
};

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

pub fn get_all_orientations() -> Vec<Orientation> {
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
    pub fn orient(&self, orientation: Orientation) -> Location {
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

    pub fn from_index(width: u8, idx: usize) -> Location {
        Location {
            x: (idx as u8 % width) as i8,
            y: (idx as u8 / width) as i8,
        }
    }

    pub fn to_index(&self, width: u8) -> usize {
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

    pub fn get_all_potentially_occupied_locations(
        &self,
        width: usize,
        height: usize,
    ) -> Vec<Vec<Location>> {
        let mut all_potentially_occupied_locations: Vec<Vec<Location>> = vec![];
        // translate this to all locs
        for lc in self.locations.iter() {
            for or in get_all_orientations() {
                for h in 0..height {
                    for w in 0..width {
                        let loc = Location {
                            x: w as i8,
                            y: h as i8,
                        };
                        let placement = Placement {
                            location: loc,
                            orientation: or,
                        };
                        let pol = self.get_potentially_occupied_locations(*lc, placement);
                        if !pol.iter().any(|llllllc| {
                            llllllc.x < 0
                                || llllllc.x >= width as i8
                                || llllllc.y < 0
                                || llllllc.y >= height as i8
                        }) {
                            all_potentially_occupied_locations.push(pol);
                        }
                    }
                }
            }
        }
        all_potentially_occupied_locations
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
