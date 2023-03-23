mod common;
mod n3ds;

use std::{fs, format};
use std::io::Error;
use std::path::{Path, PathBuf};
use std::fs::File;
use crate::common::*;
use crate::n3ds::*;


fn clean_file_name(file_name: &String) -> String {
    file_name.replace(&['\\', '/', ':', '*', '\"', '<', '>', '|'][..], "")
}

fn copy_file(orig_name: &String, clean_name: String, input_path: &Path, output_path: &Path, file_type: &FileType) -> Result<u64, Error> {
    println!("Copying {}", clean_name);
    let full_input_path: PathBuf = input_path.join(orig_name);
    let copy_result: Result<u64, Error> = match file_type {
        FileType::NCSD => fs::copy(full_input_path, output_path.join(format!("{}.{}", clean_name, "3ds"))),
        FileType::NCCH =>  fs::copy(full_input_path, output_path.join(format!("{}.standard.{}", clean_name, "cia"))),
    };
    println!("Done\n");
    return copy_result;
}

fn main() -> Result<(), Error> {
    const REGION_PRIORITY: [Region; 14] = [
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
    const DEBUG: bool = true;

    let n3ds_xml_file = File::open("3dsreleases.xml").unwrap();
    let n3ds_games = read_3ds_metadata_xml(n3ds_xml_file);
    let input_path: &Path = Path::new("./input");
    let output_path: &Path = Path::new("./output");
    let input_dir_contents = fs::read_dir(input_path).unwrap();
    let mut input_games: Vec<N3DSGame> = Vec::new();

    for games in input_dir_contents {
        let game_name: String = games.unwrap().file_name().into_string().unwrap();
        let file_type: String = game_name[game_name.len() - 3..].to_string();
        let game_file = File::open(input_path.join(&game_name))?;


        if file_type == "3ds" || file_type == "cia" {
            input_games.push(N3DSGame{orig_name: game_name, header_info: read_header_info_3ds(game_file).unwrap()});
        }
    }

    for game in input_games {
        let mut matching_games: Vec<&N3DSGameMetadata> = Vec::new();

        for game_details in &n3ds_games {
            if game_details.title_id == game.header_info.title_id {
                matching_games.push(game_details);
            }
        }

        if matching_games.len() == 1 {
            if !DEBUG {
                copy_file(&game.orig_name, clean_file_name(&matching_games[0].name), input_path, output_path, &game.header_info.file_type)?;
            } else {
                println!("Found title {} for {}", clean_file_name(&matching_games[0].name), game.orig_name);
            }
        } else {
            matching_games.sort_by_key(|g| g.region);
            let mut found_games: Vec<&String> = Vec::new();
            for matched_game in matching_games {
                for region in &REGION_PRIORITY {
                    if &matched_game.region == region && !found_games.contains(&&matched_game.title_id) {
                        found_games.push(&matched_game.title_id);
                        if !DEBUG {
                            copy_file(&game.orig_name, clean_file_name(&matched_game.name), input_path, output_path, &game.header_info.file_type)?;
                        } else {
                            println!("Found title {} for {}", clean_file_name(&matched_game.name), game.orig_name);
                        }
                        break;
                    }
                }
            }
        }
    }

    return Ok(());
}
