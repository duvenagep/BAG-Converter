use crate::helpers::deserializers::{deserialize_coords, deserialize_epsg, deserialize_pos};
use geo::{Point, Polygon as GeoPolygon};
use serde::Deserialize;

/// Geometries for each different GML type
#[derive(Deserialize)]
pub struct Geom {
    #[serde(rename = "$value")]
    pub geometrie: Geometry,
}

/// Enum of the different types of geometries in XML
#[derive(Deserialize)]
pub enum Geometry {
    #[serde(rename = "punt")]
    Punt(Punt),

    #[serde(rename = "Polygon")]
    Polygon(AttrsPoly),

    #[serde(rename = "vlak")]
    Vlak(Vlak),

    #[serde(rename = "multivlak")]
    MultiVlak(MultiVlak),
}

// Point Geometries
#[derive(Deserialize)]
pub struct Punt {
    #[serde(rename = "Point")]
    pub attributes: AttrsPoint,
}

#[derive(Deserialize)]
pub struct AttrsPoint {
    #[serde(deserialize_with = "deserialize_epsg")]
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "@srsDimension")]
    pub srs_dimension: i8,
    #[serde(deserialize_with = "deserialize_pos")]
    pub pos: Point,
}

// Polygon Geometries
#[derive(Deserialize)]
pub struct GeometriePoly {
    #[serde(rename = "Polygon")]
    pub attributes: AttrsPoly,
}

#[derive(Deserialize)]
pub struct AttrsPoly {
    #[serde(deserialize_with = "deserialize_epsg")]
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "@srsDimension")]
    pub srs_dimension: i8,
    pub exterior: Exterior,
}

#[derive(Deserialize)]
pub struct Exterior {
    #[serde(rename = "LinearRing")]
    pub linear_ring: LinearRing,
}

#[derive(Deserialize)]
pub struct LinearRing {
    #[serde(rename = "posList")]
    pub attributes: PosListAttr,
}

#[derive(Deserialize)]
pub struct PosListAttr {
    #[serde(rename = "@count")]
    pub coordinate_count: i16,
    #[serde(deserialize_with = "deserialize_coords")]
    #[serde(rename = "$value")]
    pub pos_list: GeoPolygon,
}

// Vlak Geometries
#[derive(Deserialize)]
pub struct Vlak {
    #[serde(rename = "Polygon")]
    pub attributes: AttrsPoly,
}

// MultiVlak Geometries
#[derive(Deserialize)]
pub struct MultiVlak {
    #[serde(rename = "MultiSurface")]
    pub multi_surface: MultiSurfaceAtters,
}

#[derive(Deserialize)]
pub struct MultiSurfaceAtters {
    #[serde(deserialize_with = "deserialize_epsg")]
    #[serde(rename = "@srsName")]
    pub srs_name: String,
    #[serde(rename = "@srsDimension")]
    pub srs_dimension: i8,
    #[serde(rename = "surfaceMember")]
    pub surface_member: Vec<SurfaceMember>,
}

#[derive(Deserialize)]
pub struct SurfaceMember {
    #[serde(rename = "Polygon")]
    pub polygon: MultiAttrsPoly,
}

#[derive(Deserialize)]
pub struct MultiAttrsPoly {
    pub exterior: Exterior,
}
