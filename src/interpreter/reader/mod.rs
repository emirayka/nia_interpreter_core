mod read_boolean_element;
mod read_delimited_symbols_element;
mod read_element;
mod read_elements;
mod read_float_element;
mod read_integer_element;
mod read_keyword_element;
mod read_object_element;
mod read_object_pattern_element;
mod read_prefixed_element;
mod read_s_expression_element;
mod read_short_lambda_element;
mod read_string_element;
mod read_symbol_element;

pub use read_element::read_element;
pub use read_elements::read_elements;

// macro_rules! assert_reading_result_equal {
//     ($expected:expr, $code:expr) => {
//         let mut interpreter = Interpreter::new();
//         let expected = $expected;
//
//         if let Ok(program) = parse($code) {
//             let result =
//                 read_elements(&mut interpreter, program.get_elements())
//                     .unwrap();
//
//             let len = expected.len();
//
//             nia_assert_equal(len, result.len());
//
//             for i in 0..len {
//                 let expected = expected[i];
//                 let result = result[i];
//
//                 utils::assert_deep_equal(
//                     &mut interpreter,
//                     expected,
//                     result,
//                 );
//             }
//         }
//     };
// }
