use geo::{Coord, Point, coord};
use serde::Deserialize;

pub fn deserialize_pos<'de, D>(deserializer: D) -> Result<Point, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let coordinates = String::deserialize(deserializer)?;
    let values: Vec<f64> = coordinates
        .split_whitespace()
        .map(|coord| coord.parse::<f64>().unwrap())
        .collect();

    let vbo_point = Point::new(values[0], values[1]);

    Ok(vbo_point)
}

pub fn deserialize_coords<'de, D>(deserializer: D) -> Result<Vec<Coord>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let coordinates = String::deserialize(deserializer)?;
    let coords: Vec<Coord> = coordinates
        .split_whitespace()
        .map(|coord| coord.parse::<f64>().unwrap())
        .filter(|&c| c != 0.0)
        .collect::<Vec<f64>>()
        .chunks(2)
        .map(|chunk| coord! { x:chunk[0], y:chunk[1]})
        .collect();

    Ok(coords)
}

pub fn deserialize_epsg<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let text = String::deserialize(deserializer)?;
    let epsg = extract_epsg(Some(&text));
    match epsg {
        Some(epsg) => Ok(epsg),
        None => Err(serde::de::Error::custom("invalid srsName")),
    }
}

fn extract_epsg<'a>(srs_name: Option<&'a str>) -> Option<String> {
    if let Some(srs_name) = srs_name {
        if srs_name.starts_with("urn:ogc:def:crs:EPSG::") {
            return srs_name
                .rsplit("::")
                .next()
                .and_then(|s| s.parse::<String>().ok());
        }
    }
    None
}
