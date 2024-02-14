use std::fs;

use dessin::prelude::*;
use dessin_svg::ToSVG;
fn main(){

    let circle:Shape = Circle::default().with_radius(10.).into();
    let mut circle = Style::new(circle);
    circle.fill(Fill::Color(rgb(255, 0, 0)));

    let circle: Shape = dessin!([
        Circle: #(
        radius={10.}
        fill={rgb(255,0,0)}
    ),
        
        Text: #(
            text={"test"}
            fill={rgb(0,0,0)}
        )
   ] );

   fs::write("./target/432.svg", circle.to_svg().unwrap()).unwrap();  //print in svg version
}

fn Exemple_d_un_cercle_rouge(){
    let circle: Shape = dessin!([
        Circle: #(
            radius={2.}
            fill={rgb(255,240,240)}
            stroke={Stroke::Full { color: rgb(0x96, 0x96, 0x96), width: 0.2 }}
        ),

        Text: #(
            text={"Noze in coming"}
            fill={rgb(0,0,0)}
        )
    ]);
    
    fs::write("./target/432.svg", circle.to_svg().unwrap()).unwrap();  //print in svg version
}

