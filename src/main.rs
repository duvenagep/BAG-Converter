mod bag;
mod helpers;
mod args;
mod work_dir;

use std::fs;
use std::time::Instant;
use std::io::{Read, Cursor};
use clap::Parser;
use quick_xml::de::from_str;
use serde::Serialize;
use zip::{ZipArchive, result::ZipError};
use args::{NLExtractArgs, EntityType, LVBAGSubCommand, BagObjects};
use work_dir::new_folder;
use bag::{sta,opr,num,vbo,lig,wpl,pnd};



fn main(){

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

            LVBAGSubCommand::Parse(p) => {
                let _i = p.info;
                match p.bag_object {

                    BagObjects::ALL => {

                      println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::ALL);
                    }

                    BagObjects::LIG => {
                      println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::LIG);
                    }

                    BagObjects::NUM => {
                      println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::NUM);
                      if let Err(e) = run() {
                        eprintln!("Error: {:?}", e);
                      }
                    }

                    BagObjects::OPR => {
                      println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::OPR);
                    }

                    BagObjects::VBO => {
                      println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::VBO);
                    }

                    BagObjects::PND => {
                      println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::PND);
                    }

                    BagObjects::WPL => {
                      println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::WPL);
                    }

                    BagObjects::STA => {
                      println!("Parse {:?} objects in LVBAG XML Extract", BagObjects::STA);
                    }
                }
            }
        _ => todo!()
            
        }
  }
  
  let elapsed = now.elapsed();
  println!("Elapsed: {:.4?}", elapsed);

    // let zip_file = fs::File::open(&zip_path).unwrap();
    // let mut archive = zip::ZipArchive::new(&zip_file).unwrap();

    // for i in 0..archive.len() {
    //     let file_entry:ZipFile = archive.by_index(i).unwrap();
    //     let file_name = file_entry.enclosed_name().unwrap();

    //     let str_file_name:String = match file_entry.enclosed_name() {
    //         Some(path) => path.to_owned().display().to_string(),
    //         None => continue
    //     };

    //     // println!("{}", &str_file_name);

    //     if str_file_name.contains("NUM") {
    //       println!("THIS IS WHERE THE FILE SHOULD BE PRINTED{}", &str_file_name);
    //       let nested_zip_file = archive.by_name(&str_file_name).unwrap();
    //       let mut nested_archive = zip::ZipArchive::new(nested_zip_file).unwrap();

    //       for i in 0..nested_archive.len() {
    //         let nested_file_entry:ZipFile = nested_archive.by_index(i).unwrap();
    //         let nested_file_name = nested_file_entry.enclosed_name().unwrap();
    //         println!("{:?}", &nested_file_name);
    //       }
    //     }
    // }

    
}


fn run() -> Result<(), ZipError> {

  let zip_path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/lvbag-extract-nl.zip";
  // Open the outer zip file
  let mut outer_zip = ZipArchive::new(fs::File::open(&zip_path).unwrap())?;

  // Open the nested zip file
  let mut nested_zip_file = outer_zip.by_name("9999NUM08112022.zip")?;
  let mut contents = vec![];
  nested_zip_file.read_to_end(&mut contents)?;
  let mut nested_zip = ZipArchive::new(Cursor::new(contents))?;

  // Open the XML file within the nested zip file
  let mut xml_file = nested_zip.by_name("9999NUM08112022-000001.xml")?;
  let mut xml_contents = String::new();
  xml_file.read_to_string(&mut xml_contents)?;

  // Print the file contents
  // println!("{}", xml_contents);
  parse_num(&xml_contents);

  Ok(())
}

pub fn parse_pnd() {
  let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/src/test_xmls/9999PND08112022-000001.xml";
  let content = fs::read_to_string(&path).expect("Something went wrong with the file");
  
  let bag_xml: pnd::BagStand = from_str(&content).unwrap();
  // println!("{:?}",bag_xml);

  let result =  bag_xml.stand_bestand.stand;
  println!("{:?}", result.last());
}


