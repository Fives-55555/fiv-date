use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn format_date_struct(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    let name = match tokens.next() {
        Some(TokenTree::Ident(name)) => name,
        _ => panic!("Missing Name"),
    };

    let _ = tokens.next();

    let format_str = match tokens.next() {
        Some(TokenTree::Literal(l)) => l.to_string(),
        _ => panic!("Missing Format String"),
    };

    let mut chars = format_str.chars();
    chars.next();
    let mut char = chars.next();

    if char.is_none() {
        panic!("Impossible::1")
    }

    let len = format_str.len();
    let mut i = 0;

    let mut print = String::new();
    let mut format = Vec::new();

    'outer: while i < len - 1 {
        match char.unwrap() {
            'Y' => {
                for x in 1..4 {
                    char = chars.next();
                    if !(char.is_some() && char.unwrap() == 'Y') {
                        print.push_str("Y".repeat(x).as_str());
                        i += x + 1;
                        continue 'outer;
                    }
                }
                char = chars.next();
                format.push(FormatThing::Year);
                print.push_str("{:04}");
                i += 4;
            }
            'M' => {
                char = chars.next();
                if char.is_some() && char.unwrap() == 'M' {
                    print.push_str("{:02}");
                    format.push(FormatThing::Month);
                    char = chars.next();
                    i += 2;
                } else {
                    print.push('M');
                    i += 1;
                }
            }
            'D' => {
                char = chars.next();
                if char.is_some() && char.unwrap() == 'D' {
                    char = chars.next();
                    if char.is_some() && char.unwrap() == 'D' {
                        char = chars.next();
                        format.push(FormatThing::Days);
                        print.push_str("{:03}");
                        i += 1;
                    } else {
                        format.push(FormatThing::Day);
                        print.push_str("{:02}");
                    }
                    i += 1;
                } else {
                    format.push(FormatThing::Weeknum);
                    print.push_str("{}");
                }
                i += 1;
            }
            'w' => {
                char = chars.next();
                if char.is_some() && char.unwrap() == 'w' {
                    print.push_str("{:02}");
                    format.push(FormatThing::Weeks);
                    char = chars.next();
                    i += 2;
                } else {
                    print.push('w');
                    i += 1;
                }
            }
            '\\' => {
                char = chars.next();
                if char.is_some() && char.unwrap() == '\\' {
                    char = chars.next();
                    if char.is_some() {
                        print.push(char.unwrap());
                        char = chars.next();
                        i += 3;
                    } else {
                        panic!("Empty Escape")
                    }
                }
            }
            _ => {
                print.push(char.unwrap());
                i += 1;
                char = chars.next();
            }
        }
    }
    let mut fields = Needed::default();
    for thing in format.iter() {
        match thing {
            FormatThing::Weeknum => fields.add(FormatThing::Weekday),
            _ => fields.add(thing.clone()),
        }
    }
    let field = fields.to_fields();
    let impls = fields.to_impls();
    let new = fields.to_new();
    let now = fields.to_now();
    let z = format!(
        r#"
{impls}
        
pub struct {name} {{
    {}
}}
    
impl {name} {{
    pub fn new()->{name} {{
        {name} {{
            {}
        }}
    }}
    pub fn now(s: &SystemTime)->{name} {{
        {name} {{
            {}
        }}
    }}
}}
    
impl std::fmt::Display for {name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f,"{print}", {})
    }}
}}"#,
        field,
        new,
        now,
        format
            .iter()
            .map(|thing| thing.to_fmt(fields.caldate, fields.clodate))
            .collect::<Vec<&str>>()
            .join(", ")
    );
    z.parse().unwrap()
}
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
enum FormatThing {
    Weeks, //of the Year
    Weekday,
    Weeknum, //Num of Weekday
    Day,     //of Month
    Days,    //of Year
    Month,
    Year,
    Seconds,
    Minutes,
    Hours,
    Timezone,
}

impl FormatThing {
    fn to_fmt(&self, caldate: bool, clodate: bool) -> &str {
        match self {
            FormatThing::Weeks => "self.weeks",
            FormatThing::Weeknum => "self.weekday.to_num()",
            FormatThing::Weekday => "self.weekday",
            FormatThing::Day => {
                if caldate {
                    "self.caldate.day"
                } else {
                    "self.day"
                }
            }
            FormatThing::Days => "self.days",
            FormatThing::Month => {
                if caldate {
                    "self.caldate.month"
                } else {
                    "self.month"
                }
            }
            FormatThing::Year => {
                if caldate {
                    "self.caldate.year"
                } else {
                    "self.year"
                }
            }
            FormatThing::Seconds => {
                if clodate {
                    "self.clodate.seconds"
                } else {
                    "self.seconds"
                }
            }
            FormatThing::Minutes => {
                if clodate {
                    "self.clodate.minutes"
                } else {
                    "self.minutes"
                }
            }
            FormatThing::Hours => {
                if clodate {
                    "self.clodate.hours"
                } else {
                    "self.hours"
                }
            }
            FormatThing::Timezone => "self.timezone",
            //FormatThing::Weeks => "self.weeks"
        }
    }
}

