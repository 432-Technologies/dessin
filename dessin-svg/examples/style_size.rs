use dessin::prelude::*;
use dessin_svg::ToSVG;
use nalgebra::{Rotation2, Translation2};

fn main() {
    let scene = dessin!(group: [
        { Line: #(
            stroke={(Color::RED, 5.)}
            from={[0., 0.]}
            to={[20., 0.]}
        ) }
    ]);

    let res = Shape::from(scene)
        .to_svg_with_options(dessin_svg::SVGOptions {
            size: Some((50., 50.)),
        })
        .unwrap();

    println!("{res}")
}
