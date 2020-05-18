use crate::interpreter::event_loop::interpreter_commands::*;
use crate::Action;

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

    pub fn make_get_defined_actions() -> NiaInterpreterCommand {
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
}

// define action
impl NiaInterpreterCommand {
    pub fn make_define_key_press_action_command<S>(
        action_name: S,
        key_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::KeyPress(key_code);

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_key_click_action_command<S>(
        action_name: S,
        key_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::KeyClick(key_code);

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_key_release_action_command<S>(
        action_name: S,
        key_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::KeyRelease(key_code);

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_mouse_button_press_action_command<S>(
        action_name: S,
        mouse_button_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::MouseButtonPress(mouse_button_code);

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_mouse_button_click_action_command<S>(
        action_name: S,
        mouse_button_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::MouseButtonClick(mouse_button_code);

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_mouse_button_release_action_command<S>(
        action_name: S,
        mouse_button_code: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::MouseButtonRelease(mouse_button_code);

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

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
        let action = Action::MouseAbsoluteMove(x, y);

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

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
        let action = Action::MouseRelativeMove(dx, dy);

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_text_type_action_command<S>(
        action_name: S,
        text_to_type: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::TextType(text_to_type.into());

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_wait_action_command<S>(
        action_name: S,
        ms_amount: i32,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::Wait(ms_amount.into());

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_execute_code_action_command<S>(
        action_name: S,
        code_to_execute: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::ExecuteCode(code_to_execute.into());

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_execute_function_action_command<S>(
        action_name: S,
        function_name_to_execute: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::ExecuteFunction(function_name_to_execute.into());

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }

    pub fn make_define_execute_os_command_action_command<S>(
        action_name: S,
        os_command: S,
    ) -> NiaInterpreterCommand
    where
        S: Into<String>,
    {
        let action = Action::ExecuteOSCommand(os_command.into());

        let define_action_command =
            NiaDefineActionCommand::new(action_name, action);

        NiaInterpreterCommand::DefineAction(define_action_command)
    }
}
