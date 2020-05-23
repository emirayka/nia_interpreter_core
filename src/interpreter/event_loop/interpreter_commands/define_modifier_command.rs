use crate::ModifierDescription;

#[derive(Clone, Debug)]
pub struct NiaDefineModifierCommand {
    modifier: ModifierDescription,
}

impl NiaDefineModifierCommand {
    pub fn new(modifier: ModifierDescription) -> NiaDefineModifierCommand {
        NiaDefineModifierCommand { modifier }
    }

    pub fn get_modifier(&self) -> &ModifierDescription {
        &self.modifier
    }
}
