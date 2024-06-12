/*
Minesweeper Gaming Logic
*/

pub mod game_logic {
    use core::fmt;
    use rand::Rng;
    use std::collections::HashSet;

    struct Minesweeper {
        height: usize,
        width: usize,
        mines: HashSet<(usize, usize)>,
        mines_found: HashSet<(usize, usize)>,
        board: Vec<Vec<bool>>,
    }

    impl Minesweeper {
        pub fn new(height: usize, width: usize, num_of_mines: usize) -> Minesweeper {
            let mut minesweeper = Minesweeper {
                height,
                width,
                mines: HashSet::new(),
                mines_found: HashSet::new(),
                board: vec![vec![false; width]; height],
            };
            // Initialize mines in random locations
            let mut rng = rand::thread_rng();
            while minesweeper.mines.len() < num_of_mines {
                let i = rng.gen_range(0..height);
                let j = rng.gen_range(0..width);
                if !minesweeper.board[i][j] {
                    minesweeper.mines.insert((i, j));
                    minesweeper.board[i][j] = true;
                }
            }
            minesweeper
        }

        pub fn print(&self) {
            /* Prints a text-based representation of where mines are located */
            for i in 0..self.height {
                println!("{}", "-".repeat(self.width * 2 + 1));
                for j in 0..self.width {
                    if self.board[i][j] {
                        println!("|X");
                    } else {
                        println!("| ")
                    }
                }
                println!("|");
            }
            println!("{}", "-".repeat(self.width * 2 + 1));
        }

        pub fn is_mine(&self, cell: (usize, usize)) -> bool {
            let (i, j) = cell;
            self.board[i][j]
        }

        pub fn nearby_mines(&self, cell: (usize, usize)) -> i32 {
            /* Returns the number of mines that are within one row and column of a give cell, not including the cell itself */

            let (x, y) = cell;
            let mut count = 0;

            for i in x.saturating_sub(1)..=(x + 1).min(self.height - 1) {
                for j in y.saturating_sub(1)..=(y + 1).min(self.width - 1) {
                    if (i, j) != (x, y) && self.board[i][j] {
                        count += 1;
                    }
                }
            }
            count
        }

        pub fn won(&self) -> bool {
            /* Check if all mines have been flagged */
            self.mines_found == self.mines
        }
    }

    #[derive(Debug, Clone)]
    struct Sentence {
        /* Logical statement about a  Minesweeper game
        A sentence consists of a set of board cells,
        and a count of the number of those cells which are mines.*/
        cells: HashSet<(usize, usize)>,
        count: usize,
    }

    impl Sentence {
        pub fn new(cells: HashSet<(usize, usize)>, count: usize) -> Sentence {
            Sentence { cells, count }
        }

        pub fn known_mines(&self) -> HashSet<(usize, usize)> {
            /* Returns the set of all cell sin self.cells known to be mines */
            // Cells are known to be mines if their set count equals the set length
            if self.count == self.cells.len() {
                self.cells.clone()
            } else {
                HashSet::new()
            }
        }

        pub fn known_safes(&self) -> HashSet<(usize, usize)> {
            /* Returns the set of all cells in self.cells known to be safe
               Cells are known to be safe if their set count equals zero
            */
            if self.count == 0 {
                self.cells.clone()
            } else {
                HashSet::new()
            }
        }

        pub fn mark_mine(&mut self, cell: (usize, usize)) {
            /* Updates internal knowledge representation given the fact that a cell is known to be a mine */
            if self.cells.contains(&cell) {
                self.cells.remove(&cell);
                self.count -= 1;
            }
        }

        pub fn mark_safe(&mut self, cell: (usize, usize)) {
            /* Updates internal knowledge representation given the fact that a cell is known to be safe */
            if self.cells.contains(&cell) {
                self.cells.remove(&cell);
            }
        }
    }
    // Implementing equality comparsion
    impl PartialEq for Sentence {
        fn eq(&self, other: &Self) -> bool {
            self.cells == other.cells && self.count == other.count
        }
    }
    // Implementing Display trait for string representation
    impl fmt::Display for Sentence {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?} = {}", self.cells, self.count)
        }
    }

    type Cell = (usize, usize);

    struct MinesweeperAI {
        height: usize,
        width: usize,
        moves_made: HashSet<Cell>,
        known_mines: HashSet<Cell>,
        known_safes: HashSet<Cell>,
        knowledge: Vec<Sentence>,
    }

    impl MinesweeperAI {
        pub fn new(height: usize, width: usize) -> MinesweeperAI {
            MinesweeperAI {
                height,
                width,
                moves_made: HashSet::new(),
                known_mines: HashSet::new(),
                known_safes: HashSet::new(),
                knowledge: Vec::new(),
            }
        }

        pub fn mark_mine(&mut self, cell: Cell) {
            /* Marks a cell as a mine and updates all knowledge to mark that cell as a mine as well */
            self.known_mines.insert(cell);
            for sentence in &mut self.knowledge {
                sentence.mark_mine(cell);
            }
        }

        pub fn mark_safe(&mut self, cell: Cell) {
            /* Marks a cell as safe, and updates all knowledge to mark that cell as safe as well */
            self.known_safes.insert(cell);
            for sentence in &mut self.knowledge {
                sentence.mark_safe(cell);
            }
        }

        pub fn add_knowledge(&mut self, cell: Cell, count: usize) {
            /* Called when the Minesweeper board tells us, for a given
            safe cell, how many neighboring cells have mines in them.

            This function should:
                1) mark the cell as a move that has been made
                2) mark the cell as safe
                3) add a new sentence to the AI's knowledge base
                   based on the value of `cell` and `count`
                4) mark any additional cells as safe or as mines
                   if it can be concluded based on the AI's knowledge base
                5) add any new sentences to the AI's knowledge base
                   if they can be inferred from existing knowledge
            */

            // Step 1: Add cell to moves_made set
            self.moves_made.insert(cell);

            // Step 2: Mark cell as safe
            self.mark_safe(cell);

            // Step 3: Add sentence to knowledge base by adding neighboring cells to a set
            let mut set_cells: HashSet<Cell> = HashSet::new();
            // TODO

            // Step 4: Add cells and updated mine count to knowledge base
            self.knowledge.push(Sentence::new(set_cells, count));



        }
    }
}
