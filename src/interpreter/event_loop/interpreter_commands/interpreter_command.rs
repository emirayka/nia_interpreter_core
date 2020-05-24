use crate::interpreter::event_loop::interpreter_commands::*;

use crate::{Action, Key, Mapping};
use crate::{KeyChord, ModifierDescription};

#[derive(Clone, Debug)]
pub enum NiaInterpreterCommand {
    DefineDevice(NiaDefineDeviceCommand),
    DefineModifier(NiaDefineModifierCommand),
    ExecuteCode(NiaExecuteCodeCommand),
    GetDefinedModifiers(NiaGetDefinedModifiersCommand),
    RemoveDefineDeviceByName(NiaRemoveDeviceByNameCommand),
    RemoveDeviceByPath(NiaRemoveDeviceByPathCommand),
    RemoveModifier(NiaRemoveModifierCommand),
    GetDefinedActions(NiaGetDefinedActionsCommand),
    DefineAction(NiaDefineActionCommand),
    RemoveAction(NiaRemoveActionCommand),
    GetDefinedMappings(NiaGetDefinedMappingsCommand),
    DefineMapping(NiaDefineMappingCommand),
    ChangeMapping(NiaChangeMappingCommand),
    RemoveMapping(NiaRemoveMappingCommand),
    IsListening(NiaIsListeningCommand),
    StartListening(NiaStartListeningCommand),
    StopListening(NiaStopListeningCommand),
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

    pub fn make_define_modifier_command(
        modifier: ModifierDescription,
    ) -> NiaInterpreterCommand {
        NiaInterpreterCommand::DefineModifier(NiaDefineModifierCommand::new(
            modifier,
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

    pub fn make_remove_modifier_command(key: Key) -> NiaInterpreterCommand {
        NiaInterpreterCommand::RemoveModifier(NiaRemoveModifierCommand::new(
            key,
        ))
    }

    pub fn make_get_defined_actions_command() -> NiaInterpreterCommand {
        let get_defined_actions_command = NiaGetDefinedActionsCommand::new();

        NiaInterpreterCommand::GetDefinedActions(get_defined_actions_command)
    }

    pub fn make_remove_action_command<S>(
        action_name: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let remove_action_command = NiaRemoveActionCommand::new(action_name);

        NiaInterpreterCommand::RemoveAction(remove_action_command)
    }

    pub fn make_get_defined_mappings_command() -> NiaInterpreterCommand {
        let get_defined_mappings_command = NiaGetDefinedMappingsCommand::new();

        NiaInterpreterCommand::GetDefinedMappings(get_defined_mappings_command)
    }

    pub fn make_define_mapping_command(
        mapping: Mapping,
    ) -> NiaInterpreterCommand {
        let define_mapping_command = NiaDefineMappingCommand::new(mapping);

        NiaInterpreterCommand::DefineMapping(define_mapping_command)
    }
    pub fn make_change_mapping_command(
        key_chords: Vec<KeyChord>,
        action: Action,
    ) -> NiaInterpreterCommand {
        let change_mapping_command =
            NiaChangeMappingCommand::new(key_chords, action);

        NiaInterpreterCommand::ChangeMapping(change_mapping_command)
    }

    pub fn make_remove_mapping_command(
        key_chord_sequence: Vec<KeyChord>,
    ) -> NiaInterpreterCommand {
        let remove_mapping_command =
            NiaRemoveMappingCommand::new(key_chord_sequence);

        NiaInterpreterCommand::RemoveMapping(remove_mapping_command)
    }

    pub fn make_is_listening_command() -> NiaInterpreterCommand {
        let is_listening_command = NiaIsListeningCommand::new();

        NiaInterpreterCommand::IsListening(is_listening_command)
    }

    pub fn make_start_listening_command() -> NiaInterpreterCommand {
        let start_listening_command = NiaStartListeningCommand::new();

        NiaInterpreterCommand::StartListening(start_listening_command)
    }

    pub fn make_stop_listening_command() -> NiaInterpreterCommand {
        let stop_listening_command = NiaStopListeningCommand::new();

        NiaInterpreterCommand::StopListening(stop_listening_command)
    }
}

// define action commands
impl NiaInterpreterCommand {
    pub fn make_define_key_press_action_command<S>(
        action_name: S,
        key_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action = Action::KeyPress(key_code).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_key_click_action_command<S>(
        action_name: S,
        key_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action = Action::KeyClick(key_code).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_key_release_action_command<S>(
        action_name: S,
        key_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action = Action::KeyRelease(key_code).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_mouse_button_press_action_command<S>(
        action_name: S,
        mouse_button_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action =
            Action::MouseButtonPress(mouse_button_code).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_mouse_button_click_action_command<S>(
        action_name: S,
        mouse_button_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action =
            Action::MouseButtonClick(mouse_button_code).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_mouse_button_release_action_command<S>(
        action_name: S,
        mouse_button_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action = Action::MouseButtonRelease(mouse_button_code)
            .into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_mouse_absolute_move_action_command<S>(
        action_name: S,
        x: i32,
        y: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action =
            Action::MouseAbsoluteMove(x, y).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_mouse_relative_move_action_command<S>(
        action_name: S,
        dx: i32,
        dy: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action =
            Action::MouseRelativeMove(dx, dy).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_text_type_action_command<S>(
        action_name: S,
        text_to_type: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action =
            Action::TextType(text_to_type.into()).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_wait_action_command<S>(
        action_name: S,
        ms_amount: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action =
            Action::Wait(ms_amount.into()).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_execute_code_action_command<S>(
        action_name: S,
        code_to_execute: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action =
            Action::ExecuteCode(code_to_execute.into()).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_execute_function_action_command<S>(
        action_name: S,
        function_name_to_execute: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action =
            Action::ExecuteFunction(function_name_to_execute.into())
                .into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_execute_os_command_action_command<S>(
        action_name: S,
        os_command: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let named_action =
            Action::ExecuteOSCommand(os_command.into()).into_named(action_name);

        let define_action_command = NiaDefineActionCommand::new(named_action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }
}
