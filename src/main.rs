mod algo_x;
mod backtracking;
mod puzzle;
use std::{
    fs::File,
    io::{BufWriter, Write},
    time,
};

use ::time::{format_description, OffsetDateTime};
use algo_x::Matrix;
use backtracking::{Puzzle, Solver, SolverOpts};
use puzzle::{
    make_l_shaped_piece, make_rectangle_piece, make_s_shaped_piece, make_square_piece,
    make_t_shaped_piece, Piece,
};

fn solve_using_backtracking(debug_mode: bool) {
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

    let mut backtracking_solver = Solver::new(SolverOpts {
        verbose: debug_mode,
        delay: if debug_mode {
            Some(time::Duration::from_millis(100))
        } else {
            None
        },
    });
    backtracking_solver.solve(&mut puzzle);
    println!(
        "Number of solutions: {}",
        backtracking_solver.solutions.len()
    );

    let odt: OffsetDateTime = time::SystemTime::now().into();
    let output_filename = format!(
        "output_{}.txt",
        odt.format(&format_description::parse("[year][month][day][hour][minute][second]").unwrap())
            .unwrap()
    );
    let f = File::create(output_filename).unwrap();
    let mut writer = BufWriter::new(f);
    for puzzle in backtracking_solver.solutions {
        writer.write(format!("{}", puzzle).as_bytes()).unwrap();
        writer.write("\n".as_bytes()).unwrap();
    }
}

fn solve_using_dlx(debug_mode: bool) {
    // let nodes = vec![
    //     (1, 1),
    //     (4, 1),
    //     (7, 1),
    //     (1, 2),
    //     (4, 2),
    //     (4, 3),
    //     (5, 3),
    //     (7, 3),
    //     (3, 4),
    //     (5, 4),
    //     (6, 4),
    //     (2, 5),
    //     (3, 5),
    //     (6, 5),
    //     (7, 5),
    //     (2, 6),
    //     (7, 6),
    // ];

    let pieces: Vec<Piece> = vec![
        // make_l_shaped_piece(),
        // make_l_shaped_piece(),
        // make_t_shaped_piece(),
        // make_t_shaped_piece(),
        make_square_piece(),
        // make_square_piece(),
        // make_s_shaped_piece(),
        // make_s_shaped_piece(),
        // make_rectangle_piece(),
        // make_rectangle_piece(),
    ];
    let mut row: u32 = 0;
    let mut nodes: Vec<(u32, u32)> = vec![];
    for piece in pieces {
        piece
            .get_all_potentially_occupied_locations(4, 2)
            .iter()
            .for_each(|l| {
                l.iter().for_each(|ll| {
                    println!("{:?}", l);
                    let idx = ll.to_index(3) as u32;
                    nodes.push((idx, row));
                });
                row += 1;
            });
    }
    println!("{:?}", nodes);

    let mut matrix = Matrix::new(nodes);
    matrix.solve();
    println!("{:?}", matrix.solution_rows);
}

fn main() {
    // let debug_mode = env::args().any(|arg| arg.starts_with("-d") || arg.starts_with("--debug"));
    // solve_using_backtracking(debug_mode);
    solve_using_dlx(false);
}
