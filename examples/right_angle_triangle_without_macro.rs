use std::{f32::consts::PI, fs};

use dessin::{nalgebra::Rotation2, prelude::*};
use project_root::get_project_root;

fn main() {
    let triangle = Triangle::default();

    let mut triangle = Style::new(triangle);

    // chooses the size of the first side of the triangle which is on the x axis without rotation : 3
    triangle.width_x_axis(3.);

    // chooses the size of the second side of the triangle : 4
    triangle.size_axis_angle(4.);

    // chooses a right angle in radiant which is : PI/2 or 3PI/2
    triangle.angle(PI / 2.);

    // paints the inside of the triangle in blue
    triangle.fill(Fill::Color(rgb(0, 0, 100)));

    // creates a black margin of 0.1 (0.05 outside and 0.05 inside the triangle)
    triangle.stroke(Stroke::Dashed {
        color: rgb(0, 0, 0),
        width: 0.1,
        on: 0.2,
        off: 0.1,
    });

    // chooses a rotation of 0 radians in the trigonometric direction
    triangle.rotate(Rotation2::new(0_f32.to_radians()));

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/right_angle_triangle.svg"),
        dessin_svg::to_string(&triangle.into()).unwrap(),
    )
    .unwrap();
}
