use std::{fs, time::Duration};

use dessin::{nalgebra::Rotation2, prelude::*};
use dessin_image::*;
use dessin_svg::*;
use project_root::get_project_root;

fn main() {
    let skip_animation = std::env::var("NO_ANIMATION") == Ok("1".to_string());

    let test_img = dessin2!(polygons::Triangle!(fill = Color::BLUE) > (scale = [50., 50.]))
        .rasterize()
        .unwrap();

    let triangle = Default::default();

    let frame = dessin2!(
        [
            Circle!(stroke = (Color::RED, 0.5), radius = 5.),
            Dynamic::<Image>(_ref = &triangle, image = test_img, scale = [3., 3.],),
        ] > (scale = [100., 100.])
    );

    loop {
        let final_image = SVG::from(frame.clone()).to_string().unwrap();
        fs::write("test.svg", final_image).unwrap();

        std::thread::sleep(Duration::from_millis(100));
        let mut t = triangle.write().unwrap();
        t.rotate(Rotation2::new(0.3));
    }
}
