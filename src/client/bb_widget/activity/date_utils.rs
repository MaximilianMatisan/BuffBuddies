use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, EnumIter, Display, Eq, PartialEq)]
pub enum DateScope {
    Year,
    Month,
    Week
}
#[derive(Debug, Clone, Copy, EnumIter, Display, Eq, PartialEq)]
pub enum Offset {
    Current,
    Previous,
    BeforePrevious
}