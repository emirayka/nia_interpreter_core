use crate::Value;
use crate::Error;

const ENVIRONMENT_VALUE_WRAPPER_FLAG_INTERNABLE: u8 = 0x1;
const ENVIRONMENT_VALUE_WRAPPER_FLAG_WRITABLE: u8 = 0x2;
const ENVIRONMENT_VALUE_WRAPPER_FLAG_CONFIGURABLE: u8 = 0x80;

const ENVIRONMENT_VALUE_WRAPPER_FLAG_DEFAULT: u8 = ENVIRONMENT_VALUE_WRAPPER_FLAG_WRITABLE |
    ENVIRONMENT_VALUE_WRAPPER_FLAG_INTERNABLE |
    ENVIRONMENT_VALUE_WRAPPER_FLAG_CONFIGURABLE;

const ENVIRONMENT_VALUE_WRAPPER_FLAG_CONST: u8 = ENVIRONMENT_VALUE_WRAPPER_FLAG_INTERNABLE;

#[derive(Copy, Clone, Debug)]
pub struct EnvironmentValueWrapper {
    value: Value,
    flags: u8,
}

impl EnvironmentValueWrapper {
    pub fn with_flags(value: Value, flags: u8) -> EnvironmentValueWrapper {
        EnvironmentValueWrapper {
            value,
            flags,
        }
    }

    pub fn new(value: Value) -> EnvironmentValueWrapper {
        EnvironmentValueWrapper::with_flags(value, ENVIRONMENT_VALUE_WRAPPER_FLAG_DEFAULT)
    }

    pub fn new_const(value: Value) -> EnvironmentValueWrapper {
        EnvironmentValueWrapper::with_flags(value, ENVIRONMENT_VALUE_WRAPPER_FLAG_CONST)
    }

    pub fn is_internable(&self) -> bool {
        self.flags & ENVIRONMENT_VALUE_WRAPPER_FLAG_INTERNABLE != 0
    }

    pub fn is_writable(&self) -> bool {
        self.flags & ENVIRONMENT_VALUE_WRAPPER_FLAG_WRITABLE != 0
    }

    pub fn is_configurable(&self) -> bool {
        self.flags & ENVIRONMENT_VALUE_WRAPPER_FLAG_CONFIGURABLE != 0
    }

    fn check_is_gettable(&self) -> Result<(), Error> {
        if self.is_internable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot intern not internable item."
            ).into()
        }
    }

    fn check_is_settable(&self) -> Result<(), Error> {
        if self.is_writable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot change const item."
            ).into()
        }
    }

    fn check_is_configurable(&self) -> Result<(), Error> {
        if self.is_configurable() {
            Ok(())
        } else {
            Error::generic_execution_error(
                "Cannot configure not configurable item."
            ).into()
        }
    }

    pub fn get_flags(&mut self) -> u8 {
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
        self.check_is_gettable()?;

        Ok(self.value)
    }

    // must be used by LexicalEnvironment only
    pub fn to_value(&self) -> Value {
        self.value
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
            let mut value_wrapper = EnvironmentValueWrapper::with_flags(
                Value::Integer(1),
                ENVIRONMENT_VALUE_WRAPPER_FLAG_DEFAULT
            );

            assert_eq!(Ok(Value::Integer(1)), value_wrapper.get_value());
        }

        #[test]
        fn returns_error_during_internation_of_not_internable_binding() {
            let mut value_wrapper = EnvironmentValueWrapper::with_flags(
                Value::Integer(1),
                ENVIRONMENT_VALUE_WRAPPER_FLAG_DEFAULT ^ ENVIRONMENT_VALUE_WRAPPER_FLAG_INTERNABLE
            );

            assert!(value_wrapper.get_value().is_err())
        }
    }

    #[cfg(test)]
    mod set_value {
        use super::*;

        #[test]
        fn sets_value() {
            let mut value_wrapper = EnvironmentValueWrapper::with_flags(
                Value::Integer(1),
                ENVIRONMENT_VALUE_WRAPPER_FLAG_DEFAULT
            );

            assert!(value_wrapper.set_value(Value::Integer(2)).is_ok());
            assert_eq!(Ok(Value::Integer(2)), value_wrapper.get_value());
        }

        #[test]
        fn returns_error_during_setting_of_not_writable_binding() {
            let mut value_wrapper = EnvironmentValueWrapper::with_flags(
                Value::Integer(1),
                ENVIRONMENT_VALUE_WRAPPER_FLAG_DEFAULT ^ ENVIRONMENT_VALUE_WRAPPER_FLAG_WRITABLE
            );

            assert!(value_wrapper.set_value(Value::Integer(2)).is_err())
        }
    }

    #[cfg(test)]
    mod set_flags {
        use super::*;

        #[test]
        fn sets_flags() {
            let mut value_wrapper = EnvironmentValueWrapper::with_flags(
                Value::Integer(1),
                ENVIRONMENT_VALUE_WRAPPER_FLAG_DEFAULT
            );

            assert!(value_wrapper.set_flags(0).is_ok());
            assert_eq!(0, value_wrapper.get_flags());
        }

        #[test]
        fn returns_error_during_configuring_of_not_configurable_binding() {
            let mut value_wrapper = EnvironmentValueWrapper::with_flags(
                Value::Integer(1),
                ENVIRONMENT_VALUE_WRAPPER_FLAG_DEFAULT ^ ENVIRONMENT_VALUE_WRAPPER_FLAG_CONFIGURABLE
            );

            assert!(value_wrapper.set_flags(ENVIRONMENT_VALUE_WRAPPER_FLAG_DEFAULT).is_err())
        }
    }

}
