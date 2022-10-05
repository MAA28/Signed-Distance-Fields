use math_vector::Vector;

pub trait SignedDistanceField {
    fn call(&self, p: Vector<f64>) -> f64;
}

pub mod transforms {
    use math_vector::Vector;
    
    /// Translate a SDF by p
    pub struct Translate {
        pub p: Vector<f64>,
        pub sdf: Box<dyn super::SignedDistanceField>
    }
    impl super::SignedDistanceField for Translate {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(p - self.p)
        }
    }

    /// Rotate a SDF by alpha radians
    pub struct Rotate {
        /// The angle by which the SDF will be rotated
        pub alpha: f64,
        /// The axis around which the SDF will be rotated
        pub axis: Vector<f64>,
        /// The SDF, that will be rotated
        pub sdf: Box<dyn super::SignedDistanceField>
    }
    impl super::SignedDistanceField for Rotate {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(p.rotate(self.alpha, self.axis))
        }
    }

    /// Scale a SDF by the vector 
    pub struct Scale {
        pub scale: Vector<f64>,
        pub sdf: Box<dyn super::SignedDistanceField>
    }
    impl super::SignedDistanceField for Scale {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(Vector { 
                x: p.x * self.scale.x, 
                y: p.y * self.scale.y, 
                z: 0.0 
            })
        }
    }

    pub mod boolean {
        pub struct Union {
            pub a: Box<dyn super::super::SignedDistanceField>,
            pub b: Box<dyn super::super::SignedDistanceField>
        }
        impl super::super::SignedDistanceField for Union {
            fn call(&self, p: math_vector::Vector<f64>) -> f64 {
                self.a.call(p).min(self.b.call(p))
            }
        }

        pub struct Intersection {
            pub a: Box<dyn super::super::SignedDistanceField>,
            pub b: Box<dyn super::super::SignedDistanceField>
        }
        impl super::super::SignedDistanceField for Intersection {
            fn call(&self, p: math_vector::Vector<f64>) -> f64 {
                self.a.call(p).max(self.b.call(p))
            }
        }

        pub struct Difference {
            pub a: Box<dyn super::super::SignedDistanceField>,
            pub b: Box<dyn super::super::SignedDistanceField>
        }
        impl super::super::SignedDistanceField for Difference {
            fn call(&self, p: math_vector::Vector<f64>) -> f64 {
                self.a.call(p) * self.b.call(p) - 1.0
            }
        }
    }
}




pub mod primitives {
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
}


