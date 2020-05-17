use crate::library;

#[macro_export]
macro_rules! nia_alist {
    ($interpreter:ident) => {
        {
            let alist = library::alist_new(&mut $interpreter).unwrap();
            alist
        }
    };
    ($interpreter:ident, $(($key:expr, $value:expr)),*) => {
        {
            let alist = library::alist_new(&mut $interpreter).unwrap();

            $(
                let alist = library::alist_acons(
                    &mut $interpreter,
                    $key,
                    $value,
                    alist
                ).unwrap();
            )*

            alist
        }
    };
}
