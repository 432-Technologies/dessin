use nalgebra::Transform2;

use crate::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct VerticalLayout {
    pub local_transform: Transform2<f32>,
    pub shapes: Vec<Shape>,
    pub start_bottom: bool,
    pub gap: f32,
}
impl VerticalLayout {
    #[inline]
    pub fn of<T: Into<Shape>>(&mut self, shape: T) -> &mut Self {
        match shape.into() {
            Shape::Group {
                local_transform,
                shapes,
            } => {
                self.shapes.extend(shapes);
                self.local_transform *= local_transform;
            }
            x => {
                self.shapes.push(x);
            }
        }
        self
    }

    #[inline]
    pub fn extend<T: IntoIterator<Item = Shape>>(&mut self, shapes: T) -> &mut Self {
        self.shapes.extend(shapes);
        self
    }

    #[inline]
    pub fn push(&mut self, shape: Shape) -> &mut Self {
        self.shapes.push(shape);
        self
    }
    #[inline]
    pub fn with_push(mut self, shape: Shape) -> Self {
        self.push(shape);
        self
    }

    #[inline]
    pub fn start_from_bottom(&mut self) -> &mut Self {
        self.start_bottom = true;
        self
    }
    #[inline]
    pub fn with_start_from_bottom(mut self) -> Self {
        self.start_from_bottom();
        self
    }

    #[inline]
    pub fn start_from_top(&mut self) -> &mut Self {
        self.start_bottom = false;
        self
    }
    #[inline]
    pub fn with_start_from_top(mut self) -> Self {
        self.start_from_top();
        self
    }

    #[inline]
    pub fn gap(&mut self, gap: f32) -> &mut Self {
        self.gap = gap;
        self
    }
    #[inline]
    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap(gap);
        self
    }
}

impl ShapeOp for VerticalLayout {
    #[inline]
    fn transform(&mut self, transform_matrix: Transform2<f32>) -> &mut Self {
        self.local_transform = transform_matrix * self.local_transform;
        self
    }

    #[inline]
    fn local_transform(&self) -> &Transform2<f32> {
        &self.local_transform
    }
}

impl From<VerticalLayout> for Shape {
    fn from(
        VerticalLayout {
            local_transform,
            shapes,
            start_bottom,
            gap,
        }: VerticalLayout,
    ) -> Self {
        let direction = if start_bottom { 1. } else { -1. };
        let mut y = 0.;

        dessin!(for shape in (shapes) {
            let mut shape = shape;

            let bb = shape
            	.local_bounding_box()
            	.map(BoundingBox::into_straight)
            	.unwrap_or(BoundingBox::zero());

             let shape_pos_y = if start_bottom {
                 bb.bottom_right().y
             } else {
                 bb.top_right().y
             };

            shape.translate([0., direction * y - shape_pos_y]);


            y += bb.height() + gap;

            shape
        } -> ( transform={local_transform} ))
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use nalgebra::Point2;

    #[test]
    fn base_layout() {
        // let helvetica = include_bytes!("../Helvetica.otf");
        // crate::font::add_font(
        //     "Helvetica",
        //     FontGroup {
        //         regular: font::Font::OTF(helvetica.to_vec()),
        //         bold: None,
        //         italic: None,
        //         bold_italic: None,
        //     },
        // );

        let layout = dessin!(VerticalLayout: (
            of={dessin!([
                Circle: (radius={10.}),
                Circle: (radius={10.}),
            ])}
        ));

        let Shape::Group { local_transform: _, shapes } = Shape::from(layout) else {
            panic!("Not a group")
        };

        let [c1, c2] = shapes.as_slice() else {
            panic!("Expected 2 shapes")
        };

        let p = Point2::new(0., 0.);
        assert_eq!(c1.local_transform() * p, Point2::new(0., -10.));
        assert_eq!(c2.local_transform() * p, Point2::new(0., -30.));
    }

    #[test]
    fn transformed_layout() {
        let layout = dessin!(VerticalLayout: (
            of={dessin!([
                Circle: (radius={10.} translate={[0., 5.]}),
                Circle: (radius={10.} translate={[0., -10.]}),
                Circle: (radius={10.} translate={[0., 0.]}),
            ])}
        ));

        let Shape::Group { local_transform: _, shapes } = Shape::from(layout) else {
            panic!("Not a group")
        };

        let [c1, c2, c3] = shapes.as_slice() else {
            panic!("Expected 3 shapes")
        };

        let p = Point2::new(0., 0.);
        assert_eq!(c1.local_transform() * p, Point2::new(0., -10.));
        assert_eq!(c2.local_transform() * p, Point2::new(0., -30.));
        assert_eq!(c3.local_transform() * p, Point2::new(0., -50.));
    }
}
