use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub use crate::cal::Day;
pub use crate::cal::Days;
pub use crate::cal::Month;
pub use crate::cal::Weekday;
pub use crate::cal::Year;
pub use crate::clo::CloDate;
pub use crate::clo::Hour;
pub use crate::clo::Minute;
pub use crate::traits::Time;
pub use date_macro::format_date_struct;

pub mod tests;
pub mod traits;

pub mod cal;
pub mod clo;

impl CloDate {
    pub fn new(date: &SystemTime) -> CloDate {
        let mut b = false;
        let dur = match date.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut hours = dur.as_secs() % 86400;
        if b {
            hours = 86400 - hours;
        }
        let seconds = (hours % 60) as u8;
        hours /= 60;
        let minutes = (hours % 60) as u8;
        hours /= 60;
        hours %= 24;
        CloDate {
            hour: hours as u8,
            minute: minutes,
            second: seconds,
        }
    }
}

//=====================================================================================================================================================

/*


    impl Weekday {
        pub const MONDAY: Weekday = Weekday(Wd::Monday);
        pub const TUESDAY: Weekday = Weekday(Wd::Tuesday);
        pub const WEDNESDAY: Weekday = Weekday(Wd::Wednesday);
    pub const THURSDAY: Weekday = Weekday(Wd::Thursday);
    pub const FRIDAY: Weekday = Weekday(Wd::Friday);
    pub const SATURDAY: Weekday = Weekday(Wd::Saturday);
    pub const SUNDAY: Weekday = Weekday(Wd::Sunday);

    pub fn from_u64(z: u64) -> Weekday {
        match z {
            4 => Weekday(Wd::Thursday),
            5 => Weekday(Wd::Friday),
            6 => Weekday(Wd::Saturday),
            7 => Weekday(Wd::Sunday),
            8 => Weekday(Wd::Monday),
            9 => Weekday(Wd::Tuesday),
            10 => Weekday(Wd::Wednesday),
            _ => {
                panic!("Date::2");
            }
        }
    }
    pub fn as_str(&self) -> &str {
        match self.0 {
            Wd::Monday => "Mon",
            Wd::Tuesday => "Tue",
            Wd::Wednesday => "Wed",
            Wd::Thursday => "Thu",
            Wd::Friday => "Fri",
            Wd::Saturday => "Sat",
            Wd::Sunday => "Sun",
        }
    }
}


#[derive(Clone)]
pub struct TimeZone<T: TimeZoneTr>{
    summertime: Option<bool>
}

pub trait TimeZoneTr {
    const fn get_tf_utc(&self)->(i8, i8);
}

pub struct UTC;

impl TimeZoneTr for UTC {
    const fn get_tf_utc(&self)->(i8, i8) {
        (0,0)
    }
}



#[derive(Clone)]
enum Tzone {
    UTC,
    GMT,
    EST,
    EDT,
    CST,
    CDT,
    MST,
    MDT,
    PST,
    PDT,
    AEST,
    AEDT,
    JST,
    IST,
    CET,
    CEST,
    HAST,
    AKST,
    IDLW,
    AKDT,
    CNST,
    NFT,
    IDLE,
    NZDT,
    NZST,
    ICT,
    CAT,
    BT,
    MSK,
    EAT,
    AST,
    PT,
    EEST,
    WEDT,
    WET,
    WEST,
    EEDT,
    YST,
    YDT,
    SAST,
    EET,
    CEDT,
    WAST,
    BST,
    ADT,
}

impl Timezone {
    pub const UTC: Timezone = Timezone(Tzone::UTC);
    pub const GMT: Timezone = Timezone(Tzone::GMT);
    pub const EST: Timezone = Timezone(Tzone::EST);
    pub const EDT: Timezone = Timezone(Tzone::EDT);
    pub const CST: Timezone = Timezone(Tzone::CST);
    pub const CDT: Timezone = Timezone(Tzone::CDT);
    pub const MST: Timezone = Timezone(Tzone::MST);
    pub const MDT: Timezone = Timezone(Tzone::MDT);
    pub const PST: Timezone = Timezone(Tzone::PST);
    pub const PDT: Timezone = Timezone(Tzone::PDT);
    pub const AEST: Timezone = Timezone(Tzone::AEST);
    pub const AEDT: Timezone = Timezone(Tzone::AEDT);
    pub const JST: Timezone = Timezone(Tzone::JST);
    pub const IST: Timezone = Timezone(Tzone::IST);
    pub const CET: Timezone = Timezone(Tzone::CET);
    pub const CEST: Timezone = Timezone(Tzone::CEST);
    pub const HAST: Timezone = Timezone(Tzone::HAST);
    pub const IDLW: Timezone = Timezone(Tzone::IDLW);
    pub const AKST: Timezone = Timezone(Tzone::AKST);
    pub const AKDT: Timezone = Timezone(Tzone::AKDT);
    pub const CNST: Timezone = Timezone(Tzone::CNST);
    pub const NFT: Timezone = Timezone(Tzone::NFT);
    pub const IDLE: Timezone = Timezone(Tzone::IDLE);
    pub const NZDT: Timezone = Timezone(Tzone::NZDT);
    pub const NZST: Timezone = Timezone(Tzone::NZST);
    pub const ICT: Timezone = Timezone(Tzone::ICT);
    pub const CAT: Timezone = Timezone(Tzone::CAT);
    pub const BT: Timezone = Timezone(Tzone::BT);
    pub const MSK: Timezone = Timezone(Tzone::MSK);
    pub const EAT: Timezone = Timezone(Tzone::EAT);
    pub const AST: Timezone = Timezone(Tzone::AST);
    pub const PT: Timezone = Timezone(Tzone::PT);
    pub const EEST: Timezone = Timezone(Tzone::EEST);
    pub const WEDT: Timezone = Timezone(Tzone::WEDT);
    pub const WET: Timezone = Timezone(Tzone::WET);
    pub const WEST: Timezone = Timezone(Tzone::WEST);
    pub const EEDT: Timezone = Timezone(Tzone::EEDT);
    pub const YST: Timezone = Timezone(Tzone::YST);
    pub const YDT: Timezone = Timezone(Tzone::YDT);
    pub const SAST: Timezone = Timezone(Tzone::SAST);
    pub const EET: Timezone = Timezone(Tzone::EET);
    pub const CEDT: Timezone = Timezone(Tzone::CEDT);
    pub const WAST: Timezone = Timezone(Tzone::WAST);
    pub const BST: Timezone = Timezone(Tzone::BST);
    pub const ADT: Timezone = Timezone(Tzone::ADT);
    pub fn as_str(&self) -> &str {
        match self.0 {
            Tzone::UTC => "UTC",
            Tzone::GMT => "GMT",
            Tzone::EST => "EST",
            Tzone::EDT => "EDT",
            Tzone::CST => "CST",
            Tzone::CDT => "CDT",
            Tzone::MST => "MST",
            Tzone::MDT => "MDT",
            Tzone::PST => "PST",
            Tzone::PDT => "PDT",
            Tzone::AEST => "AEST",
            Tzone::AEDT => "AEDT",
            Tzone::JST => "JST",
            Tzone::IST => "IST",
            Tzone::CET => "CET",
            Tzone::CEST => "CEST",
            Tzone::HAST => "HAST",
            Tzone::IDLW => "IDLW",
            Tzone::AKST => "AKST",
            Tzone::AKDT => "AKDT",
            Tzone::CNST => "CNST",
            Tzone::NFT => "NFT",
            Tzone::IDLE => "IDLE",
            Tzone::NZDT => "NZDT",
            Tzone::NZST => "NZST",
            Tzone::ICT => "ICT",
            Tzone::CAT => "CAT",
            Tzone::BT => "BT",
            Tzone::MSK => "MSK",
            Tzone::EAT => "EAT",
            Tzone::AST => "AST",
            Tzone::PT => "PT",
            Tzone::EEST => "EEST",
            Tzone::WEDT => "WEDT",
            Tzone::WET => "WET",
            Tzone::WEST => "WEST",
            Tzone::EEDT => "EEDT",
            Tzone::YST => "YST",
            Tzone::YDT => "YDT",
            Tzone::SAST => "SAST",
            Tzone::EET => "EET",
            Tzone::CEDT => "CEDT",
            Tzone::WAST => "WAST",
            Tzone::BST => "BST",
            Tzone::ADT => "ADT",
        }
    }
    pub fn in_hour(&self) -> i8 {
        match self.0 {
            Tzone::NZDT => 13,
            Tzone::NZST => 12,
            Tzone::IDLE => 12,
            Tzone::NFT => 11,
            Tzone::AEDT => 11,
            Tzone::AEST => 10,
            Tzone::JST => 9,
            Tzone::CNST => 8,
            Tzone::ICT => 7,
            Tzone::BT => 3,
            Tzone::MSK => 3,
            Tzone::EAT => 3,
            Tzone::AST => 3,
            Tzone::EEST => 3,
            Tzone::EEDT => 3,
            Tzone::SAST => 2,
            Tzone::CAT => 2,
            Tzone::EET => 2,
            Tzone::CEST => 2,
            Tzone::CEDT => 2,
            Tzone::WAST => 2,
            Tzone::BST => 1,
            Tzone::IST => 1,
            Tzone::CET => 1,
            Tzone::UTC => 0,
            Tzone::WET => 0,
            Tzone::WEST => 1,
            Tzone::WEDT => 1,
            Tzone::GMT => 0,
            Tzone::ADT => -3,
            Tzone::EST => -5,
            Tzone::EDT => -4,
            Tzone::CST => -6,
            Tzone::CDT => -5,
            Tzone::MST => -7,
            Tzone::MDT => -6,
            Tzone::PST => -8,
            Tzone::PDT => -7,
            Tzone::PT => -8,
            Tzone::YST => -9,
            Tzone::YDT => -8,
            Tzone::AKST => -9,
            Tzone::AKDT => -8,
            Tzone::HAST => -10,
            Tzone::IDLW => -12,
        }
    }
}

*/
//-----------------------------------------------------------------------------------------------------
//useful Funcs