struct Needed {
    weeks: bool,
    weekday: bool,
    day: bool,
    days: bool,
    month: bool,
    year: bool,
    caldate: bool,
    seconds: bool,
    minutes: bool,
    hours: bool,
    clodate: bool,
    timezone: bool,
}

impl Default for Needed {
    fn default() -> Self {
        Needed {
            weeks: false,
            weekday: false,
            day: false,
            days: false,
            month: false,
            year: false,
            caldate: false,
            seconds: false,
            minutes: false,
            hours: false,
            clodate: false,
            timezone: false,
        }
    }
}

impl Needed {
    fn to_fields(&mut self) -> String {
        if self.day & self.month & self.year {
            self.day = false;
            self.month = false;
            self.year = false;
            self.caldate = true;
        }
        if self.seconds & self.minutes & self.hours {
            self.seconds = false;
            self.minutes = false;
            self.hours = false;
            self.clodate = true;
        }
        let mut str = String::new();
        if self.weeks {
            str.push_str("weeks: Weeks,");
        }
        if self.weekday {
            str.push_str("weekday: Weekday,");
        }
        if self.day {
            str.push_str("day: Day,");
        }
        if self.month {
            str.push_str("month: Month,");
        }
        if self.year {
            str.push_str("year: Year,");
        }
        if self.caldate {
            str.push_str("caldate: CalDate,");
        }
        if self.seconds {
            str.push_str("seconds: Seconds,");
        }
        if self.minutes {
            str.push_str("minutes: Minutes,");
        }
        if self.hours {
            str.push_str("hours: Hours,");
        }
        if self.clodate {
            str.push_str("clodate: CloDate,");
        }
        if self.timezone {
            str.push_str("timezone: Timezone,");
        }
        return str;
    }
    fn to_new(&self) -> String {
        let mut str = String::new();
        if self.weeks {
            str.push_str("weeks: Weeks::new(),");
        }
        if self.weekday {
            str.push_str("weekday: Weekday::new(),");
        }
        if self.day {
            str.push_str("day: Day::new(),");
        }
        if self.month {
            str.push_str("month: Month::new(),");
        }
        if self.year {
            str.push_str("year: Year::new(),");
        }
        if self.caldate {
            str.push_str("caldate: CalDate::new(),");
        }
        if self.seconds {
            str.push_str("seconds: Seconds::new(),");
        }
        if self.minutes {
            str.push_str("minutes: Minutes::new(),");
        }
        if self.hours {
            str.push_str("hours: Hours::new(),");
        }
        if self.clodate {
            str.push_str("clodate: CloDate::new(),");
        }
        if self.timezone {
            str.push_str("timezone: Timezone::new(),");
        }
        return str;
    }
    fn to_now(&self) -> String {
        let mut str = String::new();
        if self.weeks {
            str.push_str("weeks: Weeks::now(s),");
        }
        if self.weekday {
            str.push_str("weekday: Weekday::now(s),");
        }
        if self.day {
            str.push_str("day: Day::now(s),");
        }
        if self.month {
            str.push_str("month: Month::now(s),");
        }
        if self.year {
            str.push_str("year: Year::now(s),");
        }
        if self.caldate {
            str.push_str("caldate: CalDate::now(s),");
        }
        if self.seconds {
            str.push_str("seconds: Seconds::now(s),");
        }
        if self.minutes {
            str.push_str("minutes: Minutes::now(s),");
        }
        if self.hours {
            str.push_str("hours: Hours::now(s),");
        }
        if self.clodate {
            str.push_str("clodate: CloDate::now(s),");
        }
        if self.timezone {
            str.push_str("timezone: Timezone::now(s),");
        }
        return str;
    }
    fn to_impls(&self) -> String {
        let mut str = String::from("use fiv_date::{Time,");
        if self.weeks {
            str.push_str("Weeks,");
        }
        if self.weekday {
            str.push_str("Weekday,");
        }
        if self.day {
            str.push_str("Day,");
        }
        if self.month {
            str.push_str("Month,");
        }
        if self.year {
            str.push_str("Year,");
        }
        if self.caldate {
            str.push_str("CalDate,");
        }
        if self.seconds {
            str.push_str("Seconds,");
        }
        if self.minutes {
            str.push_str("Minutes,");
        }
        if self.hours {
            str.push_str("Hours,");
        }
        if self.clodate {
            str.push_str("CloDate,");
        }
        if self.timezone {
            str.push_str("Timezone,");
        }
        str.push_str("};");
        return str;
    }
    fn add(&mut self, fmt: FormatThing) {
        match fmt {
            FormatThing::Days => self.days = true,
            FormatThing::Day => self.day = true,
            FormatThing::Month => self.month = true,
            FormatThing::Year => self.year = true,
            FormatThing::Seconds => self.seconds = true,
            FormatThing::Minutes => self.minutes = true,
            FormatThing::Hours => self.hours = true,
            FormatThing::Weekday => self.weekday = true,
            FormatThing::Weeknum => self.weekday = true,
            FormatThing::Weeks => self.weeks = true,
            FormatThing::Timezone => self.timezone = true,
        }
    }
}
