use crate::Action;
use crate::KeyChord;
use crate::Mapping;

#[derive(Clone, Debug)]
pub struct NiaDefineMappingCommand {
    mapping: Mapping,
}

impl NiaDefineMappingCommand {
    pub fn new(mapping: Mapping) -> NiaDefineMappingCommand {
        NiaDefineMappingCommand { mapping }
    }

    pub fn get_mapping(&self) -> &Mapping {
        &self.mapping
    }
}
