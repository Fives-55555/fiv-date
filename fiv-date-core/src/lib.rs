pub mod tests;
pub mod traits;

pub mod cal;
pub mod clo;

pub use crate::cal::CalDate;
pub use crate::cal::Day;
pub use crate::cal::Days;
pub use crate::cal::Month;
pub use crate::cal::Weekday;
pub use crate::cal::Weeks;
pub use crate::cal::Year;
pub use crate::clo::CloDate;
pub use crate::clo::Fraction;
pub use crate::clo::Hour;
pub use crate::clo::Minute;
pub use crate::clo::Second;
pub use crate::traits::Time;
pub use crate::ToDate;

/// Implements the Display Trait for a given struct
///
/// This struct must follow this dirrective
/// ```
///     #[macro_use]
///     use fiv_date::format_inner;
///
///     struct MyStruct(u8);
///     //The inner type must implement the Display Trait
///     format_inner!(MyStruct);
/// ```
#[macro_export]
macro_rules! format_inner {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}


trait ToDate {
    fn to_date(s: &str)->Result<(Self,&str),()>;
}