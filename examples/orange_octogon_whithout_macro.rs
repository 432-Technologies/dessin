//Attention ! It is the same way to make all polygons, you just have to replace : "Octogon" by "Polygon< the number of side you want >"

use std::fs;

use dessin::{contrib::polygons::Octogon, prelude::*};
use dessin_svg::ToSVG;

use dessin::nalgebra::Rotation2;

fn main(){

    // creates a octogon
    let octogon = Triangle_test::default();

    let mut octogon = Style::new(octogon);

    // paints the inside of the octogon in bright orange
    octogon.fill(Fill::Color(rgb(255, 191, 0)));

    // We decide to not use stroke but it is possible
    // octogon.stroke(Stroke::Dashed {
    //     color: rgb(0, 0, 0),
    //     width: 0.1,
    //     on: 0.2,
    //     off: 0.1
    // });

    // chooses a rotation of -10 radians in the trigonometric direction
    octogon.rotate(Rotation2::new(-10_f32.to_radians()));

   // prints in svg version
   fs::write("./out/orange_octogon.svg", Shape::from(octogon).to_svg().unwrap()).unwrap();
}