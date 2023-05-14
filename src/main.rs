mod num;
mod vbo;
mod lig;
mod opr;

use serde::{Serialize};
use std::fs;
use quick_xml::{de::from_str};


fn main(){
    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
      parse_opr()
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.4?}", elapsed);
    
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

pub fn parse_num() {
    let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/src/full.xml";
    let content = fs::read_to_string(&path).expect("Something went wrong with the file");

    let bag_xml: num::BagStand = from_str(&content).unwrap();
    
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