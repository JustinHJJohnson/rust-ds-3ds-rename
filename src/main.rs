use std::io::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::{fmt, fs, str, format};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::fs::File;
use strum_macros::EnumString;
use std::str::FromStr;


#[derive(PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Debug, Copy, Clone)]
enum Region {
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

/*impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}*/

#[derive(Debug, EnumString)]
enum FileType {
    NCCH,
    NCSD
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct Game {
    id: String,
	name: String,
	publisher: String,
	region: Region,
	languages: String,
	group: String,
	imagesize: String,
	serial: String,
	titleid: String,
	imgcrc: String,
	filename: String,
	releasename: String,
	trimmedsize: String,
	firmware: String,
    typeNum: String,
	card: String
}

struct HeaderInfo {
    title_id: String,
    file_type: FileType
}

fn clean_file_name(file_name: &String) -> String {
    file_name.replace(&['\\', '/', ':', '*', '\"', '<', '>', '|'][..], "")
}

fn copy_file(orig_name: &String, clean_name: String, input_path: &Path, output_path: &Path, file_type: &String) -> Result<u64, std::io::Error> {
    println!("Copying {}", clean_name);
    let full_input_path: PathBuf = input_path.join(orig_name);
    return match file_type.as_str() {
        "3ds" => fs::copy(full_input_path, output_path.join(format!("{}.trim.{}", clean_name, file_type))),
        "cia" =>  fs::copy(full_input_path, output_path.join(format!("{}.standard.{}", clean_name, file_type))),
        _ => Result::Err(Error::new(ErrorKind::InvalidData, "File is not a 3ds game file"))
    }
}

fn format_u8_to_hex_string(data: Vec<u8>) -> String {
    let mut output_vec: Vec<String> = Vec::new();
    for x in data {
        output_vec.push(format!("{:02X?}", x));
    }
    
    return output_vec.join("");
}

fn read_header_info(mut file: File) -> Result<HeaderInfo, Error> {
    const START_FILE_TYPE_NCSD: usize = 0x100;
    const END_FILE_TYPE_NCSD: usize = 0x104;
    const START_TITLE_ID_NCSD: usize = 0x108;
    const END_TITLE_ID_NCSD: usize = 0x110;
    const START_FILE_TYPE_NCCH: usize = 0x3a40;
    const END_FILE_TYPE_NCCH: usize = 0x3a44;
    const START_TITLE_ID_NCCH: usize = 0x3a48;
    const END_TITLE_ID_NCCH: usize = 0x3a50;

    let mut buffer = [0; END_TITLE_ID_NCCH];
    file.read_exact(&mut buffer)?;
    let ncsd_file_type_string: &str = str::from_utf8(&buffer[START_FILE_TYPE_NCSD..END_FILE_TYPE_NCSD]).unwrap();

    if ncsd_file_type_string == "NCSD" {
        let ncsd_file_type: FileType = FileType::from_str(ncsd_file_type_string).unwrap();
        let ncsd_title_id: String = format_u8_to_hex_string(
            buffer[START_TITLE_ID_NCSD..END_TITLE_ID_NCSD].to_owned().into_iter().rev().collect::<Vec<u8>>()
        );
        Ok(HeaderInfo{ title_id: ncsd_title_id, file_type: ncsd_file_type })
    } else {
        let ncch_file_type_string: &str = str::from_utf8(&buffer[START_FILE_TYPE_NCCH..END_FILE_TYPE_NCCH]).unwrap();
        
        if ncch_file_type_string == "NCCH" {
            let ncch_file_type: FileType = FileType::from_str(ncch_file_type_string).unwrap();
            let ncch_title_id: String = format_u8_to_hex_string(
                buffer[START_TITLE_ID_NCCH..END_TITLE_ID_NCCH].to_owned().into_iter().rev().collect::<Vec<u8>>()
            );
            Ok(HeaderInfo{ title_id: ncch_title_id, file_type: ncch_file_type })
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid file"))
        }
    }
}

fn main() -> Result<(), Error> {
    let region_priority: [Region; 14] = [
        Region::EUR,
        Region::USA,
        Region::JPN,
        Region::TWN,
        Region::ITA,
        Region::SPA,
        Region::FRA,
        Region::GER,
        Region::KOR,
        Region::CHN,
        Region::UKV,
        Region::NLD,
        Region::WLD,
        Region::RUS
    ];

    let json_str: String = fs::read_to_string("3ds_game_list.json")?;
    let input_path: &Path = Path::new("./input");
    let output_path: &Path = Path::new("./output");
    let game_list: Vec<Game> = serde_json::from_str(&json_str).unwrap();
    let input_dir_contents = fs::read_dir(input_path).unwrap();
    let mut input_games: Vec<String> = Vec::new();

    let test_game = input_path.join("00040000000F5500 Devil Summoner Soul Hackers (CTR-P-AHQP) (E) (v0.0.0).standard.cia");
    let file = File::open(test_game)?;

    let header_info = read_header_info(file).unwrap();
    
    println!("File type is: {:?}\nTitleID is: {}", header_info.file_type, header_info.title_id);

    return Ok(());

    for games in input_dir_contents {
        let game_name: String = games.unwrap().file_name().into_string().unwrap();
        let file_type: String = game_name[game_name.len() - 3..].to_string();

        if file_type == "3ds" || file_type == "cia" {
            input_games.push(game_name);
        }
    }

    for game in input_games {
        let title_id: String = game[0..16].to_string();
        let file_type: &String = &String::from(game[game.len() - 3..].to_string());
        let mut matching_games: Vec<&Game> = Vec::new();

        for game_details in &game_list {
            if game_details.titleid == title_id {
                matching_games.push(game_details);
            }
        }

        if matching_games.len() == 1 {
            copy_file(&game, clean_file_name(&matching_games[0].name), input_path, output_path, file_type)?;
        } else {
            matching_games.sort_by_key(|g| g.region);
            let mut found_games: Vec<&String> = Vec::new();
            for matched_game in matching_games {
                for region in &region_priority {
                    if &matched_game.region == region && !found_games.contains(&&matched_game.titleid) {
                        found_games.push(&matched_game.titleid);
                        copy_file(&game, clean_file_name(&matched_game.name), input_path, output_path, file_type)?;
                        break;
                    }
                }
            }
        }
    }

    return Ok(());
}
