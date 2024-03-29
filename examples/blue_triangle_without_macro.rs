use std::fs;

use dessin::{
    nalgebra::{Rotation2, Scale2},
    prelude::{polygons::Triangle, *},
};
use project_root::get_project_root;

fn main() {
    // creates a rectangle with a width of 11 and a height of 6
    let triangle = Triangle::default();

    let mut triangle = Style::new(triangle);

    triangle.scale(Scale2::new(5., 5.));

    // paints the inside of the triangle in blue
    triangle.fill(Fill::Color(rgb(0, 0, 255)));

    // creates a black margin of 0.2 (0.05 outside and 0.05 inside the triangle)
    triangle.stroke(Stroke::Full {
        color: rgb(0, 0, 0),
        width: 0.1,
    });

    //chooses a rotation of 0 radians in the trigonometric direction
    triangle.rotate(Rotation2::new(0_f32.to_radians()));

    //prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/blue_triangle.svg"),
        dessin_svg::to_string(&triangle.into()).unwrap(),
    )
    .unwrap();
}
