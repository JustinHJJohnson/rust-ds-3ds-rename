use serde_derive::{Deserialize, Serialize};
use std::{str, format};
use strum_macros::EnumString;

#[derive(PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Debug, Copy, Clone)]
pub enum Region {
    EUR,
    USA,
    JPN,
    TWN,
    ITA,
    SPA,
    FRA,
    GER,
    KOR,
    CHN,
    UKV,
    NLD,
    WLD,
    RUS
}

#[derive(Debug, EnumString)]
pub enum FileType {
    NCCH,
    NCSD
}

pub fn format_u8_to_hex_string(data: Vec<u8>) -> String {
    let mut output_vec: Vec<String> = Vec::new();
    for x in data {
        output_vec.push(format!("{:02X?}", x));
    }
    
    return output_vec.join("");
}