//! # Fiv-Date-Macro
//! Underlyning Layer of the Fiv-Date crate
//!
//! ## Usage
//! !!!Not recommended!!!
//! Because it also needs the Fiv-Date crate for Types and Funcs
//!
//! ## ONLY If you really want use this crate
//! This crate hopefully implements the Fiv-Date automaticly
//!

use proc_macro::{TokenStream, TokenTree};

///Produces a struct with a given name and a custom format for date and time
///
/// This Macro optimizes the Custom Struct hopefully at Run-Time
/// ## Usage
/// ```
///     let docs = r#"
/// /// A correct Documention String is required (The three Slashes are required)
/// "#;
///     custom_format_struct!(MyStruct, "{DD}.{MM}.{YYYY}", false, docs)
///     //                    ^^^^^^^^  ^^^^^^^^^^^^^^^^^^  ^^^^^^
///     //                    Name      Format String       Optional
/// ```
///
/// Available Formatter
/// | Name | Values(Ranges are inclusive) | Formatter |
/// | ---- | ---------------------------- | --------- |
/// | Days of week           | 1..7         | {D}    |
/// | Days of year           | 001..366     | {DDD}  |
/// | Weekday                | Mon,Tuw,etc. | {www}  |
/// | Day(of Month)          | 01..31       | {DD}   |
/// | Month                  | 01..12       | {MM#}  |
/// | Year                   | 0000..9999   | {YYYY} |
/// | Weeks of Year          | 00..53       | {ww}   |
/// | Hour                   | 00..24       | {hh}   |
/// | Minute                 | 00..59       | {mm}   |
/// | Second                 | 00..60       | {ss}   |
/// | Fraction(Milliseconds) | 000..999     | {fff}  |
///
/// ### Soon Durations
#[proc_macro]
pub fn custom_format_struct(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    let name = match tokens.next() {
        Some(TokenTree::Ident(name)) => name,
        _ => panic!("Missing Name"),
    };

    let _ = match tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => (),
        _ => panic!("Missing comma seperator"),
    };

    let format_str = match tokens.next() {
        Some(TokenTree::Literal(l))
            if l.to_string().starts_with('"') && l.to_string().ends_with('"') =>
        {
            l.to_string()
        }
        _ => panic!("Missing format String"),
    };
    //Logic for Import from the outer bracket
    let supers = 'xy: {
        let _ = match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => (),
            _ => break 'xy String::new(),
        };

        let str = match tokens.next() {
            Some(TokenTree::Ident(l)) => {
                let x = l.to_string();
                if &x == "true" {
                    String::from("super::")
                } else if &x == "false" {
                    String::new()
                } else {
                    panic!("Wrong parameter (Should be a bool)")
                }
            },
            _ => panic!("Too many commas or missing bool"),
        };
        break 'xy str;
    };

    let docs = 'xy: {
        let _ = match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => (),
            _ => break 'xy String::new(),
        };
        let str = match tokens.next() {
            Some(TokenTree::Literal(l)) if l.to_string().starts_with('"') && l.to_string().ends_with('"') => {
                let x = l.to_string();
                x[1..x.len()-1].to_string()
            }
            _ => panic!("Too many commas or missing String"),
        };
        break 'xy str;
    };

    let str = &format_str[1..format_str.len() - 1];

    let mut chars = str.chars();
    let mut char: Option<char> = chars.next();

    if char.is_none() {
        panic!("Impossible::1")
    }

    let len = str.len();
    let mut i = 1;

    let mut format = Vec::new();

    while i < len {
        match char.unwrap() {
            '{' => {
                char = chars.next();
                i += 1;
                if char.is_none() {
                    panic!("Wrong format pattern (Missing pattern)")
                };
                match char.unwrap() {
                    'Y' => {
                        for _ in 1..4 {
                            char = chars.next();
                            i += 1;
                            if !(char.is_some() && char.unwrap() == 'Y') {
                                panic!("Wrong Format Pattern (Missing 'Y')")
                            }
                        }
                        format.push(FormatThing::Year);
                    }
                    'M' => {
                        char = chars.next();
                        i += 1;
                        if char.is_some() && char.unwrap() == 'M' {
                            format.push(FormatThing::Month);
                        } else {
                            panic!("Wrong Format Pattern (Missing 'M')")
                        }
                        char = chars.next();
                        i += 1;
                        if char.is_some() && char.unwrap() == '#' {
                            char = chars.next();
                            i += 1;
                            format.pop();
                            format.push(FormatThing::MonthAlph);
                            
                        }
                        if char.is_none() || char.unwrap() != '}' {
                            panic!("Wrong Format Delimitter ('}}')")
                        }
                        continue;
                    }
                    'w' => {
                        char = chars.next();
                        i += 1;
                        if char.is_some() && char.unwrap() == 'w' {
                            format.push(FormatThing::Weeks);
                        } else {
                            panic!("Wrong Format Pattern (Missing 'w')")
                        }
                    }
                    'D' => {
                        char = chars.next();
                        i += 1;
                        if char.is_some() && char.unwrap() == 'D' {
                            char = chars.next();
                            i += 1;
                            if char.is_some() && char.unwrap() == 'D' {
                                format.push(FormatThing::Days);
                            } else {
                                format.push(FormatThing::Day);
                            }
                        } else {
                            format.push(FormatThing::Weeknum);
                        }
                        if char.is_none() || char.unwrap() != '}' {
                            panic!("Wrong Format Delimitter ('}}')")
                        }
                        continue;
                    }
                    'h' => {
                        char = chars.next();
                        i += 1;
                        if char.is_some() && char.unwrap() == 'h' {
                            format.push(FormatThing::Hours);
                        } else {
                            panic!("Wrong Format Pattern (Missing 'h')")
                        }
                    }
                    'm' => {
                        char = chars.next();
                        i += 1;
                        if char.is_some() && char.unwrap() == 'm' {
                            format.push(FormatThing::Minutes);
                        } else {
                            panic!("Wrong Format Pattern (Missing 'm')")
                        }
                    }
                    's' => {
                        char = chars.next();
                        i += 1;
                        if char.is_some() && char.unwrap() == 's' {
                            format.push(FormatThing::Seconds);
                        } else {
                            panic!("Wrong Format Pattern (Missing 's')")
                        }
                    }
                    'f' => {
                        for _ in 1..3 {
                            char = chars.next();
                            i += 1;
                            if !(char.is_some() && char.unwrap() == 'f') {
                                panic!("Wrong Format Pattern (Missing 'f')")
                            }
                        }
                        format.push(FormatThing::Fraction);
                    }
                    '{' => {
                        format.push(FormatThing::BracketR);
                    }

                    _ => {
                        panic!("Wrong Format Pattern (Wrong Pattern)")
                    }
                };
                char = chars.next();
                i += 1;
                if !(char.is_some() && char.unwrap() == '}') {
                    panic!("Wrong Format Delimitter ('}}')")
                }
            }
            '}' => {
                format.push(FormatThing::BracketL);
            }
            _ => {
                format.push(FormatThing::Extra(char.unwrap()));
            }
        }
        char = chars.next();
        i += 1;
    }
    let mut fields = Needed::default();
    for thing in format.iter() {
        match thing {
            FormatThing::Weeknum => fields.add(FormatThing::Weekday),
            _ => fields.add(thing.clone()),
        }
    }

    let field = fields.to_fields();
    let print = to_print(&format);
    let fmt = to_fmt(&format, fields.caldate, fields.clodate);
    let new = fields.to_new();
    let impls = fields.to_impl();
    let now = fields.to_now();
    let modn = name.to_string().to_ascii_lowercase();
    let to_date = to_date(&format, &format!("{name}DPErr"));
    let z = format!(
        r#"
pub use {modn}::{name};

mod {modn} {{
    use {supers}fiv_date::{{Time,{impls}}};

    {docs}
    ///
    /// ## Custom Date Format: {format_str}
    pub struct {name} {{
        {field}
    }}
        
    impl {name} {{
        pub fn new()->{name} {{
            {name} {{
                {new}
            }}
        }}
        pub fn now(s: &std::time::SystemTime)->{name} {{
            {name} {{
                {now}
            }}
        }}
    }}
        
    impl std::fmt::Display for {name} {{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
            write!(f,"{print}", {fmt})
        }}
    }}

    #[derive(Debug)]
    pub struct {name}DPErr;

    impl std::str::FromStr for {name} {{
        type Err = {name}DPErr;//Date Parse Error 
        fn from_str(s: &str) ->Result<Self, Self::Err> {{
            let mut date = {name}::new();
            let mut str: &str = s;
            {to_date}
        }}
    }}
}}"#);
    z.parse().unwrap()
}
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
enum FormatThing {
    Weeks, //of the Year //
    Weekday, 
    Weeknum, //Num of Weekday //
    Day,     //of Month //
    Days,    //of Year //
    MonthAlph,
    Month, //
    Year, //
    Fraction, //
    Seconds, //
    Minutes, //
    Hours, //
    Timezone,
    Extra(char), //
    BracketR, //
    BracketL, //
}

