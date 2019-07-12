mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        use Cell::*;
        let cells = (0..width*height)
            .map(|_| {
                if js_sys::Math::random() < 0.3 {
                    Alive
                } else {
                    Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }

    fn index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbours_cnt(&self, row: u32, col: u32) -> u8 {
        let mut cnt = 0;
        for delta_x in [self.height-1, 0, 1].iter().cloned() {
            for delta_y in [self.width-1, 0, 1].iter().cloned() {
                if delta_x == 0 && delta_y == 0 { continue; }
                let x = (row + delta_x) % self.height;
                let y = (col + delta_y) % self.width;
                cnt += self.cells[self.index(x, y)] as u8;
            }
        }
        cnt
    }

    pub fn tick(&mut self) {
        use Cell::*;
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.index(row, col);
                let cell = next[idx];
                let alive_neighbours = self.live_neighbours_cnt(row, col);
                let next_cell = match (cell, alive_neighbours) {
                    (Alive, n) if n < 2 || n > 3 => Dead,
                    (Alive, 2) | (Alive, 3)      => Alive,
                    (Dead, 3)                    => Alive,
                    (otherwise, _)               => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn get_cells_ptr(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Cell::*;
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
