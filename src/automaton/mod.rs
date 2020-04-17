mod rules;
mod utils;

use std::cmp::Ordering;
use std::iter;
use std::mem;

use js_sys;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// A two-dimensional cellular automaton with a finite grid of cells.
#[wasm_bindgen]
#[derive(Clone)]
pub struct Automaton {
    width: usize,
    height: usize,
    cells: Vec<u8>,
    cells_step: Vec<u8>,
    rules: rules::Rules,
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
            rules: rules::Rules::default(),
            neighbor_deltas,
        }
    }

    /// Resizes the automaton so that `width` is equal to `new_width`.
    ///
    /// If `new_width` is greater than `width`, the automaton's rows are
    /// extended by the difference, with each additional column filled with 0.
    /// If `new_width` is less than `width`, the automaton's rows are simply
    /// truncated.
    pub fn resize_width(&mut self, new_width: usize) {
        match new_width.cmp(&self.width) {
            Ordering::Greater => {
                let width_diff = new_width - self.width;
                let width = self.width;
                self.cells.reserve_exact(width_diff * self.height);
                for i in (0..self.height).rev().map(|n| n * width + width) {
                    self.cells.splice(i..i, iter::repeat(0).take(width_diff));
                }
                // TODO: benchmark against the following alternative
                // for _ in 0..self.height {
                //     self.cells.extend(iter::repeat(0).take(width_diff));
                //     self.cells.rotate_right(new_width);
                // }
            }
            Ordering::Less => {
                let width_diff = self.width - new_width;
                let width = self.width;
                for (start, end) in (1..=self.height)
                    .rev()
                    .map(|n| (n * width - width_diff, n * width))
                {
                    self.cells.splice(start..end, iter::empty());
                }
                // TODO: benchmark against the following alternative
                // for _ in 0..self.height {
                //     self.cells.truncate(self.cells.len() - width_diff);
                //     self.cells.rotate_right(new_width);
                // }
            }
            Ordering::Equal => (),
        }
        self.cells_step
            .resize_with(new_width * self.height, Default::default);
        self.width = new_width;
        self.set_neighbor_deltas(new_width, self.height);
    }

    /// Resizes the automaton so that `height` is equal to `new_height`.
    ///
    /// If `new_height` is greater than `height`, the automaton's columns are
    /// extended by the difference, with each additional row filled with 0. If
    /// `new_height` is less than `height`, the automaton's columns are simply
    /// truncated.
    pub fn resize_height(&mut self, new_height: usize) {
        self.cells
            .resize_with(self.width * new_height, Default::default);
        self.cells_step
            .resize_with(self.width * new_height, Default::default);
        self.height = new_height;
        self.set_neighbor_deltas(self.width, new_height);
    }

    /// Returns the automaton's cells as a raw pointer.
    #[must_use]
    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    /// Toggles the state of a cell. If the cell state is 0, it is set to 1. If
    /// the cell is any other state, it is set to 0.
    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let idx = self.index(row, col);
        if let Some(cell) = self.cells.get_mut(idx) {
            *cell = match cell {
                0 => 1,
                _ => 0,
            }
        }
    }

    /// Sets the state of cells in `locations` to 1.
    ///
    /// `locations` is a list of alternating x and y coordinates.
    pub fn set_cells_on(&mut self, cells: &[usize]) {
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
    }

    /// Sets the survival rule to a different value.
    pub fn set_survival_rule(&mut self, s: &[u8]) {
        self.rules.survival = s.to_vec();
    }

    /// Sets the birth rule to a different value.
    pub fn set_birth_rule(&mut self, b: &[u8]) {
        self.rules.birth = b.to_vec();
    }

    /// Sets the generation rule to a different value.
    pub fn set_generation_rule(&mut self, c: u8) {
        self.rules.generation = c;
    }

    /// Calculates and sets the next state of all cells in the automaton.
    pub fn step(&mut self, n: usize) {
        for _ in 0..n {
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.index(row, col);

                    self.cells_step[idx] = match (self.cells[idx], self.neighbors(row, col)) {
                        (0, n) => {
                            if self.rules.birth.contains(&n) {
                                1
                            } else {
                                0
                            }
                        }
                        (1, n) if self.rules.survival.contains(&n) => 1,
                        (s, _) if s < self.rules.generation => s + 1,
                        _ => 0,
                    }
                }
            }

            mem::swap(&mut self.cells, &mut self.cells_step);
        }
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
}

/// Functions for integration tests.
#[doc(hidden)]
impl Automaton {
    /// Get a clone of the automaton's cells.
    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.cells.clone()
    }
}
