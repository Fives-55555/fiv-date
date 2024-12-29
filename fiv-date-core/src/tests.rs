#[allow(unused_imports)]
use crate::{
    CalDate, CloDate, Day, Days, Fraction, Hour, Minute, Month, Second, Time, Weekday, Year,
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
    assert_eq!(default, Weekday::Thursday);

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
    let default = CalDate::new();
    assert_eq!(default.day.0, 1);
    assert_eq!(default.month, Month::JAN);
    assert_eq!(default.year.0, 1970);

    for secs in 0..86400 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(secs + 1);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.day.0, 1 + ((secs + 1) / 86400) as u8);
    }

    for day in 0..1461 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400 * day);
        let exp =
            (day - match day {
                //Dec
                1430.. => 1430,
                //Nov
                1400.. => 1400,
                //Oct
                1369.. => 1369,
                //Sep
                1339.. => 1339,
                //Aug
                1308.. => 1308,
                //Jul
                1277.. => 1277,
                //Jun
                1247.. => 1247,
                //May
                1216.. => 1216,
                //Apr
                1186.. => 1186,
                //Mar
                1155.. => 1155,
                //Feb
                1127.. => 1127,
                //Jan
                1096.. => 1096,
                //Dec
                1065.. => 1065,
                //Nov
                1035.. => 1035,
                //Oct
                1004.. => 1004,
                //Sep
                974.. => 974,
                //Aug
                943.. => 943,
                //Jul
                912.. => 912,
                //Jun
                882.. => 882,
                //May
                851.. => 851,
                //Apr
                821.. => 821,
                //Mar
                790.. => 790,
                //Feb
                761.. => 761,
                //Jan
                730.. => 730,
                //Dec
                699.. => 699,
                //Nov
                669.. => 669,
                //Oct
                638.. => 638,
                //Sep
                608.. => 608,
                //Aug
                577.. => 577,
                //Jul
                546.. => 546,
                //Jun
                516.. => 516,
                //May
                485.. => 485,
                //Apr
                455.. => 455,
                //Mar
                424.. => 424,
                //Feb
                396.. => 396,
                //Jan
                365.. => 365,
                //Dec
                334.. => 334,
                //Nov
                304.. => 304,
                //Oct
                273.. => 273,
                //Sep
                243.. => 243,
                //Aug
                212.. => 212,
                //Jul
                181.. => 181,
                //Jun
                151.. => 151,
                //May
                120.. => 120,
                //Apr
                90.. => 90,
                //Mar
                59.. => 59,
                //Feb
                31.. => 31,
                //Jan
                _ => 0,
            }) + 1;
        let y1 = CalDate::now(&x1);
        println!("{}--{}--{}", y1.day, y1.month, y1.year);
        assert_eq!(y1.day.0, exp as u8)
    }

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(789 * 86400);
    let y1 = CalDate::now(&x1);
    assert_eq!(y1.day.0, 29);

    for day in 0..1461 {
        let day = day + 1;
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400 * day);
        let day = 1461 - day;
        let exp =
            (day - match day {
                //Dec
                1430.. => 1430,
                //Nov
                1400.. => 1400,
                //Oct
                1369.. => 1369,
                //Sep
                1339.. => 1339,
                //Aug
                1308.. => 1308,
                //Jul
                1277.. => 1277,
                //Jun
                1247.. => 1247,
                //May
                1216.. => 1216,
                //Apr
                1186.. => 1186,
                //Mar
                1155.. => 1155,
                //Feb
                1127.. => 1127,
                //Jan
                1096.. => 1096,
                //Dec
                1065.. => 1065,
                //Nov
                1035.. => 1035,
                //Oct
                1004.. => 1004,
                //Sep
                974.. => 974,
                //Aug
                943.. => 943,
                //Jul
                912.. => 912,
                //Jun
                882.. => 882,
                //May
                851.. => 851,
                //Apr
                821.. => 821,
                //Mar
                790.. => 790,
                //Feb
                761.. => 761,
                //Jan
                730.. => 730,
                //Dec
                699.. => 699,
                //Nov
                669.. => 669,
                //Oct
                638.. => 638,
                //Sep
                608.. => 608,
                //Aug
                577.. => 577,
                //Jul
                546.. => 546,
                //Jun
                516.. => 516,
                //May
                485.. => 485,
                //Apr
                455.. => 455,
                //Mar
                424.. => 424,
                //Feb
                396.. => 396,
                //Jan
                365.. => 365,
                //Dec
                334.. => 334,
                //Nov
                304.. => 304,
                //Oct
                273.. => 273,
                //Sep
                243.. => 243,
                //Aug
                212.. => 212,
                //Jul
                181.. => 181,
                //Jun
                151.. => 151,
                //May
                120.. => 120,
                //Apr
                90.. => 90,
                //Mar
                59.. => 59,
                //Feb
                31.. => 31,
                //Jan
                _ => 0,
            }) + 1;
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.day.0, exp as u8)
    }

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(672 * 86400);
    let y1 = CalDate::now(&x1);
    assert_eq!(y1.day.0, 29);

    for day in 0..31 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::JAN);
    }
    for day in 31..59 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::FEB);
    }
    for day in 59..90 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::MAR);
    }
    for day in 90..120 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::APR);
    }
    for day in 120..151 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::MAY);
    }
    for day in 151..181 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::JUN);
    }
    for day in 181..212 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::JUL);
    }
    for day in 212..243 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::AUG);
    }
    for day in 243..273 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::SEP);
    }
    for day in 273..304 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::OCT);
    }
    for day in 304..334 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::NOV);
    }
    for day in 334..365 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(day * 86400);
        let y1 = CalDate::now(&x1);
        assert_eq!(y1.month, Month::DEC);
    }

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(364 * 86400);
    let y1 = CalDate::now(&x1);
    assert_eq!(y1.year.0, 1970);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(365 * 86400);
    let y2 = CalDate::now(&x2);
    assert_eq!(y2.year.0, 1971);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(366 * 86400);
    let y3 = CalDate::now(&x3);
    assert_eq!(y3.year.0, 1971);
    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(729 * 86400);
    let y1 = CalDate::now(&x1);
    assert_eq!(y1.year.0, 1971);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(730 * 86400);
    let y2 = CalDate::now(&x2);
    assert_eq!(y2.year.0, 1972);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(731 * 86400);
    let y3 = CalDate::now(&x3);
    assert_eq!(y3.year.0, 1972);
    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(1095 * 86400);
    let y1 = CalDate::now(&x1);
    assert_eq!(y1.year.0, 1972);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(1096 * 86400);
    let y2 = CalDate::now(&x2);
    assert_eq!(y2.year.0, 1973);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(1097 * 86400);
    let y3 = CalDate::now(&x3);
    assert_eq!(y3.year.0, 1973);
    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(1460 * 86400);
    let y1 = CalDate::now(&x1);
    assert_eq!(y1.year.0, 1973);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(1461 * 86400);
    let y2 = CalDate::now(&x2);
    assert_eq!(y2.year.0, 1974);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(1462 * 86400);
    let y3 = CalDate::now(&x3);
    assert_eq!(y3.year.0, 1974);
    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(1825 * 86400);
    let y1 = CalDate::now(&x1);
    assert_eq!(y1.year.0, 1974);
    let x2 = SystemTime::UNIX_EPOCH + Duration::from_secs(1826 * 86400);
    let y2 = CalDate::now(&x2);
    assert_eq!(y2.year.0, 1975);
    let x3 = SystemTime::UNIX_EPOCH + Duration::from_secs(1827 * 86400);
    let y3 = CalDate::now(&x3);
    assert_eq!(y3.year.0, 1975);
}

