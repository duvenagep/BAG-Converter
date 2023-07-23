use crate::bag::{geometries::*, lig::*, num::*, opr::*, pnd::*, sta::*, vbo::*, wpl::*};

use quick_xml::de::from_str;
use serde;
use serde::Deserialize;
use wkt::ToWkt;

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

#[derive(Deserialize)]
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
        CSVStruct::Num(Num {
            huisnummer: num.huisnummer,
            huisletter: match num.huisletter {
                Some(hl) => hl,
                None => String::new(),
            },
            huisnummerToevoeging: match num.huisnummertoevoeging {
                Some(ht) => ht,
                None => String::new(),
            },
            postcode: match num.postcode {
                Some(pc) => pc,
                None => String::new(),
            },
            typeAdresseerbaarObject: num.type_adresseerbaar_object,
            openbareruimteRef: num.ligtaan.openbareruimteref.openbareruimteref,
            woonplaatsRef: match num.ligtin {
                Some(ligt) => ligt.woonplaatsref.woonplaatsref,
                None => String::new(),
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
                None => String::new(),
            },
            tijdstipRegistratie: num.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match num.voorkomen.voorkomen.eindregistratie {
                Some(ert) => ert,
                None => String::new(),
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
                None => String::new(),
            },
        })
    }
}

impl From<Ligplaats> for CSVStruct {
    fn from(lig: Ligplaats) -> Self {
        CSVStruct::Lig(Lig {
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
                None => String::new(),
            },
            tijdstipRegistratie: lig.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match lig.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => String::new(),
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
                None => String::new(),
            },
            geometry: match lig.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::Vlak(vlak) => vlak
                    .attributes
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak
                    .multi_surface
                    .surface_member
                    .into_iter()
                    .map(|p| {
                        p.polygon
                            .exterior
                            .linear_ring
                            .attributes
                            .pos_list
                            .wkt_string()
                    })
                    .collect(),
            },
        })
    }
}

impl From<OpenbareRuimte> for CSVStruct {
    fn from(opr: OpenbareRuimte) -> Self {
        CSVStruct::Opr(Opr {
            naam: opr.naam,
            _type: opr.type_,
            woonplaatsRef: opr.ligtin.woonplaatsref.woonplaatsref,
            verkorteNaam: match opr.verkortenaamouter {
                Some(vkn) => vkn.verkortenaam.verkortenaamopenbareruimte.verkortenaam,
                None => String::new(),
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
                None => String::new(),
            },
            tijdstipRegistratie: opr.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match opr.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => String::new(),
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
                None => String::new(),
            },
        })
    }
}

impl From<Pand> for CSVStruct {
    fn from(pnd: Pand) -> Self {
        CSVStruct::Pnd(Pnd {
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
                None => String::new(),
            },
            tijdstipRegistratie: pnd.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match pnd.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => String::new(),
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
                None => String::new(),
            },
            geometry: match pnd.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::Vlak(vlak) => vlak
                    .attributes
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak
                    .multi_surface
                    .surface_member
                    .into_iter()
                    .map(|p| {
                        p.polygon
                            .exterior
                            .linear_ring
                            .attributes
                            .pos_list
                            .wkt_string()
                    })
                    .collect(),
            },
        })
    }
}

impl From<Standplaats> for CSVStruct {
    fn from(sta: Standplaats) -> Self {
        CSVStruct::Sta(Sta {
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
                None => String::new(),
            },
            tijdstipRegistratie: sta.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match sta.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => String::new(),
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
                None => String::new(),
            },
            geometry: match sta.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::Vlak(vlak) => vlak
                    .attributes
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak
                    .multi_surface
                    .surface_member
                    .into_iter()
                    .map(|p| {
                        p.polygon
                            .exterior
                            .linear_ring
                            .attributes
                            .pos_list
                            .wkt_string()
                    })
                    .collect(),
            },
        })
    }
}

impl From<Verblijfsobject> for CSVStruct {
    fn from(vbo: Verblijfsobject) -> Self {
        CSVStruct::Vbo(Vbo {
            // gebruiksdoel: vbo.gebruiksdoel.first(),
            oppervlakte: match vbo.oppervlakte {
                Some(opper) => opper,
                None => String::new(),
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
                None => String::new(),
            },
            tijdstipRegistratie: vbo.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match vbo.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => String::new(),
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
                None => String::new(),
            },
            geometry: match vbo.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::Vlak(vlak) => vlak
                    .attributes
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak
                    .multi_surface
                    .surface_member
                    .into_iter()
                    .map(|p| {
                        p.polygon
                            .exterior
                            .linear_ring
                            .attributes
                            .pos_list
                            .wkt_string()
                    })
                    .collect(),
            },
        })
    }
}

impl From<Woonplaats> for CSVStruct {
    fn from(wpl: Woonplaats) -> Self {
        CSVStruct::Wpl(Wpl {
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
                None => String::new(),
            },
            tijdstipRegistratie: wpl.voorkomen.voorkomen.tijdstipregistratie,
            eindRegistratie: match wpl.voorkomen.voorkomen.eindregistratie {
                Some(er) => er,
                None => String::new(),
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
                None => String::new(),
            },
            geometry: match wpl.geometrie.geometrie {
                Geometry::Punt(point) => point.attributes.pos.wkt_string(),
                Geometry::Polygon(polygon) => polygon
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::Vlak(vlak) => vlak
                    .attributes
                    .exterior
                    .linear_ring
                    .attributes
                    .pos_list
                    .wkt_string(),
                Geometry::MultiVlak(mvlak) => mvlak
                    .multi_surface
                    .surface_member
                    .into_iter()
                    .map(|p| {
                        p.polygon
                            .exterior
                            .linear_ring
                            .attributes
                            .pos_list
                            .wkt_string()
                    })
                    .collect(),
            },
        })
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
