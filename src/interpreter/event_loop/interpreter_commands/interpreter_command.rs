use crate::interpreter::event_loop::interpreter_commands::*;

#[derive(Clone, Debug)]
pub enum NiaInterpreterCommand {
    DefineKeyboard(NiaDefineKeyboardCommand),
    DefineModifier(NiaDefineModifierCommand),
    ExecuteCode(NiaExecuteCodeCommand),
    GetDefinedModifiers(NiaGetDefinedModifiersCommand),
    RemoveKeyboardByName(NiaRemoveKeyboardByNameCommand),
    RemoveKeyboardByPath(NiaRemoveKeyboardByPathCommand),
    RemoveModifier(NiaRemoveModifierCommand),
}

impl NiaInterpreterCommand {
    pub fn make_define_keyboard_command<S>(
        keyboard_path: S,
        keyboard_name: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::DefineKeyboard(NiaDefineKeyboardCommand::new(
            keyboard_path,
            keyboard_name,
        ))
    }

    pub fn make_define_modifier_command<S>(
        keyboard_path: S,
        key_code: i32,
        modifier_alias: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::DefineModifier(NiaDefineModifierCommand::new(
            keyboard_path,
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

    pub fn make_remove_keyboard_by_name_command<S>(
        keyboard_name: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::RemoveKeyboardByName(
            NiaRemoveKeyboardByNameCommand::new(keyboard_name),
        )
    }

    pub fn make_remove_keyboard_by_path_command<S>(
        keyboard_path: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::RemoveKeyboardByPath(
            NiaRemoveKeyboardByPathCommand::new(keyboard_path),
        )
    }

    pub fn make_remove_modifier_command<S>(
        keyboard_path: S,
        key_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        NiaInterpreterCommand::RemoveModifier(NiaRemoveModifierCommand::new(
            keyboard_path,
            key_code,
        ))
    }
}
