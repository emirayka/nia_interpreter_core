use crate::interpreter::event_loop::interpreter_commands::*;

#[derive(Clone, Debug)]
pub enum NiaInterpreterCommand {
    DefineDevice(NiaDefineDeviceCommand),
    DefineModifier(NiaDefineModifierCommand),
    ExecuteCode(NiaExecuteCodeCommand),
    GetDefinedModifiers(NiaGetDefinedModifiersCommand),
    RemoveDefineDeviceByName(NiaRemoveDeviceByNameCommand),
    RemoveDeviceByPath(NiaRemoveDeviceByPathCommand),
    RemoveModifier(NiaRemoveModifierCommand),
}

impl NiaInterpreterCommand {
    pub fn make_define_device_command<S>(
        device_id: i32,
        device_path: S,
        device_name: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::DefineDevice(NiaDefineDeviceCommand::new(
            device_id,
            device_path,
            device_name,
        ))
    }

    pub fn make_define_modifier_command<S>(
        device_id: i32,
        key_code: i32,
        modifier_alias: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::DefineModifier(NiaDefineModifierCommand::new(
            device_id,
            key_code,
            modifier_alias,
        ))
    }

    pub fn make_execute_code_command<S>(code: S) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::ExecuteCode(NiaExecuteCodeCommand::new(code))
    }

    pub fn make_get_defined_modifiers() -> NiaInterpreterCommand {
        NiaInterpreterCommand::GetDefinedModifiers(
            NiaGetDefinedModifiersCommand::new(),
        )
    }

    pub fn make_remove_device_by_name_command<S>(
        device_name: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::RemoveDefineDeviceByName(
            NiaRemoveDeviceByNameCommand::new(device_name),
        )
    }

    pub fn make_remove_device_by_path_command<S>(
        device_path: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::RemoveDeviceByPath(
            NiaRemoveDeviceByPathCommand::new(device_path),
        )
    }

    pub fn make_remove_modifier_command(
        device_id: i32,
        key_code: i32,
    ) -> NiaInterpreterCommand {
        NiaInterpreterCommand::RemoveModifier(NiaRemoveModifierCommand::new(
            device_id, key_code,
        ))
    }
}
