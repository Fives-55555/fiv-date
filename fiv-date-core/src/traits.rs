use crate::{
    cal::{mon, mon_a_day},
    CalDate, CloDate, Day, Days, Fraction, Hour, Minute, Month, Second, Weekday, Weeks, Year,
};
use std::time::{SystemTime, UNIX_EPOCH};

///Implements the parsing from &str to the Date
pub trait ToDate {
    fn to_date(s: &str) -> Result<(Self, &str), ()>
    where
        Self: Sized;
}

///Implements basic Time Format Operations required for the macro
pub trait Time {
    ///The Default Value
    fn new() -> Self
    where
        Self: Sized;
    ///The value extracted out of the &SystemTime using UNIX_EPOCH as anchor in Time
    fn now(s: &SystemTime) -> Self
    where
        Self: Sized;
}

impl Time for Weekday {
    fn new() -> Self {
        Self::Thursday
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut days = (dur.as_secs() / 86400) % 7;
        if b {
            days = (7 - days) % 7;
        }
        match days {
            0 => Self::Thursday,
            1 => Self::Friday,
            2 => Self::Saturday,
            3 => Self::Sunday,
            4 => Self::Monday,
            5 => Self::Tuesday,
            6 => Self::Wednesday,
            _ => unreachable!(),
        }
    }
}

impl Time for Year {
    fn new() -> Self {
        Year(1970)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut days = dur.as_secs() / 86400;
        let years = (days / 1461) as u16 * 4;
        days %= 1461;
        Year(if b {
            1970 - (years
                + match days {
                    1096.. => 3,
                    731.. => 2,
                    365.. => 1,
                    _ => 0,
                })
        } else {
            years
                + 1970
                + match days {
                    1096.. => 3,
                    730.. => 2,
                    365.. => 1,
                    _ => 0,
                }
        })
    }
}

impl Time for Month {
    fn new() -> Self {
        Month::JAN
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut days = (dur.as_secs() / 86400) % 1461;
        if b {
            days = 1461 - days;
        }
        match days {
            790.. => days -= 1,
            _ => (),
        }
        mon((days % 365) as u16 + 1)
    }
}
impl Time for Day {
    fn new() -> Self {
        Day(1)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut days = (dur.as_secs() / 86400) % 1461;
        if b {
            days = 1461 - days;
        }
        match days + 1 {
            790 => return Day(29),
            791.. => days -= 1,
            _ => (),
        }
        Day::day((days % 365) as u16 + 1)
    }
}

impl Time for CalDate {
    fn new() -> Self {
        CalDate {
            day: Day(1),
            month: Month::JAN,
            year: Year(1970),
        }
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut days = dur.as_secs() / 86400;
        let mut years = (days / 1461) as u16 * 4;
        days %= 1461;
        if b {
            days = 1461 - days;
        };
        match days + 1 {
            790 => {
                return CalDate {
                    year: Year(years + 1972),
                    month: Month::FEB,
                    day: Day(29),
                }
            }
            (791..) => {
                days -= 1;
            }
            _ => (),
        }
        years += (days / 365) as u16;
        if b {
            years = 1969 - years;
        } else {
            years += 1970;
        };
        let month = mon_a_day((days % 365) as u16 + 1);
        CalDate {
            day: Day(month.1),
            month: month.0,
            year: Year(years),
        }
    }
}

impl Time for Hour {
    fn new() -> Self {
        Hour(0)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut secs = dur.as_secs() % 86400;
        if b {
            secs = 86400 - dur.as_secs();
        }
        Hour((secs / 3600) as u8)
    }
}

impl Time for Days {
    fn new() -> Self {
        Days(1)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut days = dur.as_secs() / 86400;
        days %= 1461;
        days += 1;
        days = if b {
            (match days {
                1097.. => 1462,
                732.. => 1097,
                366.. => 732,
                _ => 366,
            }) - days
        } else {
            days - match days {
                1097.. => 1096,
                731.. => 730,
                366.. => 365,
                _ => 0,
            }
        };
        Days(days as u16)
    }
}

impl Time for Minute {
    fn new() -> Self {
        Minute(0)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut secs = dur.as_secs() % 3600;
        if b {
            secs = 3600 - secs;
        }
        Minute((secs / 60) as u8)
    }
}

impl Time for Weeks {
    fn new() -> Self {
        Self(1)
    }
    fn now(s: &SystemTime) -> Self {
        let wn = Weekday::now(s).to_idx() as u16;
        let days = Days::now(s).0 - wn;
        let mut weeks = (days / 7) as u8 + 1;
        if days % 7 != 0 {
            weeks += 1;
        }
        Weeks(weeks)
    }
}

