use crate::{Error, Value};

pub const OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE: u8 = 0x1;
pub const OBJECT_VALUE_WRAPPER_FLAG_WRITABLE: u8 = 0x2;
pub const OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE: u8 = 0x4;
pub const OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE: u8 = 0x80;

pub const OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT: u8 = OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE
    | OBJECT_VALUE_WRAPPER_FLAG_WRITABLE
    | OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE
    | OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE;

pub const OBJECT_VALUE_WRAPPER_FLAGS_NONE: u8 = 0x0;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ObjectValueWrapper {
    value: Value,
    flags: u8,
}

impl ObjectValueWrapper {
    pub fn with_flags(value: Value, flags: u8) -> ObjectValueWrapper {
        ObjectValueWrapper { value, flags }
    }

    pub fn new(value: Value) -> ObjectValueWrapper {
        ObjectValueWrapper::with_flags(value, OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT)
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
            Error::generic_execution_error("Cannot intern not internable item from an object.")
                .into()
        }
    }

    fn check_is_settable(&self) -> Result<(), Error> {
        if self.is_writable() {
            Ok(())
        } else {
            Error::generic_execution_error("Cannot overwrite not writable item of an object.")
                .into()
        }
    }

    fn check_is_enumerable(&self) -> Result<(), Error> {
        if self.is_enumerable() {
            Ok(())
        } else {
            Error::generic_execution_error("Cannot enumerate not enumerable item of an object.")
                .into()
        }
    }

    fn check_is_configurable(&self) -> Result<(), Error> {
        if self.is_configurable() {
            Ok(())
        } else {
            Error::generic_execution_error("Cannot configure not configurable item of an object.")
                .into()
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

    pub fn get_flag(&self, flag: u8) -> bool {
        self.flags & flag != 0
    }

    pub fn set_flag(&mut self, flag: u8, flag_value: bool) -> Result<(), Error> {
        self.check_is_configurable()?;

        if flag_value {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }

        Ok(())
    }

    pub fn set_internable(&mut self, internable: bool) -> Result<(), Error> {
        self.set_flag(OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE, internable)
    }

    pub fn set_writable(&mut self, writable: bool) -> Result<(), Error> {
        self.set_flag(OBJECT_VALUE_WRAPPER_FLAG_WRITABLE, writable)
    }

    pub fn set_enumerable(&mut self, enumerable: bool) -> Result<(), Error> {
        self.set_flag(OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE, enumerable)
    }

    pub fn set_configurable(&mut self, configurable: bool) -> Result<(), Error> {
        self.set_flag(OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE, configurable)
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

    #[allow(unused_imports)]
    use nia_basic_assertions::*;

    #[cfg(test)]
    mod get_value {
        use super::*;
        #[test]
        fn gets_value() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT,
            );

            nia_assert_equal(Ok(Value::Integer(1)), value_wrapper.get_value());
        }

        #[test]
        fn returns_error_during_internation_of_not_internable_binding() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE,
            );

            nia_assert(value_wrapper.get_value().is_err())
        }
    }

    #[cfg(test)]
    mod set_value {
        use super::*;

        #[test]
        fn sets_value() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT,
            );

            nia_assert(value_wrapper.set_value(Value::Integer(2)).is_ok());
            nia_assert_equal(Ok(Value::Integer(2)), value_wrapper.get_value());
        }

        #[test]
        fn returns_error_during_setting_of_not_writable_binding() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_WRITABLE,
            );

            nia_assert(value_wrapper.set_value(Value::Integer(2)).is_err())
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_flags__get_flags {
        use super::*;

        #[test]
        fn sets_flags() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT,
            );

            nia_assert_equal(
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT,
                value_wrapper.get_flags(),
            );
            nia_assert(value_wrapper.set_flags(0).is_ok());
            nia_assert_equal(0, value_wrapper.get_flags());
        }

        #[test]
        fn returns_error_during_configuring_of_not_configurable_binding() {
            let flags = OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE;
            let mut value_wrapper = ObjectValueWrapper::with_flags(Value::Integer(1), flags);

            nia_assert_equal(flags, value_wrapper.get_flags());
            nia_assert(
                value_wrapper
                    .set_flags(OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT)
                    .is_err(),
            );
            nia_assert_equal(flags, value_wrapper.get_flags());
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_flag__get_flag {
        use super::*;

        #[allow(non_snake_case)]
        #[test]
        fn gets_and_sets_flags__gets_flags() {
            let mut value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(1),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT,
            );

            nia_assert_equal(
                true,
                value_wrapper.get_flag(OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE),
            );
            nia_assert_equal(
                true,
                value_wrapper.get_flag(OBJECT_VALUE_WRAPPER_FLAG_WRITABLE),
            );
            nia_assert_equal(
                true,
                value_wrapper.get_flag(OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE),
            );
            nia_assert_equal(
                true,
                value_wrapper.get_flag(OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE),
            );

            nia_assert_equal(
                Ok(()),
                value_wrapper.set_flag(OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE, false),
            );
            nia_assert_equal(
                Ok(()),
                value_wrapper.set_flag(OBJECT_VALUE_WRAPPER_FLAG_WRITABLE, false),
            );
            nia_assert_equal(
                Ok(()),
                value_wrapper.set_flag(OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE, false),
            );
            nia_assert_equal(
                Ok(()),
                value_wrapper.set_flag(OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE, false),
            );

            nia_assert_equal(
                false,
                value_wrapper.get_flag(OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE),
            );
            nia_assert_equal(
                false,
                value_wrapper.get_flag(OBJECT_VALUE_WRAPPER_FLAG_WRITABLE),
            );
            nia_assert_equal(
                false,
                value_wrapper.get_flag(OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE),
            );
            nia_assert_equal(
                false,
                value_wrapper.get_flag(OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE),
            );

            nia_assert(
                value_wrapper
                    .set_flag(OBJECT_VALUE_WRAPPER_FLAG_INTERNABLE, false)
                    .is_err(),
            );
            nia_assert(
                value_wrapper
                    .set_flag(OBJECT_VALUE_WRAPPER_FLAG_WRITABLE, false)
                    .is_err(),
            );
            nia_assert(
                value_wrapper
                    .set_flag(OBJECT_VALUE_WRAPPER_FLAG_ENUMERABLE, false)
                    .is_err(),
            );
            nia_assert(
                value_wrapper
                    .set_flag(OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE, false)
                    .is_err(),
            );
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_internable__is_internable {
        use super::*;

        #[test]
        fn gets_and_sets_internable_flag() {
            let mut object_value_wrapper = ObjectValueWrapper::new(Value::Integer(0));

            nia_assert_equal(true, object_value_wrapper.is_internable());

            nia_assert_equal(Ok(()), object_value_wrapper.set_internable(false));
            nia_assert_equal(false, object_value_wrapper.is_internable());

            nia_assert_equal(Ok(()), object_value_wrapper.set_internable(true));
            nia_assert_equal(true, object_value_wrapper.is_internable());
        }

        #[test]
        fn returns_error_when_attempts_to_change_not_configurable_value_wrapper() {
            let mut object_value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(0),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE,
            );

            nia_assert_equal(true, object_value_wrapper.is_internable());
            nia_assert(object_value_wrapper.set_internable(false).is_err());
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_writable__is_writable {
        use super::*;

        #[test]
        fn gets_and_sets_writable_flag() {
            let mut object_value_wrapper = ObjectValueWrapper::new(Value::Integer(0));

            nia_assert_equal(true, object_value_wrapper.is_writable());

            nia_assert_equal(Ok(()), object_value_wrapper.set_writable(false));
            nia_assert_equal(false, object_value_wrapper.is_writable());

            nia_assert_equal(Ok(()), object_value_wrapper.set_writable(true));
            nia_assert_equal(true, object_value_wrapper.is_writable());
        }

        #[test]
        fn returns_error_when_attempts_to_change_not_configurable_value_wrapper() {
            let mut object_value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(0),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE,
            );

            nia_assert_equal(true, object_value_wrapper.is_writable());
            nia_assert(object_value_wrapper.set_writable(false).is_err());
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_enumerable__is_enumerable {
        use super::*;

        #[test]
        fn gets_and_sets_enumerable_flag() {
            let mut object_value_wrapper = ObjectValueWrapper::new(Value::Integer(0));

            nia_assert_equal(true, object_value_wrapper.is_enumerable());

            nia_assert_equal(Ok(()), object_value_wrapper.set_enumerable(false));
            nia_assert_equal(false, object_value_wrapper.is_enumerable());

            nia_assert_equal(Ok(()), object_value_wrapper.set_enumerable(true));
            nia_assert_equal(true, object_value_wrapper.is_enumerable());
        }

        #[test]
        fn returns_error_when_attempts_to_change_not_configurable_value_wrapper() {
            let mut object_value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(0),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE,
            );

            nia_assert_equal(true, object_value_wrapper.is_enumerable());
            nia_assert(object_value_wrapper.set_enumerable(false).is_err());
        }
    }

    #[allow(non_snake_case)]
    #[cfg(test)]
    mod set_configurable__is_configurable {
        use super::*;

        #[test]
        fn sets_configurable_flag() {
            let mut object_value_wrapper = ObjectValueWrapper::new(Value::Integer(0));

            nia_assert_equal(true, object_value_wrapper.is_configurable());

            nia_assert_equal(Ok(()), object_value_wrapper.set_configurable(false));
            nia_assert_equal(false, object_value_wrapper.is_configurable());
        }

        #[test]
        fn returns_error_when_attempts_to_change_not_configurable_value_wrapper() {
            let mut object_value_wrapper = ObjectValueWrapper::with_flags(
                Value::Integer(0),
                OBJECT_VALUE_WRAPPER_FLAGS_DEFAULT ^ OBJECT_VALUE_WRAPPER_FLAG_CONFIGURABLE,
            );

            nia_assert_equal(false, object_value_wrapper.is_configurable());
            nia_assert(object_value_wrapper.set_configurable(true).is_err());
        }
    }
}
