//! Macros

use crate::integer::Integer;

#[macro_export]
/// Create a Value::Integer value
macro_rules! vint {
    ($value:expr) => (Value::Integer(Integer(BigInt::from($value))));
}
