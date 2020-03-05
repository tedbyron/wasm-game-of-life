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
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    neighbor_locations: [[u32; 2]; 8],
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        // for delta_row in [self.height - 1, 0, 1].iter().cloned() {
        //     for delta_col in [self.width - 1, 0, 1].iter().cloned() {
        //         if delta_row == 0 && delta_col == 0 {
        //             continue;
        //         }

        //         let neighbor_row = (row + delta_row) % self.height;
        //         let neighbor_col = (col + delta_col) % self.width;
        //         let idx = self.get_index(neighbor_row, neighbor_col);
        //         count += self.cells[idx] as u8;
        //     }
        // }

        for delta in self.neighbor_locations.iter() {
            count += self.cells[self.get_index(
                (row + delta[0]) % self.height,
                (col + delta[1]) % self.width,
            )] as u8;
        }

        count
    }

    fn step(&mut self) {
        let mut cells_step = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let neighbor_count = self.get_neighbor_count(row, col);

                let next_cell = match (cell, neighbor_count) {
                    (Cell::Alive, count) if count < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, count) if count > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                cells_step[index] = next_cell;
            }
        }

        self.cells = cells_step;
    }

    fn set_neighbor_locations(&mut self) {
        self.neighbor_locations = [
            [self.height - 1, self.width - 1],
            [self.height - 1, 0],
            [self.height - 1, 1],
            [0, self.width - 1],
            [0, 1],
            [1, self.width - 1],
            [1, 0],
            [1, 1],
        ];
    }
}
