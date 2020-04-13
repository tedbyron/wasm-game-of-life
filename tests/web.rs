use wasm_bindgen_test::wasm_bindgen_test;
use wasm_game_of_life::Automaton;

fn build_automaton(width: u16, height: u16, rows: &[u16], cols: &[u16]) -> Automaton {
    let mut a = Automaton::new(width, height);
    a.set_width(width);
    a.set_height(height);
    a.set_all_cells(0);
    a.set_cells(rows, cols);
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
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_set_cells() {
        let mut a = Automaton::new(2, 2);
        a.set_cells(&[0, 0, 1, 1], &[0, 1, 0, 1]);
        assert_eq!(a.cells_vec(), vec![1, 1, 1, 1]);
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_set_all_cells() {
        let mut a = Automaton::new(2, 2);
        a.set_all_cells(1);
        a.set_all_cells(0);
        assert_eq!(a.cells_vec(), vec![0, 0, 0, 0]);
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_wrapping_1() {
        let mut a = build_automaton(2, 2, &[0, 0], &[0, 1]);
        let a_1 = build_automaton(2, 2, &[0, 0], &[0, 1]);

        a.step();
        assert_eq!(a.cells_vec(), a_1.cells_vec());
    }

    #[wasm_bindgen_test]
    pub fn test_automaton_wrapping_2() {
        let mut a = build_automaton(6, 6, &[1, 2, 3, 3, 3], &[2, 3, 1, 2, 3]);
        let a_1 = build_automaton(6, 6, &[2, 2, 3, 3, 4], &[1, 3, 2, 3, 2]);

        a.step();
        assert_eq!(a.cells_vec(), a_1.cells_vec());
    }
}
