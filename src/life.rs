use std::ops::Index;
use std::fmt::{Display, Formatter, Result};
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
#[wasm_bindgen]
pub struct Universe {
    grid: Vec<Vec<Cell>>,
    x_range: u32,
    y_range: u32,
}
pub impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for j in 0..y_range {
            for i in 0..x_range {
                let cell = &self[(i, j)];
                match cell {
                    Cell::Alive => write!(f, "◼"),
                    Cell::Dead => write!(f, "◻")
                };
            }
            write!(f, "\n");
        }
    }
}
pub impl Index<(u32, u32)> for Universe {
    fn index(&self, (x, y): (u32, u32)) -> &Cell {
        let mut x_index = x % self.x_range;
        if x_index < 0 {
            x_index += x_range;
        }
        let mut y_index = y % self.y_range;
        if y_index < 0 {
            y_index += y_range;
        }
        return &grid[x_index][y_index];
    }
}
pub impl Universe {
    // getter
    pub fn grid(&self) -> &Vec<Vec<Cell>> {
        self.grid;
    } 
    pub fn render(&self) -> String {
        self.to_string();
    }
    // default constructor
    pub fn new() -> Self {
        let x_range = 64u32;
        let y_range = 64u32;
        let grid = vec![vec![Dead, x_range]; y_range];
        for j in 0..y_range {
            for i in 0..x_range {
                // will produce initial pattern
                if i % 3 == 0 || j % 7 == 0 {
                    grid[(i, j)] = Cell::Alive;
                } else {
                    grid[(i, j)] = Cell::Dead;
                }
            }
        }
        Self {
            x_range,
            y_range,
            grid,
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
        // TODO : Rewrite using loops?
        let mut answer = 0u32;
        // immediate left
        update_answer(&answer, &grid[(x - 1, y)]);
        // immediate right
        update_answer(&answer, &grid[(x + 1, y)]);
        // immediate up
        update_answer(&answer, &grid[(x, y + 1)]);
        // immediate down
        update_answer(&answer, &grid[(x - 1, y)]);
        // diagonal up-right
        update_answer(&answer, &grid[(x + 1, y + 1)]);
        // diagonal up-left
        update_answer(&answer, &grid[(x - 1, y + 1)]);
        // diagonal down-right
        update_answer(&answer, &grid[(x + 1, y - 1)]);
        // diagonal down-left
        update_answer(&answer, &grid[(x - 1, y - 1)]);
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