impl FormatThing {
    fn to_fmt(&self, caldate: bool, clodate: bool) -> &str {
        match self {
            FormatThing::Weeks => "self.weeks,",
            FormatThing::Weeknum => "self.weekday.to_num(),",
            FormatThing::Weekday => "self.weekday,",
            FormatThing::Day => {
                if caldate {
                    "self.caldate.day,"
                } else {
                    "self.day,"
                }
            },
            FormatThing::Fraction => "self.fraction,",
            FormatThing::Days => "self.days,",
            FormatThing::Month => {
                if caldate {
                    "self.caldate.month,"
                } else {
                    "self.month,"
                }
            }
            FormatThing::MonthAlph => {
                if caldate {
                    "self.caldate.month.as_str(),"
                } else {
                    "self.month.as_str(),"
                }
            }
            FormatThing::Year => {
                if caldate {
                    "self.caldate.year,"
                } else {
                    "self.year,"
                }
            }
            FormatThing::Seconds => {
                if clodate {
                    "self.clodate.second,"
                } else {
                    "self.seconds,"
                }
            }
            FormatThing::Minutes => {
                if clodate {
                    "self.clodate.minute,"
                } else {
                    "self.minutes,"
                }
            }
            FormatThing::Hours => {
                if clodate {
                    "self.clodate.hour,"
                } else {
                    "self.hours,"
                }
            }
            FormatThing::Timezone => "self.timezone,",
            FormatThing::BracketL | &FormatThing::BracketR | &FormatThing::Extra(_) => "",
        }
    }
    fn to_type(&self)->&str {
        match self {
            FormatThing::Weeks=>"Weeks",
            FormatThing::Weeknum=>"Weekday",
            FormatThing::Day=>"Day",
            FormatThing::Month=>"Month",
            FormatThing::Year=>"Year",
            FormatThing::Seconds=>"Seconds",
            FormatThing::Minutes=>"Minutes",
            FormatThing::Hours=>"Hours",
            FormatThing::Timezone=>"Timezone",
            FormatThing::Fraction=>"Fraction",
            FormatThing::Days=>"Days",
            FormatThing::BracketL | Self::BracketR | FormatThing::MonthAlph | FormatThing::Extra(_) | FormatThing::Weekday=>"",
        }
    }
    fn to_field_name(&self, caldate: bool, clodate: bool)->&str{
        match self {
            FormatThing::Weeks => "weeks",
            FormatThing::Weeknum => "weekday.to_num()",
            FormatThing::Weekday => "weekday",
            FormatThing::Day => {
                if caldate {
                    "caldate.day"
                } else {
                    "day"
                }
            },
            FormatThing::Fraction => "fraction",
            FormatThing::Days => "days",
            FormatThing::Month => {
                if caldate {
                    "caldate.month"
                } else {
                    "month"
                }
            }
            FormatThing::MonthAlph =>{
                if caldate {
                    "caldate.month.as_str()"
                } else {
                    "month.as_str()"
                }
            },
            FormatThing::Year => {
                if caldate {
                    "caldate.year"
                } else {
                    "year"
                }
            }
            FormatThing::Seconds => {
                if clodate {
                    "clodate.second"
                } else {
                    "seconds"
                }
            }
            FormatThing::Minutes => {
                if clodate {
                    "clodate.minute"
                } else {
                    "minutes"
                }
            }
            FormatThing::Hours => {
                if clodate {
                    "clodate.hour"
                } else {
                    "hours"
                }
            }
            FormatThing::Timezone => "timezone",
            FormatThing::BracketL | &FormatThing::BracketR | &FormatThing::Extra(_) => "",
        }
    }}
}

