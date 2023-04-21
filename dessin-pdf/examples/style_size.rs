use std::fs;

use dessin::prelude::*;
use dessin_pdf::ToPDF;
use nalgebra::{Rotation2, Translation2};

fn main() {
    let scene = dessin!(group: [
        { Line: #(
            stroke={(Color::RED, 5.)}
            from={[0., 0.]}
            to={[20., 0.]}
        ) }
    ]);

    let res = Shape::from(scene).to_pdf_bytes(50., 50.).unwrap();

    fs::write("./a.pdf", res).unwrap();
}
