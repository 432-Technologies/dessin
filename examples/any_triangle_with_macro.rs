use std::fs;

use dessin::prelude::*;
use dessin_svg::ToSVG;

use dessin::nalgebra::Rotation2;

fn main() {
    let triangle: Shape = dessin!([
         Triangle: #(

         //chooses the size of the first side of the triangle which is on the x axis without rotation : 4
         width_x_axis={4.}

         //chooses the size of the second side of the triangle : 12
         size_axis_angle={12.}

         // chooses an angle of 0.5
         angle={0.5}

         // paints the inside of the triangle in bright pink
         fill={rgb(255,20,147)}

         // creates a black pointing margin with a width of 0.1 (0.05 outside and the same inside the triangle), a length of 0.2 and
         // a space of 0.1 between each of them
         stroke={Stroke::Dashed { color: rgb(0, 0, 0), width: 0.1, on: 0.2, off: 0.1}}

         // chooses a rotation of -10 radians in the trigonometric direction
         rotate={Rotation2::new(-10_f32.to_radians())}
     ),
         //here, the hypotenuse should be 5

    ]);

    // prints in svg version
    fs::write("./out/any_triangle.svg", triangle.to_svg().unwrap()).unwrap();
}
