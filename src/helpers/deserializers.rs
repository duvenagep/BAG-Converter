use geo::{coord, Coord, LineString, Point, Polygon};
use proj::Proj;
use serde::Deserialize;
use std::cell::RefCell;

thread_local! {
    // Thread-local variable for the Proj transformer
    static TRANSFORMER: RefCell<Proj> = RefCell::new(transformer("EPSG:28992", "EPSG:4326"));
}

pub fn set_transformer(from_crs: &str, to_cr: &str) {
    TRANSFORMER.with(|t| *t.borrow_mut() = Proj::new_known_crs(from_crs, to_cr, None).unwrap())
}

// fn get_transformer() -> Proj {
//     TRANSFORMER.with_borrow(|t| *t.clone())
// }

pub fn transformer(from: &str, to: &str) -> Proj {
    Proj::new_known_crs(from, to, None).unwrap()
}

pub fn deserialize_pos<'de, D>(deserializer: D) -> Result<Point, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let coordinates = String::deserialize(deserializer)?;
    let values: Vec<f64> = coordinates
        .split_whitespace()
        .map(|coord| coord.parse::<f64>().unwrap())
        .collect();

    let vbo_point = TRANSFORMER.with(|trans| {
        let transformer = trans.borrow();
        transformer
            .convert(Point::new(values[0], values[1]))
            .unwrap()
    });

    Ok(vbo_point)
}

pub fn deserialize_coords<'de, D>(deserializer: D) -> Result<Polygon, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let coordinates = String::deserialize(deserializer)?;
    let mut coords: Vec<Coord> = coordinates
        .split_whitespace()
        .map(|coord| coord.parse::<f64>().unwrap())
        .filter(|&c| c != 0.0)
        .collect::<Vec<f64>>()
        .chunks(2)
        .map(|chunk| coord! { x:chunk[0], y:chunk[1]})
        .collect();

    let _transformed = TRANSFORMER.with(|trans| {
        let transformer = trans.borrow();
        transformer.convert_array(&mut coords).unwrap()
    });

    let polygon = Polygon::new(LineString::new(coords), vec![]);

    Ok(polygon)
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

/// Extract [`EPSG`] Projection from OGC CRS string
/// urn:ogc:def:crs:EPSG::28992
fn extract_epsg(srs_name: Option<&str>) -> Option<String> {
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
