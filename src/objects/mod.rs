use math_vector::Vector;

pub trait SignedDistanceField {
    fn call(&self, p: Vector<f64>) -> f64;
}

pub mod operators; 

pub mod primitives; 
