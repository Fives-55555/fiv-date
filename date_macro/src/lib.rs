use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn format_date_struct(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    
    let name = match tokens.next() {
        Some(TokenTree::Ident(name)) => name,
        _ => panic!("Missing Name"),
    };
    
    let _ = tokens.next();
    
    let mut format_str = match tokens.next() {
        Some(TokenTree::Literal(l)) => l.to_string(),
        _ => panic!("Missing Format String"),
    };
    
    format_str.remove(0);
    
    let mut chars = format_str.chars();
    let mut char = chars.next();

    if char.is_none() {
        panic!("Impossible::1")
    }

    let len = format_str.len();
    let mut i = 0;
    
    let mut print = String::new();
    let mut format = Vec::new();
    
    'outer: while i < len {
        match char.unwrap() {
            '"' => break,
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
    let mut fields: Vec<FormatThing> = Vec::new();
    for thing in format.iter() {
        match thing {
            FormatThing::Weeknum => {
                if !fields.contains(&FormatThing::Weekday) {
                    fields.push(FormatThing::Weekday);
                }
            }
            _ => {
                if !fields.contains(thing) {
                    fields.push(thing.clone());
                }
            }
        }
    }
    if fields.contains(&FormatThing::Day)
        && fields.contains(&FormatThing::Month)
        && fields.contains(&FormatThing::Year)
    {
        let mut x = 0;
        for i in 0..fields.len() {
            match fields[i - x] {
                (FormatThing::Day | FormatThing::Month | FormatThing::Year) => {
                    fields.remove(i - x);
                    x += 1;
                }
                _ => (),
            };
        }
        x = 0;
        for i in 0..format.len() {
            match format[i - x] {
                (FormatThing::Day | FormatThing::Month | FormatThing::Year) => {
                    format.remove(i - x);
                    x += 1;
                }
                _ => (),
            };
        }
    }
    if fields.contains(&FormatThing::Seconds)
        && fields.contains(&FormatThing::Minutes)
        && fields.contains(&FormatThing::Hours)
    {
        let mut x = 0;
        for i in 0..fields.len() {
            match fields[i - x] {
                (FormatThing::Seconds | FormatThing::Minutes | FormatThing::Hours) => {
                    fields.remove(i);
                    x += 1;
                }
                _ => (),
            };
        }
        x = 0;
        for i in 0..format.len() {
            match format[i - x] {
                (FormatThing::Seconds | FormatThing::Minutes | FormatThing::Hours) => {
                    format.remove(i);
                    x += 1;
                }
                _ => (),
            };
        }
    }
    let new = fields
        .iter()
        .map(|thing| {
            let mut str = thing.to_name().to_string();
            str.push(':');
            str.push_str(thing.to_type());
            str.push_str("::new()");
            str
        })
        .collect::<Vec<String>>()
        .join(",\n            ");
    let now = fields
        .iter()
        .map(|thing| {
            let mut str = thing.to_name().to_string();
            str.push(':');
            str.push_str(thing.to_type());
            str.push_str("::now(s)");
            str
        })
        .collect::<Vec<String>>()
        .join(",\n            ");
    let field = fields
        .iter()
        .map(|thing| {
            let mut str = thing.to_name().to_string();
            str.push(':');
            str.push_str(thing.to_type());
            str
        })
        .collect::<Vec<String>>()
        .join(",\n    ");
    let z = format!(
        r#"
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
            .map(|thing| thing.to_fmt())
            .collect::<Vec<&str>>()
            .join(", ")
    );
    z.parse().unwrap()
}

#[derive(PartialEq, Clone)]
enum FormatThing {
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
            _ => (),
        }
    }
}

struct Needed {
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
