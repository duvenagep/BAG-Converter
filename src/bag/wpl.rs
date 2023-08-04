use crate::bag::geometries::*;
use crate::bag::shared::*;
use serde;
use serde::{Deserialize, Serialize};
use wkt::ToWkt;

// WPL Variant
#[derive(Deserialize)]
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
#[allow(non_snake_case)]
pub struct Wpl {
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

pub fn to_wpl(wpl: Woonplaats) -> Wpl {
    Wpl {
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
    }
}
