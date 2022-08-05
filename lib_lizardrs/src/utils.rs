use glam::{Vec2, Vec3};

pub enum GeoTypes{
    None,
    Triangle2D(Vec2,Vec2,Vec2),
    Triangle3D(Vec3,Vec3,Vec3),
    Line2D(Vec2,Vec2),
    Line3D(Vec3,Vec3),
    Convex2D(Vec<Vec2>),
    Convex3D(Vec<Vec3>)
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
pub fn cc_intersect_2d (c1: &[Vec2],c2:&[Vec2])
{
    for x in 0.. c1.len()
    {
        println!("{}", x);
    } 
}