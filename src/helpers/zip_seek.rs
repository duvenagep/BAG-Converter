use crate::bag::lib;
use human_bytes::human_bytes;
use std::io::Cursor;
use std::io::Read;
use zip::read::ZipArchive;

use indicatif::{ProgressBar, ProgressStyle};

fn should_skip_file(filename: &str) -> bool {
    let skip_conditions = ["InOnderzoek", "Inactief", "NietBag", "GEM-WPL-RELATIE"];
    skip_conditions
        .iter()
        .any(|condition| filename.contains(condition))
}

pub fn read_nested_zip(file_path: &str) -> zip::result::ZipResult<()> {
    let file = std::fs::File::open(file_path)?;
    let mut zip = ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut inner_zip_file = zip.by_index(i)?;

        if should_skip_file(inner_zip_file.name()) {
            continue;
        }

        if inner_zip_file.is_file()
            && inner_zip_file.name().ends_with(".zip")
            && inner_zip_file.name().contains("WPL")
        {
            println!(
                "FILE NAME: {} Size: {} Last Modified: {} Compression: {}",
                &inner_zip_file.name(),
                human_bytes(inner_zip_file.size() as f64),
                inner_zip_file.last_modified().year(),
                inner_zip_file.compression()
            );

            let mut inner_zip_data = Vec::new();
            inner_zip_file.read_to_end(&mut inner_zip_data)?;

            let mut inner_zip = ZipArchive::new(Cursor::new(inner_zip_data))?;

            let bar = ProgressBar::new(inner_zip.len() as u64);
            bar.set_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                )
                .unwrap()
                .progress_chars("##-"),
            );

            for j in 0..inner_zip.len() {
                bar.inc(1);

                let mut inner_file = inner_zip.by_index(j)?;

                if j == 0 {
                    let mut contents = String::new();
                    inner_file.read_to_string(&mut contents)?;
                    let bag_stand = lib::BagStand::new(&contents);
                    match bag_stand {
                        Ok(parsed_bag_stand) => {
                            let csv_data = Vec::<lib::CSVStruct>::from(parsed_bag_stand);
                            println!("{:#?}", &csv_data);
                        }
                        Err(error) => {
                            println!("Error: {}", error);
                        }
                    }
                }

                bar.finish();
            }
        }
    }
    Ok(())
}

// pub fn read_outer_zip(file_path: &str) -> zip::result::ZipResult<Vec<u8>> {
//     let file = std::fs::File::open(file_path)?;
//     let mut zip = zip::ZipArchive::new(file)?;

//     let mut inner_zip_data = Vec::new();

//     for i in 0..zip.len() {
//         let mut inner_zip_file = zip.by_index(i)?;

//         if should_skip_file(inner_zip_file.name()) {
//             continue;
//         }

//         if inner_zip_file.is_file() && inner_zip_file.name().ends_with(".zip") {
//             println!(
//                 "FILE NAME: {:?} and size {:?}",
//                 inner_zip_file.name(),
//                 inner_zip_file.size()
//             );
//             inner_zip_file.read_to_end(&mut inner_zip_data)?;
//         }
//     }

//     Ok(inner_zip_data)
// }

// pub fn read_inner_files(inner_zip_data: &[u8]) -> zip::result::ZipResult<()> {

//     let mut inner_zip = zip::ZipArchive::new(Cursor::new(inner_zip_data))?;

//     let bar = ProgressBar::new(inner_zip.len() as u64);
//     bar.set_style(
//         ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
//             .unwrap()
//             .progress_chars("##-"),
//     );

//     for j in 0..inner_zip.len() {

//         bar.inc(1);

//         let mut inner_file = inner_zip.by_index(j)?;

//         if inner_file.name().contains("NUM") && j == 0 {
//             let mut contents = String::new();
//             inner_file.read_to_string(&mut contents)?;

//             let bag_stand: Result<num::BagStand, quick_xml::de::DeError> =
//                 <num::BagStand as num::Parse>::parse(&contents);
//             match bag_stand {
//                 Ok(parsed_bag_stand) => {
//                     println!("{:?}", parsed_bag_stand);
//                 }
//                 Err(error) => {
//                     println!("Error: {}", error);
//                 }
//             }
//         }

//         bar.finish();
//     }

//     Ok(())
// }
