use serde_derive::{Deserialize, Serialize};
use std::{fmt, fs};
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

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

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
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

fn clean_file_name(file_name: &String) -> String {
    file_name.replace(&['\\', '/', ':', '*', '\"', '<', '>', '|'][..], "")
}

fn copy_file(orig_name: &String, clean_name: String, input_path: &Path, output_path: &Path, file_type: &String) -> Result<u64, std::io::Error> {
    println!("Copying {}", clean_name);
    let full_input_path: PathBuf = input_path.join(orig_name);
    return match file_type.as_str() {
        "3ds" => fs::copy(full_input_path, output_path.join(format!("{}.trim.{}", clean_name, file_type))),
        "cia" =>  fs::copy(full_input_path, output_path.join(format!("{}.standard.{}", clean_name, file_type))),
        _ => Result::Err(Error::new(ErrorKind::InvalidData, "File is not a 3ds file"))
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
