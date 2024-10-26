use crate::format_inner;

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

pub struct FractionSec(pub u16);

format_inner!(FractionSec);