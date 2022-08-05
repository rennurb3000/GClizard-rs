#[cfg(test)]
mod convex_convex_tests {
    use approx::assert_ulps_eq;
    use glam::Vec2;
    use lib_lizardrs::*;

    #[test]
    fn convex_convex1 () {
        let c1 = [Vec2{x:0.0, y:0.0}];
        let c2 =  [Vec2{x:0.0, y:1.0}];
        cc_intersect_2d(&c1,&c2);
        assert_eq!(1,1);
    }

}