impl Time for CloDate {
    fn new() -> Self {
        CloDate {
            second: Second(0),
            minute: Minute(0),
            hour: Hour(0),
        }
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
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
        CloDate {
            hour: Hour(hours as u8),
            minute: Minute(minutes),
            second: Second(seconds),
        }
    }
}

impl Time for Second {
    fn new() -> Self {
        Second(0)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut secs = (dur.as_secs() % 60) as u8;
        if b {
            secs = (60 - secs) % 60;
        };
        Second(secs)
    }
}

impl Time for Fraction {
    fn new() -> Self {
        Fraction(0)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d) => d,
            //Before Unix Epoch
            Err(d) => {
                b = true;
                d.duration()
            }
        };
        let mut milli = dur.subsec_millis();
        if b {
            milli = (1000 - milli) % 1000;
        }
        Fraction(milli as u16)
    }
}

impl ToDate for Day {
    fn to_date(str: &str) -> Result<(Self, &str), ()> {
        if str.len() < 2 {
            return Err(());
        }
        let num = &str[..2];
        let num = match num.parse::<u8>() {
            Ok(num) => num,
            Err(_) => return Err(()),
        };
        if num < 32 && num != 0 {
            return Ok((Day(num), &str[2..]));
        }
        return Err(());
    }
}

impl ToDate for Month {
    fn to_date(s: &str) -> Result<(Self, &str), ()> {
        if s.len() < 2 {
            return Err(());
        }
        match (&s[..2]).parse::<u8>() {
            Ok(num) => match Month::from_u8(num) {
                Ok(month) => return Ok((month, &s[2..])),
                Err(_) => (),
            },
            Err(_) => (),
        }
        return Err(());
    }
}

impl ToDate for Year {
    fn to_date(s: &str) -> Result<(Self, &str), ()> {
        if s.len() < 4 {
            return Err(());
        }
        match (&s[..4]).parse::<u16>() {
            Ok(num) if num < 10000 => return Ok((Year(num), &s[4..])),
            _ => return Err(()),
        }
    }
}

impl ToDate for Fraction {
    fn to_date(s: &str) -> Result<(Self, &str), ()> {
        if s.len() < 3 {
            return Err(());
        }
        match (&s[..3]).parse::<u16>() {
            Ok(num) if num < 1000 => return Ok((Fraction(num), &s[3..])),
            _ => return Err(()),
        }
    }
}

impl ToDate for Second {
    fn to_date(s: &str) -> Result<(Self, &str), ()> {
        if s.len() < 2 {
            return Err(());
        }
        match (&s[..2]).parse::<u8>() {
            Ok(num) if num < 61 => return Ok((Second(num), &s[2..])),
            _ => return Err(()),
        }
    }
}

impl ToDate for Minute {
    fn to_date(s: &str) -> Result<(Self, &str), ()> {
        if s.len() < 2 {
            return Err(());
        }
        match (&s[..2]).parse::<u8>() {
            Ok(num) if num < 60 => return Ok((Minute(num), &s[2..])),
            _ => return Err(()),
        }
    }
}

impl ToDate for Hour {
    fn to_date(s: &str) -> Result<(Self, &str), ()> {
        if s.len() < 2 {
            return Err(());
        }
        match (&s[..2]).parse::<u8>() {
            Ok(num) if num < 24 => return Ok((Hour(num), &s[2..])),
            _ => return Err(()),
        }
    }
}

impl ToDate for Weeks {
    fn to_date(s: &str) -> Result<(Self, &str), ()> {
        if s.len() < 2 {
            return Err(());
        }
        match (&s[..2]).parse::<u8>() {
            Ok(num) if num < 54 => return Ok((Weeks(num), &s[2..])),
            _ => return Err(()),
        }
    }
}

impl ToDate for Days {
    fn to_date(s: &str) -> Result<(Self, &str), ()> {
        if s.len() < 3 {
            return Err(());
        }
        match (&s[..3]).parse::<u16>() {
            Ok(num) if num < 367 => return Ok((Days(num), &s[3..])),
            _ => return Err(()),
        }
    }
}

impl ToDate for Weekday {
    fn to_date(s: &str) -> Result<(Self, &str), ()> {
        if s.len() < 1 {
            return Err(());
        }
        match (&s[..1]).parse::<u8>() {
            Ok(num) => match Weekday::from_num(num) {
                Ok(wd) => return Ok((wd, &s[1..])),
                Err(_) => (),
            },
            _ => (),
        }
        return Err(());
    }
}
