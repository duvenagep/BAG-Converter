use serde::Deserialize;
use geo::{Point, LineString, Polygon};
use proj::Proj;

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

    // let to = "EPSG:4326";
    // let from = "EPSG:28992";
    // let proj = Proj::new_known_crs(&from, &to, None).unwrap();

    // let result = proj.convert(vbo_point).unwrap();

    Ok(vbo_point)
}

pub fn deserialize_coords<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let coordinates = String::deserialize(deserializer)?;
    let values: Vec<f64> = coordinates
                        .split_whitespace()
                        .map(|coord| coord.parse::<f64>().unwrap())
                        .collect();




    // let values: Vec<(f64, f64)> = coordinates
    //                     .split_whitespace()
    //                     .map(|coord| {
    //                         let parts: Vec<f64> = coord
    //                             .split(',')
    //                             .map(|val| val.parse::<f64>().unwrap())
    //                             .collect();
    //                         (parts[0], parts[1])
    //                     })
    //                     .collect();

    // let polygon = Polygon::new(
    //     LineString::from(values),
    //     vec![],
    // );

    Ok(values)
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
