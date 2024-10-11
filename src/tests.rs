#[allow(unused_imports)]
use std::time::{SystemTime, Duration};
#[allow(unused_imports)]
use crate::{cal::{JAN,FEB,MAR,APR,MAY,JUN,JUL,AUG,SEP,OCT,NOV,DEC}, Month, Year, Time};

#[test]
fn test_year() {
    let default = Year::new();
    assert!(default == 1970);
    let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(364*86400);
    let y1 = Year::now(&x1);
    assert!(y1==1970);
    let x2 = SystemTime::UNIX_EPOCH+Duration::from_secs(365*86400);
    let y2 = Year::now(&x2);
    assert!(y2==1971);
    let x3 = SystemTime::UNIX_EPOCH+Duration::from_secs(366*86400);
    let y3 = Year::now(&x3);
    assert!(y3==1971);
    let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(729*86400);
    let y1 = Year::now(&x1);
    assert!(y1==1971);
    let x2 = SystemTime::UNIX_EPOCH+Duration::from_secs(730*86400);
    let y2 = Year::now(&x2);
    assert!(y2==1972);
    let x3 = SystemTime::UNIX_EPOCH+Duration::from_secs(731*86400);
    let y3 = Year::now(&x3);
    assert!(y3==1972);
    let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(1095*86400);
    let y1 = Year::now(&x1);
    assert!(y1==1972);
    let x2 = SystemTime::UNIX_EPOCH+Duration::from_secs(1096*86400);
    let y2 = Year::now(&x2);
    assert!(y2==1973);
    let x3 = SystemTime::UNIX_EPOCH+Duration::from_secs(1097*86400);
    let y3 = Year::now(&x3);
    assert!(y3==1973);
    let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(1460*86400);
    let y1 = Year::now(&x1);
    assert!(y1==1973);
    let x2 = SystemTime::UNIX_EPOCH+Duration::from_secs(1461*86400);
    let y2 = Year::now(&x2);
    assert!(y2==1974);
    let x3 = SystemTime::UNIX_EPOCH+Duration::from_secs(1462*86400);
    let y3 = Year::now(&x3);
    assert!(y3==1974);
    let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(1825*86400);
    let y1 = Year::now(&x1);
    assert!(y1==1974);
    let x2 = SystemTime::UNIX_EPOCH+Duration::from_secs(1826*86400);
    let y2 = Year::now(&x2);
    assert!(y2==1975);
    let x3 = SystemTime::UNIX_EPOCH+Duration::from_secs(1827*86400);
    let y3 = Year::now(&x3);
    assert!(y3==1975);
}

#[test]
fn test_month() {
    let default = Month::new();
    assert!(default == JAN);
    for day in 0..31 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==JAN);
    }
    for day in 31..59 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==FEB);
    }
    for day in 59..90 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==MAR);
    }
    for day in 90..120 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==APR);
    }
    for day in 120..151 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==MAY);
    }
    for day in 151..181 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==JUN);
    }
    for day in 181..212 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==JUL);
    }
    for day in 212..243 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==AUG);
    }
    for day in 243..273 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==SEP);
    }
    for day in 273..304 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==OCT);
    }
    for day in 304..334 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==NOV);
    }
    for day in 334..365 {
        let x1 = SystemTime::UNIX_EPOCH+Duration::from_secs(day*86400);
        let y1 = Month::now(&x1);
        assert!(y1==DEC);
    }
}