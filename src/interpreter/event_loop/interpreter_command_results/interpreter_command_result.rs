use crate::interpreter::event_loop::interpreter_command_results::*;

#[derive(Clone, Debug)]
pub enum NiaInterpreterCommandResult {
    DefineDevice(NiaDefineDeviceCommandResult),
    DefineModifier(NiaDefineModifierCommandResult),
    ExecuteCode(NiaExecuteCodeCommandResult),
    GetDefinedModifiers(NiaGetDefinedModifiersCommandResult),
    RemoveDeviceByName(NiaRemoveDeviceByNameCommandResult),
    RemoveDeviceByPath(NiaRemoveDeviceByPathCommandResult),
    RemoveModifier(NiaRemoveModifierCommandResult),
}

macro_rules! make_from_impl {
    ($command_result_type: ident, $variant: path) => {
        impl From<$command_result_type> for NiaInterpreterCommandResult {
            fn from(result: $command_result_type) -> Self {
                $variant(result)
            }
        }
    };
}

make_from_impl!(
    NiaDefineDeviceCommandResult,
    NiaInterpreterCommandResult::DefineDevice
);
make_from_impl!(
    NiaDefineModifierCommandResult,
    NiaInterpreterCommandResult::DefineModifier
);
make_from_impl!(
    NiaExecuteCodeCommandResult,
    NiaInterpreterCommandResult::ExecuteCode
);
make_from_impl!(
    NiaGetDefinedModifiersCommandResult,
    NiaInterpreterCommandResult::GetDefinedModifiers
);
make_from_impl!(
    NiaRemoveDeviceByNameCommandResult,
    NiaInterpreterCommandResult::RemoveDeviceByName
);
make_from_impl!(
    NiaRemoveDeviceByPathCommandResult,
    NiaInterpreterCommandResult::RemoveDeviceByPath
);
make_from_impl!(
    NiaRemoveModifierCommandResult,
    NiaInterpreterCommandResult::RemoveModifier
);

impl std::fmt::Display for NiaInterpreterCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
