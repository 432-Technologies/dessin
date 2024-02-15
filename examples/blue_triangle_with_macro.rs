use std::fs;

use dessin::prelude::*;
use dessin_svg::ToSVG;

use dessin::{
    nalgebra::{Rotation2, Scale2},
};

use dessin::prelude::polygons::Triangle;

fn main(){

    let triangle: Shape = dessin!([
        Triangle: #(
        
        // chooses an equilateral triangle [(x,x) => equilateral] with a size of 5. [if you want an isosceles triangle : (x,y)]
        scale={Scale2::new(5., 5.)}

        // paints the inside of the triangle in green
        fill={rgb(0,0,255)}

        // creates a black margin of 0.2 (0.05 outside and the same inside the triangle)
        stroke={Stroke::Full { color: rgb(0, 0, 0), width: 0.1 }}

        //chooses a rotation of 0 radians in the trigonometric direction
        rotate={Rotation2::new(0_f32.to_radians())}  
    ),

   ]);

   // prints in svg version
   fs::write("./out/blue_triangle.svg", triangle.to_svg().unwrap()).unwrap();
}