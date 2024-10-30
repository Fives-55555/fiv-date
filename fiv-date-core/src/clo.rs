use crate::format_inner;
///Module to Format Time

///Hour Struct ONLY VALUES BETWEEN 0 and 24 including 24 if it is a leap second
pub struct Hour(pub u8);

format_inner!(Hour);

//-----------------------------------------------------------------------------------------------------

pub struct Minute(pub u8);

format_inner!(Minute);

//-----------------------------------------------------------------------------------------------------

pub struct Second(pub u8);

format_inner!(Second);

//-----------------------------------------------------------------------------------------------------

pub struct CloDate {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
}

//-----------------------------------------------------------------------------------------------------

pub struct Fraction(pub u16);

format_inner!(Fraction);
