use std::fs;

use dessin::prelude::*;
use dessin_svg::ToSVG;

use dessin::nalgebra::Rotation2;

fn main() {
    let circle: Shape = dessin!([
         Circle: #(

         // chooses a radius of 11
         radius={11.}  //11. is like a proportion of the box allowed

         // paints the inside of the circle in red
         fill={rgb(255,0,0)}

         // creates a grey margin of 0.2 (0.1 outside and 0.1 inside the circle)
         stroke={Stroke::Full { color: rgb(0x96, 0x96, 0x96), width: 0.2 }}

         rotate={Rotation2::new(0_f32.to_radians())}  //not visible yet but it's possible to see it in some conditions
     ),

    ]);

    // prints in svg version
    fs::write("./out/red_circle.svg", circle.to_svg().unwrap()).unwrap();
}
