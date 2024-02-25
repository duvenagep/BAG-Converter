use crate::bag::{lig::*, num::*, opr::*, pnd::*, sta::*, vbo::*, wpl::*};
use quick_xml::de::from_str;
use serde;
use serde::Deserialize;

// Main xml Structure shared by all Enum Variants
#[derive(Deserialize)]
pub struct BagStand {
    #[serde(rename = "standBestand")]
    pub stand_bestand: StandBestand,
}

#[derive(Deserialize)]
pub struct StandBestand {
    pub dataset: String,
    pub inhoud: Inhoud,
    pub stand: Vec<Stand>,
}

#[derive(Deserialize)]
pub struct Inhoud {
    pub gebied: String,
    #[serde(rename = "leveringsId")]
    pub leverings_id: String,
    #[serde(rename = "objectTypen")]
    pub object_typen: ObjectTypen,
}

#[derive(Deserialize)]
pub struct ObjectTypen {
    #[serde(rename = "objectType")]
    pub object_type: String,
}

#[derive(Deserialize)]
pub struct Stand {
    #[serde(rename = "bagObject")]
    pub bag_object: BagObject,
}

// Enum Variants to be deserialised VBO,OPR,WPL,LIG,PND,NUM,STA
// Quick-xml auto-detects variant option via "$value" parameter
#[derive(Deserialize)]
pub struct BagObject {
    #[serde(rename = "$value")]
    pub objecten: BagObjectType,
}

#[derive(Deserialize)]
pub enum BagObjectType {
    #[serde(rename = "Nummeraanduiding")]
    Nummeraanduiding(Nummeraanduiding),

    #[serde(rename = "Ligplaats")]
    Ligplaats(Ligplaats),

    #[serde(rename = "OpenbareRuimte")]
    OpenbareRuimte(OpenbareRuimte),

    #[serde(rename = "Pand")]
    Pand(Pand),

    #[serde(rename = "Standplaats")]
    Standplaats(Standplaats),

    #[serde(rename = "Verblijfsobject")]
    Verblijfsobject(Verblijfsobject),

    #[serde(rename = "Woonplaats")]
    Woonplaats(Woonplaats),
}

impl BagStand {
    // Constructor Pattern to create new Bagstand from quick_xml::de::from_str()
    pub fn new(xml_str: &str) -> Result<Self, quick_xml::de::DeError> {
        from_str(xml_str)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum CSVStruct {
    Num(Num),
    Lig(Lig),
    Opr(Opr),
    Pnd(Pnd),
    Sta(Sta),
    Vbo(Vbo),
    Wpl(Wpl),
}

impl CSVStruct {
    pub fn to_csv(&self, wtr: &mut csv::Writer<std::fs::File>) {
        use CSVStruct::*;

        match self {
            Num(data) => {
                wtr.serialize(data).unwrap();
                wtr.flush().unwrap();
            }
            Lig(data) => {
                wtr.serialize(data).unwrap();
                wtr.flush().unwrap();
            }
            Opr(data) => {
                wtr.serialize(data).unwrap();
                wtr.flush().unwrap();
            }
            Pnd(data) => {
                wtr.serialize(data).unwrap();
                wtr.flush().unwrap();
            }
            Sta(data) => {
                wtr.serialize(data).unwrap();
                wtr.flush().unwrap();
            }
            Vbo(data) => {
                wtr.serialize(data).unwrap();
                wtr.flush().unwrap();
            }
            Wpl(data) => {
                wtr.serialize(data).unwrap();
                wtr.flush().unwrap();
            }
        }
    }
}

impl From<Nummeraanduiding> for CSVStruct {
    fn from(num: Nummeraanduiding) -> Self {
        CSVStruct::Num(to_num(num))
    }
}

impl From<Ligplaats> for CSVStruct {
    fn from(lig: Ligplaats) -> Self {
        CSVStruct::Lig(to_lig(lig))
    }
}

impl From<OpenbareRuimte> for CSVStruct {
    fn from(opr: OpenbareRuimte) -> Self {
        CSVStruct::Opr(to_opr(opr))
    }
}

impl From<Pand> for CSVStruct {
    fn from(pnd: Pand) -> Self {
        CSVStruct::Pnd(to_pnd(pnd))
    }
}

impl From<Standplaats> for CSVStruct {
    fn from(sta: Standplaats) -> Self {
        CSVStruct::Sta(to_sta(sta))
    }
}

impl From<Verblijfsobject> for CSVStruct {
    fn from(vbo: Verblijfsobject) -> Self {
        CSVStruct::Vbo(to_vbo(vbo))
    }
}

impl From<Woonplaats> for CSVStruct {
    fn from(wpl: Woonplaats) -> Self {
        CSVStruct::Wpl(to_wpl(wpl))
    }
}

impl From<BagStand> for Vec<CSVStruct> {
    fn from(b: BagStand) -> Self {
        use BagObjectType::*;

        b.stand_bestand
            .stand
            .into_iter()
            .map(|stand| match stand.bag_object.objecten {
                Nummeraanduiding(nummeraanduiding) => CSVStruct::from(nummeraanduiding),

                Ligplaats(ligplaats) => CSVStruct::from(ligplaats),

                OpenbareRuimte(openbareruimte) => CSVStruct::from(openbareruimte),

                Pand(pand) => CSVStruct::from(pand),

                Standplaats(standplaats) => CSVStruct::from(standplaats),

                Verblijfsobject(verblijfsobject) => CSVStruct::from(verblijfsobject),

                Woonplaats(woonplaats) => CSVStruct::from(woonplaats),
            })
            .collect()
    }
}
