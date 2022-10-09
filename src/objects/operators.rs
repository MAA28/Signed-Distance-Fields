use math_vector::Vector;

/// This operator rounds the edges of functions
pub struct Smooth {
    /// The SDF, that will be smoothed
    pub sdf: Box<dyn super::SignedDistanceField>,
    /// Smoothing factor by which the SDF will be smoothed
    pub k: f64,
}
impl super::SignedDistanceField for Smooth {
    fn call(&self, p: Vector<f64>) -> f64 {
        self.sdf.call(p) - self.k
    }
}

/// The operators in this module apply linear transforms to SDFs
pub mod transforms {
    use math_vector::Vector;
    /// Translate a SDF
    pub struct Translate<'a> {
        /// The SDF will be translated by p
        pub p: Vector<f64>,
        /// The SDF, that will be translated
        pub sdf: Box<&'a dyn super::super::SignedDistanceField>,
    }
    impl super::super::SignedDistanceField for Translate<'_> {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(p - self.p)
        }
    }

    /// Rotate a SDF
    pub struct Rotate<'a> {
        /// The angle (in radians) by which the SDF will be rotated
        pub alpha: f64,
        /// The axis around which the SDF will be rotated
        pub axis: Vector<f64>,
        /// The SDF, that will be rotated
        pub sdf: Box<&'a dyn super::super::SignedDistanceField>,
    }
    impl super::super::SignedDistanceField for Rotate<'_> {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(p.rotate(self.alpha, self.axis))
        }
    }

    /// Scale a SDF
    pub struct Scale<'a> {
        /// The SDF will be scaled by the components of scale
        pub scale: Vector<f64>,
        /// The SDF, that will be scaled
        pub sdf: Box<&'a dyn super::super::SignedDistanceField>,
    }
    impl super::super::SignedDistanceField for Scale<'_> {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(Vector {
                x: p.x * self.scale.x,
                y: p.y * self.scale.y,
                z: 0.0,
            })
        }
    }

    /// Multiplie a SDF
    pub struct Matrix<'a> {
        /// The SDF will be multiplied with this matrix
        pub matrix: [[f64; 2]; 2],
        /// The SDF, that will be multiplied
        pub sdf: Box<&'a dyn super::super::SignedDistanceField>,
    }
    impl super::super::SignedDistanceField for Matrix<'_> {
        fn call(&self, p: Vector<f64>) -> f64 {
            self.sdf.call(Vector {
                x: self.matrix[0][0] * p.x + self.matrix[0][1] * p.y,
                y: self.matrix[1][0] * p.x + self.matrix[1][1] * p.y,
                z: 0.0,
            })
        }
    }
}

/// Use boolean logic operators on SDFs
pub mod boolean {
    /// Create a union of the SDFs (`a ∪ b`)
    pub struct Union<'a> {
        /// The one part of the union
        pub a: Box<&'a dyn super::super::SignedDistanceField>,
        /// The other part of the union
        pub b: Box<&'a dyn super::super::SignedDistanceField>,
    }
    impl super::super::SignedDistanceField for Union<'_> {
        fn call(&self, p: math_vector::Vector<f64>) -> f64 {
            self.a.call(p).min(self.b.call(p))
        }
    }

    /// Create a intersection of the SDFs (`a ∩ b`)
    pub struct Intersection<'a> {
        /// The one part of the intersection
        pub a: Box<&'a dyn super::super::SignedDistanceField>,
        /// The other part of the intersection
        pub b: Box<&'a dyn super::super::SignedDistanceField>,
    }
    impl super::super::SignedDistanceField for Intersection<'_> {
        fn call(&self, p: math_vector::Vector<f64>) -> f64 {
            self.a.call(p).max(self.b.call(p))
        }
    }

    /// Create the negative of a SDFs (`a\b`)
    pub struct Difference<'a> {
        /// The SDF to take away from
        pub a: Box<&'a dyn super::super::SignedDistanceField>,
        /// The SDF that will be taken away
        pub b: Box<&'a dyn super::super::SignedDistanceField>,
    }
    impl super::super::SignedDistanceField for Difference<'_> {
        fn call(&self, p: math_vector::Vector<f64>) -> f64 {
           self.a.call(p).min(-self.b.call(p)) 
        }
    }
}
