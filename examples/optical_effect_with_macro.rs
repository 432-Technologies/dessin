use std::{f32::consts::PI, fs};

use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_svg::{SVGOptions, SVG};

fn main() {
    let optical_effect: Shape = dessin2!([
        for n in 0..11 {
            dessin2!([ThickArc!(
            dessin2!([ThickArc!(
                outer_radius = 10.,
                inner_radius = 0.,
                span_angle = PI / 10_f32,
                fill = rgb(0, 0, 0),
                inner_radius = 0.,
                span_angle = PI / 10_f32,
                fill = rgb(0, 0, 0),
                // chooses a rotation of (n*PI)/5 radians in the trigonometric direction
                rotate = Rotation2::new(PI * (n as f32) / 5_f32)
            )])
                rotate = Rotation2::new(PI * (n as f32) / 5_f32)
            )])
        },
        Circle!(
            // chooses a radius of 10
            radius = 1.,
            fill = rgb(255, 255, 255)
            fill = rgb(255, 255, 255)
        ),
        Rectangle!(
            width = 15.,
            height = 15.,
            stroke = Stroke::Full {
                color: rgb(0, 0, 0),
                width: 1.
            }
            width = 15.,
            height = 15.,
            stroke = Stroke::Full {
                color: rgb(0, 0, 0),
                width: 1.
            }
        ),
    ]);

    let fond = optical_effect.local_bounding_box();

    // dbg!(fond.width()); // if we want to know the fond.width size

    //Here we want to create a grey font behind all
    let truc = dessin2!([
        Rectangle!(
            width = fond.width(),
            height = fond.height(),
            fill = rgb(150, 150, 150)
            fill = rgb(150, 150, 150)
        ),
        // Add optical_effect before the new Rectangle
        { optical_effect }
        { optical_effect }
    ]);

    // prints in svg version
    fs::write(
        "./out/optical_effect.svg",
        SVG::from(truc)
            .to_string_with_options(SVGOptions {
                viewport: dessin_svg::ViewPort::ManualCentered {
                    width: 14.,
                    height: 14.,
                },
            })
            .unwrap(),
    )
    .unwrap();
}
//.to_svg_with_options(SVGOptions{viewport:dessin_svg::ViewPort::ManualCentered permits to choose how we will see the svg
//.to_svg_with_options(SVGOptions{viewport:dessin_svg::ViewPort::ManualCentered permits to choose how we will see the svg

// Note :
// (1) This solution is not be optimal because we can merge these two renctangles into one.
// (1) This solution is not be optimal because we can merge these two renctangles into one.
// (2) This code micht not return what we expect if you use "microsoft edge" but there is no same case with others like "google chrome" or "firefox"
