use core::time;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    thread::sleep,
};

pub trait Backtrackable<T: Backtrackable<T> + Clone + Display + Eq + Hash + PartialEq> {
    fn get_root_candidate(&self) -> T;
    fn get_next_candidates(&self) -> Vec<T>;
    fn is_solution(&self) -> bool;
    fn get_solution_hash(&self) -> String;
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

pub struct Solver<T: Backtrackable<T> + Clone + Display + Eq + Hash + PartialEq> {
    opts: SolverOpts,
    pub root_candidate: T,
    pub explored_candidates: HashSet<String>,
    pub solutions: HashMap<String, T>,
}

impl<T: Backtrackable<T> + Clone + Display + Eq + Hash + PartialEq> Solver<T> {
    pub fn new(opts: SolverOpts, backtrackable: impl Backtrackable<T>) -> Self {
        let root_candidate = backtrackable.get_root_candidate();
        Solver {
            opts,
            root_candidate,
            explored_candidates: HashSet::new(),
            solutions: HashMap::new(),
        }
    }

    pub fn solve(&mut self, candidate_opt: Option<&mut T>) {
        let candidate: &mut T = candidate_opt.unwrap_or(&mut self.root_candidate);
        if let Some(dur) = self.opts.delay {
            sleep(dur);
        }

        if self.opts.verbose {
            print!("\x1B[2J\x1B[1;1H");
            println!("\n\nCurrent candidate:\n{}\n", candidate);
        }

        if self
            .explored_candidates
            .contains(&candidate.get_solution_hash())
        {
            if self.opts.verbose {
                println!("Has been explored");
            }
            return;
        }

        if candidate.is_solution() {
            // solved-end
            if self.opts.verbose {
                println!("Solved!");
            }
            let hash = candidate.get_solution_hash();
            self.explored_candidates.insert(hash.clone());
            self.solutions.insert(hash.clone(), candidate.clone());
            return;
        }

        let next_candidates = candidate.get_next_candidates();
        let mut unexplored_candidates: Vec<T> = next_candidates
            .iter()
            .filter(|c| !self.explored_candidates.contains(&c.get_solution_hash()))
            .map(|c| c.clone())
            .collect();

        if unexplored_candidates.len() == 0 {
            // dead-end
            if self.opts.verbose {
                println!("No unexplored candidates left");
            }
            self.explored_candidates
                .insert(candidate.get_solution_hash());
        } else {
            for uc in unexplored_candidates.iter_mut() {
                self.solve(Some(uc));
            }
        }
    }
}
