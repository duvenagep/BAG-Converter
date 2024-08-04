// #![allow(unused)]
// #![warn(clippy::all, clippy::pedantic, clippy::restriction)]
// #![warn(missing_docs)]

mod args;
mod bag;
mod error;
mod helpers;
mod input;
mod work_dir;

use args::{BagObjects, LVBAGSubCommand, NLExtractArgs};
use clap::Parser;
use error::BagResult;
use helpers::deserializers::set_transformer;
use helpers::zip_seek::libdeflate;
use indicatif::MultiProgress;
use memmap2::Mmap;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use work_dir::new_folder;

fn main() -> BagResult<()> {
    let now = Instant::now();
    let cli = NLExtractArgs::parse();
    println!("{:?}", cli);

    match cli.entity_type {
        LVBAGSubCommand::Download(u) => {
            let url = u.url;
            let workdir = u.destination_folder;

            println!("URL: {}", &url);
            println!("Destination folder: {}", &workdir);

            let _temp_folder = new_folder(&workdir);
        }

        LVBAGSubCommand::Parse(parse) => {
            let _output_folder = new_folder("output");
            if let Some(proj) = &parse.projection {
                set_transformer("EPSG:28992", proj.as_str());
            }

            let path = &parse.file;
            let file = File::open(path).expect("failed to open the file");

            let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };
            println!("{:?}", &parse);

            match parse.bag_object {
                None => {
                    println!("Parsing all");
                    let obj = vec![
                        BagObjects::Wpl,
                        BagObjects::Lig,
                        BagObjects::Opr,
                        BagObjects::Num,
                        BagObjects::Sta,
                        BagObjects::Vbo,
                        BagObjects::Pnd,
                    ];

                    let multi_pb = Arc::new(Mutex::new(MultiProgress::new()));

                    obj.into_par_iter().for_each(|o| {
                        let _ = libdeflate(&mmap[..], o.to_string(), &multi_pb);
                    });
                }

                Some(list) => {
                    let multi_pb = Arc::new(Mutex::new(MultiProgress::new()));
                    let set: HashSet<_> = list.into_iter().collect();

                    set.into_par_iter().for_each(|o| match o {
                        BagObjects::Lig
                        | BagObjects::Num
                        | BagObjects::Opr
                        | BagObjects::Pnd
                        | BagObjects::Sta
                        | BagObjects::Vbo
                        | BagObjects::Wpl => {
                            println!("Parsing {:?}", &o);
                            let _ = libdeflate(&mmap[..], o.to_string(), &multi_pb);
                        }
                    });
                }
            }
        }
        LVBAGSubCommand::Info => {
            todo!()
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
    Ok(())
}
