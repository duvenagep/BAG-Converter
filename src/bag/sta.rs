use crate::bag::geometries::*;
use crate::bag::shared::*;
use serde;
use serde::{Deserialize, Serialize};
use wkt::ToWkt;

// STA Variant
#[derive(Deserialize)]
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

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Sta {
    pub hoofdadresNummeraanduidingRef: String,
    pub nevenadresNummeraanduidingRef: String,
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

pub fn to_sta(sta: Standplaats) -> Sta {
    Sta {
        hoofdadresNummeraanduidingRef: sta
            .heeftalshoofdadres
            .nummeraanduidingref
            .nummeraanduidingref,
        nevenadresNummeraanduidingRef: match sta.heeftalsnevenadres {
            Some(neven_adress) => neven_adress
                .into_iter()
                .map(|n| n.nummeraanduidingref.nummeraanduidingref)
                .collect(),
            None => String::new(),
        },
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
    }
}
