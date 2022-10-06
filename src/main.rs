use math_vector::Vector;

/// Objects and tools that are useful for working with objects
pub mod objects;

/// Renderers and tools that are useful for working with renderers
pub mod render;

fn main() {
    let sdf = objects::operators::Smooth { 
        sdf: Box::new(objects::operators::boolean::Not {
            sdf: Box::new(objects::operators::boolean::Union {
                a: Box::new(objects::operators::boolean::Not {
                    sdf: Box::new(objects::operators::transforms::Rotate {
                        alpha: 2.0,
                        axis: Vector {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        sdf: Box::new(objects::operators::transforms::Translate {
                            p: Vector {
                                x: 5.0,
                                y: 0.0,
                                z: 0.0,
                            },
                            sdf: Box::new(objects::primitives::Rectangle { w: 5.0, h: 7.5 }),
                        }),
                    }),
                }),
                b: Box::new(objects::primitives::Circle { r: 5.0 }),
            })
        }),
        k: 0.0
    };
    // let sdf = objects::primitives::Rectangle{w:5.0, h:6.0};
    let text = render::text(
        &sdf,
        &Domain {
            p0: Vector {
                x: -10.0,
                y: -10.0,
                z: 0.0,
            },
            p1: Vector {
                x: 10.0,
                y: 10.0,
                z: 0.0,
            },
            steps: Vector {
                x: 100,
                y: 50,
                z: 0,
            },
        },
        &render::text_mappers::fill_inside,
    );
    println!("{text}");

    let img = render::image(
        &sdf,
        &Domain {
            p0: Vector {
                x: -10.0,
                y: -10.0,
                z: 0.0,
            },
            p1: Vector {
                x: 10.0,
                y: 10.0,
                z: 0.0,
            },
            steps: Vector {
                x: 512,
                y: 512,
                z: 0,
            },
        },
        &render::color_mappers::red_blue_repeating,
    );

    img.save("render.png").unwrap();
}

/// A rectangular space from which points can be samples
pub struct Domain {
    p0: Vector<f64>,
    p1: Vector<f64>,
    steps: Vector<usize>,
}
