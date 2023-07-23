// #![allow(unused)]

mod args;
mod bag;
mod helpers;
mod work_dir;
use args::{BagObjects, EntityType, LVBAGSubCommand, NLExtractArgs};
use clap::Parser;
use helpers::zip_seek::read_nested_zip;
use indicatif::MultiProgress;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use std::time::Instant;
use work_dir::new_folder;

fn main() {
    let now = Instant::now();
    let cli = NLExtractArgs::parse();

    match cli.entity_type {
        EntityType::Lvbag(c) => match c.command {
            LVBAGSubCommand::Download(u) => {
                let url = u.url;
                let workdir = u.destination_folder;

                println!("URL: {}", &url);
                println!("Destination folder: {}", &workdir);

                let _temp_folder = new_folder(&workdir);
            }

            LVBAGSubCommand::Parse(parse) => {
                let _output_folder = new_folder("output");

                println!("{:?}", &parse);
                let path = &parse.file;

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
                            let _ = read_nested_zip(path, o.to_string(), &multi_pb);
                        });
                    }

                    Some(list) => {
                        let multi_pb = Arc::new(Mutex::new(MultiProgress::new()));
                        let set: HashSet<_> = list.clone().into_iter().collect();

                        set.into_par_iter().for_each(|o| match o {
                            BagObjects::Lig
                            | BagObjects::Num
                            | BagObjects::Opr
                            | BagObjects::Pnd
                            | BagObjects::Sta
                            | BagObjects::Vbo
                            | BagObjects::Wpl => {
                                println!("Parsing {:?}", &o);
                                let _ = read_nested_zip(path, o.to_string(), &multi_pb);
                            }
                        });
                    }
                }
            }
        },
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.4?}");
}
