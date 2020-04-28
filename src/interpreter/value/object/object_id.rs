#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId {
    id: usize
}

impl ObjectId {
    pub fn new(index: usize) -> ObjectId {
        ObjectId {
            id: index
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}
