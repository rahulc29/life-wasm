pub enum Cell {
    Dead,
    Alive,
}
pub struct Universe {
    grid: Vec<Vec<Cell>>,
    x_range: u32,
    y_range: u32,
}
pub impl Universe {
    // getter
    pub fn grid(&self) -> &Vec<Vec<Cell>> {
        self.grid;
    } 
    // default constructor
    pub fn new() -> Self {
        Self {
            x_range: 30,
            y_range: 30,
            // initally all cells are dead
            grid: vec![vec![Dead, x_range]; y_range],
        }
    }
    // private utility function to update answer
    fn update_answer(answer: &u32, cell: &Cell) {
        match cell {
            Cell::Alive => answer++,
            Cell::Dead => ()
        }
    }
    // private utility to get number of alive cells for a give location
    fn alive_cells(&self, (x, y): (u32, u32)) -> u32 {
        // TODO : Rewrite using macros
        let mut answer = 0u32;
        if x == (self.x_range - 1) || y == (self.y_range - 1) {
            if x == (x_range - 1) && y == (y_range - 1) {
                // immediate up neighbour
                update_answer(&answer, &grid[x][y + 1]);
                // immediate left neighbour
                update_answer(&answer, &grid[x - 1][y]);
                // diagonal up-left neighbour
                update_answer(&answer, &grid[x - 1][y + 1]);
            } else if x == (x_range - 1) {
                if y == 0u32 {
                    // immediate left neighbour
                    update_answer(&answer, &grid[x - 1][y]);
                    // immeidate down neighbour
                    update_answer(&answer, &grid[x][y - 1]);
                    // diagonal down-left neighbour
                    update_answer(&answer, &grid[x - 1][y - 1]);
                } else {
                    // immediate left neighbour
                    update_answer(&answer, &grid[x - 1][y]);
                    // immediate up neighbour
                    update_answer(&answer, &grid[x][y + 1]);
                    // immediate down neighbour
                    update_answer(&answer, &grid[x][y - 1]);
                    // diagonal up-left neighbour
                    update_answer(&answer, &grid[x - 1][y + 1]);
                    // diagonal down-left neighbour
                    update_answer(&answer, &grid[x - 1][y - 1]);
                }
            } else if y == (y_range - 1) {
                if x == 0u32 {
                    // immediate right neighbour
                    update_answer(&answer, &grid[x + 1][y]);
                    // immediate up neighbour
                    update_answer(&answer, &grid[x][y + 1]);
                    // diagonal up-right neighbour
                    update_answer(&answer, &grid[x + 1][y + 1]);
                } else {
                    // immediate left neighbour
                    update_answer(&answer, &grid[x - 1][y]);
                    // immediate right neighbour
                    update_answer(&answer, &grid[x + 1][y]);
                    // immediate up neighbour
                    update_answer(&answer, &grid[x][y + 1]);
                    // diagonal up-left neighbour
                    update_answer(&answer, &grid[x - 1][y + 1]);
                    // diagonal up-right neighbour
                    update_answer(&answer, &grid[x + 1][y + 1]);
                }
            }
        } else if x == 0u32 || y == 0u32 {
            if x == 0u32 && y == 0u32 {
                // top left corner
                // only three neighbours
                // immediate up neighbour
                update_answer(&answer, &grid[x][y + 1]);
                // immediate right neighbour
                update_answer(&answer, &grid[x + 1][y]);
                // diagonal down-right neighbour
                update_answer(&answer, &grid[x + 1][y + 1]);
            } else if y == 0u32 {
                // immediate left neighbour
                update_answer(&answer, &grid[x - 1][y]);
                // immediate right neighbour
                update_answer(&answer, &grid[x + 1][y]);
                // immediate below neighbour
                update_answer(&answer, &grid[x][y - 1]);
                // diagonal left neighbour
                update_answer(&answer, &grid[x - 1][y - 1]);
                // diagonal right neighbour
                update_answer(&answer, &grid[x + 1][y + 1]);
            } else if x == 0u32 {
                // immediate right neighbour
                update_answer(&answer, &grid[x + 1][y]);
                // immediate up neighbour
                update_answer(&answer, &grid[x][y + 1]);
                // immediate down neighbour
                update_answer(&answer, &grid[x][y - 1]);
                // diagonal up-right neighbour
                update_answer(&answer, &grid[x + 1][y + 1]);
                // diagonal down-right neighbour
                update_answer(&answer, &grid[x + 1][y - 1]);
            } 
        } else {
            // purely general non-corner case
            // immediate left neighbour
            update_answer(&answer, &grid[x - 1][y]);
            // immediate right neighbour
            update_answer(&answer, &grid[x + 1][y]);
            // immediate below neighbour
            update_answer(&answer, &grid[x][y - 1]);
            // immediate above neighbour
            update_answer(&answer, &grid[x][y + 1]);
            // diagonal up-right neighbour
            update_answer(&answer, &grid[x + 1][y + 1]);
            // diagonal down-right neighbour
            update_answer(&answer, &grid[x + 1][y - 1]);
            // diagonal up-left neighbour
            update_answer(&answer, &grid[x - 1][y + 1]);
            // diagonal down-left neighbour
            update_answer(&answer, &grid[x - 1][y - 1]);
        }
        return answer;
    }
    pub fn update(&mut self) {
        // yes, `i` and `j` are intentionally interchanged
        // the tuple parameter is (x, y)
        // the tuple argument should be (i, j)
        for j in 0..self.y_range {
            for i in 0..self.x_range {
                let alive = self.alive_cells((i, j));
                match grid[i][j] {
                    Cell::Alive => {
                        if alive < 2 {
                            grid[i][j] = Cell::Dead;
                        } else if alive == 2 || alive == 3 {
                            grid[i][j] = Cell::Alive;
                        } else if alive > 3 {
                            grid[i][j] = Cell::Dead;
                        }
                    },
                    Cell::Dead => {
                        if alive == 3 {
                            grid[i][j] = Cell::Alive;
                        }
                    }
                }
            }
        }
    }
}
pub impl From<(u32, u32)> for Universe {
    fn from((x_range, y_range): (u32, u32)) -> Self {
        Self {
            x_range,
            y_range,
            // initally all cells are dead
            grid: vec![vec![Dead, x_range]; y_range]
        }
    }
}