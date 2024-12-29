use crate::format_inner;
///Module to Format Time

///Hour Struct ONLY VALUES BETWEEN 0 and 24 including 24 if it is a leap second
pub struct Hour(pub u8);

format_inner!(Hour, 2);

//-----------------------------------------------------------------------------------------------------

pub struct Minute(pub u8);

format_inner!(Minute, 2);

//-----------------------------------------------------------------------------------------------------

pub struct Second(pub u8);

format_inner!(Second, 2);

//-----------------------------------------------------------------------------------------------------

pub struct CloDate {
    pub second: Second,
    pub minute: Minute,
    pub hour: Hour,
}

//-----------------------------------------------------------------------------------------------------

pub struct Fraction(pub u16);

format_inner!(Fraction, 3);
