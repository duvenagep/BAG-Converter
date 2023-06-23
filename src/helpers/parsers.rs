use csv::Writer;
use quick_xml::de::from_str;
use rayon::prelude::*;
use serde::{Serialize, Serializer};
use std::fs;
use std::fs::OpenOptions;
use std::sync::Mutex;

use crate::bag::{lig, num, opr, pnd, sta, vbo, wpl};

pub fn parse_pnd() {
    let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/src/test/Source/9999PND08112022-000001.xml";
    let content = fs::read_to_string(&path).expect("Something went wrong with the file");

    let bag_xml: pnd::BagStand = from_str(&content).unwrap();

    let result = bag_xml.stand_bestand.stand;
    println!("{:?}", result);
}

pub fn parse_wpl() {
    let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/src/test/Source/9999WPL08112022-000001.xml";
    let content = fs::read_to_string(&path).expect("Something went wrong with the file");

    let bag_xml: wpl::BagStand = from_str(&content).unwrap();

    let result = bag_xml.stand_bestand.stand;
    println!("{:?}", result);
}

pub fn parse_sta() {
    let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/src/test/Source/9999STA08112022-000001.xml";
    let content = fs::read_to_string(&path).expect("Something went wrong with the file");

    let bag_xml: sta::BagStand = from_str(&content).unwrap();

    let result = bag_xml.stand_bestand.stand;
    println!("{:?}", result);
}

pub fn parse_opr() {
    let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/src/test/Source/9999OPR08112022-000001.xml";
    let content = fs::read_to_string(&path).expect("Something went wrong with the file");

    let bag_xml: opr::BagStand = from_str(&content).unwrap();

    let result = bag_xml.stand_bestand.stand;
    println!("{:?}", result);
}

pub fn parse_lig() {
    let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/src/test/Source/9999LIG08112022-000001.xml";
    let content = fs::read_to_string(&path).expect("Something went wrong with the file");

    let bag_xml: lig::BagStand = from_str(&content).unwrap();

    let result = bag_xml.stand_bestand.stand;
    println!("{:?}", result);
}

pub fn parse_vbo() {
    let path = "/Users/paulduvenage/Documents/Rust_Development/Experiments/quick_xml_parse/src/test/Source/9999VBO08112022-000001.xml";
    let content = fs::read_to_string(&path).expect("Something went wrong with the file");

    let bag_xml: vbo::BagStand = from_str(&content).unwrap();

    let result = bag_xml.stand_bestand.stand;
    println!("{:?}", result);
}

pub fn parse_num(xml_c: &str) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("num.csv")
        .unwrap();

    // let mut writer = Writer::from_writer(file);
    let writer = Mutex::new(Writer::from_writer(file));

    let bag_xml: num::BagStand = from_str(&xml_c).unwrap();

    let results = bag_xml.stand_bestand.stand;
    let bag_addresses: Vec<BagAddress> = results
        .par_iter()
        .map(|element| BagAddress {
            identificatie: element.bag_object.nummeraanduiding.identificatie.clone(),
            huisnummer: element.bag_object.nummeraanduiding.huisnummer.clone(),
            huisletter: element.bag_object.nummeraanduiding.huisletter.clone(),
            huisnummertoevoeging: element
                .bag_object
                .nummeraanduiding
                .huisnummertoevoeging
                .clone(),
            postcode: element.bag_object.nummeraanduiding.postcode.clone(),
            type_adresseerbaar_object: element
                .bag_object
                .nummeraanduiding
                .type_adresseerbaar_object
                .clone(),
            status: element.bag_object.nummeraanduiding.status.clone(),
            geconstateerd: element.bag_object.nummeraanduiding.geconstateerd.clone(),
            documentdatum: element.bag_object.nummeraanduiding.documentdatum.clone(),
            documentnummer: element.bag_object.nummeraanduiding.documentnummer.clone(),
            openbareruimteref: element
                .bag_object
                .nummeraanduiding
                .ligtaan
                .openbareruimteref
                .clone(),
        })
        .collect();

    bag_addresses.par_iter().for_each(|address| {
        let mut writer = writer.lock().unwrap();
        writer.serialize(address).unwrap();
    });
    let mut writer = writer.lock().unwrap();
    writer.flush().unwrap();
}

#[derive(Debug, Serialize)]
struct BagAddress {
    pub identificatie: String,
    pub huisnummer: String,
    pub huisletter: Option<String>,
    pub huisnummertoevoeging: Option<String>,
    pub postcode: Option<String>,
    #[serde(rename = "typeAdresseerbaarObject")]
    pub type_adresseerbaar_object: String,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
    #[serde(rename = "OpenbareRuimteRef")]
    pub openbareruimteref: String,
}

use serde::de::DeserializeOwned;

pub fn parse_bag_stand<T>(xml_c: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let parsed: T = from_str(xml_c)?;
    Ok(parsed)
}
