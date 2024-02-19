use std::{f32::consts::PI, fs};

use dessin::prelude::*;
use dessin_svg::ToSVG;
    
use dessin::{
    nalgebra::{Rotation2,},
};
    
    
fn main(){
    
    // creates a rectangle with a width of 11 and a height of 6
    let thick_arc: ThickArc = ThickArc::default();

    let mut thick_arc = Style::new(thick_arc);

    // chooses a radius of 10 for the outer curve
    thick_arc.outer_radius=(10.);

    // chooses a radius of 5 for the inner curve
    thick_arc.inner_radius=(5.);

    // chooses an angle of PI to show the area of the thick arc (which depends of the 2 curve and this angle)
    thick_arc.span_angle(PI);

    // paints the inside of the thick_arc in yellow
    thick_arc.fill(Fill::Color(rgb(255, 255, 0)));

    // creates a black margin of 0.1 (0.05 outside and 0.05 inside the thick_arc)
    thick_arc.stroke(Stroke::Full {
        color: rgb(0, 0, 0),
        width: 0.1,
    
    });

    // chooses a rotation of PI/3 radians in the trigonometric direction
    thick_arc.rotate(Rotation2::new(PI/3_f32.to_radians()));
    
    // prints in svg version
    fs::write("./out/yellow_thick_arc.svg", Shape::from(thick_arc).to_svg().unwrap()).unwrap();
}