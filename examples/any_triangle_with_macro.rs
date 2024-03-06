use dessin::{nalgebra::Rotation2, prelude::*};
use palette::named;
use palette::{LinSrgb, Srgb};
use project_root::get_project_root;
use std::fs;

fn main() {
    let triangle: Shape = dessin2!([
        Triangle!(
            //chooses the size of the first side of the triangle which is on the x axis without rotation : 4
            width_x_axis = 4.,
            //chooses the size of the second side of the triangle : 12
            size_axis_angle = 12.,
            // chooses an angle of 0.5
            angle = 0.5,
            // paints the inside of the triangle in bright pink
            fill = rgb(255, 20, 147),
            // creates a black pointing margin with a width of 0.1 (0.05 outside and the same inside the triangle), a length of 0.2 and
            // a space of 0.1 between each of them
            stroke = Stroke::Dashed {
                color: rgb(0, 0, 0),
                width: 0.1,
                on: 0.2,
                off: 0.1
            },
            // chooses a rotation of -10 radians in the trigonometric direction
            rotate = Rotation2::new(-10_f32.to_radians())
        ),
        //here, the hypotenuse should be 5
    ]);

    // prints in svg version
    fs::write(
        get_project_root()
            .unwrap()
            .join("examples/out/any_triangle.svg"),
        dessin_svg::to_string(&triangle).unwrap(),
    )
    .unwrap();

    let orangeish = Srgb::new(1.0, 0.6, 0.0).into_linear();
    let blueish = Srgb::new(0.0, 0.2, 1.0).into_linear();
    // let result = do_something(orangeish, blueish);
}
