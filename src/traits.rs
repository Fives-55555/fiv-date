use crate::{
    cal::{mon, mon_a_day}, CalDate, CloDate, Day, Days, Fraction, Hour, Minute, Month, Second, Weekday, Weeks, Year
};
use std::time::{SystemTime, UNIX_EPOCH};


///Implements basic Time Format Operations required for the macro
pub trait Time {
    ///The Default Value
    fn new() -> Self where Self: Sized;
    ///The value extracted out of the &SystemTime using UNIX_EPOCH as anchor in Time
    fn now(s: &SystemTime) -> Self where Self: Sized;
}

impl Time for Weekday {
    fn new() -> Self {
        Self::Monday
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
        match days {
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
            day: 1,
            month: Month::JAN,
            year: 1970,
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
        let mut days = (dur.as_secs() / 86400) + 1;
        let mut years = (days / 1461) as u16 * 4;
        days %= 1461;
        let month: (Month, u8) = if b {
            match days {
                (672..) => {
                    days -= 1;
                }
                671 => {
                    return CalDate {
                        year: 1969 - years,
                        month: Month::FEB,
                        day: 29,
                    }
                }
                _ => (),
            }
            years += (days / 365) as u16;
            days %= 365;
            years = 1970 - years;
            mon_a_day(365 - days as u16)
        } else {
            match days {
                (791..) => {
                    days -= 1;
                }
                790 => {
                    return CalDate {
                        year: years + 1972,
                        month: Month::FEB,
                        day: 29,
                    }
                }
                _ => (),
            }
            years += (days / 365) as u16;
            days %= 365;
            years += 1970;
            mon_a_day(days as u16)
        };
        CalDate {
            day: month.1,
            month: month.0,
            year: years,
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
            second: 0,
            minute: 0,
            hour: 0,
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
            hour: hours as u8,
            minute: minutes,
            second: seconds,
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
            milli = (1000 - milli)%1000;
        }
        Fraction(milli as u16)
    }
}
