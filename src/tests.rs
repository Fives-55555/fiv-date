#[allow(unused_imports)]
use std::{str::FromStr, time::SystemTime};

use crate as fiv_date;
use fiv_date_macro::custom_format_struct;

custom_format_struct!(
    All,
    "abc}}{{{D}|{DDD}|{www}|{DD}|{MM#}|{YYYY}|{ww}|{hh}|{mm}|{ss}|{fff}",
    true
);

#[test]
fn test() {
    let default = All::new();
    println!("{default}");
    let now = All::now(&SystemTime::now());
    println!("{now}");
    let str = &format!("{now}");
    let may = All::from_str(str);
    let x = may.unwrap();
    println!("{x}")
}
