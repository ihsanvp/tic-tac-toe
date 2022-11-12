#[derive(Debug, PartialEq)]
enum CellValue {
    Empty,
    Cross,
    Circle,
}

#[derive(Debug)]
pub struct Board {
    cells: Vec<Vec<BoardCell>>,
}

#[derive(Debug)]
struct BoardCell {
    value: CellValue,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cells: vec![
                vec![
                    BoardCell::new(CellValue::Empty),
                    BoardCell::new(CellValue::Empty),
                    BoardCell::new(CellValue::Empty),
                ],
                vec![
                    BoardCell::new(CellValue::Empty),
                    BoardCell::new(CellValue::Empty),
                    BoardCell::new(CellValue::Empty),
                ],
                vec![
                    BoardCell::new(CellValue::Empty),
                    BoardCell::new(CellValue::Empty),
                    BoardCell::new(CellValue::Empty),
                ],
            ],
        }
    }

    pub fn display(&self) {
        let board: Vec<Vec<char>> = self
            .cells
            .iter()
            .map(|row| row.iter().map(|c| c.get_char()).collect())
            .collect();

        display_board(&board);
    }

    pub fn get_remaining_postions(&self) -> Vec<u32> {
        let mut remaining: Vec<u32> = vec![];

        for i in 0..self.cells.len() {
            let row = &self.cells[i];

            for j in 0..row.len() {
                let cell = &row[j];

                if cell.is_empty() {
                    remaining.push(((i * 3) + j + 1) as u32);
                }
            }
        }

        remaining
    }
}

impl BoardCell {
    fn new(value: CellValue) -> BoardCell {
        BoardCell { value: value }
    }

    fn set_value(&mut self, value: CellValue) {
        self.value = value;
    }

    fn display(&self) {
        match self.value {
            CellValue::Empty => print!(" "),
            CellValue::Cross => print!("X"),
            CellValue::Circle => print!("O"),
        }
    }

    fn get_char(&self) -> char {
        return match self.value {
            CellValue::Empty => ' ',
            CellValue::Cross => 'X',
            CellValue::Circle => 'O',
        };
    }

    fn is_empty(&self) -> bool {
        return self.value == CellValue::Empty;
    }
}

pub fn display_board(board: &Vec<Vec<char>>) {
    for i in 0..board.len() {
        let row = &board[i];

        for j in 0..row.len() {
            let c = &row[j];

            print!("{c}");

            if j <= row.len() - 2 {
                print!("|");
            }
        }

        if i <= row.len() - 2 {
            println!("\n-----");
        }
    }
    println!("");
    println!("");
}
