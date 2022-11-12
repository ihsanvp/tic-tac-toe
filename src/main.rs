mod model;

fn main() {
    let board = model::board::Board::new();
    let option_board = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    model::board::display_board(&option_board);
    board.display();

    println!("{:?}", board.get_remaining_postions())
}
