#[allow(unused_imports)]
use crate::{
    Day, Days, Hour, Minute, Month, Second, Time, Weekday, Year,
};
#[allow(unused_imports)]
use std::time::{Duration, SystemTime};

#[test]
fn test_year() {
    let default = Year::new();
    assert_eq!(default.0, 1970);
    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(364 * 86400);
    let y1 = Year::now(&x1);
    assert_eq!(y1.0, 1970);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(365 * 86400);
    let y2 = Year::now(&x2);
    assert_eq!(y2.0, 1971);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(366 * 86400);
    let y3 = Year::now(&x3);
    assert_eq!(y3.0, 1971);
    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(729 * 86400);
    let y1 = Year::now(&x1);
    assert_eq!(y1.0, 1971);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(730 * 86400);
    let y2 = Year::now(&x2);
    assert_eq!(y2.0, 1972);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(731 * 86400);
    let y3 = Year::now(&x3);
    assert_eq!(y3.0, 1972);
    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(1095 * 86400);
    let y1 = Year::now(&x1);
    assert_eq!(y1.0, 1972);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(1096 * 86400);
    let y2 = Year::now(&x2);
    assert_eq!(y2.0, 1973);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(1097 * 86400);
    let y3 = Year::now(&x3);
    assert_eq!(y3.0, 1973);
    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(1460 * 86400);
    let y1 = Year::now(&x1);
    assert_eq!(y1.0, 1973);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(1461 * 86400);
    let y2 = Year::now(&x2);
    assert_eq!(y2.0, 1974);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(1462 * 86400);
    let y3 = Year::now(&x3);
    assert_eq!(y3.0, 1974);
    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(1825 * 86400);
    let y1 = Year::now(&x1);
    assert_eq!(y1.0, 1974);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(1826 * 86400);
    let y2 = Year::now(&x2);
    assert_eq!(y2.0, 1975);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(1827 * 86400);
    let y3 = Year::now(&x3);
    assert_eq!(y3.0, 1975);
}

#[test]
fn test_month() {
    let default = Month::new();
    assert_eq!(default, Month::JAN);
    for day in 0..31 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::JAN);
    }
    for day in 31..59 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::FEB);
    }
    for day in 59..90 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::MAR);
    }
    for day in 90..120 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::APR);
    }
    for day in 120..151 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::MAY);
    }
    for day in 151..181 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::JUN);
    }
    for day in 181..212 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::JUL);
    }
    for day in 212..243 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::AUG);
    }
    for day in 243..273 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::SEP);
    }
    for day in 273..304 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::OCT);
    }
    for day in 304..334 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::NOV);
    }
    for day in 334..365 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = Month::now(&x1);
        assert_eq!(y1, Month::DEC);
    }
}

#[test]
fn test_weekday() {
    let default = Weekday::new();
    assert_eq!(default, Weekday::Monday);

    let x1 = SystemTime::UNIX_EPOCH;
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Thursday);

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Friday);

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400 * 2);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Saturday);

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400 * 3);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Sunday);

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400 * 4);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Monday);

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400 * 5);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Tuesday);

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400 * 6);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Wednesday);

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400 * 7);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Thursday);

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400 * 8);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Friday);

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Wednesday);

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400 * 2);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Tuesday);

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400 * 3);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Monday);

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400 * 4);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Sunday);

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400 * 5);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Saturday);

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400 * 6);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Friday);

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400 * 7);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Thursday);

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400 * 8);
    let y1 = Weekday::now(&x1);
    assert_eq!(y1, Weekday::Wednesday);
}

#[test]
fn test_days() {
    let default = Days::new();
    assert_eq!(default.0, 1);

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(1);
    let y1 = Days::now(&x1);
    assert_eq!(y1.0, 1);

    for i in 0..365 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(i * 86400);
        let y1 = Days::now(&x1);
        assert_eq!(y1.0, i as u16 + 1)
    }

    for i in 0..365 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs((i + 365) * 86400);
        let y1 = Days::now(&x1);
        assert_eq!(y1.0, i as u16 + 1)
    }

    for i in 0..366 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs((i + 730) * 86400);
        let y1 = Days::now(&x1);
        assert_eq!(y1.0, i as u16 + 1)
    }

    for i in 0..365 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs((i + 1096) * 86400);
        let y1 = Days::now(&x1);
        assert_eq!(y1.0, i as u16 + 1)
    }

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(1461 * 86400);
    let y1 = Days::now(&x1);
    assert_eq!(y1.0, 1);

    //----

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(1);
    let y1 = Days::now(&x1);
    assert_eq!(y1.0, 365);

    for i in 0..365 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(i * 86400 + 1);
        let y1 = Days::now(&x1);
        assert_eq!(y1.0, 365 - i as u16)
    }

    for i in 0..366 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs((i + 365) * 86400 + 1);
        let y1 = Days::now(&x1);
        assert_eq!(y1.0, 366 - i as u16)
    }

    for i in 0..365 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs((i + 731) * 86400 + 1);
        let y1 = Days::now(&x1);
        assert_eq!(y1.0, 365 - i as u16)
    }

    for i in 0..365 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs((i + 1096) * 86400 + 1);
        let y1 = Days::now(&x1);
        assert_eq!(y1.0, 365 - i as u16)
    }
}

