mod args;
mod bag;
mod helpers;
mod work_dir;

use args::{BagObjects, EntityType, LVBAGSubCommand, NLExtractArgs};
use clap::Parser;
use helpers::zip_seek::{read_nested_zip};
use std::collections::HashSet;
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
                println!("{:?}", &parse);
                let path = &parse.file;

                match parse.bag_object {
                    None => println!("Parsing all"),
                    Some(list) => {
                        let set: HashSet<_> = list.clone().into_iter().collect();

                        for l in set {
                            match l {
                                BagObjects::LIG => {
                                    println!("Parsing {:?}", l);
                                    let r = read_nested_zip(&path);
                                }
                                BagObjects::NUM => {
                                    println!("Parsing {:?}", l);
                                    let r = read_nested_zip(&path);
                                }
                                BagObjects::STA => {
                                  println!("Parsing {:?}", l);
                                  let r = read_nested_zip(&path);
                              },
                                BagObjects::WPL => {
                                    println!("Parsing {:?}", l);
                                    let r = read_nested_zip(&path);
                                }
                                BagObjects::PND => {
                                  println!("Parsing {:?}", l);
                                  let r = read_nested_zip(&path);
                              },
                                BagObjects::VBO => {
                                  println!("Parsing {:?}", l);
                                  let r = read_nested_zip(&path);
                              },
                                BagObjects::OPR => {
                                  println!("Parsing {:?}", l);
                                  let r = read_nested_zip(&path);
                              },
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
