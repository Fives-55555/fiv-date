use crate::custom_format_struct;

use crate as fiv_date;

custom_format_struct!(BasicDate, "{YYYY}{MM}{DD}", true);
custom_format_struct!(BasicDateWS, "{YYYY}-{MM}-{DD}", true);
custom_format_struct!(YearMonth, "{YYYY}{MM}", true);
custom_format_struct!(YearMonthWS, "{YYYY}{MM}", true);

#[test]
fn test() {
    let x = BasicDate::new();
    println!("BasicDate: {};", x);
    let y = BasicDate::now(&std::time::SystemTime::now());
    println!("BasicDateWS: {}", y);
}
