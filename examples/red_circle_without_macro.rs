use std::fs;

use dessin::prelude::*;
use dessin_svg::ToSVG;

use dessin::{
    nalgebra::{Point2, Rotation2, Translation2},
    prelude::*,
};

fn main(){

    /// create a circle with radius of 11
    let circle:Shape = Circle::default().with_radius(11.).into();

    let mut circle = Style::new(circle);

    /// paints the inside of the circle in red
    circle.fill(Fill::Color(rgb(255, 0, 0)));

    /// creates a grey margin of 0.2 (0.1 outside and 0.1 inside the circle)
    circle.stroke=(Stroke::Full { color: rgb(0x96, 0x96, 0x96), width: 0.2 });
    
    fs::write("./target/432.svg", circle.to_svg().unwrap()).unwrap();  //print in svg version
}