use math_vector::Vector;

/// SDF of a rectangle
///
/// ![A SDF render of a rectangle with a width of 14.0 and a height of 5.0](https://github.com/MAA28/Signed-Distance-Fields/blob/main/images/signed_distance_field::objects::primitives::Rectangle%20%7B%20w:%2014.0,%20h:%205.0%20%7D.png?raw=true)
pub struct Rectangle {
    /// Width
    pub w: f64,
    /// Height
    pub h: f64,
}
impl super::SignedDistanceField for Rectangle {
    fn call(&self, p: Vector<f64>) -> f64 {
        let half_size = Vector {
            x: 0.5 * self.w,
            y: 0.5 * self.h,
            z: 0.0,
        };
        let component_wise_edge_distance = Vector {
            x: p.x.abs() - half_size.x,
            y: p.y.abs() - half_size.y,
            z: 0.0,
        };
        let outside_distance = Vector {
            x: component_wise_edge_distance.x.max(0.0),
            y: component_wise_edge_distance.y.max(0.0),
            z: 0.0,
        }
        .length();
        let inside_distance = component_wise_edge_distance
            .x
            .max(component_wise_edge_distance.y);

        let distance = outside_distance + inside_distance;

        distance
    }
}

/// SDF for a circle
///
/// ![A SDF render of a circle with a radius of 7.5](https://github.com/MAA28/Signed-Distance-Fields/blob/main/images/signed_distance_field::objects::primitives::Circle%20%7B%20r:%207.5%20%7D.png?raw=true")
pub struct Circle {
    /// Radius
    pub r: f64,
}
impl super::SignedDistanceField for Circle {
    fn call(&self, p: Vector<f64>) -> f64 {
        p.length() - self.r
    }
}

/// SDF for a infinite line
///
/// ![A SDF render of a infinite line](https://github.com/MAA28/Signed-Distance-Fields/blob/main/images/signed_distance_field::objects::primitives::Straight%20%7B%7D.png?raw=true)
pub struct Straight {}
impl super::SignedDistanceField for Straight {
    fn call(&self, p: Vector<f64>) -> f64 {
        p.y.abs()
    }
}

/// SDF for a line
///
/// ![A SDF render of a line with a length of 7.5](https://github.com/MAA28/Signed-Distance-Fields/blob/main/images/signed_distance_field::objects::primitives::Line%20%7B%20l:%206.0%20%7D.png?raw=true)
pub struct Line {
    /// Length of the line
    pub l: f64,
}
impl super::SignedDistanceField for Line {
    fn call(&self, p: Vector<f64>) -> f64 {
        if -0.5 * self.l > p.x {
            (Vector {
                x: -0.5 * self.l,
                y: 0.0,
                z: 0.0,
            } - p)
                .length()
        } else if p.x > 0.5 * self.l {
            (Vector {
                x: 0.5 * self.l,
                y: 0.0,
                z: 0.0,
            } - p)
                .length()
        } else {
            p.y.abs()
        }
    }
}