pub fn parse_wpl() {
  let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/lvbag-extract-nl/9999WPL08112022-000001.xml";
  let content = fs::read_to_string(&path).expect("Something went wrong with the file");
  
  let bag_xml: wpl::BagStand = from_str(&content).unwrap();
  println!("{:?}",bag_xml);
}


pub fn parse_sta() {
  let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/lvbag-extract-nl/9999STA08112022/9999STA08112022-000001.xml";
  let content = fs::read_to_string(&path).expect("Something went wrong with the file");
  
  let bag_xml: sta::BagStand = from_str(&content).unwrap();
  // println!("{:?}",bag_xml);

  let result =  bag_xml.stand_bestand.stand;
  // println!("{:?}", result.last());


  for elements in result {
      println!("{:?}",elements.bag_object.standplaats.geometrie);
  }
}

pub fn parse_opr() {
  let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/lvbag-extract-nl/9999OPR08112022/9999OPR08112022-000001.xml";
  let content = fs::read_to_string(&path).expect("Something went wrong with the file");
  
  let bag_xml: opr::BagStand = from_str(&content).unwrap();
  // println!("{:?}",bag_xml);

  let result =  bag_xml.stand_bestand.stand;
  // // println!("{:?}", result);


  for elements in result {
      println!("{:?}",elements.bag_object.openbareruimte);
  }
}

pub fn parse_lig() {
  let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/lvbag-extract-nl/9999LIG08112022/9999LIG08112022-000001.xml";
  let content = fs::read_to_string(&path).expect("Something went wrong with the file");
  
  let bag_xml: lig::BagStand = from_str(&content).unwrap();
  println!("{:?}",bag_xml);

  // let result =  bag_xml.stand_bestand.stand;
  // // println!("{:?}", result);


  // for elements in result {
  //     println!("{:?}",elements.bag_object.ligplaats.geometrie);
  // }
}

pub fn parse_vbo() {
    let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/src/9999VBO08112022-000001.xml";
    let content = fs::read_to_string(&path).expect("Something went wrong with the file");

    let bag_xml: vbo::BagStand = from_str(&content).unwrap();

    let result =  bag_xml.stand_bestand.stand;
    // println!("{:?}", result.last());


    for elements in result {
        println!("{:?}",elements.bag_object.verblijfsobject.geometrie);
    }

  //   for elements in result {
  //     let vbo = BagVerblijfsobject { 
  //         nummeraanduidingref: elements.bag_object.verblijfsobject.heeftalshoofdadres.nummeraanduidingref,
  //         identificatie: "None".to_string(),
  //         gebruiksdoel: elements.bag_object.verblijfsobject.gebruiksdoel[0].to_string(), 
  //         oppervlakte: elements.bag_object.verblijfsobject.oppervlakte, 
  //         status: elements.bag_object.verblijfsobject.status, 
  //         geconstateerd: elements.bag_object.verblijfsobject.geconstateerd, 
  //         documentdatum: elements.bag_object.verblijfsobject.documentdatum, 
  //         documentnummer: elements.bag_object.verblijfsobject.documentnummer, 
  //         pandref: elements.bag_object.verblijfsobject.maaktdeelditvan.pandref[0].to_string(), 
  //         latitude: parse_coords(elements.bag_object.verblijfsobject.geometrie.punt.attributes.pos)[0], 
  //         longitude: 32.0,
  //     };
  //     println!("{:?}",vbo);
  // }
}

