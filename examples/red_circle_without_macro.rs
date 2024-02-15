use std::fs;

use dessin::prelude::*;
use dessin_svg::ToSVG;

fn main() {
    // creates a circle with radius of 11
    let circle = Circle::default().with_radius(11.);

    let mut circle = Style::new(circle);

    // paints the inside of the circle in red
    circle.fill(Fill::Color(rgb(255, 0, 0)));

    // creates a grey margin of 0.2 (0.1 outside and 0.1 inside the circle)
    circle.stroke(Stroke::Full {
        color: rgb(96, 96, 96),
        width: 0.2,
    });

    let circle = Style::new(circle).with_fill(Fill::Color(rgb(255, 0, 0))).with_stroke(Stroke::Full {
        color: rgb(96, 96, 96),
        width: 0.2,
    });
    
    //prints in svg version
    fs::write("./out/red_circle.svg", Shape::from(circle).to_svg().unwrap()).unwrap();
}
