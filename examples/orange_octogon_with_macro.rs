//Attention ! It is the same way to make all polygons, you just have to replace : "Octogon" by "Polygon< the number of side you want >"

use dessin::{contrib::polygons::Octogon, nalgebra::Rotation2, prelude::*};
use dessin_svg::ToSVG;
use project_root::get_project_root;
use std::fs;

fn main() {
    let octogon: Shape = dessin!(
         Octogon: #(

         // paints the inside of the octogon in bright orange
         fill={rgb(255,191,0)}

         // We decide to not use stroke but it is possible
         // stroke={Stroke::Dashed { color: rgb(0, 0, 0), width: 0.1, on: 0.2, off: 0.1}}

         // chooses a rotation of -2 radians in the trigonometric direction
         rotate={Rotation2::new(-2_f32.to_radians())}
     ) -> ()
         // here, the hypotenuse should be 5

    );

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/orange_octogon.svg"),
        octogon.to_svg().unwrap(),
    )
    .unwrap();
}
