//! # Fiv-Date
//! 
//! Fiv-Date is a Library for performatly Implementing Custom Time Format Structs
//! 
//! ## Ideal Usage
//!  - You want use optimized custom Date Format
//!  - You want ISO 8601 with the capability to later use custom Formats
//!  - You have no idea what to use
//! 
//! ## Usage
//! 
//! ```toml
//! [dependencies]
//! fiv-date={git="The Git URL of this project",rev="The full SHA of the special Commit (eg. b4d115cabdb528a537a293a4bcd82c4aa743cbe4)"}
//! ```
//! 
//! If you want a custom Time Format:
//! ```edition2021
//! #[macro_use]
//! use fiv_date::custom_format_struct;
//! //     ^
//! //   !!IMPORTANT!!
//! use std::time::SystemTime;
//! 
//! 
//! custom_format_struct!(My_Struct, "Custom Format: {{Day: {DD}, Month: {MM}, Year: {YYYY}}}")
//! 
//! fn main() {
//!     let now = SystemTime::now();
//!     let custom_struct = MyStruct::now(&now);
//!     println!("{custom_struct}"); 
//!     // This would print the actual time: "Custom Format: {Day: 01, Month: 01, Year: 1970}"
//!     
//!     // Also backwards compatible (SOON)
//!     let date = "Custom Format: {Day: 01, Month: 01, Year: 1970}";
//!     let custom_struct: My_Struct = date.parse();
//!     println!("{custom_struct}");
//! }   
//! ```
//! 
//! If you want ISO 8601:
//! ```edition2021
//! use fiv_date::iso_8601::Basic_Date;
//! //     ^
//! //   !!IMPORTANT!!
//! use std::time::SystemTime;
//!
//! 
//! fn main() {
//!     let now = SystemTime::now();
//!     let iso = Basic_Date::now(&now);
//!     println!("{iso}");
//!     // This would print the actual time: "19700101"
//!     
//!     // Also backwards compatible (SOON)
//!     let date = "19700101";
//!     let iso: Basic_Date = date.parse();
//!     println!("{iso}");
//! }   
//! ```

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
pub use date_macro::custom_format_struct;

pub mod tests;
pub mod traits;

pub mod cal;
pub mod clo;
/// Implements the Display Trait for a given struct
/// 
/// This struct must follow this dirrective
/// ```
///     #[macro_use]
///     use fiv_date::format_inner;
/// 
///     struct MyStruct(T);
///     //T must implement Display
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
