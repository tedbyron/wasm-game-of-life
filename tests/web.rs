use std::iter;
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_game_of_life::Automaton;

/// flatten a slice of tuples that contain (x, y) locations of cells
fn flatten_locations(locations: &[(usize, usize)]) -> Vec<usize> {
    locations
        .iter()
        .flat_map(|l| iter::once(l.0).chain(iter::once(l.1)))
        .collect()
}

/// build an automaton with width, height, and locations of live cells
fn build_automaton(width: usize, height: usize, locations: &[(usize, usize)]) -> Automaton {
    let mut a = Automaton::new(width, height);
    a.set_width(width);
    a.set_height(height);
    a.set_all_cells(0);
    a.set_cells(&flatten_locations(locations));
    a
}

mod tests {
    use super::*;

    #[wasm_bindgen_test]
    pub fn test_automaton_new() {
        let a = Automaton::new(64, 64);
        assert_eq!(a.width(), 64);
        assert_eq!(a.height(), 64);
        assert_eq!(a.cells_vec().len(), 64 * 64);
        assert_eq!(a.generation(), 0);
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_set_cells() {
        let mut a = Automaton::new(3, 3);
        a.set_cells(&flatten_locations(&[
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 1),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2),
        ]));
        assert_eq!(a.cells_vec(), vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_set_all_cells() {
        let mut a = Automaton::new(3, 3);
        a.set_all_cells(1);
        assert_eq!(a.cells_vec(), vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);
        a.set_all_cells(0);
        assert_eq!(a.cells_vec(), vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_set_width_larger() {
        let mut a = Automaton::new(3, 3);
        a.set_all_cells(1);
        a.set_width(4);
        assert_eq!(a.cells_vec(), vec![1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0]);
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_set_width_smaller() {
        let mut a = Automaton::new(3, 3);
        a.set_all_cells(1);
        a.set_width(2);
        assert_eq!(a.cells_vec(), vec![1, 1, 1, 1, 1, 1]);
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_set_height_larger() {
        let mut a = Automaton::new(3, 3);
        a.set_all_cells(1);
        a.set_height(4);
        assert_eq!(a.cells_vec(), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0]);
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_set_height_smaller() {
        let mut a = Automaton::new(3, 3);
        a.set_all_cells(1);
        a.set_height(4);
        assert_eq!(a.cells_vec(), vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0]);
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_wrapping() {
        let mut a = build_automaton(2, 2, &[(0, 0), (0, 1)]);
        let a_1 = build_automaton(2, 2, &[(0, 0), (0, 1)]);

        a.step(1);
        assert_eq!(a.cells_vec(), a_1.cells_vec());
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_tick() {
        let mut a = build_automaton(6, 6, &[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
        let a_1 = build_automaton(6, 6, &[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);

        a.step(1);
        assert_eq!(a.cells_vec(), a_1.cells_vec());
    }
}
