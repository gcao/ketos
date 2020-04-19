//! Macros

/// Create a Value::Integer value
#[macro_export]
macro_rules! vint {
    ($value:expr) => (Value::Integer(Integer(BigInt::from($value))));
}

/// Create a **HashMap** from a list of key-value pairs
#[macro_export]
macro_rules! map {
    // trailing comma case
    ($($key:ident : $value:expr,)+) => (map!($($key : $value),+));

    ( $($key:ident : $value:expr),* ) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                let _ = _map.insert(stringify!($key).into(), $value);
            )*
            _map
        }
    };
}