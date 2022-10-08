//! This crate is an implementation of SDFs (signed distance functions)
//!
//! You can combine, transform and develop complex SDFs and even add complete new ones to be
//! rendered with one of this libraries multiple renders.
#![warn(missing_docs)]


use math_vector::Vector;

/// Objects and tools that are useful for working with objects
pub mod objects;

/// Renderers and tools that are useful for working with renderers
pub mod render;

/// A rectangular space from which points can be samples
pub struct Domain {
    p0: Vector<f64>,
    p1: Vector<f64>,
    steps: Vector<usize>,
}