struct Needed {
    weeks: bool,
    weekday: bool,
    day: bool,
    days: bool,
    month: bool,
    year: bool,
    caldate: bool,
    fraction: bool,
    seconds: bool,
    minutes: bool,
    hours: bool,
    clodate: bool,
    timezone: bool,
}

impl Default for Needed {
    fn default() -> Self {
        Needed {
            fraction: false,
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
        if self.fraction {
            str.push_str("fraction: Fraction,");
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
        if self.fraction {
            str.push_str("fraction: Fraction::new(),");
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
        if self.fraction {
            str.push_str("fraction: Fraction::now(s),");
        }
        return str;
    }
    fn to_impl(&self) -> String {
        let mut str = String::new();
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
            str.push_str("CalDate, Year, Month, Day,");
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
            str.push_str("CloDate, Seconds, Minutes, Hours,");
        }
        if self.timezone {
            str.push_str("Timezone,");
        }
        if self.fraction {
            str.push_str("Fraction,");
        }
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
            FormatThing::Fraction => self.fraction = true,
            FormatThing::MonthAlph => (),
            FormatThing::BracketL => (),
            FormatThing::BracketR => (),
            FormatThing::Extra(_) => (),
        }
    }
}

fn to_print(v: &Vec<FormatThing>) -> String {
    let mut str = String::with_capacity(v.len() * 3);
    for elem in v.iter() {
        str.push_str(match elem {
            FormatThing::BracketL => "}}",
            FormatThing::BracketR => "{{",
            FormatThing::Weeknum
            | FormatThing::Month
            | FormatThing::Timezone
            | FormatThing::MonthAlph
            | FormatThing::Weekday => "{}",
            FormatThing::Day
            | FormatThing::Hours
            | FormatThing::Seconds
            | FormatThing::Minutes
            | FormatThing::Weeks => "{:02}",
            FormatThing::Days | FormatThing::Fraction => "{:03}",
            FormatThing::Year => "{:04}",
            FormatThing::Extra(char) => {
                str.push(char.clone());
                continue;
            }
        });
    }
    str
}

fn to_fmt(v: &Vec<FormatThing>, caldate: bool, clodate: bool) -> String {
    let mut str = String::with_capacity(v.len() * 8);
    for elem in v.iter() {
        str.push_str("self.");
        str.push_str(elem.to_field_name(caldate, clodate);
        str.push(',');
    }
    str
}

fn to_date(v: &Vec<FormatThing>, errname: &str, caldate: bool, clodate: bool)->String{
    let mut str = String::new();
    for elem in v.iter() {
        match elem {
            FormatThing::BracketL =>{
                str.push_str(&format!("let char = str.char().next();
                if char.is_none() || char.unwrap() != '{{' {{
                    return Err({errname}{{}});
                }}else{{
                    str = &str[1..];
                }}
                ")); 
            },
            FormatThing::BracketR => {
                str.push_str(&format!("let char = str.char().next();
                if char.is_none() || char.unwrap() != '}}' {{
                    return Err({errname}{{}});
                }}else{{
                    str = &str[1..];
                }}
                "));
            },
            FormatThing::Extra(char) => {
                str.push_str(&format!("let char = str.char().next();
                if char.is_none() || char.unwrap() != '{char}' {{
                    return Err({errname}{{}});
                }}else{{
                    str = &str[1..];
                }}
                "));
            },
            FormatThing::MonthAlph=>{
                str.push_str(&format!(r#"match Month::from_str(&str[..3]) {{
                    Ok(month)=>{{
                        str = &str[3..];
                        if caldate {{
                            date.caldate.month = month;
                        }}else{{
                            date.month = month;
                        }}
                    }},
                    Err(_)=>return Err({errname}{{}}),
                }}"#));
            }
            FormatThing::Weekday=>{
                str.push_str(&format!(r#"match Weekday::from_str(&str[..3]) {{
                        Ok(date)=>{{
                            str = &str[3..];
                            date.weekday = date;
                        }},
                        Err(_)=>{errname}{{}}
                    }}"#));
            }
            _=>{
                str.push_str(&format!(r#"match {}::to_date(str) {{
                    Ok(date, n_str)=>{{
                        str = n_str;
                        date.{} = date;
                    }},
                    Err(_)=>return Err({errname}{{}}),
                }}
                "#, elem.to_type(), elem.to_field_name(caldate, clodate)));
            }
        }
    }
    str
}