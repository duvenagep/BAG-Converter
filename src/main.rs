#![allow(unused)]

mod args;
mod bag;
mod helpers;
mod work_dir;

use crate::bag::lib::*;
use args::{BagObjects, EntityType, LVBAGSubCommand, NLExtractArgs};
use clap::Parser;
use helpers::zip_seek::read_nested_zip;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use work_dir::new_folder;

fn main() {
    let now = Instant::now();
    let cli = NLExtractArgs::parse();

    match cli.entity_type {
        EntityType::LVBAG(c) => match c.command {
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
                            BagObjects::OPR,
                            BagObjects::WPL,
                            BagObjects::LIG,
                            BagObjects::NUM,
                            BagObjects::STA,
                            BagObjects::VBO,
                            BagObjects::PND,
                        ];


                        let _parse = obj
                            .into_iter()
                            .for_each(|o| { read_nested_zip(&path, o.to_string()); });

                    }
                    Some(list) => {
                        let set: HashSet<_> = list.clone().into_iter().collect();

                        for l in set {
                            match l {
                                BagObjects::LIG => {
                                    println!("Parsing {:?}", l);
                                    let _r = read_nested_zip(&path, BagObjects::LIG.to_string());
                                }
                                BagObjects::NUM => {
                                    println!("Parsing {:?}", l);
                                    let _r = read_nested_zip(&path, BagObjects::NUM.to_string());
                                }
                                BagObjects::STA => {
                                    println!("Parsing {:?}", l);
                                    let _r = read_nested_zip(&path, BagObjects::STA.to_string());
                                    println!("{:?}", _r);
                                }
                                BagObjects::WPL => {
                                    println!("Parsing {:?}", l);
                                    let _r = read_nested_zip(&path, BagObjects::WPL.to_string());
                                }
                                BagObjects::PND => {
                                    println!("Parsing {:?}", l);
                                    let _r = read_nested_zip(&path, BagObjects::PND.to_string());
                                }
                                BagObjects::VBO => {
                                    println!("Parsing {:?}", l);
                                    let _r = read_nested_zip(&path, BagObjects::VBO.to_string());
                                }
                                BagObjects::OPR => {
                                    println!("Parsing {:?}", l);
                                    let _r = read_nested_zip(&path, BagObjects::OPR.to_string());
                                }
                            }
                        }
                    }
                }
            }
        },
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.4?}", elapsed);
}
