mod args;
mod bag;
mod helpers;
mod work_dir;

use args::{BagObjects, EntityType, LVBAGSubCommand, NLExtractArgs};
use clap::Parser;
use std::time::Instant;
use work_dir::new_folder;

use helpers::zip_seek::read_nested_zip;
use helpers::parsers::{parse_vbo,parse_lig,parse_opr,parse_pnd,parse_sta,parse_wpl};

fn main() {
    let now = Instant::now();
    let cli = NLExtractArgs::parse();
    println!("{:?}", cli);

    match cli.entity_type {
        EntityType::LVBAG(c) => match c.command {
            LVBAGSubCommand::Download(u) => {
                let url = u.url;
                let workdir = u.destination_folder;

                println!("URL: {}", &url);
                println!("Destination folder: {}", &workdir);

                let _temp_folder = new_folder(&workdir);
            }

            LVBAGSubCommand::Parse(parse) => match parse.bag_object {
                BagObjects::ALL => {
                    println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::ALL);
                    let file_path = &parse.file;
                    if let Err(err) = read_nested_zip(file_path) {
                        eprintln!("Error: {}", err);
                    }
                }

                BagObjects::LIG => {
                  parse_lig();
                  println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::LIG);
                }

                BagObjects::NUM => {
                  println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::NUM);
                  let file_path = &parse.file;
                  if let Err(err) = read_nested_zip(file_path) {
                      eprintln!("Error: {}", err);
                  }
                }

                BagObjects::OPR => {
                  parse_opr();
                  println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::OPR);
                }

                BagObjects::VBO => {
                  println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::VBO);
                  parse_vbo();
                }

                BagObjects::PND => {
                  parse_pnd();
                  println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::PND);
                }

                BagObjects::WPL => {
                  parse_wpl();
                  println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::WPL);
                }

                BagObjects::STA => {
                  parse_sta();
                  println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::STA);
                }
            },
            _ => todo!(),
        },
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.4?}", elapsed);
}
