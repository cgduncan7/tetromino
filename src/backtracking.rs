use core::time;
use std::{collections::HashSet, fmt::Display, thread::sleep};

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
