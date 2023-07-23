use serde::Deserialize;

/// Identity Struct Shared by all `BagObject` types
#[derive(Deserialize)]
pub struct Identity {
    #[serde(rename = "@domein")]
    pub domein: String,
    #[serde(rename = "$value")]
    pub identificatie: String,
}

/// Voorkomen Struct Shared by all `BagObject` types
#[derive(Deserialize)]
pub struct Voorkomen {
    #[serde(rename = "Voorkomen")]
    pub voorkomen: VoorkomenContent,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct BeschikbaarLV {
    #[serde(rename = "tijdstipRegistratieLV")]
    pub tijdstipregistratie_lv: String,
    #[serde(rename = "tijdstipEindRegistratieLV")]
    pub tijdstipeindregistratie_lv: Option<String>,
}

/// LigIn Struct Shared by all `BagObject` types
#[derive(Deserialize)]
pub struct LigtIn {
    #[serde(rename = "WoonplaatsRef")]
    pub woonplaatsref: WoonplaatsRef,
}
#[derive(Deserialize)]
pub struct WoonplaatsRef {
    #[serde(rename = "@domein")]
    pub domein: String,
    #[serde(rename = "$value")]
    pub woonplaatsref: String,
}

// HeeftAlsHoofdadres Shared by multiple `BagObject`
#[derive(Deserialize)]
pub struct HeeftAlsHoofdadres {
    #[serde(rename = "NummeraanduidingRef")]
    pub nummeraanduidingref: NummeraanduidingRef,
}

#[derive(Deserialize)]
pub struct HeeftAlsNevenadres {
    #[serde(rename = "heeftAlsNevenadres")]
    pub nummeraanduidingref: Option<NummeraanduidingRef>,
}

#[derive(Deserialize)]
pub struct NummeraanduidingRef {
    #[serde(rename = "@domein")]
    pub domein: String,
    #[serde(rename = "$value")]
    pub nummeraanduidingref: String,
}
