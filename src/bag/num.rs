use crate::bag::shared::*;
use serde::{Deserialize, Serialize};

// NUM Variant Structure
#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct LigtAan {
    #[serde(rename = "OpenbareRuimteRef")]
    pub openbareruimteref: OpenbareRuimteRef,
}

#[derive(Deserialize)]
pub struct OpenbareRuimteRef {
    #[serde(rename = "@domein")]
    pub domein: String,
    #[serde(rename = "$value")]
    pub openbareruimteref: String,
}

/// Intermediate Dataframe Compliant struct
/// TODO -> Implement Zero Copy optimizations Cow<&str>
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_snake_case)]
pub struct Num {
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

pub fn to_num(num: Nummeraanduiding) -> Num {
    Num {
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
    }
}
