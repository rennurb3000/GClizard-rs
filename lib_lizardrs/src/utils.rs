use glam::{Vec2, Vec3};
use std::convert::{TryFrom,TryInto};
pub enum GeoError{
    PolygonNotConvex,
    PointsAreIncident,
    NotWorkingYet
}
pub enum GeoTypes {
    None,
    Triangle2D(Vec2, Vec2, Vec2),
    Triangle3D(Vec3, Vec3, Vec3),
    Line2D(Vec2, Vec2),
    Line3D(Vec3, Vec3),
    Convex2D(Vec<Vec2>),
    Convex3D(Vec<Vec3>),
}
pub struct IntersectionResult2D;
pub struct Triangle2D {
    a: Vec2,
    b: Vec2,
    c: Vec2,
}

pub struct IndexedTriangles2D<'a>{
    vertices : &'a Vec<Vec2>,
    indices : Vec<i32>
}
pub struct IndexedTriangles3D<'a>{
    vertices : &'a Vec<Vec3>,
    indices : Vec<i32>
}
pub struct Convex2D {
    points: Vec<Vec2>,
}

pub struct 2DCoordsOnHP{

}
impl TryFrom<&Triangle2D> for Convex2D{
    type Error = GeoError;
    fn try_from(tri:&Triangle2D)->Result<Self,Self::Error>
    {
        // I can only convert a 2d triangle into a convex polygon, when the area is greater than 0
        if ulps_ne!((tri.b-tri.a).perp_dot(tri.b-tri.c),0.0)
        {
            Ok(Convex2D{points:vec!(tri.a,tri.b,tri.c)})
        }
        else
        {
            Err(GeoError::PolygonNotConvex)
        }
    } 
}
impl TryFrom<&Vec<Vec2>> for Convex2D{
    type Error = GeoError;
    fn try_from(points:&Vec<Vec2>)->Result<Self,Self::Error>
    {
        Err(GeoError::NotWorkingYet)   
    }
}
impl TryFrom<&[Vec2]> for Convex2D{
    type Error = GeoError;
    fn try_from(points:&[Vec2]) -> Result<Self,Self::Error>
    {
        Err(GeoError::NotWorkingYet)
    }
}

pub fn vv_intersect2d(p1: Vec2, v1: Vec2, p2: Vec2, v2: Vec2) -> Option<Vec2> {
    vv_intersect2d_ts(p1, v1, p2, v2).map(|ts| p1 + ts * v1)
}
fn vv_intersect2d_ts(p1: Vec2, v1: Vec2, p2: Vec2, v2: Vec2) -> Option<f32> {
    let a = v1.perp_dot(v2);
    let b = p1 - p2;
    if ulps_eq!(a, 0.0) {
        None
    } else {
        let ts = b.perp_dot(v2) / (-a);
        Some(ts)
    }
}

//TODO thx to clippy i tried slices hope, it was the right descicion
pub fn cc_intersect_2d(c1: &[Vec2], c2: &[Vec2]) -> Option<IntersectionResult2D> {
    //cannot intersect if there is nothing
    if c1.len() < 3 || c2.len() < 3 {
        return None;
    }
    // find start point
    let mut startfound: bool = false;
    for startC1 in 0..c1.len() - 1 {
        //TODO note incident lines
        //TODO points can be incident
        //TODO both "rotations" need to be the same way
        //TODO handle "degenerated" convex polygons
        for startC2 in 0..c2.len() - 1 {
            let ts_result = vv_intersect2d_ts(
                c1[startC1],
                c1[startC1 + 1] - c1[startC1],
                c2[startC2],
                c2[startC2 + 1] - c2[startC2],
            );
            let ts = ts_result.unwrap();
            //TODO if None, check distance of edges
            if ts > 0.0 || ts < 1.0 {
                startfound = true;
                break;
            }
            //TODO handle case if ts is 0.0 or 1.0
        }
    }
    return None;
}
