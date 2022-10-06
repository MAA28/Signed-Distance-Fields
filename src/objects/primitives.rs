use math_vector::Vector;

/// SDF for rectangle
pub struct Rectangle {
    /// Width of the rectangle
    pub w: f64,
    /// Height of the rectangle
    pub h: f64
}
impl super::SignedDistanceField for Rectangle {
    fn call(&self, p: Vector<f64>) -> f64 {
        let half_size = Vector { x: 0.5 * self.w , y: 0.5 * self.h, z: 0.0 };
        let component_wise_edge_distance = Vector {
            x: p.x.abs() - half_size.x,
            y: p.y.abs() - half_size.y,
            z: 0.0
        };
        let outside_distance = Vector {
            x: component_wise_edge_distance.x.max(0.0),
            y: component_wise_edge_distance.y.max(0.0),
            z: 0.0
        }.length();
        let inside_distance = component_wise_edge_distance.x.max(component_wise_edge_distance.y);
         
        let distance = outside_distance + inside_distance;

        distance
    }
}


/// SDF for a circle
pub struct Circle {
    /// Radius
    pub r: f64,
}
impl super::SignedDistanceField for Circle {
    fn call(&self, p: Vector<f64>) -> f64 {
        p.length() - self.r
    }
}
