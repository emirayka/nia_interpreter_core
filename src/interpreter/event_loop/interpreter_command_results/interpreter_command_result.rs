use crate::interpreter::event_loop::interpreter_command_results::*;
use crate::interpreter::event_loop::interpreter_command_results::get_defined_mappings_command_result::NiaGetDefinedMappingsCommandResult;

#[derive(Clone, Debug)]
pub enum NiaInterpreterCommandResult {
    DefineDevice(NiaDefineDeviceCommandResult),
    DefineModifier(NiaDefineModifierCommandResult),
    ExecuteCode(NiaExecuteCodeCommandResult),
    GetDefinedModifiers(NiaGetDefinedModifiersCommandResult),
    RemoveDeviceByName(NiaRemoveDeviceByNameCommandResult),
    RemoveDeviceByPath(NiaRemoveDeviceByPathCommandResult),
    RemoveModifier(NiaRemoveModifierCommandResult),
    GetDefinedActions(NiaGetDefinedActionsCommandResult),
    DefineAction(NiaDefineActionCommandResult),
    RemoveAction(NiaRemoveActionCommandResult),
    GetDefinedMappings(NiaGetDefinedMappingsCommandResult),
    DefineMapping(NiaDefineMappingCommandResult),
    ChangeMapping(NiaChangeMappingCommandResult),
    RemoveMapping(NiaRemoveMappingCommandResult),
    IsListening(NiaIsListeningCommandResult),
    StartListening(NiaStartListeningCommandResult),
    StopListening(NiaStopListeningCommandResult),
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
make_from_impl!(
    NiaGetDefinedActionsCommandResult,
    NiaInterpreterCommandResult::GetDefinedActions
);
make_from_impl!(
    NiaDefineActionCommandResult,
    NiaInterpreterCommandResult::DefineAction
);
make_from_impl!(
    NiaRemoveActionCommandResult,
    NiaInterpreterCommandResult::RemoveAction
);
make_from_impl!(
    NiaGetDefinedMappingsCommandResult,
    NiaInterpreterCommandResult::GetDefinedMappings
);
make_from_impl!(
    NiaDefineMappingCommandResult,
    NiaInterpreterCommandResult::DefineMapping
);
make_from_impl!(
    NiaChangeMappingCommandResult,
    NiaInterpreterCommandResult::ChangeMapping
);
make_from_impl!(
    NiaRemoveMappingCommandResult,
    NiaInterpreterCommandResult::RemoveMapping
);
make_from_impl!(
    NiaIsListeningCommandResult,
    NiaInterpreterCommandResult::IsListening
);
make_from_impl!(
    NiaStartListeningCommandResult,
    NiaInterpreterCommandResult::StartListening
);
make_from_impl!(
    NiaStopListeningCommandResult,
    NiaInterpreterCommandResult::StopListening
);

impl std::fmt::Display for NiaInterpreterCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
