mod puzzle;
use puzzle::{
    make_l_shaped_piece, make_rectangle_piece, make_s_shaped_piece, make_square_piece,
    make_t_shaped_piece, Location, Piece, Puzzle,
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

    for (_, piece) in puzzle.pieces {
        let occupied_spaces = piece.get_potentially_occupied_spaces(Location { x: 0, y: 0 });

        for occupied_space in occupied_spaces {
            println!("{}", occupied_space);
        }
    }
}
