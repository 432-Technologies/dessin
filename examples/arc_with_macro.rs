use std::{f32::consts::PI, fs};

use dessin::prelude::*;
use dessin_svg::ToSVG;

use dessin::nalgebra::Rotation2;

fn main() {
    let arc: Shape = dessin!([
        Arc: #(

        start_angle={0.}

        end_angle={PI/4.}

        // creates a black pointing margin with a width of 0.1
        stroke={Stroke::Full { color: rgb(0, 50, 75), width: 0.1}}

        // chooses a rotation of -10 radians in the trigonometric direction
        rotate={Rotation2::new(-10_f32.to_radians())}
    ),
    ]);

    // prints in svg version
    fs::write("./out/arc.svg", arc.to_svg().unwrap()).unwrap();
}
