use math_vector::Vector;
use signed_distance_fields::{Domain, objects::{primitives::{Circle, Rectangle}, operators::boolean::{Union, Intersection, Difference}}, render::{self, text_mappers}};

fn main() {
    let domain = Domain {
        p0: Vector { x: -10.0, y: -10.0, z: 0.0 },
        p1: Vector { x: 10.0, y: 10.0, z: 0.0 },
        steps: Vector { x: 100, y: 50, z: 0 }
    };
    
    let a = Circle { r: 5.0 };

    println!("a:\n{}", render::text(&a, &domain, &text_mappers::default));
    
    let b = Rectangle { w: 15.0, h: 5.0 };

    println!("b:\n{}", render::text(&b, &domain, &text_mappers::default));

    let union = Union { a: Box::new(&a), b: Box::new(&b) };

    println!("a ∪ b:\n{}", render::text(&union, &domain, &text_mappers::default));
    
    let intersection = Intersection { a: Box::new(&a), b: Box::new(&b) };
    
    println!("a ∩ b:\n{}", render::text(&intersection, &domain, &text_mappers::default));

    let difference = Difference { a: Box::new(&a), b: Box::new(&b) };

    println!("a\\b:\n{}", render::text(&difference, &domain, &text_mappers::default));

}
