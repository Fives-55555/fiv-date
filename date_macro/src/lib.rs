use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn format_date_struct(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let name = match tokens.next() {
        Some(TokenTree::Ident(name))=>name,
        _=>panic!("Missing Name")
    };
    let _ = tokens.next();
    let mut format_str = match tokens.next() {
        Some(TokenTree::Literal(l))=>l.to_string(),
        _=>panic!("Missing Format String")
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
            '"'=>break,
            'Y'=>{
                for x in 1..4 {
                    char=chars.next();
                    if !(char.is_some() && char.unwrap() == 'Y') {
                        print.push_str("Y".repeat(x).as_str());
                        i+=x+1;
                        continue 'outer;
                    }
                }
                char=chars.next();
                format.push(Things::Year);
                print.push_str("{:04}");
                i+=4;
            },
            'M'=>{
                char=chars.next();
                if char.is_some() && char.unwrap() =='M' {
                    print.push_str("{:02}");
                    format.push(Things::Month);
                    i+=2;
                }else {
                    print.push('M');
                    i+=1;
                }
            }
            'D'=>{
                char=chars.next();
                if char.is_some() && char.unwrap() == 'D' {
                    char=chars.next();
                    if char.is_some() && char.unwrap() == 'D' {
                        char=chars.next();
                        format.push(Things::Days);
                        print.push_str("{:03}");
                        i+=1;
                    }else{
                        format.push(Things::Day);
                        print.push_str("{:02}");
                    }
                    i+=1;
                }else {
                    format.push(Things::Weeknum);
                    print.push_str("{}");
                }
                i+=1;
            },
            '\\'=>{
                char=chars.next();
                if char.is_some() && char.unwrap() == '\\' {
                    char=chars.next();
                    if char.is_some() {
                        print.push(char.unwrap());
                        char=chars.next();
                        i+=3;
                    }else {
                        panic!("Empty Escape")
                    }
                }
            },
            _=>{
                print.push(char.unwrap());
                i+=1;
                char=chars.next();
            }
        }
    };
    let mut fields: Vec<Things> = Vec::new();
    for thing in format.iter() {
        match thing {
            Things::Weeknum => {
                if !fields.contains(&Things::Weekday) {
                    fields.push(Things::Weekday);
                }
            },
            _=>{
                if !fields.contains(thing) {
                    fields.push(thing.clone());
                }
            }
        }
    }
    let new = fields.iter().map(|thing|{
        let mut str = thing.to_name().to_string();
        str.push(':');
        str.push_str(thing.to_type());
        str.push_str("::new()");
        str
    }).collect::<Vec<String>>().join(",\n            ");
    let now = fields.iter().map(|thing|{
        let mut str = thing.to_name().to_string();
        str.push(':');
        str.push_str(thing.to_type());
        str.push_str("::now(s)");
        str
    }).collect::<Vec<String>>().join(",\n            ");
    let field = fields.iter().map(|thing|{
        let mut str = thing.to_name().to_string();
        str.push(':');
        str.push_str(thing.to_type());
        str
    }).collect::<Vec<String>>().join(",\n    ");
    let z = format!(r#"
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
}}"#, field, new, now, format.iter().map(|thing|thing.to_fmt()).collect::<Vec<&str>>().join(", "));
    z.parse().unwrap()
}

#[derive(PartialEq, Clone)]
enum Things {
    Year,
    Weekday,
    Weeknum,//Num of Weekday
    Day,//of Month
    Days,//of Year
    Month,
    Timezone,
}

impl Things {
    fn to_fmt(&self)->&str {
        match self {
            Things::Weeknum=>"self.weekday.to_num()",
            Things::Day=>"self.day",
            Things::Days=>"self.days",
            Things::Weekday=>"self.weekday",
            Things::Month=>"self.month",
            Things::Year=>"self.year",
            Things::Timezone=>"self.timezone"
        }
    }
    fn to_name(&self)->&str {
        match self {
            Things::Day=>"day",
            Things::Days=>"days",
            Things::Weekday=>"weekday",
            Things::Month=>"month",
            Things::Year=>"year",
            Things::Timezone=>"timezone",
            _=>panic!("Impossible")
        }
    }
    fn to_type(&self)->&str {
        match self {
            Things::Day=>"Day",
            Things::Days=>"Days",
            Things::Weekday=>"Weekday",
            Things::Month=>"Month",
            Things::Year=>"Year",
            Things::Timezone=>"Timezone",
            _=>panic!("Impossible")
        }
    }
}