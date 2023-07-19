use crate::bag::lib::*;
use csv::Writer;
use human_bytes::human_bytes;
use zip::result::ZipResult;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Cursor;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;

use rayon::prelude::*;

use zip::read::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipError;


use indicatif::{ProgressBar, ProgressStyle};

fn should_skip_file(filename: &str) -> bool {
    let skip_conditions = ["InOnderzoek", "Inactief", "NietBag", "GEM-WPL-RELATIE"];
    skip_conditions
        .iter()
        .any(|condition| filename.contains(condition))
}

pub fn read_nested_zip(file_path: &str, obj: String) -> zip::result::ZipResult<()> {
    let file = std::fs::File::open(file_path)?;
    let mut zip = ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut inner_zip_file = zip.by_index(i)?;

        if should_skip_file(inner_zip_file.name()) {
            continue;
        }

        if inner_zip_file.is_file()
            && inner_zip_file.name().ends_with(".zip")
            && inner_zip_file.name().contains(&obj)
        {
            println!("{}", inner_zip_file.info());

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

            let output_file_name = format!("output/{}.csv", obj);
            let _file = File::create(&output_file_name)?;
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&output_file_name)?;

            let mut writer = Writer::from_writer(file);

            for j in 0..inner_zip.len() {
                bar.inc(1);

                let mut inner_file = inner_zip.by_index(j)?;

                let mut contents = String::new();
                inner_file.read_to_string(&mut contents)?;
                let bag_stand = BagStand::new(&contents);
                match bag_stand {
                    Ok(parsed_bag_stand) => {
                        let csv_data = Vec::<CSVStruct>::from(parsed_bag_stand);

                        csv_data.into_iter().for_each(|element| {
                            match element {
                                CSVStruct::VBO(data) => {
                                    writer.serialize(data).unwrap();
                                    writer.flush().unwrap();
                                },
                                CSVStruct::OPR(data) => {
                                    writer.serialize(data).unwrap();
                                    writer.flush().unwrap();
                                },
                                CSVStruct::WPL(data) => {
                                    writer.serialize(data).unwrap();
                                    writer.flush().unwrap();
                                },
                                CSVStruct::LIG(data) => {
                                    writer.serialize(data).unwrap();
                                    writer.flush().unwrap();
                                },
                                CSVStruct::PND(data) => {
                                    writer.serialize(data).unwrap();
                                    writer.flush().unwrap();
                                },
                                CSVStruct::NUM(data) => {
                                    writer.serialize(data).unwrap();
                                    writer.flush().unwrap();
                                },
                                CSVStruct::STA(data) => {
                                    writer.serialize(data).unwrap();
                                    writer.flush().unwrap();
                                }
                            };
                        });

                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            }
            bar.finish();
        }
    }
    Ok(())
}

trait Info {
    fn info(&self) -> String;
}

impl Info for ZipFile<'_> {
    fn info(&self) -> String {
        let info = format!(
            "File Name: {} Size: {} Last Modified: {} Compression: {}",
            &self.name(),
            human_bytes(self.size() as f64),
            &self.last_modified().year(),
            &self.compression()
        );

        info
    }
}





// pub fn read_nested_zip(file_path: &str, obj: String) -> ZipResult<()> {
//     let file = std::fs::File::open(file_path)?;
//     let mut zip = ZipArchive::new(file)?;

//     for i in 0..zip.len() {
//         let mut inner_zip_file = zip.by_index(i)?;

//         if should_skip_file(inner_zip_file.name()) {
//             continue;
//         }

//         if inner_zip_file.is_file()
//             && inner_zip_file.name().ends_with(".zip")
//             && inner_zip_file.name().contains(&obj)
//         {
//             println!("{}", inner_zip_file.info());

//             let mut inner_zip_data = Vec::new();
//             inner_zip_file.read_to_end(&mut inner_zip_data)?;

//             let mut inner_zip = ZipArchive::new(Cursor::new(inner_zip_data))?;

//             let bar = Arc::new(Mutex::new(ProgressBar::new(inner_zip.len() as u64)));
//             bar.lock().unwrap().set_style(
//                 ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
//                     .unwrap()
//                     .progress_chars("##-"),
//             );

//             let output_file_name = format!("output/{}.csv", obj);
//             let _file = File::create(&output_file_name)?;
//             let file = OpenOptions::new()
//                 .write(true)
//                 .append(true)
//                 .open(&output_file_name)?;

//             let writer = Arc::new(Mutex::new(Writer::from_writer(file)));
//             let range = 0..inner_zip.len();

           
//             for j in 0..inner_zip.len() {
//                 bar.lock().unwrap().inc(1);

//                 let mut inner_file = inner_zip.by_index(j)?;

//                 let mut contents = String::new();
//                 inner_file.read_to_string(&mut contents)?;
//                 let bag_stand = BagStand::new(&contents);
//                 match bag_stand {
//                     Ok(parsed_bag_stand) => {
//                         let csv_data = Vec::<CSVStruct>::from(parsed_bag_stand);

//                         let cloned_bar = Arc::clone(&bar);
//                         let cloned_writer = Arc::clone(&writer);

//                         csv_data.into_iter().for_each(|element| {
//                             match element {
//                                 CSVStruct::VBO(data) => {
//                                     cloned_writer.lock().unwrap().serialize(data).unwrap();
//                                     cloned_writer.lock().unwrap().flush().unwrap();
//                                 }
//                                 CSVStruct::OPR(data) => {
//                                     cloned_writer.lock().unwrap().serialize(data).unwrap();
//                                     cloned_writer.lock().unwrap().flush().unwrap();
//                                 }
//                                 CSVStruct::WPL(data) => {
//                                     cloned_writer.lock().unwrap().serialize(data).unwrap();
//                                     cloned_writer.lock().unwrap().flush().unwrap();
//                                 }
//                                 CSVStruct::LIG(data) => {
//                                     cloned_writer.lock().unwrap().serialize(data).unwrap();
//                                     cloned_writer.lock().unwrap().flush().unwrap();
//                                 }
//                                 CSVStruct::PND(data) => {
//                                     cloned_writer.lock().unwrap().serialize(data).unwrap();
//                                     cloned_writer.lock().unwrap().flush().unwrap();
//                                 }
//                                 CSVStruct::NUM(data) => {
//                                     cloned_writer.lock().unwrap().flush().unwrap();
//                                     cloned_writer.lock().unwrap().serialize(data).unwrap();
//                                 }
//                                 CSVStruct::STA(data) => {
//                                     cloned_writer.lock().unwrap().serialize(data).unwrap();
//                                     cloned_writer.lock().unwrap().flush().unwrap();
//                                 }
//                             };
//                         });
//                     }
//                     Err(error) => {
//                         println!("Error: {}", error);
//                     }
//                 }
//             }
//             bar.lock().unwrap().finish();
//         }
//     }
//     Ok(())
// }

// trait Info {
//     fn info(&self) -> String;
// }

// impl Info for ZipFile<'_> {
//     fn info(&self) -> String {
//         let info = format!(
//             "File Name: {} Size: {} Last Modified: {} Compression: {}",
//             &self.name(),
//             human_bytes(self.size() as f64),
//             &self.last_modified().year(),
//             &self.compression()
//         );

//         info
//     }
// }
