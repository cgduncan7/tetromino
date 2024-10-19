mod algo_x;
mod backtracking;
mod graph;
mod linked_list;
mod puzzle;
use std::{
    cell::RefCell,
    fs::File,
    io::{BufWriter, Write},
    rc::Rc,
    time,
};

use ::time::{format_description, OffsetDateTime};
use backtracking::{Solver, SolverOpts};
use graph::{Matrix, MatrixControlNode, MatrixNode};
use puzzle::{
    make_l_shaped_piece, make_rectangle_piece, make_s_shaped_piece, make_square_piece,
    make_t_shaped_piece, Piece, Puzzle,
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
    let mut matrix = Matrix::new();

    matrix.add_node(MatrixNode::new((1, 1)));
    matrix.add_node(MatrixNode::new((4, 1)));
    matrix.add_node(MatrixNode::new((7, 1)));

    matrix.add_node(MatrixNode::new((1, 2)));
    matrix.add_node(MatrixNode::new((4, 2)));

    matrix.add_node(MatrixNode::new((4, 3)));
    matrix.add_node(MatrixNode::new((5, 3)));
    matrix.add_node(MatrixNode::new((7, 3)));

    matrix.add_node(MatrixNode::new((3, 4)));
    matrix.add_node(MatrixNode::new((5, 4)));
    matrix.add_node(MatrixNode::new((6, 4)));

    matrix.add_node(MatrixNode::new((2, 5)));
    matrix.add_node(MatrixNode::new((3, 5)));
    matrix.add_node(MatrixNode::new((6, 5)));
    matrix.add_node(MatrixNode::new((7, 5)));

    matrix.add_node(MatrixNode::new((2, 6)));
    matrix.add_node(MatrixNode::new((7, 6)));

    let current_sparsest_column = matrix.find_sparsest_column();
    if let Some(current_sparsest_column) = current_sparsest_column {
        println!(
            "Solving column {} with rows {} ",
            current_sparsest_column.borrow().column,
            current_sparsest_column.borrow().num_rows
        );
        matrix.remove_columns(current_sparsest_column.borrow().column)
    } else {
        panic!("no columns left")
    }
}

fn main() {
    // let debug_mode = env::args().any(|arg| arg.starts_with("-d") || arg.starts_with("--debug"));
    // solve_using_backtracking(debug_mode);
    solve_using_dlx(false);
}
