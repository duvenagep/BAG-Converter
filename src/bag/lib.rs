use crate::helpers::deserializers::{deserialize_coords, deserialize_epsg, deserialize_pos};
use geo::{Point, Polygon as GeoPolygon};
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use wkt::ToWkt;
use std::borrow::Cow;

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
    Woonplaats(Woonplaats),
}

// NUM Variant Structure
#[derive(Debug, Deserialize, PartialEq, Serialize)]
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct LigtAan {
    #[serde(rename = "OpenbareRuimteRef")]
    pub openbareruimteref: OpenbareRuimteRef,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct OpenbareRuimteRef {
    #[serde(rename = "@domein")]
    pub domein: String,
    #[serde(rename = "$value")]
    pub openbareruimteref: String,
}

// LIG Variant Structure
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Ligplaats {
    #[serde(rename = "heeftAlsHoofdadres")]
    pub heeftalshoofdadres: HeeftAlsHoofdadres,
    #[serde(rename = "heeftAlsNevenadres")]
    pub heeftalsnevenadres: Option<Vec<HeeftAlsNevenadres>>,
    pub voorkomen: Voorkomen,
    pub identificatie: Identity,
    pub geometrie: Geom,
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
    pub tijdstipeindregistratie_lv: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct HeeftAlsHoofdadres {
    #[serde(rename = "NummeraanduidingRef")]
    pub nummeraanduidingref: NummeraanduidingRef,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct HeeftAlsNevenadres {
    #[serde(rename = "heeftAlsNevenadres")]
    pub nummeraanduidingref: Option<NummeraanduidingRef>,
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
    pub attributes: PosListAttr,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PosListAttr {
    #[serde(rename = "@count")]
    pub coordinate_count: i16,
    #[serde(deserialize_with = "deserialize_coords")]
    #[serde(rename = "$value")]
    pub pos_list: GeoPolygon,
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
    #[serde(rename = "Objecten:verkorteNaam")]
    pub verkortenaamouter: Option<VerkorteNaamOuter>,
    pub voorkomen: Voorkomen,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct VerkorteNaamOuter {
    #[serde(rename = "verkorteNaam")]
    pub verkortenaam: VerkorteNaamOpenbareRuimte,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct VerkorteNaamOpenbareRuimte {
    #[serde(rename = "VerkorteNaamOpenbareRuimte")]
    pub verkortenaamopenbareruimte: VerkorteNaam,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct VerkorteNaam {
    #[serde(rename = "verkorteNaam")]
    pub verkortenaam: String,
}

//PND Variant
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Pand {
    pub identificatie: Identity,
    pub geometrie: Geom,
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
    #[serde(rename = "heeftAlsNevenadres")]
    pub heeftalsnevenadres: Option<Vec<HeeftAlsNevenadres>>,
    pub voorkomen: Voorkomen,
    pub identificatie: Identity,
    pub status: String,
    pub geometrie: Geom,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Geom {
    #[serde(rename = "$value")]
    pub geometrie: Geometry,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Geometry {
    #[serde(rename = "punt")]
    Punt(Punt),

    #[serde(rename = "Polygon")]
    Polygon(AttrsPoly),
    
    #[serde(rename = "vlak")]
    Vlak(Vlak),

    #[serde(rename = "multivlak")]
    MultiVlak(MultiVlak)
}

// VBO Variant
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Verblijfsobject {
    #[serde(rename = "heeftAlsHoofdadres")]
    pub heeftalshoofdadres: HeeftAlsHoofdadres,
    pub voorkomen: Voorkomen,
    pub identificatie: Identity,
    pub geometrie: Geom,
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

#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
pub struct Punt {
    #[serde(rename = "Point")]
    pub attributes: AttrsPoint,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Default)]
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
    pub geometrie: Geom,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
    pub voorkomen: Voorkomen,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MultiVlak {
    #[serde(rename = "MultiSurface")]
    pub multi_surface: MultiSurfaceAtters,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MultiSurfaceAtters {
    #[serde(deserialize_with = "deserialize_epsg")]
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "@srsDimension")]
    pub srs_dimension: i8,
    #[serde(rename = "surfaceMember")]
    pub surface_member: Vec<SurfaceMember>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct SurfaceMember {
    #[serde(rename = "Polygon")]
    pub polygon: MultiAttrsPoly,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct MultiAttrsPoly {
    pub exterior: Exterior,
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
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum CSVStruct {
    NUM(NUM),
    LIG(LIG),
    OPR(OPR),
    PND(PND),
    STA(STA),
    VBO(VBO),
    WPL(WPL),
}

/// Intermediate Dataframe Compliant struct
/// TODO -> Implement Zero Copy optimizations Cow<&str>
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct NUM {
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct LIG {
    pub hoofdadresNummeraanduidingRef: String,
    // pub nevenadresNummeraanduidingRef: Vec<HeeftAlsNevenadres>,
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
    pub geometry: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct OPR {
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

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct PND {
    pub oorspronkelijkBouwjaar: String,
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
    pub geometry: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct STA {
    pub hoofdadresNummeraanduidingRef: String,
    // pub nevenadresNummeraanduidingRef: String,
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
    pub geometry: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct VBO {
    // pub gebruiksdoel: String,
    pub oppervlakte: String,
    pub hoofdadresNummeraanduidingRef: String,
    // pub nevenadresNummeraanduidingRef: String,
    // pub pandRef: String,
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
    pub geometry: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct WPL {
    pub naam: String,
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
    pub geometry: String,
}

impl From<Nummeraanduiding> for CSVStruct {
    fn from(num: Nummeraanduiding) -> Self {
        CSVStruct::NUM(NUM {
            huisnummer: num.huisnummer,
            huisletter: match num.huisletter {
                Some(hl) => hl,
                None => "".to_owned(),
            },
            huisnummerToevoeging: match num.huisnummertoevoeging {
                Some(ht) => ht,
                None => "".to_owned(),
            },
            postcode: match num.postcode {
                Some(pc) => pc,
                None => "".to_owned(),
            },
            typeAdresseerbaarObject: num.type_adresseerbaar_object,
            openbareruimteRef: num.ligtaan.openbareruimteref.openbareruimteref,
            woonplaatsRef: match num.ligtin {
                Some(ligt) => ligt.woonplaatsref.woonplaatsref,
                None => "".to_owned(),
            },
            identificatie: num.identificatie.identificatie,
            status: num.status,
            geconstateerd: num.geconstateerd,
            documentDatum: num.documentdatum,
            documentNummer: num.documentnummer,
            voorkomenIdentificatie: num.voorkomen.voorkomen.voorkomenidentificatie,
            beginGeldigheid: num.voorkomen.voorkomen.begingeldigheid,
            eindGeldigheid: match num.voorkomen.voorkomen.eindgeldigheid {
                Some(egh) => egh,
                None => "".to_owned(),
            },
            tijdstipRegistratie: num.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match num.voorkomen.voorkomen.eindregistratie {
                Some(ert) => ert,
                None => "".to_owned(),
            },
            tijdstipRegistratieLV: num
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipregistratie_lv,
            tijdstipEindRegistratieLV: match num
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipeindregistratie_lv
            {
                Some(tlv) => tlv,
                None => "".to_owned(),
            },
        })
    }
}

impl From<Ligplaats> for CSVStruct {
    fn from(lig: Ligplaats) -> Self {
        CSVStruct::LIG(LIG {
            hoofdadresNummeraanduidingRef: lig
                .heeftalshoofdadres
                .nummeraanduidingref
                .nummeraanduidingref,
            // nevenadresNummeraanduidingRef: match lig.heeftalsnevenadres {
            //     Some(neven_adress) => neven_adress,
            //     None => todo!(),
            // },
            identificatie: lig.identificatie.identificatie,
            status: lig.status,
            geconstateerd: lig.geconstateerd,
            documentDatum: lig.documentdatum,
            documentNummer: lig.documentnummer,
            voorkomenIdentificatie: lig.voorkomen.voorkomen.voorkomenidentificatie,
            beginGeldigheid: lig.voorkomen.voorkomen.begingeldigheid,
            eindGeldigheid: match lig.voorkomen.voorkomen.eindgeldigheid {
                Some(egh) => egh,
                None => "".to_owned(),
            },
            tijdstipRegistratie: lig.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match lig.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => "".to_owned(),
            },
            tijdstipRegistratieLV: lig
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipregistratie_lv,
            tijdstipEindRegistratieLV: match lig
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipeindregistratie_lv
            {
                Some(ter) => ter,
                None => "".to_owned(),
            },
            geometry: match lig.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::Vlak(vlak) => vlak.attributes.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak.multi_surface.surface_member
                .into_iter()
                .map(|p| p.polygon.exterior.linear_ring.attributes.pos_list.wkt_string())
                .collect(),
            } 

        })
    }
}

impl From<OpenbareRuimte> for CSVStruct {
    fn from(opr: OpenbareRuimte) -> Self {
        CSVStruct::OPR(OPR {
            naam: opr.naam,
            _type: opr.type_,
            woonplaatsRef: opr.ligtin.woonplaatsref.woonplaatsref,
            verkorteNaam: match opr.verkortenaamouter {
                Some(vkn) => vkn.verkortenaam.verkortenaamopenbareruimte.verkortenaam,
                None => "".into(),
            },
            identificatie: opr.identificatie.identificatie,
            status: opr.status,
            geconstateerd: opr.geconstateerd,
            documentDatum: opr.documentdatum,
            documentNummer: opr.documentnummer,
            voorkomenIdentificatie: opr.voorkomen.voorkomen.voorkomenidentificatie,
            beginGeldigheid: opr.voorkomen.voorkomen.begingeldigheid,
            eindGeldigheid: match opr.voorkomen.voorkomen.eindgeldigheid {
                Some(egh) => egh,
                None => "".into(),
            },
            tijdstipRegistratie: opr.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match opr.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => "".to_owned(),
            },
            tijdstipRegistratieLV: opr
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipregistratie_lv,
            tijdstipEindRegistratieLV: match opr
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipeindregistratie_lv
            {
                Some(ter) => ter,
                None => "".to_owned(),
            },
        })
    }
}

impl From<Pand> for CSVStruct {
    fn from(pnd: Pand) -> Self {
        CSVStruct::PND(PND {
            oorspronkelijkBouwjaar: pnd.oorspronkelijkbouwjaar,
            identificatie: pnd.identificatie.identificatie,
            status: pnd.status,
            geconstateerd: pnd.geconstateerd,
            documentDatum: pnd.documentdatum,
            documentNummer: pnd.documentnummer,
            voorkomenIdentificatie: pnd.voorkomen.voorkomen.voorkomenidentificatie,
            beginGeldigheid: pnd.voorkomen.voorkomen.begingeldigheid,
            eindGeldigheid: match pnd.voorkomen.voorkomen.eindgeldigheid {
                Some(egh) => egh,
                None => "".to_owned(),
            },
            tijdstipRegistratie: pnd.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match pnd.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => "".to_owned(),
            },
            tijdstipRegistratieLV: pnd
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipregistratie_lv,
            tijdstipEindRegistratieLV: match pnd
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipeindregistratie_lv
            {
                Some(ter) => ter,
                None => "".to_owned(),
            },
            geometry: match pnd.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::Vlak(vlak) => vlak.attributes.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak.multi_surface.surface_member
                .into_iter()
                .map(|p| p.polygon.exterior.linear_ring.attributes.pos_list.wkt_string())
                .collect(),
            } 
        })
    }
}

impl From<Standplaats> for CSVStruct {
    fn from(sta: Standplaats) -> Self {
        CSVStruct::STA(STA {
            hoofdadresNummeraanduidingRef: sta
                .heeftalshoofdadres
                .nummeraanduidingref
                .nummeraanduidingref,
            // nevenadresNummeraanduidingRef: match sta.heeftalsnevenadres {
            //     Some(neven_adress) => neven_adress,
            //     None => todo!(),
            // },
            identificatie: sta.identificatie.identificatie,
            status: sta.status,
            geconstateerd: sta.geconstateerd,
            documentDatum: sta.documentdatum,
            documentNummer: sta.documentnummer,
            voorkomenIdentificatie: sta.voorkomen.voorkomen.voorkomenidentificatie,
            beginGeldigheid: sta.voorkomen.voorkomen.begingeldigheid,
            eindGeldigheid: match sta.voorkomen.voorkomen.eindgeldigheid {
                Some(egh) => egh,
                None => "".to_owned(),
            },
            tijdstipRegistratie: sta.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match sta.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => "".to_owned(),
            },
            tijdstipRegistratieLV: sta
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipregistratie_lv,
            tijdstipEindRegistratieLV: match sta
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipeindregistratie_lv
            {
                Some(ter) => ter,
                None => "".to_owned(),
            },
            geometry: match sta.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::Vlak(vlak) => vlak.attributes.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak.multi_surface.surface_member
                .into_iter()
                .map(|p| p.polygon.exterior.linear_ring.attributes.pos_list.wkt_string())
                .collect(),
            } 
        })
    }
}

impl From<Verblijfsobject> for CSVStruct {
    fn from(vbo: Verblijfsobject) -> Self {
        CSVStruct::VBO(VBO {
            // gebruiksdoel: vbo.gebruiksdoel.first(),
            oppervlakte: match vbo.oppervlakte {
                Some(opper) => opper,
                None => "".to_owned(),
            },
            hoofdadresNummeraanduidingRef: vbo
                .heeftalshoofdadres
                .nummeraanduidingref
                .nummeraanduidingref,
            // nevenadresNummeraanduidingRef: match vbo.heeftalsnevenadres {
            //     Some(neven_adress) => neven_adress,
            //     None => todo!(),
            // },
            // pandRef: vbo.maaktdeelditvan.pandref,
            identificatie: vbo.identificatie.identificatie,
            status: vbo.status,
            geconstateerd: vbo.geconstateerd,
            documentDatum: vbo.documentdatum,
            documentNummer: vbo.documentnummer,
            voorkomenIdentificatie: vbo.voorkomen.voorkomen.voorkomenidentificatie,
            beginGeldigheid: vbo.voorkomen.voorkomen.begingeldigheid,
            eindGeldigheid: match vbo.voorkomen.voorkomen.eindgeldigheid {
                Some(egh) => egh,
                None => "".to_owned(),
            },
            tijdstipRegistratie: vbo.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match vbo.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => "".to_owned(),
            },
            tijdstipRegistratieLV: vbo
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipregistratie_lv,
            tijdstipEindRegistratieLV: match vbo
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipeindregistratie_lv
            {
                Some(ter) => ter,
                None => "".to_owned(),
            },
            geometry: match vbo.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::Vlak(vlak) => vlak.attributes.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak.multi_surface.surface_member
                .into_iter()
                .map(|p| p.polygon.exterior.linear_ring.attributes.pos_list.wkt_string())
                .collect(),
            },
        })
    }
}

impl From<Woonplaats> for CSVStruct {
    fn from(wpl: Woonplaats) -> Self {
        CSVStruct::WPL(WPL {
            naam: wpl.naam,
            identificatie: wpl.identificatie.identificatie,
            status: wpl.status,
            geconstateerd: wpl.geconstateerd,
            documentDatum: wpl.documentdatum,
            documentNummer: wpl.documentnummer,
            voorkomenIdentificatie: wpl.voorkomen.voorkomen.voorkomenidentificatie,
            beginGeldigheid: wpl.voorkomen.voorkomen.begingeldigheid,
            eindGeldigheid: match wpl.voorkomen.voorkomen.eindgeldigheid {
                Some(egh) => egh,
                None => "".to_owned(),
            },
            tijdstipRegistratie: wpl.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match wpl.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => "".to_owned(),
            },
            tijdstipRegistratieLV: wpl
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipregistratie_lv,
            tijdstipEindRegistratieLV: match wpl
                .voorkomen
                .voorkomen
                .beschikbaar_lv
                .tijdstipeindregistratie_lv
            {
                Some(ter) => ter,
                None => "".to_owned(),
            },
            geometry: match wpl.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::Vlak(vlak) => vlak.attributes.exterior.linear_ring.attributes.pos_list.wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak.multi_surface.surface_member
                .into_iter()
                .map(|p| p.polygon.exterior.linear_ring.attributes.pos_list.wkt_string())
                .collect(),
            },
        })
    }
}

impl From<BagStand> for Vec<CSVStruct> {
    fn from(b: BagStand) -> Self {
        b.stand_bestand
            .stand
            .into_iter()
            .filter_map(|stand| match stand.bag_object.objecten {
                BagObjectType::Nummeraanduiding(nummeraanduiding) => {
                    Some(CSVStruct::from(nummeraanduiding))
                }
                BagObjectType::Ligplaats(ligplaats) => Some(CSVStruct::from(ligplaats)),
                BagObjectType::OpenbareRuimte(openbareruimte) => {
                    Some(CSVStruct::from(openbareruimte))
                }
                BagObjectType::Pand(pand) => Some(CSVStruct::from(pand)),
                BagObjectType::Standplaats(standplaats) => Some(CSVStruct::from(standplaats)),
                BagObjectType::Verblijfsobject(verblijfsobject) => {
                    Some(CSVStruct::from(verblijfsobject))
                }
                BagObjectType::Woonplaats(woonplaats) => Some(CSVStruct::from(woonplaats)),
            })
            .collect()
    }
}
