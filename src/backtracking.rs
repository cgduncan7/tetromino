use core::time;
use std::{collections::HashSet, fmt::Display, thread::sleep};

use crate::puzzle::{get_all_orientations, Location, Piece, Placement};

pub trait Backtrackable<T: Backtrackable<T> + Clone + Display + Eq + PartialEq> {
    fn get_next_candidates(&self) -> Vec<T>;
    fn is_solution(&self) -> bool;
    fn insert_explorations(&self, hash_set: &mut HashSet<String>);
    fn is_candidate_explored(&self, hash_set: &HashSet<String>) -> bool;
}

pub struct SolverOpts {
    pub verbose: bool,
    pub delay: Option<time::Duration>,
}

impl Default for SolverOpts {
    fn default() -> Self {
        Self {
            verbose: false,
            delay: None,
        }
    }
}

pub struct Solver<T: Backtrackable<T> + Clone + Display + Eq + PartialEq> {
    opts: SolverOpts,
    pub explored_candidates: HashSet<String>,
    pub solutions: Vec<T>,
}

impl<T: Backtrackable<T> + Clone + Display + Eq + PartialEq> Solver<T> {
    pub fn new(opts: SolverOpts) -> Self {
        Solver {
            opts,
            explored_candidates: HashSet::new(),
            solutions: Vec::new(),
        }
    }

    pub fn solve(&mut self, candidate: &mut T) {
        if let Some(dur) = self.opts.delay {
            sleep(dur);
        }

        if self.opts.verbose {
            print!("\x1B[2J\x1B[1;1H");
            println!("Explored candidates: {}", self.explored_candidates.len());
            println!("Current solutions: {}", self.solutions.len());
            println!("\n\nCurrent candidate:\n{}\n", candidate);
        }

        if candidate.is_candidate_explored(&self.explored_candidates) {
            if self.opts.verbose {
                println!("Has been explored");
            }
            return;
        }

        if candidate.is_solution() && !candidate.is_candidate_explored(&self.explored_candidates) {
            // solved-end
            if self.opts.verbose {
                println!("Solved!");
            }
            candidate.insert_explorations(&mut self.explored_candidates);
            self.solutions.push(candidate.to_owned());
            return;
        }

        let mut next_candidates = candidate.get_next_candidates();
        let mut unique_candidates = HashSet::new();
        let mut unexplored_candidates: Vec<&mut T> = Vec::new();
        next_candidates.iter_mut().for_each(|c| {
            if !c.is_candidate_explored(&self.explored_candidates)
                && !c.is_candidate_explored(&unique_candidates)
            {
                c.insert_explorations(&mut unique_candidates);
                unexplored_candidates.push(c);
            }
        });

        if unexplored_candidates.len() == 0 {
            // dead-end
            if self.opts.verbose {
                println!("No unexplored candidates left");
            }
        } else {
            for uc in unexplored_candidates.iter_mut() {
                self.solve(uc);
            }
        }
        candidate.insert_explorations(&mut self.explored_candidates);
    }
}

#[derive(Clone, Debug, Eq, PartialOrd, Ord)]
pub struct PuzzleHash {
    pub forwards: String,
    pub rotated_180: String,
    pub mirrored_horizontally: String,
    pub mirrored_vertically: String,
}

impl PuzzleHash {
    pub fn new(height: usize, width: usize, forwards: String) -> Self {
        let mut rotated_180 = String::new();
        let mut mirrored_horizontally = String::new();
        let mut mirrored_vertically = String::new();
        let chars = forwards.chars().collect::<Vec<char>>();
        for y in 0..height {
            for x in 0..width {
                let rotated_y = height - y - 1;
                let mirrored_h_y = height - y - 1;
                let mirrored_v_y = y;

                let rotated_x = width - x - 1;
                let mirrored_h_x = x;
                let mirrored_v_x = width - x - 1;

                let rotated_idx = rotated_y * width + rotated_x;
                let mirrored_h_idx = mirrored_h_y * width + mirrored_h_x;
                let mirrored_v_idx = mirrored_v_y * width + mirrored_v_x;

                let rotated_char = chars.get(rotated_idx).unwrap();
                let mirrored_h_char = chars.get(mirrored_h_idx).unwrap();
                let mirrored_v_char = chars.get(mirrored_v_idx).unwrap();

                rotated_180.push(*rotated_char);
                mirrored_horizontally.push(*mirrored_h_char);
                mirrored_vertically.push(*mirrored_v_char);
            }
        }

        Self {
            forwards,
            rotated_180,
            mirrored_horizontally,
            mirrored_vertically,
        }
    }
}

impl PartialEq for PuzzleHash {
    fn eq(&self, other: &Self) -> bool {
        self.forwards == other.forwards
            || self.rotated_180 == other.forwards
            || self.mirrored_horizontally == other.forwards
            || self.mirrored_vertically == other.forwards
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
            hash: PuzzleHash::new(
                usize::try_from(height).unwrap(),
                usize::try_from(width).unwrap(),
                String::from("-".repeat(usize::try_from(height * width).unwrap())),
            ),
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

        PuzzleHash::new(
            usize::try_from(self.height).unwrap(),
            usize::try_from(self.width).unwrap(),
            acc,
        )
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
        self.get_next_empty_space().is_none() && self.pieces.iter().all(|p| p.placement.is_some())
    }

    fn insert_explorations(&self, hash_set: &mut HashSet<String>) {
        hash_set.insert(self.hash.forwards.clone());
        hash_set.insert(self.hash.rotated_180.clone());
        hash_set.insert(self.hash.mirrored_horizontally.clone());
        hash_set.insert(self.hash.mirrored_vertically.clone());
    }

    fn is_candidate_explored(&self, hash_set: &HashSet<String>) -> bool {
        hash_set.contains(&self.hash.forwards)
            || hash_set.contains(&self.hash.rotated_180)
            || hash_set.contains(&self.hash.mirrored_horizontally)
            || hash_set.contains(&self.hash.mirrored_vertically)
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
                    Some(c) => self.pieces.get(*c).unwrap().shape,
                };
                ret.push(ch);
            }
            ret.push('\n');
        }
        f.write_str(ret.as_str())
    }
}