fn leap_sec_add(dur: &mut Duration, b: bool) {
    *dur -= Duration::from_secs(if b {
        27
    } else {
        match dur.as_secs() {
            ..=78796799 => 27,  //1972.06.30-23::59::59
            ..=94694399 => 26,  //1972.12.31-23::59::59
            ..=126230399 => 25, //1973.12.31-23::59::59
            ..=157766399 => 24, //1974.12.31-23::59::59
            ..=189302399 => 23, //1975.12.31-23::59::59
            ..=220924799 => 22, //1976.12.31-23::59::59
            ..=252460799 => 21, //1977.12.31-23::59::59
            ..=283996799 => 20, //1978.12.31-23::59::59
            ..=315532799 => 19, //1979.12.31-23::59::59
            ..=362793599 => 18, //1981.06.3-23::59::59
            ..=394329599 => 17, //1982.06.3-23::59::59
            ..=425865599 => 16, //1983.06.3-23::59::59
            ..=489023999 => 15, //1985.06.3-23::59::59
            ..=567993599 => 14, //1987.12.31-23::59::59
            ..=631151999 => 13, //1989.12.31-23::59::59
            ..=662687999 => 12, //1990.12.31-23::59::59
            ..=709948799 => 11, //1992.06.3-23::59::59
            ..=741484799 => 10, //1993.06.3-23::59::59
            ..=773020799 => 9,  //1994.06.3-23::59::59
            ..=820454399 => 8,  //1995.12.31-23::59::59
            ..=867715199 => 7,  //1997.06.3-23::59::59
            ..=915148799 => 6,  //1998.12.31-23::59::59
            ..=1136073599 => 5, //2005.12.31-23::59::59
            ..=1230767999 => 4, //2008.12.31-23::59::59
            ..=1341100799 => 3, //2012.06.3-23::59::59
            ..=1435708799 => 2, //2015.06.3-23::59::59
            ..=1483228799 => 1, //2016.12.31-23::59::59
            _ => unreachable!("Hä"),
        }
    })
}
use date_macro::format_date_struct;
format_date_struct!(Ddt, "YYYY\\D\\AMM");

#[test]
fn test() {
    let x = Ddt::new();
    println!("{x}");
    let year = Year::now(&SystemTime::now());
    let month = Month::now(&SystemTime::now());
    let day = Day::now(&SystemTime::now());
}

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
