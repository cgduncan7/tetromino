mod backtracking;
mod puzzle;
use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    time,
};

use ::time::{format_description, OffsetDateTime};
use backtracking::{Solver, SolverOpts};
use puzzle::{
    make_l_shaped_piece, make_rectangle_piece, make_s_shaped_piece, make_square_piece,
    make_t_shaped_piece, Piece, Puzzle,
};

fn main() {
    let debug_mode = env::args().any(|arg| arg.starts_with("-d") || arg.starts_with("--debug"));

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

    let mut puzzle = Puzzle::new(5, 8, pieces.clone());

    let mut solver = Solver::new(SolverOpts {
        verbose: debug_mode,
        delay: if debug_mode {
            Some(time::Duration::from_millis(100))
        } else {
            None
        },
    });
    solver.solve(&mut puzzle);
    println!("Number of solutions: {}", solver.solutions.len());

    let odt: OffsetDateTime = time::SystemTime::now().into();
    let output_filename = format!(
        "output_{}.txt",
        odt.format(&format_description::parse("[year][month][day][hour][minute][second]").unwrap())
            .unwrap()
    );
    let f = File::create(output_filename).unwrap();
    let mut writer = BufWriter::new(f);
    for puzzle in solver.solutions {
        writer.write(format!("{}", puzzle).as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
}
