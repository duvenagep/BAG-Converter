use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
// #[command(author, version, about, long_about = None, help_template = HELP_TEMPLATE)]
#[command(author, version, about, long_about = None)]
pub struct NLExtractArgs {
    #[clap(subcommand)]
    pub entity_type: LVBAGSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum LVBAGSubCommand {
    /// Download BAG 2.0 Extract
    Download(DownloadLvbag),

    /// Parse BAG 2.0 Extract
    Parse(ParseLvbag),

    /// Info BAG 2.0 Extract
    Info,
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
    #[arg(short, default_value = "source_data")]
    pub destination_folder: String,
}

#[derive(Debug, Args)]
pub struct ParseLvbag {
    /// Info about files in the BAG 2.0 Extract to Parse
    #[arg(short, long)]
    pub info: bool,

    /// List of BAG objects that are Parsable,num_args = 0.., value_delimiter = ',', use_value_delimiter=true
    #[arg(short = 'b', value_enum, num_args = 0..=7)]
    pub bag_object: Option<Vec<BagObjects>>,

    /// Bag file to be parsed (eq. lvbag-extract-nl.zip)
    #[arg(short = 'l')]
    pub file: String,

    /// Supported Parse formats for LVBAG XML
    #[arg(short, value_enum)]
    pub format: Option<OutputMode>,

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
    Vbo,
    Opr,
    Wpl,
    Lig,
    Pnd,
    Num,
    Sta,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord, ValueEnum)]
pub enum OutputMode {
    #[default]
    CSV,
    GEOJSON,
    SHAPEFILE,
    GEODATABASE,
    GEOPACKAGE,
    KML,
    GEOBUF,
    POSTGIS,
}

impl ToString for BagObjects {
    fn to_string(&self) -> String {
        use BagObjects::*;
        match &self {
            Vbo => "VBO".to_string(),
            Opr => "OPR".to_string(),
            Wpl => "WPL".to_string(),
            Lig => "LIG".to_string(),
            Pnd => "PND".to_string(),
            Num => "NUM".to_string(),
            Sta => "STA".to_string(),
        }
    }
}

const HELP_TEMPLATE: &str = r#"
{bin} {version}
{about}

USAGE:
    {usage} <COMMAND>

SUBCOMMANDS:
{subcommands}

    try "{usage} -h" to get more info about a specific tool

OPTIONS:
{options}

EXAMPLES:
    {bin} download -u https://service.pdok.nl/kadaster/adressen/atom/v1_0/downloads/lvbag-extract-nl.zip -d /home/documents/nlx
    {bin} parse -l lvbag-extract-nl.zip
    {bin} parse -b sta lig vbo -l lvbag-extract-nl.zip
    {bin} parse -b sta lig vbo -l lvbag-extract-nl.zip -f SHAPEFILE
    {bin} parse -b sta lig vbo -l lvbag-extract-nl.zip -f SHAPEFILE -p EPSG:4632
    {bin} parse -b sta lig vbo -l lvbag-extract-nl.zip -f SHAPEFILE -p EPSG:4632 -k a,b,c,d,e,f
    {bin} parse -b sta lig vbo -l lvbag-extract-nl.zip -f SHAPEFILE -p EPSG:4632 -e g,h,i
    {bin} info -l lvbag-extract-nl.zip"#;
