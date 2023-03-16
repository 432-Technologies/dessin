use dessin::prelude::*;
use dessin_svg::ToSVG;
use nalgebra::{Rotation2, Translation2};

fn main() {
    let circle = dessin!(Circle: (
        radius={ 10. }
    ));

    let rosace = dessin!(do {0..6}: |x| {
        dessin!(var |circle|: (
            translate={ Translation2::new(10., 0.) }
            rotate={ Rotation2::new(60_f32.to_radians() * x as f32) }
        ))
    });

    let scene = dessin!(group: [
        {
            use |rosace|: #( stroke={ Stroke::Full { color: Color::RED, width: 1. } } )
        }
        {
            do {0..10}: |x| {
                dessin!(Circle: #(
                    stroke={ Stroke::Full { color: Color::BLUE, width: 0.1 + 0.9_f32.powf(x as f32) } }
                    radius={ 20. + 2. * x as f32 }
                ))
            }
        }
    ]);

    let res = Shape::from(scene).to_svg().unwrap();

    println!("{res}")
}
