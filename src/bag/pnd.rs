use crate::bag::geometries::*;
use crate::bag::shared::*;
use serde;
use serde::{Deserialize, Serialize};
use wkt::ToWkt;

//PND Variant
#[derive(Deserialize)]
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

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Pnd {
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

pub fn to_pnd(pnd: Pand) -> Pnd {
    Pnd {
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
    }
}
