use crate::common::*;
use std::io::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::str::{self, FromStr};
use std::io::{Error, ErrorKind, BufReader};
use xml::reader::{EventReader, XmlEvent};
use std::fs::File;

/*#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct N3DSGameMetadata {
    pub id: String,
	pub name: String,
	pub publisher: String,
	pub region: Region,
	pub languages: String,
	pub group: String,
	pub imagesize: String,
	pub serial: String,
	pub titleid: String,
	pub imgcrc: String,
	pub filename: String,
	pub releasename: String,
	pub trimmedsize: String,
	pub firmware: String,
    pub typeNum: String,
	pub card: String
}*/

#[derive(Deserialize, Serialize, Debug)]
pub struct N3DSGameMetadata {
    pub name: String,
	pub region: Region,
	pub title_id: String,
}

pub struct N3DSGame {
    pub orig_name: String,
    pub header_info: N3DSHeaderInfo
}

pub struct N3DSHeaderInfo {
    pub title_id: String,
    pub file_type: FileType
}

pub fn read_3ds_metadata_xml(file: File) -> Vec<N3DSGameMetadata> {
    let mut n3ds_games: Vec<N3DSGameMetadata> = Vec::new();
    let reader = EventReader::new(BufReader::new(file));

    let mut current_metadata: [String; 3] = Default::default();
    let mut metadata_index = 0; 
    let mut get_next_text = false;

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                let element = name.local_name;
                if element == "name" || element == "region" || element == "titleid" {
                    get_next_text = true;
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "release" {
                    n3ds_games.push(N3DSGameMetadata {
                        name: current_metadata[0].to_owned(),
                        region: current_metadata[1].parse().unwrap(),
                        title_id: current_metadata[2].to_owned()
                    });
                    
                    current_metadata = Default::default();
                    metadata_index = 0;
                    get_next_text = false;
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                if get_next_text {
                    current_metadata[metadata_index] = text;
                    metadata_index += 1;
                    get_next_text = false
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    return n3ds_games;
}

pub fn read_header_info_3ds(mut file: File) -> Result<N3DSHeaderInfo, Error> {
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
        Ok(N3DSHeaderInfo{ title_id: ncsd_title_id, file_type: ncsd_file_type })
    } else {
        let ncch_file_type_string: &str = str::from_utf8(&buffer[START_FILE_TYPE_NCCH..END_FILE_TYPE_NCCH]).unwrap();
        
        if ncch_file_type_string == "NCCH" {
            let ncch_file_type: FileType = FileType::from_str(ncch_file_type_string).unwrap();
            let ncch_title_id: String = format_u8_to_hex_string(
                buffer[START_TITLE_ID_NCCH..END_TITLE_ID_NCCH].to_owned().into_iter().rev().collect::<Vec<u8>>()
            );
            Ok(N3DSHeaderInfo{ title_id: ncch_title_id, file_type: ncch_file_type })
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid file"))
        }
    }
}