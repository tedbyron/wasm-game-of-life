/// A ruleset consisting of survival, birth, and generation rules.
#[derive(Clone)]
pub struct Rules {
    pub survival: Vec<u8>,
    pub birth: Vec<u8>,
    pub generation: u8,
}

impl Default for Rules {
    /// Returns a ruleset using rules from Conway's Game of Life.
    fn default() -> Self {
        Self {
            survival: vec![2, 3],
            birth: vec![3],
            generation: 1,
        }
    }
}
