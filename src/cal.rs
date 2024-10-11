pub struct Day (pub u8);

impl Day {
    pub fn day(day: u16)->Day {
        Day(match day {
            335..366 => {
                day-334
            }
            305.. => {
                day-304
            }
            274.. => {
                day-273
            }
            244.. => {
                day-243
            }
            213.. => {
                day-212
            }
            182.. => {
                day-181
            }
            152.. => {
                day-151
            }
            121.. => {
                day-120
            }
            91.. => {
                day-90
            }
            60.. => {
                day-59
            }
            32.. => {
                day-31
            }
            0.. => day,
        } as u8)
    }
}

//-----------------------------------------------------------------------------------------------------

pub type Month = Mon;

pub const JAN: Month = Mon::Jan;
pub const FEB: Month = Mon::Feb;
pub const MAR: Month = Mon::Mar;
pub const APR: Month = Mon::Apr;
pub const MAY: Month = Mon::May;
pub const JUN: Month = Mon::Jun;
pub const JUL: Month = Mon::Jul;
pub const AUG: Month = Mon::Aug;
pub const SEP: Month = Mon::Sep;
pub const OCT: Month = Mon::Oct;
pub const NOV: Month = Mon::Nov;
pub const DEC: Month = Mon::Dec;

impl Month {
    pub fn from_u8(z: u8) -> Month {
        match z {
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
        }
    }
    pub fn as_num(&self) -> u8 {
        match self {
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
        match self {
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

#[derive(PartialEq)]
pub enum Mon {
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

pub fn mon(day: u16)->Month {
    match day {
        335..366 => {
            DEC
        }
        305.. => {
            NOV
        }
        274.. => {
            OCT
        }
        244.. => {
            SEP
        }
        213.. => {
            AUG
        }
        182.. => {
            JUL
        }
        152.. => {
            JUN
        }
        121.. => {
            MAY
        }
        91.. => {
            APR
        }
        60.. => {
            MAR
        }
        32.. => {
            FEB
        }
        0.. => JAN,
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}",
            self.as_num()
        )
    }
}

pub fn mon_a_day(mut day: u16)->(Month,u8) {
    let month = match day%365 {
        335..=365 => {
            day -= 334;
            DEC
        }
        305.. => {
            day -= 304;
            NOV
        }
        274.. => {
            day -= 273;
            OCT
        }
        244.. => {
            day -= 243;
            SEP
        }
        213.. => {
            day -= 212;
            AUG
        }
        182.. => {
            day -= 181;
            JUL
        }
        152.. => {
            day -= 151;
            JUN
        }
        121.. => {
            day -= 120;
            MAY
        }
        91.. => {
            day -= 90;
            APR
        }
        60.. => {
            day -= 59;
            MAR
        }
        32.. => {
            day -= 31;
            FEB
        }
        0.. => JAN,
    };
    (month, day as u8)
}

//-----------------------------------------------------------------------------------------------------

pub struct Year(pub u16);

//-----------------------------------------------------------------------------------------------------

pub struct CalDate {
    pub day: u8,
    pub month: Month,
    pub year: u16
}

//-----------------------------------------------------------------------------------------------------

pub struct Days(pub u16);

//-----------------------------------------------------------------------------------------------------