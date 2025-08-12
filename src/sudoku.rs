pub struct Sudoku {
    board: [[i32; 9]; 9],
}

impl Sudoku {
    pub fn default() -> Sudoku {
        Sudoku { board: [[0; 9]; 9] }
    }

    pub fn is_solved(&self) -> bool {
        return true;
    }

    pub fn print(self) {
        println!("Solved? {}", self.is_solved());
        for (i, row) in self.board.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                println!("-----------");
            }
            for (j, cell) in row.iter().enumerate() {
                if j > 0 && j % 3 == 0 {
                    print!("|");
                }
                print!("{}", cell);
            }
            println!();
        }
        println!();
    }
}