#[test]
fn test_hour() {
    let default = Hour::new();
    assert_eq!(default.0, 0);

    let x1 = SystemTime::UNIX_EPOCH;
    let y1 = Hour::now(&x1);
    assert_eq!(y1.0, 0);

    for mins in 0..1440 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(mins * 60);
        let y1 = Hour::now(&x1);
        assert_eq!(y1.0, (mins / 60) as u8 % 24)
    }

    for mins in 0..1440 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(mins * 60);
        let y1 = Hour::now(&x1);
        assert_eq!(y1.0, ((1440 - mins) / 60) as u8 % 24)
    }
}

#[test]
fn test_minutes() {
    let default = Minute::new();
    assert_eq!(default.0, 0);

    let x1 = SystemTime::UNIX_EPOCH;
    let y1 = Minute::now(&x1);
    assert_eq!(y1.0, 0);

    for secs in 0..3600 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(secs);
        let y1 = Minute::now(&x1);
        assert_eq!(y1.0, (secs / 60) as u8 % 60)
    }

    for secs in 0..3600 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(secs);
        let y1 = Minute::now(&x1);
        assert_eq!(y1.0, ((3600 - secs) / 60) as u8 % 60)
    }
}

#[test]
fn test_secs() {
    let default = Second::new();
    assert_eq!(default.0, 0);

    let x1 = SystemTime::UNIX_EPOCH;
    let y1 = Minute::now(&x1);
    assert_eq!(y1.0, 0);

    for secs in 0..120 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(secs);
        let y1 = Second::now(&x1);
        assert_eq!(y1.0, (secs % 60) as u8)
    }

    for secs in 0..120 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(secs);
        let y1 = Second::now(&x1);
        assert_eq!(y1.0, (60 - (secs % 60)) as u8 % 60)
    }
}

#[test]
fn test_caldate() {
    let default = Day::new();
    assert_eq!(default.0, 1);

    let x1 = SystemTime::UNIX_EPOCH;
    let y1 = Day::now(&x1);
    assert_eq!(y1.0, 1);

    for secs in 0..86400 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(secs + 1);
        let y1 = Day::now(&x1);
        assert_eq!(y1.0, 1 + ((secs + 1) / 86400) as u8);
    }
}

#[test]
fn test_clodate() {
    let default = Day::new();
    assert_eq!(default.0, 1);

    let x1 = SystemTime::UNIX_EPOCH;
    let y1 = Day::now(&x1);
    assert_eq!(y1.0, 1);

    for secs in 0..86400 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(secs + 1);
        let y1 = Day::now(&x1);
        println!("{}", y1.0);
        assert_eq!(y1.0, 1 + ((secs + 1) / 86400) as u8);
    }
}

#[test]
fn test_day() {
    let default = Day::new();
    assert_eq!(default.0, 1);

    let x1 = SystemTime::UNIX_EPOCH;
    let y1 = Day::now(&x1);
    assert_eq!(y1.0, 1);

    for secs in 0..86400 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(secs + 1);
        let y1 = Day::now(&x1);
        assert_eq!(y1.0, 1 + ((secs + 1) / 86400) as u8);
    }

    for day in 0..1461 {
        let x1 = SystemTime
        let exp = 'outer:{
            match day {
                0..31=>0,
                31..
                790=>break 'outer 29,
                _=>panic!()
            }///////////////////////////////////////////////////////
        };
    }

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(790*86400);
    let y1 = Day::now(&x1);
    assert_eq!(y1.0, 29);

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(790*86400);
    let y1 = Day::now(&x1);
    assert_eq!(y1.0, 29);

    todo!("ToDo the Day MEss")
}
