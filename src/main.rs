mod model;

fn main() {
    let board = model::board::Board::new();

    board.display();
}
