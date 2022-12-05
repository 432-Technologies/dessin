mod shapes;

use dessin::{vec2, Drawing, Vec2};
use printpdf::{IndirectFontRef, Mm, PdfDocument, PdfDocumentReference, PdfLayerReference};
use std::{error::Error, io::BufWriter};

const ARIAL_REGULAR: &[u8] = include_bytes!("Arial.ttf");
const ARIAL_BOLD: &[u8] = include_bytes!("Arial Bold.ttf");
const ARIAL_ITALIC: &[u8] = include_bytes!("Arial Italic.ttf");
const ARIAL_BOLD_ITALIC: &[u8] = include_bytes!("Arial Bold Italic.ttf");

const DPI: f64 = 96.;

pub struct PDF(pub PdfDocumentReference);
impl PDF {
    pub fn into_bytes(self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buff = BufWriter::new(vec![]);
        self.0.save(&mut buff)?;
        Ok(buff.into_inner()?)
    }
}

pub trait ToPDF {
    fn to_pdf(&self) -> Result<PDF, Box<dyn Error>>;
}

impl ToPDF for Drawing {
    fn to_pdf(&self) -> Result<PDF, Box<dyn Error>> {
        let Vec2 {
            x: width,
            y: height,
        } = self.canvas_size();

        let (doc, page1, layer1) =
            PdfDocument::new("PDF", Mm(width as f64), Mm(height as f64), "Layer1");

        let font = doc.add_external_font(ARIAL_REGULAR)?;
        let current_layer = doc.get_page(page1).get_layer(layer1);

        let offset = vec2(self.canvas_size().x / 2., self.canvas_size().x / 2.);

        self.shapes()
            .iter()
            .map(|v| v.to_pdf_part(DPI, offset, &font, &current_layer))
            .collect::<Result<(), Box<dyn std::error::Error>>>()?;

        Ok(PDF(doc))
    }
}

trait ToPDFPart {
    fn to_pdf_part(
        &self,
        dpi: f64,
        offset: Vec2,
        font: &IndirectFontRef,
        layer: &PdfLayerReference,
    ) -> Result<(), Box<dyn Error>>;
}
