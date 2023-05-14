use serde::{Serialize,Deserialize};
use crate::helpers::helpers::{deserialize_epsg,deserialize_pos};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BagStand {
  #[serde(rename = "standBestand")]
  pub stand_bestand: StandBestand
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct StandBestand {
  pub dataset: String,
  pub inhoud: Inhoud,
  pub stand: Vec<Stand>
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Inhoud {
  pub gebied: String,
  #[serde(rename = "leveringsId")]
  pub leverings_id: String,
  #[serde(rename = "objectTypen")]
  pub object_typen: ObjectTypen
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct ObjectTypen {
  #[serde(rename = "objectType")]
  pub object_type: String
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Stand {
  #[serde(rename = "bagObject",)]
  pub bag_object: BagObject
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BagObject {
  #[serde(rename = "Verblijfsobject")]
  pub verblijfsobject: Verblijfsobject
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Verblijfsobject {
  #[serde(rename = "heeftAlsHoofdadres")]
  pub heeftalshoofdadres: HeeftAlsHoofdadres,
  pub identificatie: Identity,
  pub geometrie: Geometrie,
  pub gebruiksdoel: Vec<String>,
  pub oppervlakte: Option<String>,
  pub status: String,
  pub geconstateerd: String,
  pub documentdatum: String,
  pub documentnummer: String,
  #[serde(rename = "maaktDeelUitVan")]
  pub maaktdeelditvan: MaaktDeelUitVan
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Identity {
  #[serde(rename = "@domein")]
  pub domein: String,
  pub identificatie: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MaaktDeelUitVan {
  #[serde(rename = "PandRef")]
  pub pandref: Vec<String>
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct HeeftAlsHoofdadres {
  #[serde(rename = "NummeraanduidingRef")]
  pub nummeraanduidingref: String
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Geometrie {
  pub punt: Point
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Point {
  #[serde(rename = "Point")]
  pub attributes: Attrs,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Attrs {
  #[serde(deserialize_with = "deserialize_epsg")]
  #[serde(rename = "@srsName")]
  pub srs_name: String,
  #[serde(rename = "@srsDimension")]
  pub srs_dimension: i8,
  #[serde(deserialize_with = "deserialize_pos")]
  pub pos: Vec<f32>
}