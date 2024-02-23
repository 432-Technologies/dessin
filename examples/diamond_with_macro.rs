use std::fs;

use dessin::prelude::*;
use dessin_svg::ToSVG;

use dessin::nalgebra::Rotation2;

fn main() {
    let diamond: Shape = dessin!([

         // here we use the circle as a point to have a reference to use when moving the diamond
         Circle: #(
             radius={0.1}
         ),

         Diamond: #(

         // chooses a width of 4 for following the x axis
         width={4.}

         // chooses a size of 5 between the origin and the diamond top apex following the y axis
         height_top={5.}

         // chooses a size of 3 between the origin and the diamond bottom apex following the y axis
         height_bottom={3.}

         // paints the inside of the diamond in diamond color
         fill={rgb(185,242,255)}

         // creates a black margin with a width of 0.1 (0.05 outside and the same inside the diamond)
         stroke={Stroke::Full { color: rgb(0, 0, 0), width: 0.1}}

         // chooses a rotation of -10 radians in the trigonometric direction
         rotate={Rotation2::new(-10_f32.to_radians())}

         // moves of 15 following the x axis and 5 following the y axis
         translate={[15.,5.]}
     ),

    ]);

    // prints in svg version
    fs::write("./out/diamond.svg", diamond.to_svg().unwrap()).unwrap();
}
