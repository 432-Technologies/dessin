use std::{fs::File, io::BufWriter};

use dessin::prelude::*;
use dessin_pdf::ToPDF;
use nalgebra::{Rotation2, Translation2};
use printpdf;

fn main() {
    let circle = dessin!(Circle: (
        radius={ 10. }
    ));

    let rosace = dessin!(for x in {0..6}: {
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
            for x in {0..10}: {
                dessin!(Circle: #(
                    stroke={ Stroke::Full { color: Color::BLUE, width: 0.1 + 0.9_f32.powf(x as f32) } }
                    radius={ 20. + 2. * x as f32 }
                ))
            }
        }
    ]);

    let (doc, page, layer) =
        printpdf::PdfDocument::new("Rosace", printpdf::Mm(100.), printpdf::Mm(100.), "layer");
    let current_layer = doc.get_page(page).get_layer(layer);

    Shape::from(scene)
        .draw_on_layer(&current_layer, 100., 100.)
        .unwrap();

    let f = File::create("./rosace.pdf").unwrap();
    let mut b = BufWriter::new(f);

    doc.save(&mut b).unwrap();
}
