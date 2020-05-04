use std::collections::HashMap;

use crate::Module;
use crate::{EnvironmentId, Error, ModuleId};

#[derive(Clone, Debug)]
pub struct ModuleArena {
    modules: HashMap<ModuleId, Module>,
    mapping: HashMap<String, ModuleId>,
    next_id: usize,
}

impl ModuleArena {
    pub fn new() -> ModuleArena {
        ModuleArena {
            modules: HashMap::new(),
            mapping: HashMap::new(),
            next_id: 0,
        }
    }

    fn register_module(&mut self, path: String, module: Module) -> ModuleId {
        let module_id = ModuleId::new(self.next_id);

        self.next_id += 1;
        self.modules.insert(module_id, module);
        self.mapping.insert(path, module_id);

        module_id
    }

    pub fn make(&mut self, path: String, environment_id: EnvironmentId) -> ModuleId {
        let module = Module::new(path.clone(), environment_id);

        self.register_module(path, module)
    }

    pub fn make_with_empty_path(&mut self, environment_id: EnvironmentId) -> ModuleId {
        let module = Module::new_root_module(environment_id);

        self.register_module(String::from(""), module)
    }

    pub fn get_module_id(&self, path: &str) -> Option<ModuleId> {
        self.mapping.get(path).map(|module_id| *module_id)
    }

    pub fn get_module_id_required(&self, path: &str) -> Result<ModuleId, Error> {
        self.mapping
            .get(path)
            .map(|module_id| *module_id)
            .ok_or_else(|| Error::failure(format!("Cannot find module with path: {}", path)))
    }

    pub fn get_module(&self, module_id: ModuleId) -> Option<&Module> {
        self.modules.get(&module_id)
    }

    pub fn get_module_mut(&mut self, module_id: ModuleId) -> Option<&mut Module> {
        self.modules.get_mut(&module_id)
    }

    pub fn get_module_required_soft(&self, module_id: ModuleId) -> Result<&Module, Error> {
        self.modules.get(&module_id).ok_or_else(|| {
            Error::generic_execution_error(&format!("Cannot find module with id: {}", module_id))
        })
    }

    pub fn get_module_mut_required_soft(
        &mut self,
        module_id: ModuleId,
    ) -> Result<&mut Module, Error> {
        self.modules.get_mut(&module_id).ok_or_else(|| {
            Error::generic_execution_error(&format!("Cannot find module with id: {}", module_id))
        })
    }

    pub fn get_module_required_hard(&self, module_id: ModuleId) -> Result<&Module, Error> {
        self.modules.get(&module_id).ok_or_else(|| {
            Error::generic_execution_error(&format!("Cannot find module with id: {}", module_id))
        })
    }

    pub fn get_module_mut_required_hard(
        &mut self,
        module_id: ModuleId,
    ) -> Result<&mut Module, Error> {
        self.modules.get_mut(&module_id).ok_or_else(|| {
            Error::generic_execution_error(&format!("Cannot find module with id: {}", module_id))
        })
    }
}
