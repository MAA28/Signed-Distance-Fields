use image::{ImageBuffer, Rgb, RgbImage};
use itertools_num::linspace;
use math_vector::Vector;

use crate::{objects::SignedDistanceField, Domain};

/// Renders a SDF into a images
pub fn image(
    sdf: &dyn SignedDistanceField,
    domain: &Domain,
    color_mapper: &dyn Fn(&f64) -> Rgb<u8>,
) -> RgbImage {
    let matrix = matrix(sdf, domain);

    ImageBuffer::from_fn(
        domain.steps.x as u32,
        domain.steps.y as u32,
        |x: u32, y: u32| -> Rgb<u8> {
            color_mapper(matrix.get(x as usize).unwrap().get(y as usize).unwrap())
        },
    )
}

/// Renders a SDF into a text
/// * `sdf` - The SDF, that is to be rendered
/// * `domain` - The domain, that is to be rendered _(I recommend a ratio of 1/2 between `domain.step.y / domain.step.x` to aviod streching)_
/// * `text_mapper` - The mapper, that will interprete the values
/// Example output:
/// ```
///                                                                                                     
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                           ##############                                           
///                                     ##########################                                     
///                                 ##################################                                 
///                              ########################################                              
///                            ############################################                            
///                          ################################################                          
///                        ####################################################                        
///                       ######################################################                       
///                      ########################################################                      
///                     ##########################################################                     
///                    ############################################################                    
///                   ##############################################################                   
///                   ##############################################################                   
///                  ################################################################                  
///                  ################################################################                  
///                  ################################################################                  
///                  ################################################################                  
///                  ################################################################                  
///                  ################################################################                  
///                   ##############################################################                   
///                   ##############################################################                   
///                    ############################################################                    
///                     ##########################################################                     
///                      ########################################################                      
///                       ######################################################                       
///                        ####################################################                        
///                          ################################################                          
///                            ############################################                            
///                              ########################################                              
///                                 ##################################                                 
///                                     ##########################                                     
///                                           ##############                                           
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///                                                                                                    
///
///
/// ```
pub fn text(
    sdf: &dyn SignedDistanceField,
    domain: &Domain,
    text_mapper: &dyn Fn(&f64) -> char,
) -> String {
    let matrix = matrix(sdf, domain);

    let mut s = String::new();
    for j in 0..domain.steps.y {
        for i in 0..domain.steps.x {
            let value = matrix.get(i).unwrap().get(j).unwrap();
            s.push(text_mapper(value));
        }
        s.push('\n');
    }

    s
}

/// Renders a SDF into a matrix
pub fn matrix(sdf: &dyn SignedDistanceField, domain: &Domain) -> Vec<Vec<f64>> {
    let mut matrix = Vec::new();

    for x in linspace(domain.p0.x, domain.p1.x, domain.steps.x) {
        matrix.push(Vec::new());
        for y in linspace(domain.p0.y, domain.p1.y, domain.steps.y) {
            let p = Vector { x, y, z: 0.0 };
            let value = sdf.call(p);
            matrix.last_mut().unwrap().push(value);
        }
    }

    matrix
}

/// Change the style of the text render
pub mod text_mappers {

    /// Drawes the edge ('*') and show which parts are inside ('-') and which are outside ('-')
    pub fn default(x: &f64) -> char {
        if *x == 0.0 {
            '*'
        } else if 0.0 > *x && *x > -0.5 {
            '-'
        } else if 0.0 < *x && *x < 0.5 {
            '+'
        } else {
            ' '
        }
    }
    
    /// Fills the inside with '#'
    pub fn fill_inside(x: &f64) -> char {
        if *x < 0.0 {
            '#'
        } else {
            ' '
        }
    }
}

/// Change the style of the image render
pub mod color_mappers {
    use image::Rgb;
    use palette::{Gradient, LinSrgb};

    /// Turns the distance into a linear grayscale, where a distance of 0 becomes 128
    pub fn default(x: &f64) -> Rgb<u8> {
        let value = ((x + 0.5) * 256.0).floor().ceil() as u8;
        Rgb([value; 3])
    }

    /// Maps the distance to a color on the range of red to dark red, if the distance is positive
    /// and blue to dark blue, if the distance is negative
    pub fn red_blue_repeating(x: &f64) -> Rgb<u8> {
        let red = Gradient::new(vec![
            LinSrgb::new(1.0, 0.0, 0.0),
            LinSrgb::new(0.5, 0.0, 0.0),
        ]);
        let blue = Gradient::new(vec![
            LinSrgb::new(0.0, 0.0, 1.0),
            LinSrgb::new(0.0, 0.0, 0.5),
        ]);

        let col = {
            if *x < 0.0 {
                red.get(x.abs() % 1.0)
            } else {
                blue.get(x % 1.0)
            }
        };

        Rgb([
            (col.red * 256.0) as u8,
            (col.green * 256.0) as u8,
            (col.blue * 256.0) as u8,
        ])
    }
    
    /// If the value of that pixel is above 0.0 it's going to drawn in white else black.
    /// Hence it creates something like a hard mask of the SDF
    pub fn inside_black_outside_white(x: &f64) -> Rgb<u8> {
        if *x < 0.0 {
            Rgb([255, 255, 255])
        } else {
            Rgb([0, 0, 0])
        }
    }
}
