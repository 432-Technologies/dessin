use std::{f32::consts::PI, fs};

use dessin::{nalgebra::Rotation2, prelude::*};
use project_root::get_project_root;

fn main() {
    let arc: Shape = dessin2!([Arc!(
        start_angle = 0.,
        end_angle = PI / 4.,
        // creates a black pointing margin with a width of 0.1
        stroke = Stroke::Full {
            color: rgb(0, 50, 75),
            width: 0.1
        },
        // chooses a rotation of -10 radians in the trigonometric direction
        rotate = Rotation2::new(-10_f32.to_radians())
    ),]);

    // prints in svg version
    fs::write(
        get_project_root().unwrap().join("examples/out/arc.svg"),
        dessin_svg::to_string(&arc).unwrap(),
    )
    .unwrap();
}
