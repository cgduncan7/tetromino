mod backtracking;
mod puzzle;
use core::time;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use backtracking::{Solver, SolverOpts};
use puzzle::{
    make_l_shaped_piece, make_rectangle_piece, make_s_shaped_piece, make_square_piece,
    make_t_shaped_piece, Piece, Puzzle,
};

fn main() {
    let pieces: Vec<Piece> = vec![
        make_l_shaped_piece(),
        make_l_shaped_piece(),
        make_t_shaped_piece(),
        make_t_shaped_piece(),
        make_square_piece(),
        make_square_piece(),
        make_s_shaped_piece(),
        make_s_shaped_piece(),
        make_rectangle_piece(),
        make_rectangle_piece(),
    ];

    let puzzle = Puzzle::new(8, 5, pieces);

    let mut solver = Solver::new(
        SolverOpts {
            verbose: true,
            delay: Some(time::Duration::from_millis(100)),
        },
        puzzle,
    );
    solver.solve(None);
    println!("Number of solutions: {}", solver.solutions.len());

    let f = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(f);
    for (_, puzzle) in solver.solutions {
        writer.write(puzzle.to_string().as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
}
