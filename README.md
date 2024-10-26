# Fiv-Date
**Fiv-Date is a Low-Level ***hopefully*** high performence Way to format Dates and Stuff around it**
---

## General Infos

It is using any date from 0 AD. to u16::MAX AD and provides a dynamic way to handle dates in binary and text.(soon)
This is a Lib in Rust only for learning Purpose

## How to use

```rust
use fiv_date::{format_date_struct};

//Add a new Struct which does need a Name and how it should be formated as a ISO 8601 String
format_date_struct!(MyStruct, "DD.MM.YYYY")

fn main() {
    //From nearly any SystemTime
    let now = MyStruct::now(&SystemTime::now());
    //Also Safely from formatted Strings
    let sometimes = MyStruct::from("01.01.2001").unrwap();
    //Also with safeguards from Duration with leap secs in mind
    let cool_year = MyStruct::from_dur(LeapInMind::from_years(14));
    //Print with the special Format and optimizations
    println!("Date now: {}\nDate Somewhere: {}\nDate With LeapSecs: {}", now, sometimes, coolyear);
}
```

## Getting help

Ask me in a Electronic-Mail-Letter on Github. :)

<br>

#### License

<sup>
Licensed under <a href="LICENSE">Apache License, Version
2.0</a>
</sup>