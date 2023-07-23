use crate::bag::lib::*;
use anyhow::{Context, bail};
use csv::Writer;
use human_bytes::human_bytes;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Cursor;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;
use zip::result::ZipResult;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use zip::read::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipError;



// #[derive(Debug, Clone, Copy)]
// struct FileInfo {
//     start: usize,
//     end: usize,
//     inflated_size: usize,
// }

// fn archive_info<R: Read + std::io::Seek>(archive: &mut zip::ZipArchive<R>) -> anyhow::Result<Vec<FileInfo>> {
//     let mut info = Vec::with_capacity(archive.len());
//     for i in 0..archive.len() {
//         // Use zip's raw function to ignore the compression method for now
//         let file = archive.by_index_raw(i).context("expected zip file")?;
//         if file.compression() != zip::CompressionMethod::DEFLATE {
//             bail!("this test is only for deflated zips");
//         }

//         info.push(FileInfo {
//             start: file.data_start() as usize,
//             end: (file.data_start() + file.compressed_size()) as usize,
//             inflated_size: file.size() as usize,
//         });
//     }
//     Ok(info)
// }













fn should_skip_file(filename: &str) -> bool {
    let skip_conditions = ["InOnderzoek", "Inactief", "NietBag", "GEM-WPL-RELATIE"];
    skip_conditions
        .iter()
        .any(|condition| filename.contains(condition))
}

pub fn read_nested_zip(
    file_path: &str,
    obj: String,
    multi_pb: Arc<Mutex<MultiProgress>>,
) -> zip::result::ZipResult<()> {
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


            let mut inner_zip_data = Vec::new();
            inner_zip_file.read_to_end(&mut inner_zip_data)?;


            let mut inner_zip = ZipArchive::new(Cursor::new(&inner_zip_data))?;

            let bar = multi_pb
                .lock()
                .unwrap()
                .add(ProgressBar::new(inner_zip.len() as u64));
            let msg = inner_zip_file.info();
            bar.set_message(msg);
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
                        let csv_data: Vec::<CSVStruct> = parsed_bag_stand.into();

                        csv_data
                            .into_iter()
                            .for_each(|element| element.to_csv(&mut writer));
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

// Parallelised with Rayon
// pub fn read_nested_zip(file_path: &str, obj: String, multi_pb: Arc<Mutex<MultiProgress>>) -> zip::result::ZipResult<()> {
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

//             let bar = multi_pb.lock().unwrap().add(ProgressBar::new(inner_zip.len() as u64));
//             let msg = inner_zip_file.info();
//             bar.set_message(msg);
//             bar.set_style(
//                 ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
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
//                 bar.inc(1);

//                 let mut inner_file = inner_zip.by_index(j)?;

//                 let mut contents = String::new();
//                 inner_file.read_to_string(&mut contents)?;
//                 let bag_stand = BagStand::new(&contents);
//                 match bag_stand {
//                     Ok(parsed_bag_stand) => {
//                         let csv_data = Vec::<CSVStruct>::from(parsed_bag_stand);

//                         // let cloned_bar = Arc::clone(&bar);
//                         let cloned_writer = Arc::clone(&writer);

//                         csv_data.into_par_iter().for_each(|element| {
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
//             bar.finish();
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
