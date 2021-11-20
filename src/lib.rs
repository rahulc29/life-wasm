mod utils;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};
use wasm_bindgen::prelude::*;
use js_sys::Math;
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
impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for j in 0..self.y_range {
            for i in 0..self.x_range {
                let cell = &self[(i, j)];
                match cell {
                    Cell::Alive => write!(f, "◼"),
                    Cell::Dead => write!(f, "◻"),
                }
                .ok();
            }
            write!(f, "\n").ok();
        }
        Ok(())
    }
}
impl Index<(u32, u32)> for Universe {
    type Output = Cell;
    fn index(&self, (x, y): (u32, u32)) -> &Cell {
        // TODO : avoid repetition in indexing code
        let x_index = (x % self.x_range) as usize;
        let y_index = (y % self.y_range) as usize;
        return &self.grid[x_index][y_index];
    }
}
impl IndexMut<(u32, u32)> for Universe {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Cell {
        // TODO : avoid repetition in indexing code
        let x_index = (x % self.x_range) as usize;
        let y_index = (y % self.y_range) as usize;
        return &mut self.grid[x_index][y_index];
    }
}
#[wasm_bindgen]
impl Universe {
    pub fn grid(&self) -> *const Cell {
        // TODO : Implement `delta` based data transactions
        self.grid[0].as_ptr()
    }
    pub fn get(&self, x: u32, y: u32) -> Cell {
        return self[(x, y)];
    }
    pub fn height(&self) -> u32 {
        return self.y_range;
    }
    pub fn width(&self) -> u32 {
        return self.x_range;
    }
    pub fn render(&self) -> String {
        return self.to_string();
    }
    // default constructor
    pub fn new() -> Self {
        utils::set_panic_hook();
        let x_range = 64u32;
        let y_range = 64u32;
        let mut grid = vec![vec![Cell::Dead; x_range as usize]; y_range as usize];
        for j in 0..y_range {
            for i in 0..x_range {
                // will produce initial pattern
                if Math::random() > 0.5f64 {
                    grid[i as usize][j as usize] = Cell::Alive;
                } else {
                    grid[i as usize][j as usize] = Cell::Dead;
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
    fn update_answer(answer: &mut u32, cell: Cell) {
        match cell {
            Cell::Alive => {
                *answer = *answer + 1;
            }
            Cell::Dead => (),
        }
    }
    // private utility to get number of alive cells for a give location
    fn alive_cells(&self, (x, y): (u32, u32)) -> u32 {
        // TODO : Rewrite using loops?
        let mut answer = 0u32;
        // immediate left
        Universe::update_answer(&mut answer, self[(x - 1, y)]);
        // immediate right
        Universe::update_answer(&mut answer, self[(x + 1, y)]);
        // immediate up
        Universe::update_answer(&mut answer, self[(x, y + 1)]);
        // immediate down
        Universe::update_answer(&mut answer, self[(x - 1, y)]);
        // diagonal up-right
        Universe::update_answer(&mut answer, self[(x + 1, y + 1)]);
        // diagonal up-left
        Universe::update_answer(&mut answer, self[(x - 1, y + 1)]);
        // diagonal down-right
        Universe::update_answer(&mut answer, self[(x + 1, y - 1)]);
        // diagonal down-left
        Universe::update_answer(&mut answer, self[(x - 1, y - 1)]);
        return answer;
    }
    pub fn update(&mut self) -> *const (u32, u32) {
        let mut delta = Vec::new();
        // yes, `i` and `j` are intentionally interchanged
        // the tuple parameter is (x, y)
        // the tuple argument should be (i, j)
        for j in 0..self.y_range {
            for i in 0..self.x_range {
                let alive = self.alive_cells((i, j));
                match self[(i, j)] {
                    Cell::Alive => {
                        if alive < 2 {
                            self[(i, j)] = Cell::Dead;
                            delta.push((i, j));
                        } else if alive == 2 || alive == 3 {
                            self[(i, j)] = Cell::Alive;
                        } else if alive > 3 {
                            self[(i, j)] = Cell::Dead;
                            delta.push((i, j));
                        }
                    }
                    Cell::Dead => {
                        if alive == 3 {
                            self[(i, j)] = Cell::Alive;
                            delta.push((i, j));
                        }
                    }
                }
            }
        }
        return delta.as_ptr();
    }
}
impl From<(u32, u32)> for Universe {
    fn from((x_range, y_range): (u32, u32)) -> Self {
        Self {
            x_range,
            y_range,
            // initally all cells are dead
            grid: vec![vec![Cell::Dead; x_range as usize]; y_range as usize],
        }
    }
}
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name)[..]);
}
