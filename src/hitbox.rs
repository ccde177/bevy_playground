use bevy::prelude::*;
pub struct Hitbox {
    area: Vec<(Vec2, Vec2)>,
    visible: bool,
}

impl Default for Hitbox {
    fn default() -> Self {
        Self {
            area: vec![],
            visible: false,
        }
    }
}

impl Hitbox {
    pub fn new(area: Vec<(Vec2, Vec2)>) -> Self {
        Self {
            area,
            visible: false,
        }
    }
}

pub fn intersection(hb: &Hitbox, point: Vec2) -> bool {
    hb.area
        .iter()
        .map(|(v1, v2)| (v1.min(*v2), v1.max(*v2)))
        .any(|(v1, v2)| (point.cmpge(v1) & point.cmple(v2)).all())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inside_single() {
        let point = Vec2::new(5., 5.);
        let hb = Hitbox::new(vec![(Vec2::new(0., 0.), Vec2::new(10., 10.))]);
        assert!(intersection(&hb, point));
    }

    #[test]
    fn outside_left_single() {
        let point = Vec2::new(-1., 5.);
        let hb = Hitbox::new(vec![(Vec2::new(0., 0.), Vec2::new(10., 10.))]);
        assert!(!intersection(&hb, point));
    }

    #[test]
    fn outside_right_single() {
        let point = Vec2::new(11., 5.);
        let hb = Hitbox::new(vec![(Vec2::new(0., 0.), Vec2::new(10., 10.))]);
        assert!(!intersection(&hb, point));
    }

    #[test]
    fn outside_above_single() {
        let point = Vec2::new(5., 11.);
        let hb = Hitbox::new(vec![(Vec2::new(0., 0.), Vec2::new(10., 10.))]);
        assert!(!intersection(&hb, point));
    }

    #[test]
    fn outside_below_single() {
        let point = Vec2::new(5., -11.);
        let hb = Hitbox::new(vec![(Vec2::new(0., 0.), Vec2::new(10., 10.))]);
        assert!(!intersection(&hb, point));
    }

    #[test]
    fn border_left_single() {
        let point = Vec2::new(0., 5.);
        let hb = Hitbox::new(vec![(Vec2::new(0., 0.), Vec2::new(10., 10.))]);
        assert!(intersection(&hb, point));
    }

    #[test]
    fn border_right_single() {
        let point = Vec2::new(10., 5.);
        let hb = Hitbox::new(vec![(Vec2::new(0., 0.), Vec2::new(10., 10.))]);
        assert!(intersection(&hb, point));
    }

    #[test]
    fn border_upper_single() {
        let point = Vec2::new(5., 10.);
        let hb = Hitbox::new(vec![(Vec2::new(0., 0.), Vec2::new(10., 10.))]);
        assert!(intersection(&hb, point));
    }

    #[test]
    fn border_lower_single() {
        let point = Vec2::new(5., 0.);
        let hb = Hitbox::new(vec![(Vec2::new(0., 0.), Vec2::new(10., 10.))]);
        assert!(intersection(&hb, point));
    }
}