pub fn parse_num(xml_c:&str) {

    let bag_xml: num::BagStand = from_str(&xml_c).unwrap();
    
    let results = bag_xml.stand_bestand.stand;
    println!("{:?}", results.len());

    let mut wtr = csv::Writer::from_path("num.csv").unwrap();

    for element in results {
        let addr = BagAddress {
            identificatie: element.bag_object.nummeraanduiding.identificatie,
            huisnummer: element.bag_object.nummeraanduiding.huisnummer,
            huisletter: element.bag_object.nummeraanduiding.huisletter,
            huisnummertoevoeging: element.bag_object.nummeraanduiding.huisnummertoevoeging,
            postcode: element.bag_object.nummeraanduiding.postcode,
            type_adresseerbaar_object: element.bag_object.nummeraanduiding.type_adresseerbaar_object,
            status: element.bag_object.nummeraanduiding.status,
            geconstateerd: element.bag_object.nummeraanduiding.geconstateerd,
            documentdatum: element.bag_object.nummeraanduiding.documentdatum,
            documentnummer: element.bag_object.nummeraanduiding.documentnummer,
            openbareruimteref: element.bag_object.nummeraanduiding.ligtaan.openbareruimteref
        };
        wtr.serialize(addr).unwrap();
        wtr.flush().unwrap();
    }
}


#[derive(Debug, Serialize)]
struct BagAddress {
    pub identificatie: String,
    pub huisnummer: String,
    pub huisletter:Option<String>,
    pub huisnummertoevoeging:Option<String>,
    pub postcode: Option<String>,
    #[serde(rename="typeAdresseerbaarObject")]
    pub type_adresseerbaar_object: String,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
    #[serde(rename="OpenbareRuimteRef")]
    pub openbareruimteref: String,
}

#[derive(Debug, Serialize)]
struct BagVerblijfsobject {
    // pub nummeraanduidingref: String,
    // pub identificatie: String,
    // pub gebruiksdoel: String,
    // pub oppervlakte: Option<String>,
    // pub status: String,
    pub geconstateerd: String,
    // pub documentdatum: String,
    // pub documentnummer: String,
    // pub pandref: String,
    // pub latitude: f32,
    // pub longitude: f32
}


#[test]
fn test(){

    let filename = "/Users/paulduvenage/Documents/Rust_Development/Experiments/xml_parse/src/9999NUM08112022-000001.xml"; 
    let content = fs::read_to_string(&filename).expect("something went wrong reading the file");
  
    let loaded: num::BagStand = from_str(&content).unwrap();
    println!("{:?}", loaded);
  
    let reference = num::BagStand {
      stand_bestand: num::StandBestand {
        dataset: "LVBAG".to_string(),
        inhoud: num::Inhoud { 
          gebied: "Nederland".to_string(), 
          leverings_id: "0000000001".to_string(),
          object_typen: num::ObjectTypen {
            object_type: "NUM".to_string()
          }
        },
        stand: vec![num::Stand {
          bag_object: num::BagObject {
            nummeraanduiding: num::Nummeraanduiding { 
              identificatie: "0000200000057534".to_string(),
              huisnummer: "32".to_string(), 
              huisletter: Some("A".to_string()), 
              huisnummertoevoeging: None,
              postcode: Some("6131BE".to_string()), 
              type_adresseerbaar_object: "Verblijfsobject".to_string(), 
              status: "Naamgeving uitgegeven".to_string(), 
              geconstateerd: "N".to_string(), 
              documentdatum: "2018-03-26".to_string(), 
              documentnummer: "BV05.00043-HLG".to_string(),
              ligtaan: num::LigtAan { openbareruimteref: "1883300000001522".to_string() }
              }
            }
         },
         num::Stand {
          bag_object: num::BagObject {
            nummeraanduiding: num::Nummeraanduiding { 
              identificatie: "0000200000057534".to_string(),
              huisnummer: "32".to_string(), 
              huisletter: Some("A".to_string()),
              huisnummertoevoeging: None, 
              postcode: Some("6131BE".to_string()), 
              type_adresseerbaar_object: "Verblijfsobject".to_string(), 
              status: "Naamgeving ingetrokken".to_string(), 
              geconstateerd: "N".to_string(), 
              documentdatum: "2018-04-04".to_string(), 
              documentnummer: "correctie".to_string(),
              ligtaan: num::LigtAan { openbareruimteref: "1883300000001522".to_string() } 
              }
            }
         }]
      }
    };
  
    assert_eq!(loaded, reference);
}