use std::{fs, time::Duration};

use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_image::*;
use dessin_svg::*;

fn main() {
    let test_img = dessin!(polygons::Triangle: #(
        fill={Color::BLUE}
    ) -> (
        scale={[50., 50.]}
    ))
    .rasterize()
    .unwrap();

    let triangle = Default::default();

    let frame = dessin!([
        Circle: #(
            stroke={(Color::RED, 0.5)}
            radius={5.}
        ),
        Dynamic<Image>: (
            _ref={&triangle}
            image={test_img}
            scale={[3., 3.]}
        ),
    ] -> (
        scale={[100., 100.]}
    ));

    loop {
        let final_image = frame.to_svg().unwrap();
        fs::write("test.svg", final_image).unwrap();

        std::thread::sleep(Duration::from_millis(100));
        let mut t = triangle.write().unwrap();
        t.rotate(Rotation2::new(0.3));
    }
}
