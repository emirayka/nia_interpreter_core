use crate::{Value, Error};

const OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE: u8 = 0x1;
const OBJECT_VALUE_WRAPPER_FLAG_WRITABLE: u8 = 0x2;
const OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE: u8 = 0x4;
const OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE: u8 = 0x80;

const OBJECT_VALUE_WRAPPER_FLAG_DEFAULT: u8 = OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE |
    OBJECT_VALUE_WRAPPER_FLAG_WRITABLE |
    OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE |
    OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ObjectValueWrapper {
    value: Value,
    flags: u8,
}

impl ObjectValueWrapper {
    pub fn with_flags(value: Value, flags: u8) -> ObjectValueWrapper {
        ObjectValueWrapper {
            value,
            flags,
        }
    }

    pub fn new(value: Value) -> ObjectValueWrapper {
        ObjectValueWrapper::with_flags(value, OBJECT_VALUE_WRAPPER_FLAG_DEFAULT)
    }

    pub fn is_internable(&self) -> bool {
        self.flags & OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE != 0
    }

    pub fn is_writable(&self) -> bool {
        self.flags & OBJECT_VALUE_WRAPPER_FLAG_WRITABLE != 0
    }

    pub fn is_enumerable(&self) -> bool {
        self.flags & OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE != 0
    }

    pub fn is_configurable(&self) -> bool {
        self.flags & OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE != 0
    }

    fn check_is_internable(&self) -> Result<(), Error> {
        if self.is_internable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot intern not internable item from an object."
            ).into()
        }
    }

    fn check_is_settable(&self) -> Result<(), Error> {
        if self.is_writable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot overwrite not writable item of an object."
            ).into()
        }
    }

    fn check_is_enumerable(&self) -> Result<(), Error> {
        if self.is_enumerable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot enumerate not enumerable item of an object."
            ).into()
        }
    }

    fn check_is_configurable(&self) -> Result<(), Error> {
        if self.is_configurable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot configure not configurable item of an object."
            ).into()
        }
    }

    pub fn get_flags(&self) -> u8 {
        self.flags
    }

    pub fn set_flags(&mut self, flags: u8) -> Result<(), Error> {
        self.check_is_configurable()?;
        self.flags = flags;

        Ok(())
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), Error> {
        self.check_is_settable()?;
        self.value = value;

        Ok(())
    }

    pub fn get_value(&self) -> Result<Value, Error> {
        self.check_is_internable()?;

        Ok(self.value)
    }

    // must be used carefully
    pub fn force_get_value(&self) -> Value {
        self.value
    }

    pub fn force_set_value(&mut self, value: Value) {
        self.value = value;
    }

    pub fn force_set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod get_value {
        use super::*;
        #[test]
        fn gets_value() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAG_DEFAULT
            );

            assert_eq!(Ok(Value::Integer(1)), value_wrapper.get_value());
        }

        #[test]
        fn returns_error_during_internation_of_not_internable_binding() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAG_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE
            );

            assert!(value_wrapper.get_value().is_err())
        }
    }

    #[cfg(test)]
    mod set_value {
        use super::*;

        #[test]
        fn sets_value() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAG_DEFAULT
            );

            assert!(value_wrapper.set_value(Value::Integer(2)).is_ok());
            assert_eq!(Ok(Value::Integer(2)), value_wrapper.get_value());
        }

        #[test]
        fn returns_error_during_setting_of_not_writable_binding() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAG_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_WRITABLE
            );

            assert!(value_wrapper.set_value(Value::Integer(2)).is_err())
        }
    }

    #[cfg(test)]
    mod set_flags {
        use super::*;

        #[test]
        fn sets_flags() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAG_DEFAULT
            );

            assert!(value_wrapper.set_flags(0).is_ok());
            assert_eq!(0, value_wrapper.get_flags());
        }

        #[test]
        fn returns_error_during_configuring_of_not_configurable_binding() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAG_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE
            );

            assert!(value_wrapper.set_flags(OBJECT_VALUE_WRAPPER_FLAG_DEFAULT).is_err())
        }
    }
}
