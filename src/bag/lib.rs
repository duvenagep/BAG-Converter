use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use geo::{Coord, Point};
use crate::helpers::deserializers::{deserialize_epsg, deserialize_pos, deserialize_coords};


// Main xml Structure shared by all Enum Variants
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

// Enum Variants to be deserialised VBO,OPR,WPL,LIG,PND,NUM,STA
// Quick-xml auto-detects variant option via "$value" parameter
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BagObject {
    #[serde(rename = "$value")]
    pub objecten: BagObjectType,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
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
    Woonplaats(Woonplaats)
}

// NUM Variant Structure
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

// LIG Variant Structure
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Ligplaats {
    #[serde(rename = "heeftAlsHoofdadres")]
    pub heeftalshoofdadres: HeeftAlsHoofdadres,
    pub voorkomen: Voorkomen,
    pub identificatie: Identity,
    pub geometrie: GeometriePoly,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
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
    #[serde(rename = "tijdstipRegistratie")]
    pub tijdstipregistratie: String,
    #[serde(rename = "BeschikbaarLV")]
    pub beschikbaar_lv: BeschikbaarLV,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BeschikbaarLV {
    #[serde(rename = "tijdstipRegistratieLV")]
    pub tijdstipregistratie_lv: String,
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
pub struct GeometriePoly {
    #[serde(rename = "Polygon")]
    pub attributes: AttrsPoly,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AttrsPoly {
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
    #[serde(rename = "posList")]
    pub attributes: PosListAttr
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PosListAttr {
    #[serde(rename = "@count")]
    pub coordinate_count: i16,
    #[serde(deserialize_with = "deserialize_coords")]
    #[serde(rename = "$value")]
    pub pos_list: Vec<Coord>,
}

// OPR Variant Structure
#[derive(Debug, Deserialize, PartialEq, Serialize)]
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
    pub voorkomen: Voorkomen,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LigtIn {
    #[serde(rename = "WoonplaatsRef")]
    pub woonplaatsref: WoonplaatsRef,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct WoonplaatsRef {
    #[serde(rename = "@domein")]
    pub domein: String,
    #[serde(rename = "$value")]
    pub woonplaatsref: String,
}

//PND Variant
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Pand {
    pub identificatie: Identity,
    pub geometrie: GeometriePoly,
    #[serde(rename = "oorspronkelijkBouwjaar")]
    pub oorspronkelijkbouwjaar: String,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
    pub voorkomen: Voorkomen,
}

// STA Variant
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Standplaats {
    #[serde(rename = "heeftAlsHoofdadres")]
    pub heeftalshoofdadres: HeeftAlsHoofdadres,
    pub voorkomen: Voorkomen,
    pub identificatie: Identity,
    pub status: String,
    pub geometrie: GeometriePoly,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
}

// VBO Variant
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Verblijfsobject {
    #[serde(rename = "heeftAlsHoofdadres")]
    pub heeftalshoofdadres: HeeftAlsHoofdadres,
    pub identificatie: Identity,
    pub geometrie: GeometriePoint,
    pub gebruiksdoel: Vec<String>,
    pub oppervlakte: Option<String>,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
    #[serde(rename = "maaktDeelUitVan")]
    pub maaktdeelditvan: MaaktDeelUitVan,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MaaktDeelUitVan {
    #[serde(rename = "PandRef")]
    pub pandref: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct GeometriePoint {
    pub punt: Points,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Points {
    #[serde(rename = "Point")]
    pub attributes: AttrsPoint,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct AttrsPoint {
    #[serde(deserialize_with = "deserialize_epsg")]
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "@srsDimension")]
    pub srs_dimension: i8,
    #[serde(deserialize_with = "deserialize_pos")]
    pub pos: Point,
}

// WPL Variant
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Woonplaats {
    pub identificatie: Identity,
    pub naam: String,
    pub geometrie: GeometrieVlak,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
    pub voorkomen: Voorkomen,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct GeometrieVlak {
    pub vlak: Option<Vlak>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Vlak {
    #[serde(rename = "Polygon")]
    pub attributes: AttrsPoly,
}

impl BagStand {
    // Constructor Pattern to create new Bagstand from quick_xml::de::from_str()
    pub fn new(xml_str: &str) -> Result<Self, quick_xml::de::DeError> {
        from_str(xml_str)
    }

    // pub fn to_csv(bag_stand_object: &Self) -> Result<(), csv::Error>  {
    //     let tl = bag_stand_object.stand_bestand.stand;
        
    // }
}