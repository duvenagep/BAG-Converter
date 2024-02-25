use crate::bag::geometries::*;
use crate::bag::shared::*;
use serde;
use serde::{Deserialize, Serialize};
use wkt::ToWkt;

// LIG Variant Structure
#[derive(Deserialize)]
pub struct Ligplaats {
    #[serde(rename = "heeftAlsHoofdadres")]
    pub heeftalshoofdadres: HeeftAlsHoofdadres,
    #[serde(rename = "heeftAlsNevenadres")]
    pub heeftalsnevenadres: Option<HeeftAlsNevenadres>,
    pub voorkomen: Voorkomen,
    pub identificatie: Identity,
    pub geometrie: Geom,
    pub status: String,
    pub geconstateerd: String,
    pub documentdatum: String,
    pub documentnummer: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Lig {
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

pub fn to_lig(lig: Ligplaats) -> Lig {
    Lig {
        hoofdadresNummeraanduidingRef: lig
            .heeftalshoofdadres
            .nummeraanduidingref
            .nummeraanduidingref,
        nevenadresNummeraanduidingRef: match lig.heeftalsnevenadres {
            Some(neven_adress) => neven_adress
                .nummeraanduidingref
                .into_iter()
                .map(|n| n.nummeraanduidingref)
                .collect::<Vec<_>>()
                .join(", "),
            None => String::new(),
        },
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
    }
}
