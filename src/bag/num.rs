use crate::bag::shared::*;
use serde::{Deserialize, Serialize};

// NUM Variant Structure
#[derive(Deserialize)]
pub struct Nummeraanduiding {
    pub identificatie: Identity,
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
    #[serde(rename = "ligtIn")]
    pub ligtin: Option<LigtIn>,
    pub voorkomen: Voorkomen,
}

#[derive(Deserialize)]
pub struct LigtAan {
    #[serde(rename = "OpenbareRuimteRef")]
    pub openbareruimteref: OpenbareRuimteRef,
}

#[derive(Deserialize)]
pub struct OpenbareRuimteRef {
    #[serde(rename = "@domein")]
    pub domein: String,
    #[serde(rename = "$value")]
    pub openbareruimteref: String,
}

/// Intermediate Dataframe Compliant struct
/// TODO -> Implement Zero Copy optimizations Cow<&str>
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct Num {
    pub huisnummer: String,
    pub huisletter: String,
    pub huisnummerToevoeging: String,
    pub postcode: String,
    pub typeAdresseerbaarObject: String,
    pub openbareruimteRef: String,
    pub woonplaatsRef: String,
    pub identificatie: String,
    pub status: String,
    pub geconstateerd: String,
    pub documentDatum: String,
    pub documentNummer: String,
    pub voorkomenIdentificatie: String,
    pub beginGeldigheid: String,
    pub eindGeldigheid: String,
    pub tijdstipRegistratie: String,
    pub eindRegistratie: String,
    pub tijdstipRegistratieLV: String,
    pub tijdstipEindRegistratieLV: String,
}
