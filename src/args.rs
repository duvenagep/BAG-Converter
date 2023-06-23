use clap::{Args, Parser, Subcommand, ValueEnum};

/// ***************************************************************************
///     _   ____       _______  ____________  ___   ____________   _    _____
///    / | / / /      / ____/ |/ /_  __/ __ \/   | / ____/_  __/  | |  / /__ \
///   /  |/ / /      / __/  |   / / / / /_/ / /| |/ /     / /     | | / /__/ /
///  / /|  / /___   / /___ /   | / / / _, _/ ___ / /___  / /      | |/ // __/
/// /_/ |_/_____/  /_____//_/|_|/_/ /_/ |_/_/  |_\____/ /_/       |___//____/
/// ***************************************************************************                                                                           
/// This is a CLI for the NLBAG v2 extract
/// This CLI has 3 main methods:
///  - Download LVBAG extract v2.0
///  - Parse BAG XML to Postgresql or CSV
///  - Parse & Upload to s3 Bucket
#[derive(Debug, Parser)]
#[clap(verbatim_doc_comment)]
#[command(author, version, about, long_about = None)]
pub struct NLExtractArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Specify LVBAG (Landelijke Voorziening Basisregistratie Adressen en Gebouwen) BAG 2.0 Extract download URL and destination folder
    LVBAG(LVBAGCommand),
}

#[derive(Debug, Args)]
pub struct LVBAGCommand {
    #[clap(subcommand)]
    pub command: LVBAGSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum LVBAGSubCommand {
    /// Download BAG 2.0 Extract
    Download(DownloadLvbag),

    /// Parse BAG 2.0 Extract
    Parse(ParseLvbag),
}

#[derive(Debug, Args)]
pub struct DownloadLvbag {
    /// BAG 2.0 Extract download URL
    #[arg(
        short,
        default_value = "https://service.pdok.nl/kadaster/adressen/atom/v1_0/downloads/lvbag-extract-nl.zip"
    )]
    pub url: String,

    /// destination_folder
    #[arg(short)]
    pub destination_folder: String,
}

#[derive(Debug, Args)]
pub struct ParseLvbag {
    /// Info about files in the BAG 2.0 Extract to Parse
    #[arg(short)]
    pub info: Option<bool>,

    /// List of BAG objects that are Parsable,num_args = 0.., value_delimiter = ',', use_value_delimiter=true
    #[arg(short = 'b', value_enum, num_args = 0..=7)]
    pub bag_object: Option<Vec<BagObjects>>,

    /// Bag file to be parsed (eq. lvbag-extract-nl.zip)
    #[arg(short = 'l')]
    pub file: String,

    /// Supported Parse formats for LVBAG XML (POSTGIS, DSV, GEOJSON)
    #[arg(short, default_value = "CSV")]
    pub format: Option<String>,

    /// Any supported projection (EPSG:4632, EPSG:28892)
    #[arg(short, default_value = "EPSG:28892")]
    pub projection: Option<String>,

    /// List of columns to keep during parsing
    #[arg(short)]
    pub keep_columns: Option<Vec<String>>,

    /// List of columns to exclude during parsing
    #[arg(short)]
    pub exclude_columns: Option<Vec<String>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, ValueEnum)]
pub enum BagObjects {
    VBO,
    OPR,
    WPL,
    LIG,
    PND,
    NUM,
    STA,
}
