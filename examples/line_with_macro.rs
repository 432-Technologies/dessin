use std::fs;

use dessin::{nalgebra::Point2, prelude::*};
use dessin_svg::ToSVG;

fn main() {
    let line: Shape = dessin2!([
        // creates a little circle as reference for a movement
        Circle!(radius = 0.1),
        // creates a line
        Line!(
            // chooses the starting point of the line
            from = Point2::new(1., 0.),
            // chooses the ending point of the line
            to = Point2::new(12., 5.2),
            // not needed here
            fill = rgb(255, 100, 100),
            stroke = Stroke::Full {
                color: rgb(255, 100, 100),
                width: 0.05
            },
            translate = [5., 1.]
        )
    ]);

    // prints in svg version
    fs::write("./out/line.svg", line.to_svg().unwrap()).unwrap();
}
