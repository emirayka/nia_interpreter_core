use crate::DeviceInfo;
use crate::Error;
use crate::Interpreter;

use crate::library;

pub fn get_defined_devices_info(
    interpreter: &mut Interpreter,
) -> Result<Vec<DeviceInfo>, Error> {
    let device_list = library::get_defined_devices_list(interpreter)?;

    let device_info_values = library::read_as_vector(interpreter, device_list)?;
    let devices_info = device_info_values
        .into_iter()
        .map(|device_info_value| {
            library::list_to_device_info(interpreter, device_info_value)
        })
        .collect::<Result<Vec<DeviceInfo>, Error>>()?;

    Ok(devices_info)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[test]
    fn returns_defined_devices_info() {
        let mut interpreter = Interpreter::new();

        let specs = vec![
            (
                vec![DeviceInfo::new(1, "/dev/input/event1", "first")],
                1,
                "/dev/input/event1",
                "first",
            ),
            (
                vec![
                    DeviceInfo::new(2, "/dev/input/event2", "second"),
                    DeviceInfo::new(1, "/dev/input/event1", "first"),
                ],
                2,
                "/dev/input/event2",
                "second",
            ),
            (
                vec![
                    DeviceInfo::new(3, "/dev/input/event3", "third"),
                    DeviceInfo::new(2, "/dev/input/event2", "second"),
                    DeviceInfo::new(1, "/dev/input/event1", "first"),
                ],
                3,
                "/dev/input/event3",
                "third",
            ),
        ];

        for (expected, device_id, device_path, device_name) in specs {
            nia_assert_is_ok(&library::define_device(
                &mut interpreter,
                device_id,
                device_path,
                device_name,
            ));

            let result = get_defined_devices_info(&mut interpreter).unwrap();

            nia_assert_equal(expected, result);
        }
    }
}
