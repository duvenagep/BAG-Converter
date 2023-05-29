use csv::Writer;
use std::io::Cursor;
use std::io::Read;
use zip::read::ZipArchive;

use crate::helpers::parsers::parse_num;

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

    let mut _wtr = Writer::from_path("num.csv").unwrap();

    for i in 0..zip.len() {
        let mut inner_zip_file = zip.by_index(i)?;

        if should_skip_file(inner_zip_file.name()) {
            continue;
        }

        if inner_zip_file.is_file() && inner_zip_file.name().ends_with(".zip") {
            println!("FILE NAME: {:?}", inner_zip_file.name());

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

                if inner_file.name().contains("NUM") {
                    // println!(
                    //     "File inside nested zip: {}",
                    //     inner_file.name()
                    // );
                    let mut contents = String::new();
                    inner_file.read_to_string(&mut contents)?;
                    parse_num(&contents);
                }
            }
            bar.finish();
        }
    }

    Ok(())
}
