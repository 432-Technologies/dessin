use nalgebra::Transform2;

use crate::prelude::*;

/// Display children on top of one another
///
/// `of={ Into<Shape> }`
/// ``
#[derive(Debug, Default, Clone, Shape)]
pub struct VerticalLayout {
    #[local_transform]
    pub local_transform: Transform2<f32>,
    pub shapes: Vec<Shape>,
    #[shape(skip)]
    pub start_bottom: bool,
    pub gap: f32,
    pub metadata: Vec<(String, String)>,
}
impl VerticalLayout {
    ///
    /// In the case of a Group, local_transform is discarded as the shapes will be rearranged in a vertical layout
    #[inline]
    pub fn of<T: Into<Shape>>(&mut self, shape: T) -> &mut Self {
        match shape.into() {
            Shape::Group {
                local_transform: _,
                shapes,
                metadata,
            } => {
                self.metadata.extend(metadata);
                self.shapes.extend(shapes);
            }
            x => {
                self.shapes.push(x);
            }
        }

        self
    }
    #[inline]
    pub fn with<T: Into<Shape>>(mut self, shape: T) -> Self {
        self.of(shape);
        self
    }

    #[inline]
    pub fn extend<T: IntoIterator<Item = Shape>>(&mut self, shapes: T) -> &mut Self {
        self.shapes.extend(shapes);
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
}

impl From<VerticalLayout> for Shape {
    fn from(
        VerticalLayout {
            local_transform,
            shapes,
            start_bottom,
            gap,
            metadata,
        }: VerticalLayout,
    ) -> Self {
        let direction = if start_bottom { 1. } else { -1. };
        let mut y = 0.;

        dessin!(for shape in (shapes) {
            let mut shape = shape;

            let bb = shape
            	.local_bounding_box()
            	.map(BoundingBox::<UnParticular>::into_straight)
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
    use assert_float_eq::*;
    use nalgebra::Point2;

    #[test]
    fn base_layout() {
        let layout = dessin!(VerticalLayout: (
            of={dessin!([
                Circle: (radius={10.}),
                Circle: (radius={10.}),
            ])}
        ));

        let Shape::Group { local_transform: _, shapes, .. } = Shape::from(layout) else {
            panic!("Not a group")
        };

        let [c1, c2] = shapes.as_slice() else {
            panic!("Expected 2 shapes, got {:#?}", shapes)
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

        let Shape::Group { local_transform: _, shapes, .. } = Shape::from(layout) else {
            panic!("Not a group")
        };

        let [c1, c2, c3] = shapes.as_slice() else {
            panic!("Expected 3 shapes, get: {:#?}", shapes)
        };

        let p = Point2::new(0., 0.);
        assert_eq!(c1.local_transform() * p, Point2::new(0., -10.));
        assert_eq!(c2.local_transform() * p, Point2::new(0., -30.));
        assert_eq!(c3.local_transform() * p, Point2::new(0., -50.));
    }

    #[test]
    fn layout_of_polygons() {
        let height_triangle = polygones::Triangle::default()
            .as_shape()
            .local_bounding_box()
            .unwrap()
            .height();

        assert_float_absolute_eq!(height_triangle, 2. * (3f32.sqrt() / 2.), 10e-5);

        let shape = dessin!([
            VerticalLayout: (
                of={dessin!(polygones::Triangle: () )}
                of={dessin!(Circle: (radius={1.}))}
            )
        ]);

        let bb = shape.local_bounding_box().unwrap();

        dbg!(shape);

        assert_eq!(bb.width(), 2.);
        assert_eq!(bb.height(), 2. + height_triangle);
    }
}
