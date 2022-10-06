use math_vector::Vector;

/// Trait that every SDF must implement
pub trait SignedDistanceField {
    /// This is the function that returns the distance for a point
    fn call(&self, p: Vector<f64>) -> f64;
}

/// Combine, change, develop SDFs
pub mod operators;

/// Primitive shapes
pub mod primitives;
