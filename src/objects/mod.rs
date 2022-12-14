use math_vector::Vector;

/// Trait that every SDF must implement
pub trait SignedDistanceField {
    /// This is the function that returns the distance for a point
    fn call(&self, p: Vector<f64>) -> f64;
}

/// A general purpose SDF with just a distance function
pub struct F {
    /// The distance function
    pub f: Box<dyn Fn(Vector<f64>) -> f64>,
}
impl SignedDistanceField for F {
    fn call(&self, p: Vector<f64>) -> f64 {
        (self.f)(p)
    }
}

/// Combine, change, develop SDFs
pub mod operators;

/// Primitive shapes
pub mod primitives;
