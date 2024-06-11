/*
Minesweeper Gaming Logic
*/

pub mod game_logic {
    use rand::Rng;
    use std::collections::HashSet;

    struct Minesweeper {
        height: u32,
        width: u32,
        mines: HashSet<(usize, usize)>,
        mines_found: HashSet<(usize, usize)>,
        board: [[bool; 8]; 8],
    }

    impl Minesweeper {
        pub fn new(height: u32, width: u32, num_of_mines: usize) -> Self {
            let mut game = Self {
                height,
                width,
                mines: HashSet::new(),
                mines_found: HashSet::new(),
                board: [[false; 8]; 8],
            };
            game.initialize(num_of_mines);
            game
        }
        // Initialize mines in random locations
        fn initialize(&mut self, num_of_mines: usize) {
            while self.mines.len() != num_of_mines {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0..self.height) as usize;
                let j = rng.gen_range(0..self.width) as usize;
                if !self.board[i][j] {
                    self.mines.insert((i, j));
                    self.board[i][j] = true;
                };
            }
        }

        fn print(&self) {
            
        }
    }

}
