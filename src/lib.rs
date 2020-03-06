mod utils;

use wasm_bindgen::prelude::*;

// when the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Grid {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    neighbor_locations: [[u32; 2]; 8],
}

#[wasm_bindgen]
impl Grid {
    pub fn new() -> Grid {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        let neighbor_locations = [
            [height - 1, width - 1],
            [height - 1, 0],
            [height - 1, 1],
            [0, width - 1],
            [0, 1],
            [1, width - 1],
            [1, 0],
            [1, 1],
        ];

        Grid {
            width,
            height,
            cells,
            neighbor_locations,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta in self.neighbor_locations.iter() {
            count += self.cells[self.get_index(
                (row + delta[0]) % self.height,
                (col + delta[1]) % self.width,
            )] as u8;
        }

        count
    }

    pub fn step(&mut self) {
        let mut cells_step = self.cells.clone();

        // TODO: loop through cells directly
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);

                cells_step[index] = match (self.cells[index], self.get_neighbor_count(row, col)) {
                    (Cell::Alive, count) if count < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, count) if count > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (previous_state, _) => previous_state,
                };
            }
        }

        self.cells = cells_step;
    }
}
