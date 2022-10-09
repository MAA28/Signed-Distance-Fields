use math_vector::Vector;
use signed_distance_fields::{objects::{SignedDistanceField, primitives::Rectangle, operators::transforms::{Translate, Rotate, Scale, Matrix}}, Domain, render::{self, text_mappers}};

fn main() {
    let domain = Domain {
        p0: Vector { x: -10.0, y: -10.0, z: 0.0 },
        p1: Vector { x: 10.0, y: 10.0, z: 0.0 },
        steps: Vector { x: 100, y: 50, z: 0 }
    };
    
    let none = none();

    println!(
        "None:\n{}",
        render::text(&none, &domain, &text_mappers::default)
    );

    println!(
        "Translate:\n{}",
        render::text(&translate(&none), &domain, &text_mappers::default)
    );
    
    println!(
        "Rotate:\n{}",
        render::text(&rotate(&none), &domain, &text_mappers::default)
    );

    println!(
        "Scale:\n{}",
        render::text(&scale(&none), &domain, &text_mappers::default)
    );

    println!(
        "Matrix:\n{}",
        render::text(&matrix(&none), &domain, &text_mappers::default)
    );
}

fn none() -> impl SignedDistanceField {
    Rectangle { w: 7.5, h: 15.0 }
}


fn translate<'a>(rectangle: &'a dyn SignedDistanceField) -> impl SignedDistanceField + 'a {
    Translate { 
        sdf: Box::new(rectangle),
        p: Vector {
            x: -5.0,
            y: 3.0,
            z: 0.0
        }
    } 
}

fn rotate<'a>(rectangle: &'a dyn SignedDistanceField) -> impl SignedDistanceField + 'a {
    Rotate { 
        sdf: Box::new(rectangle),
        axis: Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0
        },
        alpha: 5.0
    }
}

fn scale<'a>(rectangle: &'a dyn SignedDistanceField) -> impl SignedDistanceField + 'a {
    Scale { 
        sdf: Box::new(rectangle),
        scale: Vector {
            x: 0.5,
            y: 2.0,
            z: 1.0
        }
    }
}

fn matrix<'a>(rectangle: &'a dyn SignedDistanceField) -> impl SignedDistanceField + 'a {
    Matrix {
        sdf: Box::new(rectangle),
        matrix: [
            [1.0, -1.1], 
            [0.3, -2.0]
        ]
    }
}
