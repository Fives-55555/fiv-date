

#[derive(Clone)]
pub struct TimeZone<T: TimeZoneTr>{
    summertime: Option<bool>
}

pub trait TimeZoneTr {
    const fn get_tf_utc(&self)->(i8, i8);
}

pub struct UTC;

impl TimeZoneTr for UTC {
    const fn get_tf_utc(&self)->(i8, i8) {
        (0,0)
    }
}



#[derive(Clone)]
enum Tzone {
    UTC,
    GMT,
    EST,
    EDT,
    CST,
    CDT,
    MST,
    MDT,
    PST,
    PDT,
    AEST,
    AEDT,
    JST,
    IST,
    CET,
    CEST,
    HAST,
    AKST,
    IDLW,
    AKDT,
    CNST,
    NFT,
    IDLE,
    NZDT,
    NZST,
    ICT,
    CAT,
    BT,
    MSK,
    EAT,
    AST,
    PT,
    EEST,
    WEDT,
    WET,
    WEST,
    EEDT,
    YST,
    YDT,
    SAST,
    EET,
    CEDT,
    WAST,
    BST,
    ADT,
}

impl Timezone {
    pub const UTC: Timezone = Timezone(Tzone::UTC);
    pub const GMT: Timezone = Timezone(Tzone::GMT);
    pub const EST: Timezone = Timezone(Tzone::EST);
    pub const EDT: Timezone = Timezone(Tzone::EDT);
    pub const CST: Timezone = Timezone(Tzone::CST);
    pub const CDT: Timezone = Timezone(Tzone::CDT);
    pub const MST: Timezone = Timezone(Tzone::MST);
    pub const MDT: Timezone = Timezone(Tzone::MDT);
    pub const PST: Timezone = Timezone(Tzone::PST);
    pub const PDT: Timezone = Timezone(Tzone::PDT);
    pub const AEST: Timezone = Timezone(Tzone::AEST);
    pub const AEDT: Timezone = Timezone(Tzone::AEDT);
    pub const JST: Timezone = Timezone(Tzone::JST);
    pub const IST: Timezone = Timezone(Tzone::IST);
    pub const CET: Timezone = Timezone(Tzone::CET);
    pub const CEST: Timezone = Timezone(Tzone::CEST);
    pub const HAST: Timezone = Timezone(Tzone::HAST);
    pub const IDLW: Timezone = Timezone(Tzone::IDLW);
    pub const AKST: Timezone = Timezone(Tzone::AKST);
    pub const AKDT: Timezone = Timezone(Tzone::AKDT);
    pub const CNST: Timezone = Timezone(Tzone::CNST);
    pub const NFT: Timezone = Timezone(Tzone::NFT);
    pub const IDLE: Timezone = Timezone(Tzone::IDLE);
    pub const NZDT: Timezone = Timezone(Tzone::NZDT);
    pub const NZST: Timezone = Timezone(Tzone::NZST);
    pub const ICT: Timezone = Timezone(Tzone::ICT);
    pub const CAT: Timezone = Timezone(Tzone::CAT);
    pub const BT: Timezone = Timezone(Tzone::BT);
    pub const MSK: Timezone = Timezone(Tzone::MSK);
    pub const EAT: Timezone = Timezone(Tzone::EAT);
    pub const AST: Timezone = Timezone(Tzone::AST);
    pub const PT: Timezone = Timezone(Tzone::PT);
    pub const EEST: Timezone = Timezone(Tzone::EEST);
    pub const WEDT: Timezone = Timezone(Tzone::WEDT);
    pub const WET: Timezone = Timezone(Tzone::WET);
    pub const WEST: Timezone = Timezone(Tzone::WEST);
    pub const EEDT: Timezone = Timezone(Tzone::EEDT);
    pub const YST: Timezone = Timezone(Tzone::YST);
    pub const YDT: Timezone = Timezone(Tzone::YDT);
    pub const SAST: Timezone = Timezone(Tzone::SAST);
    pub const EET: Timezone = Timezone(Tzone::EET);
    pub const CEDT: Timezone = Timezone(Tzone::CEDT);
    pub const WAST: Timezone = Timezone(Tzone::WAST);
    pub const BST: Timezone = Timezone(Tzone::BST);
    pub const ADT: Timezone = Timezone(Tzone::ADT);
    pub fn as_str(&self) -> &str {
        match self.0 {
            Tzone::UTC => "UTC",
            Tzone::GMT => "GMT",
            Tzone::EST => "EST",
            Tzone::EDT => "EDT",
            Tzone::CST => "CST",
            Tzone::CDT => "CDT",
            Tzone::MST => "MST",
            Tzone::MDT => "MDT",
            Tzone::PST => "PST",
            Tzone::PDT => "PDT",
            Tzone::AEST => "AEST",
            Tzone::AEDT => "AEDT",
            Tzone::JST => "JST",
            Tzone::IST => "IST",
            Tzone::CET => "CET",
            Tzone::CEST => "CEST",
            Tzone::HAST => "HAST",
            Tzone::IDLW => "IDLW",
            Tzone::AKST => "AKST",
            Tzone::AKDT => "AKDT",
            Tzone::CNST => "CNST",
            Tzone::NFT => "NFT",
            Tzone::IDLE => "IDLE",
            Tzone::NZDT => "NZDT",
            Tzone::NZST => "NZST",
            Tzone::ICT => "ICT",
            Tzone::CAT => "CAT",
            Tzone::BT => "BT",
            Tzone::MSK => "MSK",
            Tzone::EAT => "EAT",
            Tzone::AST => "AST",
            Tzone::PT => "PT",
            Tzone::EEST => "EEST",
            Tzone::WEDT => "WEDT",
            Tzone::WET => "WET",
            Tzone::WEST => "WEST",
            Tzone::EEDT => "EEDT",
            Tzone::YST => "YST",
            Tzone::YDT => "YDT",
            Tzone::SAST => "SAST",
            Tzone::EET => "EET",
            Tzone::CEDT => "CEDT",
            Tzone::WAST => "WAST",
            Tzone::BST => "BST",
            Tzone::ADT => "ADT",
        }
    }
    pub fn in_hour(&self) -> i8 {
        match self.0 {
            Tzone::NZDT => 13,
            Tzone::NZST => 12,
            Tzone::IDLE => 12,
            Tzone::NFT => 11,
            Tzone::AEDT => 11,
            Tzone::AEST => 10,
            Tzone::JST => 9,
            Tzone::CNST => 8,
            Tzone::ICT => 7,
            Tzone::BT => 3,
            Tzone::MSK => 3,
            Tzone::EAT => 3,
            Tzone::AST => 3,
            Tzone::EEST => 3,
            Tzone::EEDT => 3,
            Tzone::SAST => 2,
            Tzone::CAT => 2,
            Tzone::EET => 2,
            Tzone::CEST => 2,
            Tzone::CEDT => 2,
            Tzone::WAST => 2,
            Tzone::BST => 1,
            Tzone::IST => 1,
            Tzone::CET => 1,
            Tzone::UTC => 0,
            Tzone::WET => 0,
            Tzone::WEST => 1,
            Tzone::WEDT => 1,
            Tzone::GMT => 0,
            Tzone::ADT => -3,
            Tzone::EST => -5,
            Tzone::EDT => -4,
            Tzone::CST => -6,
            Tzone::CDT => -5,
            Tzone::MST => -7,
            Tzone::MDT => -6,
            Tzone::PST => -8,
            Tzone::PDT => -7,
            Tzone::PT => -8,
            Tzone::YST => -9,
            Tzone::YDT => -8,
            Tzone::AKST => -9,
            Tzone::AKDT => -8,
            Tzone::HAST => -10,
            Tzone::IDLW => -12,
        }
    }
}
