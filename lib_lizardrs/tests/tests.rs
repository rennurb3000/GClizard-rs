#[cfg(test)]
mod tests {
    use approx::assert_ulps_eq;
    use glam::Vec2;
    use lib_lizardrs::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn vector_intersect2d() {
        let v1 = Vec2::new(1.0, 1.0); //does not need to be normalized
        let p1 = Vec2::new(0.0, 0.0);
        let v2 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 0.0);
        let res = vv_intersect2d(p1, v1, p2, v2);

        assert!(res.is_some());
        assert_ulps_eq!(res.unwrap(), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn no_vector_intersect2d() {
        let v1 = Vec2::new(0.0, 1.0); //does not need to be normalized
        let p1 = Vec2::new(0.0, 0.0);
        let v2 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 0.0);
        let res = vv_intersect2d(p1, v1, p2, v2);
        assert!(res.is_none());
    }
}
