use crate::DeviceInfo;
use crate::Error;
use crate::Interpreter;
use crate::Value;

use crate::library;

pub fn list_to_device_info(
    interpreter: &Interpreter,
    device_info_list: Value,
) -> Result<DeviceInfo, Error> {
    let values = library::read_as_vector(interpreter, device_info_list)?;

    if values.len() != 3 {
        return Error::invalid_argument_error("Expected three value list.")
            .into();
    }

    let device_id = library::read_as_i64(values[0])?;
    let device_path = library::read_as_string(interpreter, values[1])?.clone();
    let device_name = library::read_as_string(interpreter, values[2])?.clone();

    let device_info =
        DeviceInfo::new(device_id as i32, device_path, device_name);

    Ok(device_info)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn returns_correct_device_info() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                DeviceInfo::new(1, "/dev/input/event6", "name"),
                "'(1 \"/dev/input/event6\" \"name\")",
            ),
            (
                DeviceInfo::new(2, "/dev/input/event6", "name"),
                "'(2 \"/dev/input/event6\" \"name\")",
            ),
            (
                DeviceInfo::new(1, "/dev/input/event7", "name"),
                "'(1 \"/dev/input/event7\" \"name\")",
            ),
            (
                DeviceInfo::new(1, "/dev/input/event6", "name2"),
                "'(1 \"/dev/input/event6\" \"name2\")",
            ),
        ];

        for (expected, code) in specs {
            let value = interpreter.execute_in_main_environment(code).unwrap();
            let result = list_to_device_info(&mut interpreter, value).unwrap();

            nia_assert_equal(expected, result);
        }
    }

    #[test]
    fn returns_invalid_argument_error_when_value_is_not_a_three_element_list() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "1",
            "1.1",
            "#t",
            "#f",
            "\"string\"",
            ":keyword",
            "'symbol",
            "'()",
            "'(1)",
            "'(1 2)",
            "'(1 2 3 4)",
            "{}",
            "#()",
        ];

        for spec in specs {
            let value = interpreter.execute_in_main_environment(spec).unwrap();
            let result = list_to_device_info(&interpreter, value);

            crate::utils::assert_invalid_argument_error(&result);
        }
    }

    #[test]
    fn returns_invalid_argument_error_when_list_cannot_be_parsed_as_device_info(
    ) {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            "(list:new 1.1 \"/dev/input/event6\" \"name\")",
            "(list:new #t \"/dev/input/event6\" \"name\")",
            "(list:new #f \"/dev/input/event6\" \"name\")",
            "(list:new \"string\" \"/dev/input/event6\" \"name\")",
            "(list:new :keyword \"/dev/input/event6\" \"name\")",
            "(list:new 'symbol \"/dev/input/event6\" \"name\")",
            "(list:new '(list) \"/dev/input/event6\" \"name\")",
            "(list:new {} \"/dev/input/event6\" \"name\")",
            "(list:new #() \"/dev/input/event6\" \"name\")",
            "(list:new 1 1 \"name\")",
            "(list:new 1 1.1 \"name\")",
            "(list:new 1 #t \"name\")",
            "(list:new 1 #f \"name\")",
            "(list:new 1 :keyword \"name\")",
            "(list:new 1 'symbol \"name\")",
            "(list:new 1 '(list) \"name\")",
            "(list:new 1 {} \"name\")",
            "(list:new 1 #() \"name\")",
            "(list:new 1 \"/dev/input/event6\" 1)",
            "(list:new 1 \"/dev/input/event6\" 1.1)",
            "(list:new 1 \"/dev/input/event6\" #t)",
            "(list:new 1 \"/dev/input/event6\" #f)",
            "(list:new 1 \"/dev/input/event6\" :keyword)",
            "(list:new 1 \"/dev/input/event6\" 'symbol)",
            "(list:new 1 \"/dev/input/event6\" '(list))",
            "(list:new 1 \"/dev/input/event6\" {})",
            "(list:new 1 \"/dev/input/event6\" #())",
        ];

        for spec in specs {
            let value = interpreter.execute_in_main_environment(spec).unwrap();
            let result = list_to_device_info(&interpreter, value);

            crate::utils::assert_invalid_argument_error(&result);
        }
    }
}
