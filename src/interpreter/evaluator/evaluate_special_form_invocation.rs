use crate::EnvironmentId;
use crate::Error;
use crate::Interpreter;
use crate::SpecialFormFunction;
use crate::Value;

pub fn evaluate_special_form_invocation(
    interpreter: &mut Interpreter,
    execution_environment: EnvironmentId,
    special_form: &SpecialFormFunction,
    arguments: Vec<Value>,
) -> Result<Value, Error> {
    (special_form.get_func())(interpreter, execution_environment, arguments)
}
