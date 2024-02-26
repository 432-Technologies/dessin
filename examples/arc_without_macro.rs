use std::{f32::consts::PI, fs};

use dessin::{nalgebra::Rotation2, prelude::*};
use project_root::get_project_root;

fn main() {
    // creates a triangle
    let arc = Arc::default();

    let mut arc = Style::new(arc);

    arc.start_angle(0.);

    arc.end_angle(PI / 4.);

    // creates a black margin of 0.1
    arc.stroke(Stroke::Full {
        color: rgb(0, 50, 75),
        width: 0.1, //do not worry if it'big. 0.1 is like a proportion, but here, it's the biggest
    });

    // chooses a rotation of -10 radians in the trigonometric direction
    arc.rotate(Rotation2::new(-10_f32.to_radians()));

    // prints in svg version
    fs::write(
        get_project_root().unwrap().join("examples/out/arc.svg"),
        dessin_svg::to_string(&arc.into()).unwrap(),
    )
    .unwrap();
}
