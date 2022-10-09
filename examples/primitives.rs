use math_vector::Vector;
use signed_distance_fields::{
    objects::primitives::{Circle, Rectangle, Line, Straight, Plane},
    render::{self, text_mappers},
    Domain,
};

fn main() {
    let domain = Domain {
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
    };


    println!("Circle:");
    example_circle(&domain);
    
    println!("Rectangle:");
    example_rectangle(&domain);
    
    println!("Line:");
    example_line(&domain);
    
    println!("Straight:");
    example_straight(&domain);
    
    println!("Plane:");
    example_plane(&domain);

}

fn example_circle(domain: &Domain) {
    let circle = Circle { r: 7.5 };

    let text = render::text(
        &circle,
        domain,
        &text_mappers::default,
    );
    println!("{text}");
}

fn example_rectangle(domain: &Domain) {
    let rectangle = Rectangle { w: 7.5, h: 14.0 };

    let text = render::text(
        &rectangle,
        domain,
        &text_mappers::default,
    );
    println!("{text}");
}

fn example_line(domain: &Domain) {
    let line = Line { l: 14.0 };
    
    let text = render::text(
        &line,
        domain,
        &text_mappers::default,
    );
    println!("{text}");
}

fn example_straight(domain: &Domain) {
    let straight = Straight {};
    
    let text = render::text(
        &straight,
        domain,
        &text_mappers::default,
    );
    println!("{text}");
}

fn example_plane(domain: &Domain) {
    let plane = Plane {};
    
    let text = render::text(
        &plane,
        domain,
        &text_mappers::default,
    );
    println!("{text}");
}
