use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BagStand {
    #[serde(rename = "standBestand")]
    pub stand_bestand: StandBestand,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StandBestand {
    pub dataset: String,
    pub inhoud: Inhoud,
    pub stand: Vec<Stand>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Inhoud {
    pub gebied: String,
    #[serde(rename = "leveringsId")]
    pub leverings_id: String,
    #[serde(rename = "objectTypen")]
    pub object_typen: ObjectTypen,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectTypen {
    #[serde(rename = "objectType")]
    pub object_type: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Stand {
    #[serde(rename = "bagObject")]
    pub bag_object: BagObject,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BagObject {
    #[serde(rename = "Nummeraanduiding")]
    pub nummeraanduiding: Nummeraanduiding,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Nummeraanduiding {
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
    #[serde(rename = "ligtAan")]
    pub ligtaan: LigtAan,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LigtAan {
    #[serde(rename = "OpenbareRuimteRef")]
    pub openbareruimteref: String,
}

pub trait Parse {
    fn parse(xml_str: &str) -> Result<Self, quick_xml::de::DeError>
    where
        Self: Sized;
}

impl Parse for BagStand {
    fn parse(xml_str: &str) -> Result<Self, quick_xml::de::DeError> {
        from_str(xml_str)
    }
}