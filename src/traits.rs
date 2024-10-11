use std::time::{SystemTime, UNIX_EPOCH};
use crate::{cal::{CalDate, mon, mon_a_day, JAN, FEB}, Day, Days, Month, Year, Hour, leap_sec};


pub trait Time {
    fn new() -> Self;
    //Without Leap Seconds
    fn now(s: &SystemTime) -> Self;
}

impl Time for Year {
    fn new() -> Self {
        Year(1970)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let mut dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d)=>d,
            //Before Unix Epoch
            Err(d)=>{
                b=true;
                d.duration()
            },
        };
        leap_sec(&mut dur, b);
        let mut days = dur.as_secs()/86400;
        let years = (days/1461) as u16 *4;
        days%=1461;
        Year(if b {
            1970-(years+match days {
                1096.. =>3,
                731.. =>2,
                365.. =>1,
                _=>0
            })
        } else {
            years+1970+match days {
                1096.. =>3,
                730.. =>2,
                365.. =>1,
                _=>0
            }
        })
    }
}

impl Time for Month {
    fn new() -> Self {
        JAN
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let mut dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d)=>d,
            //Before Unix Epoch
            Err(d)=>{
                b=true;
                d.duration()
            },
        };
        leap_sec(&mut dur, b);
        let mut days = (dur.as_secs()/86400)%1461;
        if b {
            days=1461-days;
        }
        match days {
            790.. =>days-=1,
            _=>()
        }
        mon((days%365) as u16+1)
    }
}
impl Time for Day {
    fn new() -> Self {
        Day(1)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let mut dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d)=>d,
            //Before Unix Epoch
            Err(d)=>{
                b=true;
                d.duration()
            },
        };
        leap_sec(&mut dur, b);
        let mut days = (dur.as_secs()/86400)%1461;
        if b {
            days=1461-days;
        }
        match days {
            790=>return Day(29),
            791.. =>days-=1,
            _=>()
        }
        Day::day((days%365) as u16+1)
    }
}

impl Time for CalDate {
    fn new() -> Self {
        CalDate {
            day: 1,
            month: JAN,
            year: 1970
        }
    }
    fn now(s: &SystemTime)->Self {
        let mut b = false;
        let mut dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d)=>d,
            //Before Unix Epoch
            Err(d)=>{
                b=true;
                d.duration()
            },
        };
        leap_sec(&mut dur, b);
        let mut days = (dur.as_secs()/86400)+1;
        let mut years = (days/1461) as u16 *4;
        days%=1461;
        let month: (Month, u8) = if b {
            match days {
                (672..)=>{days-=1;},
                671=> return CalDate {
                    year: 1970-(years+1),
                    month: FEB,
                    day: 29
                },
                _=>()
            }
            years+=(days/365) as u16;
            days%=365;
            years=1970-years;
            mon_a_day(365-days as u16)
        } else {
            match days {
                (791..)=>{days-=1;},
                790=> return CalDate {
                    year: years+1972,
                    month: FEB,
                    day: 29
                },
                _=>()
            }
            years+=(days/365) as u16;
            days%=365;
            years+=1970;
            mon_a_day(days as u16)
        };
        CalDate {
            day: month.1,
            month: month.0,
            year: years
        }
    }
}

impl Time for Hour {
    fn new()->Self {
        Hour(0)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let mut dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d)=>d,
            //Before Unix Epoch
            Err(d)=>{
                b=true;
                d.duration()
            },
        };
        leap_sec(&mut dur, b);
        let mut secs = dur.as_secs()%86400;
        if b {
            secs = 86400-dur.as_secs();
        }
        Hour((secs/3600) as u8)
    }
}

impl Time for Days {
    fn new()->Self {
        Days(0)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let mut dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d)=>d,
            //Before Unix Epoch
            Err(d)=>{
                b=true;
                d.duration()
            },
        };
        leap_sec(&mut dur, b);
        let mut days = dur.as_secs()/86400;
        days%=1461;
        days = if b {
            days-match days {
                1096.. =>731,
                731.. =>2,
                365.. =>days,
                _=>0
            };
        } else {
            days-match days {
                
            }
        };
        Days(days)
    }
}

impl Time for Minute {
    fn new()->Self {
        Minute(0)
    }
    fn now(s: &SystemTime) -> Self {
        let mut b = false;
        let mut dur = match s.duration_since(UNIX_EPOCH) {
            Ok(d)=>d,
            //Before Unix Epoch
            Err(d)=>{
                b=true;
                d.duration()
            },
        };
        leap_sec(&mut dur, b);
        let mut secs = dur.as_secs()%3600;
        if b {
            secs = 3600-secs;
        }
        Minute((secs/60) as u8)
    }
}