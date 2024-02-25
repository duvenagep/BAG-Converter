// use crate::bag::geometries::transformer;
use crate::bag::lib::*;
use crate::input::*;
use anyhow::{bail, Context};
use csv::Writer;
use human_bytes::human_bytes;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Cursor;
use std::sync::Arc;
use std::sync::Mutex;
use zip::read::ZipArchive;
use zip::read::ZipFile;

// #[derive(Debug, Clone)]
// struct FileInfo {
//     start: usize,
//     end: usize,
//     inflated_size: usize,
//     name: String,
// }

// fn archive_info<R: Read + std::io::Seek>(
//     archive: &mut zip::ZipArchive<R>,
// ) -> anyhow::Result<Vec<FileInfo>> {
//     let mut info = Vec::with_capacity(archive.len());
//     for i in 0..archive.len() {
//         let file = archive.by_index_raw(i).context("expected zip file")?;
//         if file.compression() != zip::CompressionMethod::DEFLATE {
//             bail!("this test is only for deflated zips");
//         }

//         info.push(FileInfo {
//             start: file.data_start() as usize,
//             end: (file.data_start() + file.compressed_size()) as usize,
//             inflated_size: file.size() as usize,
//             name: file.name().to_owned(),
//         });
//     }
//     Ok(info)
// }

pub fn libdeflate(
    zip_data: &[u8],
    obj: String,
    multi_pb: &Arc<Mutex<MultiProgress>>,
) -> Result<(), anyhow::Error> {
    // let prj = transformer();
    let reader = Cursor::new(zip_data);

    let mut archive = zip::ZipArchive::new(reader).context("unable to parse zip archive")?;
    // let info = archive_info(&mut archive)?;
    let info = archive_info(zip_data);

    for file in info {
        if should_skip_file(&file.name) {
            continue;
        }

        if file.name.contains(&obj) {
            let out_len = file.inflated_size;
            let mut inflated = vec![0u8; out_len];
            let inflated_start = 0;

            let written = libdeflater::Decompressor::new()
                .deflate_decompress(
                    &zip_data[file.start..file.end],
                    &mut inflated[inflated_start..],
                )
                .map_err(|e| anyhow::anyhow!("unable to libdeflate: {}", e))?;

            if written != file.inflated_size {
                bail!("unexpected number of bytes written");
            }

            let mut inner_zip = ZipArchive::new(Cursor::new(&inflated))?;
            let inner_info = archive_info(&inflated);
            // let inner_info = archive_info(&mut inner_zip)?;

            let bar = multi_pb
                .lock()
                .unwrap()
                .add(ProgressBar::new(inner_zip.len() as u64));
            let msg = file.name;
            bar.set_message(msg);
            bar.set_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                )
                .unwrap()
                .progress_chars("##-"),
            );
            let output_file_name = format!("output/{obj}.csv");
            let _file = File::create(&output_file_name)?;
            let cfile = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&output_file_name)?;

            let mut writer = Writer::from_writer(cfile);

            for inner_file in inner_info {
                bar.inc(1);
                let inner_out_len = inner_file.inflated_size;
                let mut inner_inflated = vec![0u8; inner_out_len];
                let inner_inflated_start = 0;

                let _inner_written = libdeflater::Decompressor::new()
                    .deflate_decompress(
                        &inflated[inner_file.start..inner_file.end],
                        &mut inner_inflated[inner_inflated_start..],
                    )
                    .map_err(|e| anyhow::anyhow!("unable to libdeflate: {}", e))?;

                let inner_string = std::str::from_utf8(&inner_inflated).unwrap();
                let bag_stand = BagStand::new(inner_string);
                match bag_stand {
                    Ok(parsed_bag_stand) => {
                        let mut csv_data: Vec<CSVStruct> = parsed_bag_stand.into();

                        csv_data
                            .into_iter()
                            .for_each(|element| element.to_csv(&mut writer));
                    }
                    Err(error) => {
                        println!("Error: {error}");
                    }
                }
                inner_inflated.clear();
            }
            inflated.clear();
            bar.finish();
        }
    }
    Ok(())
}

fn should_skip_file(filename: &str) -> bool {
    let skip_conditions = ["InOnderzoek", "Inactief", "NietBag", "GEM-WPL-RELATIE"];
    skip_conditions
        .iter()
        .any(|condition| filename.contains(condition))
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
