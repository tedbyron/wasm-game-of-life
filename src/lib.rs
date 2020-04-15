#![warn(clippy::all, clippy::pedantic)]

//! Cellular automaton simulation tools targeting
//! [`WebAssembly`](https://webassembly.org).

mod utils;

use js_sys;
use std::{cmp::Ordering, iter, mem};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// A two-dimensional cellular automaton with a finite grid of cells.
#[wasm_bindgen]
pub struct Automaton {
    width: usize,
    height: usize,
    cells: Vec<u8>,
    cells_step: Vec<u8>,
    generation: usize,
    neighbor_deltas: [[usize; 2]; 8],
}

#[wasm_bindgen]
impl Automaton {
    /// Constructs a new automaton with all cell states set to 0.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        utils::set_panic_hook();

        let cells = vec![0; width * height];
        let cells_step = vec![0; width * height];
        let neighbor_deltas = [
            [height - 1, width - 1],
            [height - 1, 0],
            [height - 1, 1],
            [0, width - 1],
            [0, 1],
            [1, width - 1],
            [1, 0],
            [1, 1],
        ];

        Self {
            width,
            height,
            cells,
            cells_step,
            generation: 0,
            neighbor_deltas,
        }
    }

    /// Returns the width (horizontal length) of the automaton.
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Resizes the automaton so that `width` is equal to `new_width`.
    ///
    /// If `new_width` is greater than `width`, the automaton's rows are
    /// extended by the difference, with each additional column filled with 0.
    /// If `new_width` is less than `width`, the automaton's rows are simply
    /// truncated.
    pub fn set_width(&mut self, new_width: usize) {
        match new_width.cmp(&self.width) {
            Ordering::Greater => {
                let width_diff = new_width - self.width;
                self.cells.reserve_exact(width_diff * self.height);
                for _ in 0..self.height {
                    self.cells.extend(iter::repeat(0).take(width_diff));
                    self.cells.rotate_right(new_width);
                }
            }
            Ordering::Less => {
                let width_diff = self.width - new_width;
                for _ in 0..self.height {
                    self.cells.truncate(self.cells.len() - width_diff);
                    self.cells.rotate_right(new_width);
                }
            }
            Ordering::Equal => (),
        }
        self.cells_step
            .resize_with(new_width * self.height, Default::default);
        self.width = new_width;
        self.set_neighbor_deltas(new_width, self.height);
    }

    /// Returns the height (vertical length) of the automaton.
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Resizes the automaton so that `height` is equal to `new_height`.
    ///
    /// If `new_height` is greater than `height`, the automaton's columns are
    /// extended by the difference, with each additional row filled with 0. If
    /// `new_height` is less than `height`, the automaton's columns are simply
    /// truncated.
    pub fn set_height(&mut self, new_height: usize) {
        self.cells
            .resize_with(self.width * new_height, Default::default);
        self.cells_step
            .resize_with(self.width * new_height, Default::default);
        self.height = new_height;
        self.set_neighbor_deltas(self.width, new_height);
    }

    /// Toggles the state of a cell. If the cell state is 0, it is set to 1. If
    /// the cell is any other state, it is set to 0.
    pub fn cell_toggle(&mut self, row: usize, col: usize) {
        let idx = self.index(row, col);
        self.cells[idx] = match self.cells[idx] {
            0 => 1,
            _ => 0,
        }
    }

    /// Returns the automaton's cells as a raw pointer.
    #[must_use]
    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    /// Sets the state of cells in `locations` to 1.
    pub fn set_cells(&mut self, cells: &[usize]) {
        for (&row, &col) in cells.iter().step_by(2).zip(cells.iter().skip(1).step_by(2)) {
            let idx = self.index(row, col);
            if let Some(cell) = self.cells.get_mut(idx) {
                *cell = 1;
            }
        }
    }

    /// Sets the cell state of all the automaton's cells to `n`.
    pub fn set_all_cells(&mut self, n: u8) {
        for cell in &mut self.cells {
            *cell = n;
        }
        self.reset_generation();
    }

    /// Randomizes the cell state of all the automaton's cells.
    ///
    /// Loops through the automaton's cells and if `js_sys::Math::random` is
    /// less than the percentage `n`, the cell state is set to 1.
    pub fn randomize_cells(&mut self, n: f64) {
        for cell in &mut self.cells {
            *cell = if js_sys::Math::random() < n / 100.0 {
                1
            } else {
                0
            };
        }
        self.reset_generation();
    }

    /// Returns the number of generations elapsed.
    #[must_use]
    pub fn generation(&self) -> usize {
        self.generation
    }

    /// Calculates and sets the next state of all cells in the automaton.
    pub fn step(&mut self, n: usize) {
        for _ in 0..n {
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.index(row, col);

                    self.cells_step[idx] = match (self.cells[idx], self.neighbors(row, col)) {
                        (1, neighbors) if neighbors < 2 || neighbors > 3 => 0,
                        (1, _) | (0, 3) => 1,
                        _ => 0,
                    }
                }
            }

            mem::swap(&mut self.cells, &mut self.cells_step);
        }

        self.generation += n;
    }

    /// Returns the index of a cell in the automaton.
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    /// Returns the count of a cell's live, first-generation neighbors.
    fn neighbors(&self, row: usize, col: usize) -> u8 {
        self.neighbor_deltas.iter().fold(0, |count, delta| {
            match self.cells[self.index(
                (row + delta[0]) % self.height,
                (col + delta[1]) % self.width,
            )] {
                1 => count + 1,
                _ => count,
            }
        })
    }

    /// Returns the offsets of neighboring cell locations; these deltas are required
    /// for an automaton's `neighbors` method.
    fn set_neighbor_deltas(&mut self, width: usize, height: usize) {
        self.neighbor_deltas = [
            [height - 1, width - 1],
            [height - 1, 0],
            [height - 1, 1],
            [0, width - 1],
            [0, 1],
            [1, width - 1],
            [1, 0],
            [1, 1],
        ]
    }

    /// Resets the generation count.
    fn reset_generation(&mut self) {
        self.generation = 0;
    }
}

// Functions for integration tests.
impl Automaton {
    /// Get a clone of the automaton's cells.
    #[must_use]
    pub fn get_cells(&self) -> Vec<u8> {
        self.cells.clone()
    }
}
