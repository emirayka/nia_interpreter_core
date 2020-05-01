#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnvironmentId {
    index: usize,
}

impl EnvironmentId {
    pub fn new(index: usize) -> EnvironmentId {
        EnvironmentId {
            index,
        }
    }

    pub fn get_id(&self) -> usize {
        self.index
    }
}
