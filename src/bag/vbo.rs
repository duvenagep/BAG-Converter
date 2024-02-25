use crate::bag::geometries::*;
use crate::bag::shared::*;
use serde;
use serde::{Deserialize, Serialize};
use wkt::ToWkt;

// VBO Variant
#[derive(Deserialize)]
pub struct Verblijfsobject {
    #[serde(rename = "heeftAlsHoofdadres")]
    pub heeftalshoofdadres: HeeftAlsHoofdadres,
    #[serde(rename = "heeftAlsNevenadres")]
    pub heeftalsnevenadres: Option<Vec<HeeftAlsNevenadres>>,
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

#[derive(Deserialize)]
pub struct MaaktDeelUitVan {
    #[serde(rename = "PandRef")]
    pub pandref: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Vbo {
    pub gebruiksdoel: String,
    pub oppervlakte: String,
    pub hoofdadresNummeraanduidingRef: String,
    pub nevenadresNummeraanduidingRef: String,
    pub pandRef: String,
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

pub fn to_vbo(vbo: Verblijfsobject) -> Vbo {
    Vbo {
        gebruiksdoel: vbo.gebruiksdoel.join(","),
        oppervlakte: match vbo.oppervlakte {
            Some(opper) => opper,
            None => String::new(),
        },
        hoofdadresNummeraanduidingRef: vbo
            .heeftalshoofdadres
            .nummeraanduidingref
            .nummeraanduidingref,
        nevenadresNummeraanduidingRef: match vbo.heeftalsnevenadres {
            Some(neven_adress) => neven_adress
                .into_iter()
                .map(|n| n.nummeraanduidingref.nummeraanduidingref)
                .collect(),
            None => todo!(),
        },
        pandRef: vbo.maaktdeelditvan.pandref.join(","),
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
    }
}
