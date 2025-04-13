use subtype::{transform_as_validate_as_check, Check};

pub struct IsNumeric;

impl<T: IsNumericTrait> Check<T> for IsNumeric {
    fn check(value: &T) -> bool {
        value.is_numeric()
    }
}

transform_as_validate_as_check!(impl[T: IsNumericTrait] of [T] for IsNumeric);

pub trait IsNumericTrait {
    fn is_numeric(&self) -> bool;
}

impl IsNumericTrait for &str {
    fn is_numeric(&self) -> bool {
        self.chars().all(char::is_numeric)
    }
}

impl IsNumericTrait for String {
    fn is_numeric(&self) -> bool {
        self.as_str().is_numeric()
    }
}
