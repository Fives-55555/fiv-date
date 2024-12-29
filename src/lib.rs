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
//! custom_format_struct!(MyStruct, "Custom Format: {{Day: {DD}, Month: {MM}, Year: {YYYY}}}");
//!
//! fn main() {
//!     let now = SystemTime::now();
//!     let custom_struct = MyStruct::now(&now);
//!     println!("{custom_struct}");
//!     // This would print the actual time: "Custom Format: {Day: 01, Month: 01, Year: 1970}"
//!     
//!     // Also reverse compatible
//!     let date = "Custom Format: {Day: 01, Month: 01, Year: 1970}";
//!     let custom_struct: MyStruct = date.parse().unwrap();
//!     println!("{custom_struct}");
//! }   
//! ```
//!
//! If you want ISO 8601:
//! ```edition2021
//! use fiv_date::iso_8601::BasicDate;
//! //     ^
//! //   !!IMPORTANT!!
//! use std::time::SystemTime;
//!
//!
//! fn main() {
//!     let now = SystemTime::now();
//!     let iso = BasicDate::now(&now);
//!     println!("{iso}");
//!     // This would print the actual time: "19700101"
//!     
//!     // Also reverse compatible
//!     let date = "19700101";
//!     let iso: BasicDate = date.parse().unwrap();
//!     println!("{iso}");
//! }   
//! ```

pub mod iso_8601;

pub use crate as fiv_date;

pub use fiv_date_macro::custom_format_struct;

pub use fiv_date_core::CalDate;
pub use fiv_date_core::CloDate;
pub use fiv_date_core::Day;
pub use fiv_date_core::Days;
pub use fiv_date_core::Fraction;
pub use fiv_date_core::Hour;
pub use fiv_date_core::Minute;
pub use fiv_date_core::Month;
pub use fiv_date_core::Second;
pub use fiv_date_core::Time;
pub use fiv_date_core::ToDate;
pub use fiv_date_core::Weekday;
pub use fiv_date_core::Weeks;
pub use fiv_date_core::Year;

mod tests;
