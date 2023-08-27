#[derive(Debug, Clone, Default)]
pub struct State(pub chrono::NaiveDate, pub u8);

#[derive(Debug)]
pub struct StateMap(pub Vec<State>);

impl<'a> Default for StateMap {
    fn default() -> Self {
        let mut v = Vec::new();
        for _ in 0..crate::consts::GRID_SIZE {
            v.push(State::default())
        }

        Self(v)
    }
}
