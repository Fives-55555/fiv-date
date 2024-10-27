//! # Calendaric Types
//! This is 
use crate::format_inner;
pub struct Day(pub u8);

impl Day {
    pub fn day(day: u16) -> Day {
        Day(match day {
            335..366 => day - 334,
            305.. => day - 304,
            274.. => day - 273,
            244.. => day - 243,
            213.. => day - 212,
            182.. => day - 181,
            152.. => day - 151,
            121.. => day - 120,
            91.. => day - 90,
            60.. => day - 59,
            32.. => day - 31,
            0.. => day,
        } as u8)
    }
}

//-----------------------------------------------------------------------------------------------------

#[derive(PartialEq, Debug)]
pub struct Month(Mon);

impl Month {
    /// January
    pub const JAN: Month = Month(Mon::Jan);
    /// Febuary
    pub const FEB: Month = Month(Mon::Feb);
    /// March
    pub const MAR: Month = Month(Mon::Mar);
    /// April
    pub const APR: Month = Month(Mon::Apr);
    /// May
    pub const MAY: Month = Month(Mon::May);
    /// June
    pub const JUN: Month = Month(Mon::Jun);
    /// July
    pub const JUL: Month = Month(Mon::Jul);
    /// August
    pub const AUG: Month = Month(Mon::Aug);
    /// September
    pub const SEP: Month = Month(Mon::Sep);
    /// October
    pub const OCT: Month = Month(Mon::Oct);
    /// November
    pub const NOV: Month = Month(Mon::Nov);
    /// December
    pub const DEC: Month = Month(Mon::Dec);
}

impl Month {
    pub fn from_u8(z: u8) -> Month {
        Month(match z {
            1 => Mon::Jan,
            2 => Mon::Feb,
            3 => Mon::Mar,
            4 => Mon::Apr,
            5 => Mon::May,
            6 => Mon::Jun,
            7 => Mon::Jul,
            8 => Mon::Aug,
            9 => Mon::Sep,
            10 => Mon::Oct,
            11 => Mon::Nov,
            12 => Mon::Dec,
            _ => {
                panic!("Date::1");
            }
        })
    }
    pub fn as_num(&self) -> u8 {
        match self.0 {
            Mon::Jan => 1,
            Mon::Feb => 2,
            Mon::Mar => 3,
            Mon::Apr => 4,
            Mon::May => 5,
            Mon::Jun => 6,
            Mon::Jul => 7,
            Mon::Aug => 8,
            Mon::Sep => 9,
            Mon::Oct => 10,
            Mon::Nov => 11,
            Mon::Dec => 12,
        }
    }
    pub fn as_str(&self) -> &str {
        match self.0 {
            Mon::Jan => "Jan",
            Mon::Feb => "Feb",
            Mon::Mar => "Mar",
            Mon::Apr => "Apr",
            Mon::May => "May",
            Mon::Jun => "Jun",
            Mon::Jul => "Jul",
            Mon::Aug => "Aug",
            Mon::Sep => "Sep",
            Mon::Oct => "Oct",
            Mon::Nov => "Nov",
            Mon::Dec => "Dec",
        }
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_num())
    }
}

#[derive(PartialEq, Debug)]
enum Mon {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

pub fn mon(day: u16) -> Month {
    match day {
        335..366 => Month::DEC,
        305.. => Month::NOV,
        274.. => Month::OCT,
        244.. => Month::SEP,
        213.. => Month::AUG,
        182.. => Month::JUL,
        152.. => Month::JUN,
        121.. => Month::MAY,
        91.. => Month::APR,
        60.. => Month::MAR,
        32.. => Month::FEB,
        0.. => Month::JAN,
    }
}

pub fn mon_a_day(mut day: u16) -> (Month, u8) {
    let month = match day % 365 {
        335..=365 => {
            day -= 334;
            Month::DEC
        }
        305.. => {
            day -= 304;
            Month::NOV
        }
        274.. => {
            day -= 273;
            Month::OCT
        }
        244.. => {
            day -= 243;
            Month::SEP
        }
        213.. => {
            day -= 212;
            Month::AUG
        }
        182.. => {
            day -= 181;
            Month::JUL
        }
        152.. => {
            day -= 151;
            Month::JUN
        }
        121.. => {
            day -= 120;
            Month::MAY
        }
        91.. => {
            day -= 90;
            Month::APR
        }
        60.. => {
            day -= 59;
            Month::MAR
        }
        32.. => {
            day -= 31;
            Month::FEB
        }
        0.. => Month::JAN,
    };
    (month, day as u8)
}

//-----------------------------------------------------------------------------------------------------

pub struct Year(pub u16);

//-----------------------------------------------------------------------------------------------------

/// Compacted Calendar Type for Performence
pub struct CalDate {
    pub day: u8,
    pub month: Month,
    pub year: u16,
}

//-----------------------------------------------------------------------------------------------------

/// Days of the Year
/// 
/// ## Available Values
/// (Range is inclusive)
/// 
/// 1..366
pub struct Days(pub u16);

//-----------------------------------------------------------------------------------------------------

pub type Weekday = Wd;

#[derive(PartialEq, Debug)]
pub enum Wd {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Monday => "Mon",
            Self::Tuesday => "Tue",
            Self::Wednesday => "Wed",
            Self::Thursday => "Thu",
            Self::Friday => "Fri",
            Self::Saturday => "Sat",
            Self::Sunday => "Sun",
        }
    }
    pub fn to_num(&self) -> u8 {
        match self {
            Self::Monday => 1,
            Self::Tuesday => 2,
            Self::Wednesday => 3,
            Self::Thursday => 4,
            Self::Friday => 5,
            Self::Saturday => 6,
            Self::Sunday => 7,
        }
    }
    pub fn to_idx(&self) -> u8 {
        self.to_num() - 1
    }
}

//-----------------------------------------------------------------------------------------------------

pub struct Weeks(pub u8);

//-----------------------------------------------------------------------------------------------------

format_inner!(Year);
format_inner!(Day);
format_inner!(Days);
format_inner!(Weeks);
