#![warn(clippy::all, clippy::pedantic)]

//! Cellular automaton simulation tools targeting
//! [`WebAssembly`](https://webassembly.org).

mod utils;

use js_sys;
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
    generation: usize,
    neighbor_deltas: [[usize; 2]; 8],
}

#[wasm_bindgen]
impl Automaton {
    /// Constructs a new automaton with cell states randomly assigned to 0 or 1.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        utils::set_panic_hook();

        let cells = (0..width * height)
            .map(|_| if js_sys::Math::random() < 0.5 { 1 } else { 0 })
            .collect();
        let neighbor_deltas = get_neighbor_deltas(width, height);

        Self {
            width,
            height,
            cells,
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
    /// extended by the difference, with each additional cell filled with 0. If
    /// `new_width` is less than `width`, the automaton's rows are simply
    /// truncated.
    pub fn set_width(&mut self, new_width: usize) {
        // TODO: minimize allocations
        self.cells = self
            .cells
            .chunks_exact(self.width)
            .flat_map(|chunk| {
                let mut chunk_vec = chunk.to_vec();
                chunk_vec.resize(new_width, 0);
                chunk_vec
            })
            .collect();
        self.width = new_width;
        self.neighbor_deltas = get_neighbor_deltas(new_width, self.height);
    }

    /// Returns the height (vertical length) of the automaton.
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Resizes the automaton so that `height` is equal to `new_height`.
    ///
    /// If `new_height` is greater than `height`, the automaton's grid is
    /// extended by the difference, with each additional row filled with 0. If
    /// `new_height` is less than `height`, the automaton's grid is simply
    /// truncated.
    pub fn set_height(&mut self, new_height: usize) {
        self.cells.resize(self.width * new_height, 0);
        self.height = new_height;
        self.neighbor_deltas = get_neighbor_deltas(self.width, new_height);
    }

    /// Toggles the state of a cell. If the cell state is 0, it is set to 1. If
    /// the cell is any other state, it is set to 0.
    pub fn cell_toggle(&mut self, row: usize, col: usize) {
        let idx = self.index(row, col);
        self.cells[idx] = match self.cells[idx] {
            0 => 1,
            _ => 0,
        };
    }

    /// Returns the automaton's cells as a raw pointer.
    #[must_use]
    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    /// Returns a copy of the automaton's cells.
    #[must_use]
    pub fn cells_vec(&self) -> Vec<u8> {
        self.cells.clone()
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
    }

    /// Returns the number of generations elapsed.
    #[must_use]
    pub fn generation(&self) -> usize {
        self.generation
    }

    /// Calculates and sets the next state of all cells in the automaton.
    pub fn step(&mut self, n: usize) {
        for _ in 0..n {
            let mut cells_next = self.cells.clone();

            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.index(row, col);

                    cells_next[idx] = match (self.cells[idx], self.neighbor_count(row, col)) {
                        (1, neighbors) if neighbors < 2 || neighbors > 3 => 0,
                        (1, _) | (0, 3) => 1,
                        _ => 0,
                    };
                }
            }

            self.cells = cells_next;
            self.generation += 1;
        }
    }

    /// Returns the index of a cell in the automaton.
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    /// Returns the count of a cell's live, first-generation neighbors.
    fn neighbor_count(&self, row: usize, col: usize) -> u8 {
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
}

/// Returns the offsets of neighboring cell locations; these deltas are required
/// for an automaton's `get_neighbor_count` method.
fn get_neighbor_deltas(width: usize, height: usize) -> [[usize; 2]; 8] {
    [
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