#[test]
fn test_clodate() {
    let default = CloDate::new();
    assert_eq!(default.hour.0, 0);
    assert_eq!(default.minute.0, 0);
    assert_eq!(default.second.0, 0);

    for mins in 0..1440 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(mins * 60);
        let y1 = CloDate::now(&x1);
        assert_eq!(y1.hour.0, (mins / 60) as u8 % 24)
    }

    for mins in 0..1440 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(mins * 60);
        let y1 = CloDate::now(&x1);
        assert_eq!(y1.hour.0, ((1440 - mins) / 60) as u8 % 24)
    }

    for secs in 0..3600 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(secs);
        let y1 = CloDate::now(&x1);
        assert_eq!(y1.minute.0, (secs / 60) as u8 % 60)
    }

    for secs in 0..3600 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(secs);
        let y1 = CloDate::now(&x1);
        assert_eq!(y1.minute.0, ((3600 - secs) / 60) as u8 % 60)
    }

    for secs in 0..120 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(secs);
        let y1 = CloDate::now(&x1);
        assert_eq!(y1.second.0, (secs % 60) as u8)
    }

    for secs in 0..120 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(secs);
        let y1 = CloDate::now(&x1);
        assert_eq!(y1.second.0, (60 - (secs % 60)) as u8 % 60)
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
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(86400 * day);
        let exp =
            (day - match day {
                //Dec
                1430.. => 1430,
                //Nov
                1400.. => 1400,
                //Oct
                1369.. => 1369,
                //Sep
                1339.. => 1339,
                //Aug
                1308.. => 1308,
                //Jul
                1277.. => 1277,
                //Jun
                1247.. => 1247,
                //May
                1216.. => 1216,
                //Apr
                1186.. => 1186,
                //Mar
                1155.. => 1155,
                //Feb
                1127.. => 1127,
                //Jan
                1096.. => 1096,
                //Dec
                1065.. => 1065,
                //Nov
                1035.. => 1035,
                //Oct
                1004.. => 1004,
                //Sep
                974.. => 974,
                //Aug
                943.. => 943,
                //Jul
                912.. => 912,
                //Jun
                882.. => 882,
                //May
                851.. => 851,
                //Apr
                821.. => 821,
                //Mar
                790.. => 790,
                //Feb
                761.. => 761,
                //Jan
                730.. => 730,
                //Dec
                699.. => 699,
                //Nov
                669.. => 669,
                //Oct
                638.. => 638,
                //Sep
                608.. => 608,
                //Aug
                577.. => 577,
                //Jul
                546.. => 546,
                //Jun
                516.. => 516,
                //May
                485.. => 485,
                //Apr
                455.. => 455,
                //Mar
                424.. => 424,
                //Feb
                396.. => 396,
                //Jan
                365.. => 365,
                //Dec
                334.. => 334,
                //Nov
                304.. => 304,
                //Oct
                273.. => 273,
                //Sep
                243.. => 243,
                //Aug
                212.. => 212,
                //Jul
                181.. => 181,
                //Jun
                151.. => 151,
                //May
                120.. => 120,
                //Apr
                90.. => 90,
                //Mar
                59.. => 59,
                //Feb
                31.. => 31,
                //Jan
                _ => 0,
            }) + 1;
        let y1 = Day::now(&x1);
        assert_eq!(y1.0, exp as u8)
    }

    let x1 = SystemTime::UNIX_EPOCH + Duration::from_secs(789 * 86400);
    let y1 = Day::now(&x1);
    assert_eq!(y1.0, 29);

    for day in 0..1461 {
        let day = day + 1;
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(86400 * day);
        let day = 1461 - day;
        let exp =
            (day - match day {
                //Dec
                1430.. => 1430,
                //Nov
                1400.. => 1400,
                //Oct
                1369.. => 1369,
                //Sep
                1339.. => 1339,
                //Aug
                1308.. => 1308,
                //Jul
                1277.. => 1277,
                //Jun
                1247.. => 1247,
                //May
                1216.. => 1216,
                //Apr
                1186.. => 1186,
                //Mar
                1155.. => 1155,
                //Feb
                1127.. => 1127,
                //Jan
                1096.. => 1096,
                //Dec
                1065.. => 1065,
                //Nov
                1035.. => 1035,
                //Oct
                1004.. => 1004,
                //Sep
                974.. => 974,
                //Aug
                943.. => 943,
                //Jul
                912.. => 912,
                //Jun
                882.. => 882,
                //May
                851.. => 851,
                //Apr
                821.. => 821,
                //Mar
                790.. => 790,
                //Feb
                761.. => 761,
                //Jan
                730.. => 730,
                //Dec
                699.. => 699,
                //Nov
                669.. => 669,
                //Oct
                638.. => 638,
                //Sep
                608.. => 608,
                //Aug
                577.. => 577,
                //Jul
                546.. => 546,
                //Jun
                516.. => 516,
                //May
                485.. => 485,
                //Apr
                455.. => 455,
                //Mar
                424.. => 424,
                //Feb
                396.. => 396,
                //Jan
                365.. => 365,
                //Dec
                334.. => 334,
                //Nov
                304.. => 304,
                //Oct
                273.. => 273,
                //Sep
                243.. => 243,
                //Aug
                212.. => 212,
                //Jul
                181.. => 181,
                //Jun
                151.. => 151,
                //May
                120.. => 120,
                //Apr
                90.. => 90,
                //Mar
                59.. => 59,
                //Feb
                31.. => 31,
                //Jan
                _ => 0,
            }) + 1;
        let y1 = Day::now(&x1);
        assert_eq!(y1.0, exp as u8)
    }

    let x1 = SystemTime::UNIX_EPOCH - Duration::from_secs(672 * 86400);
    let y1 = Day::now(&x1);
    assert_eq!(y1.0, 29);
}

#[test]
fn test_fraction() {
    let default = Fraction::new();
    assert_eq!(default.0, 0);

    for milli in 0..10000 {
        let x1 = SystemTime::UNIX_EPOCH + Duration::from_millis(milli);
        let y1 = Fraction::now(&x1);
        assert_eq!(y1.0, (milli % 1000) as u16)
    }

    for milli in 0..10000 {
        let x1 = SystemTime::UNIX_EPOCH - Duration::from_millis(milli);
        let y1 = Fraction::now(&x1);
        assert_eq!(y1.0, ((1000 - (milli % 1000)) % 1000) as u16)
    }
}
