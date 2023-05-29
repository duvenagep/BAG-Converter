use crate::helpers::deserializers::{deserialize_epsg, deserialize_coords};
use serde::{Deserialize, Serialize};
use geo::Polygon;

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
    #[serde(rename = "Woonplaats")]
    pub woonplaats: Woonplaats,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Woonplaats {
    pub identificatie: Identity,
    pub naam: String,
    pub geometrie: Geometrie,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
    pub voorkomen: Voorkomen,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Voorkomen {
    #[serde(rename = "Voorkomen")]
    pub voorkomen: VoorkomenContent,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct VoorkomenContent {
    pub voorkomenidentificatie: String,
    #[serde(rename = "beginGeldigheid")]
    pub begingeldigheid: String,
    #[serde(rename = "eindGeldigheid")]
    pub eindgeldigheid: Option<String>,
    #[serde(rename = "tijdstipRegistratie")]
    pub tijdstipregistratie: String,
    #[serde(rename = "eindRegistratie")]
    pub eindregistratie: Option<String>,
    #[serde(rename = "BeschikbaarLV")]
    pub beschikbaar_lv: BeschikbaarLV,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BeschikbaarLV {
    #[serde(rename = "tijdstipRegistratieLV")]
    pub tijdstipregistratie_lv: String,
    #[serde(rename = "tijdstipEindRegistratieLV")]
    pub tijdstipeindeegistratie_lv: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct HeeftAlsHoofdadres {
    #[serde(rename = "NummeraanduidingRef")]
    pub nummeraanduidingref: NummeraanduidingRef,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct NummeraanduidingRef {
    #[serde(rename = "@domein")]
    pub domein: String,
    #[serde(rename = "$value")]
    pub nummeraanduidingref: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Identity {
    #[serde(rename = "@domein")]
    pub domein: String,
    #[serde(rename = "$value")]
    pub identificatie: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Geometrie {
    pub vlak: Option<Vlak>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Vlak {
    #[serde(rename = "Polygon")]
    pub attributes: Attrs,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Attrs {
    #[serde(deserialize_with = "deserialize_epsg")]
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "@srsDimension")]
    pub srs_dimension: i8,
    pub exterior: Exterior,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Exterior {
    #[serde(rename = "LinearRing")]
    pub linear_ring: LinearRing,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LinearRing {
    #[serde(deserialize_with = "deserialize_coords")]
    #[serde(rename = "posList")]
    pub pos_list: Vec<f64>,
}
