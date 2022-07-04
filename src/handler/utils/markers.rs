use std::fmt::{Display, Formatter};

/// Just an empty struct used to mark generic type if nothing is expected
#[derive(Copy, Clone, Debug)]
pub struct Empty;

impl Display for Empty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
