use math_vector::Vector;
pub struct Smooth {
    pub sdf: Box<dyn super::SignedDistanceField>,
    pub k: f64
}
impl super::SignedDistanceField for Smooth {
    fn call(&self, p: Vector<f64>) -> f64 {
        self.sdf.call(p) - self.k
    }
}

pub mod transforms {
    use math_vector::Vector;
    /// Translate a SDF by p
    pub struct Translate {
        pub p: Vector<f64>,
        pub sdf: Box<dyn super::super::SignedDistanceField>
    }
    impl super::super::SignedDistanceField for Translate {
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
        pub sdf: Box<dyn super::super::SignedDistanceField>
    }
    impl super::super::SignedDistanceField for Rotate {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(p.rotate(self.alpha, self.axis))
        }
    }

    /// Scale a SDF by the vector 
    pub struct Scale {
        pub scale: Vector<f64>,
        pub sdf: Box<dyn super::super::SignedDistanceField>
    }
    impl super::super::SignedDistanceField for Scale {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(Vector { 
                x: p.x * self.scale.x, 
                y: p.y * self.scale.y, 
                z: 0.0 
            })
        }
    }

    /// Multiplies a SDF by a matrix 
    pub struct Matrix {
        pub matrix: [[f64; 2]; 2],
        pub sdf: Box<dyn super::super::SignedDistanceField>
    }
    impl super::super::SignedDistanceField for Matrix {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(Vector { 
                x: self.matrix[0][0] * p.x + self.matrix[0][1] * p.y, 
                y: self.matrix[1][0] * p.x + self.matrix[1][1] * p.y, 
                z: 0.0 
            })
        }
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

    pub struct Not {
        pub a: Box<dyn super::super::SignedDistanceField>,
        pub b: Box<dyn super::super::SignedDistanceField>
    }
    impl super::super::SignedDistanceField for Not {
        fn call(&self, p: math_vector::Vector<f64>) -> f64 {
            let a = self.a.call(p);
            let b = self.b.call(p);
            -a.min(-b)
        }
    }
}
