use dessin::prelude::*;
use project_root::get_project_root;
use std::fs;

fn main() {
    // let circle = dessin2!(Circle!(radius = 0.01));

    let rectangle_1 = dessin2!(Rectangle!(
        width = 3.,
        height = 2.,
        translate = [1., 0.],
        fill = rgb(255, 0, 0)
    ));

    let base = dessin2!(Padding<Style<Rectangle>>(

        shape = rectangle_1.clone(),
        padding_left = 1.5,
        padding_right = 1.,
        padding_top = 0.8,
        padding_bottom = 1.,

    ));

    let rectangle_2 = dessin2!(Rectangle!(
        width = 5.5,
        height = 3.8,
        stroke = Stroke::Full {
            color: rgb(0, 150, 0),
            width: 0.1
        },
        translate = [0.75, -0.1]
    ));

    // let circle = Shape::from(circle);
    let base = Shape::from(base);
    let rectangle_1 = Shape::from(rectangle_1);
    let rectangle_2 = Shape::from(rectangle_2);

    // creates a group
    let mut group = Group::default();

    group.shapes = vec![];

    // group.shapes.push(circle);
    group.shapes.push(base);
    group.shapes.push(rectangle_1);
    group.shapes.push(rectangle_2);

    // prints in svg version
    fs::write(
        get_project_root().unwrap().join("examples/out/padding.svg"),
        dessin_svg::to_string(&Shape::Group(group)).unwrap(),
    )
    .unwrap();
}
