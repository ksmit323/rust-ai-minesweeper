/*
Minesweeper Gaming Logic
*/

pub mod game_logic {
    use core::fmt;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use rand::Rng;
    use std::collections::HashSet;

    pub struct Minesweeper {
        pub height: usize,
        pub width: usize,
        pub mines: HashSet<(usize, usize)>,
        pub mines_found: HashSet<(usize, usize)>,
        pub board: Vec<Vec<bool>>,
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

        pub fn nearby_mines(&self, cell: (usize, usize)) -> usize {
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
    pub struct Sentence {
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

    pub struct MinesweeperAI {
        pub height: usize,
        pub width: usize,
        pub moves_made: HashSet<Cell>,
        pub known_mines: HashSet<Cell>,
        pub known_safes: HashSet<Cell>,
        pub knowledge: Vec<Sentence>,
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

        pub fn add_knowledge(&mut self, cell: Cell, mut count: usize) {
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

            for i in cell.0.saturating_sub(1)..=(cell.0 + 1).min(self.height - 1) {
                for j in cell.1.saturating_sub(1)..=(cell.1 + 1).min(self.width - 1) {
                    // Ignore the cell itself
                    if (i, j) == cell {
                        continue;
                    }
                    // Add cell to set if cell is undetermined
                    if !self.moves_made.contains(&(i, j))
                        && !self.known_safes.contains(&(i, j))
                        && !self.known_mines.contains(&(i, j))
                    {
                        set_cells.insert((i, j));
                    }
                    // Adjust count if cell is a known mine
                    if self.known_mines.contains(&(i, j)) {
                        count -= 1;
                    }
                }
            }
            // Add cells and updated mine count to knowledge base
            if !set_cells.is_empty() {
                self.knowledge.push(Sentence::new(set_cells, count));
            }

            // Loop to update knowledge until there are no more changes
            let mut changes = true;
            while changes {
                changes = false;

                // Step 4: Mark additional cells as safe or mines if it can be included
                let mut safes_to_mark = Vec::new();
                let mut mines_to_mark = Vec::new();

                for sentence in &self.knowledge {
                    let known_safes: HashSet<(usize, usize)> = sentence.known_safes().clone();
                    let known_mines: HashSet<(usize, usize)> = sentence.known_mines().clone();

                    if !known_safes.is_empty() {
                        safes_to_mark.extend(known_safes);
                        changes = true;
                    }
                    if !known_mines.is_empty() {
                        mines_to_mark.extend(known_mines);
                        changes = true;
                    }
                }
                // Apply collected changes
                for safe in safes_to_mark {
                    self.mark_safe(safe);
                }
                for mine in mines_to_mark {
                    self.mark_mine(mine);
                }

                // Remove any empty sentences from knowledge base
                self.knowledge.retain(|sentence| !sentence.cells.is_empty());

                // Step 5: Add new sentences based on inferred subset method
                let knowledge_snapshot = self.knowledge.clone();
                let mut new_knowledge = Vec::new();
                for s1 in &knowledge_snapshot {
                    for s2 in &knowledge_snapshot {
                        if s1 == s2 {
                            continue;
                        }
                        if s2.cells.is_subset(&s1.cells) {
                            let difference: HashSet<Cell> =
                                s1.cells.difference(&s2.cells).cloned().collect();
                            if !difference.is_empty() {
                                new_knowledge.push(Sentence::new(difference, s1.count - s2.count));
                                changes = true;
                            }
                        }
                    }
                }
                self.knowledge.extend(new_knowledge);
            }
        }

        pub fn make_safe_move(&self) -> Option<Cell> {
            /*
            Returns a safe cell to choose on the Minesweeper board.
            The move must be known to be safe, and not already a move
            that has been made.

            This function may use the knowledge in self.mines, self.safes
            and self.moves_made, but should not modify any of those values.
            */
            for i in 0..self.height {
                for j in 0..self.width {
                    if !self.moves_made.contains(&(i, j)) && self.known_safes.contains(&(i, j)) {
                        return Some((i, j));
                    }
                }
            }
            None
        }

        pub fn make_random_move(&self) -> Option<Cell> {
            let mut random_moves = Vec::new();
            for i in 0..self.height {
                for j in 0..self.width {
                    if !self.moves_made.contains(&(i, j)) && !self.known_mines.contains(&(i, j)) {
                        random_moves.push((i, j));
                    }
                }
            }
            let mut rng = thread_rng();
            random_moves.choose(&mut rng).cloned()
        }
    }
}
