use std::time::SystemTime;

pub struct Date {
    pub millisec: u8,
    pub second: u8,
    pub minutes: u8,
    pub hour: u8,
    pub day: u8,
    pub month: Month,
    pub year: u16,
    pub weekday: Weekday,
    pub timezone: Timezone,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{},{}:{}:{}",
            self.day,
            self.month.as_str(),
            self.year,
            self.hour,
            self.minutes,
            self.second
        )
    }
}

impl Date {
    pub fn new() -> Date {
        Date {
            millisec: 0,
            second: 0,
            minutes: 0,
            hour: 0,
            day: 1,
            month: Month::JANUARY,
            year: 1970,
            weekday: Weekday::THURSDAY,
            timezone: Timezone::GMT,
        }
    }
    pub fn to_http_string(&self) -> String {
        format!(
            "Date: {}, {:02} {} {:04} {:02}:{:02}:{:02} {}",
            self.weekday.as_str(),
            self.day,
            self.month.as_str(),
            self.year,
            self.hour,
            self.minutes,
            self.second,
            self.timezone.as_str()
        )
    }
}

pub struct Month(Mon);

impl Month {
    pub const JANUARY: Month = Month(Mon::Jan);
    pub const FEBUARY: Month = Month(Mon::Feb);
    pub const MARCH: Month = Month(Mon::Mar);
    pub const APRIL: Month = Month(Mon::Apr);
    pub const MAY: Month = Month(Mon::May);
    pub const JUNE: Month = Month(Mon::Jun);
    pub const JULY: Month = Month(Mon::Jul);
    pub const AUGUST: Month = Month(Mon::Aug);
    pub const SEPTEMBER: Month = Month(Mon::Sep);
    pub const OCTOBER: Month = Month(Mon::Oct);
    pub const NOVEMBER: Month = Month(Mon::Nov);
    pub const DECEMBER: Month = Month(Mon::Dec);

    pub fn from_u8(z: u8) -> Month {
        match z {
            1 => Month(Mon::Jan),
            2 => Month(Mon::Feb),
            3 => Month(Mon::Mar),
            4 => Month(Mon::Apr),
            5 => Month(Mon::May),
            6 => Month(Mon::Jun),
            7 => Month(Mon::Jul),
            8 => Month(Mon::Aug),
            9 => Month(Mon::Sep),
            10 => Month(Mon::Oct),
            11 => Month(Mon::Nov),
            12 => Month(Mon::Dec),
            _ => {
                panic!("Date::1");
            }
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

pub struct Weekday(Wd);

enum Wd {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

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

impl Date {
    pub fn now(mut self) -> Date {
        let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d,
            Err(_) => return self,
        };
        let mut times: String = format!("{:?}", time);
        times.pop();
        let f: f64 = times.parse::<f64>().unwrap();
        let millisec: u8 = ((f % 1.0)*1000.0).floor() as u8;
        let rf: u64 = f.floor() as u64;
        let seconds: u8 = (rf % 60) as u8;
        let minutes: u8 = (rf % 3600 / 60) as u8;
        let hours: u8 = (rf % 86400 / 3600) as u8;
        let mut days: u64 = rf / 86400;
        let wd: u64 = days % 7;
        let mut year: u64 = 1970;
        if days >= 1095 {
            days -= 1095;
            year += 3;
            while days >= 1461 {
                days -= 1461;
                year += 4;
            }
            while days >= 365 {
                days -= 365;
                year += 1;
            }
            if year % 4 == 0 && days > 90 {
                days -= 1;
            }
        } else {
            panic!("Date::3");
        };
        let (month, day) = parse_month(days);
        self.day = day;
        self.hour = hours;
        self.minutes = minutes;
        self.month = month;
        self.second = seconds;
        self.timezone = Timezone::GMT;
        self.year = year as u16;
        self.weekday = Weekday::from_u64(4 + wd);
        self.millisec = millisec;
        self
    }
    pub fn to_timezone(mut self, t: Timezone) -> Date {
        let h: i8 = self.hour.clone() as i8 - (self.timezone.in_hour() - t.in_hour());
        match h {
            ..=-1 => {
                self.hour = (h + 24) as u8;
                self.day -= 1;
            }
            0..=24 => {
                self.hour = h as u8;
            }
            25.. => {
                self.day += 1;
                self.hour = (h - 24) as u8;
            }
        }
        self.timezone = t.clone();
        self
    }
}

fn parse_month(mut days: u64) -> (Month, u8) {
    let month: Month = match days {
        335..=365 => {
            days -= 334;
            Month::DECEMBER
        }
        305.. => {
            days -= 304;
            Month::NOVEMBER
        }
        274.. => {
            days -= 273;
            Month::OCTOBER
        }
        244.. => {
            days -= 212;
            Month::SEPTEMBER
        }
        213.. => {
            days -= 212;
            Month::AUGUST
        }
        182.. => {
            days -= 181;
            Month::JULY
        }
        152.. => {
            days -= 151;
            Month::JUNE
        }
        121.. => {
            days -= 120;
            Month::MAY
        }
        91.. => {
            days -= 90;
            Month::APRIL
        }
        60.. => {
            days -= 59;
            Month::MARCH
        }
        32.. => {
            days -= 31;
            Month::FEBUARY
        }
        0.. => Month::JANUARY,
    };
    (month, days as u8)
}

#[derive(Clone)]
pub struct Timezone(Tzone);

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
