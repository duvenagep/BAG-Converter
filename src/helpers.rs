use serde::Deserialize;


pub fn deserialize_pos<'de, D>(deserializer: D) -> Result<Vec<f32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut values = Vec::new();

    let text = String::deserialize(deserializer)?;
    for s in text.split_whitespace() {
        values.push(s.parse().map_err(serde::de::Error::custom)?);
    }

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