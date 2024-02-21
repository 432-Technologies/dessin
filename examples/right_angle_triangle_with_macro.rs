use std::{f32::consts::PI, fs};

use dessin::prelude::*;
use dessin_svg::ToSVG;

use dessin::nalgebra::Rotation2;

fn main() {
    let triangle: Shape = dessin!([
         Triangle_test: #(

         //chooses the size of the first side of the triangle which is on the x axis without rotation : 3
         width_x_axis={3.}

         //chooses the size of the second side of the triangle : 4
         size_axis_angle={4.}

         // chooses a right angle in radiant which is : PI/2 or 3PI/2
         angle={PI/2.}

         // paints the inside of the triangle in green
         fill={rgb(0,0,100)}

         // creates a black pointing margin with a width of 0.1 (0.05 outside and the same inside the triangle), a length of 0.2 and
         // a space of 0.1 between each of them
         stroke={Stroke::Dashed { color: rgb(0, 0, 0), width: 0.1, on: 0.2, off: 0.1}}

         // chooses a rotation of 0 radians in the trigonometric direction
         rotate={Rotation2::new(0_f32.to_radians())}
     ),
         //here, the hypotenuse should be 5

    ]);

    // prints in svg version
    fs::write("./out/right_angle_triangle.svg", triangle.to_svg().unwrap()).unwrap();
}
