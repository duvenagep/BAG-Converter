use crate::bag::shared::*;
use serde::{Deserialize, Serialize};

// OPR Variant Structure
#[derive(Deserialize)]
pub struct OpenbareRuimte {
    pub identificatie: Identity,
    pub naam: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
    #[serde(rename = "ligtIn")]
    pub ligtin: LigtIn,
    #[serde(rename = "Objecten:verkorteNaam")]
    pub verkortenaamouter: Option<VerkorteNaamOuter>,
    pub voorkomen: Voorkomen,
}

#[derive(Deserialize)]
pub struct VerkorteNaamOuter {
    #[serde(rename = "verkorteNaam")]
    pub verkortenaam: VerkorteNaamOpenbareRuimte,
}

#[derive(Deserialize)]
pub struct VerkorteNaamOpenbareRuimte {
    #[serde(rename = "VerkorteNaamOpenbareRuimte")]
    pub verkortenaamopenbareruimte: VerkorteNaam,
}

#[derive(Deserialize)]
pub struct VerkorteNaam {
    #[serde(rename = "verkorteNaam")]
    pub verkortenaam: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct Opr {
    pub naam: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub woonplaatsRef: String,
    pub verkorteNaam: String,
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
