use dessin::prelude::*;

fn main() {
    let a = 2;
    let c = dessin2!(Circle(radius = 2. + 3., fill = Color::RED) > (scale = [2., 2.]));

    dessin2!(
        [
            Curve(closed, fill = Color::RED,),
            var[c](scale = [2., 2.]),
            for _a in [1, 2] {
                dessin2!()
            },
            if a == 2 {
            } else {
            },
        ] > ()
    );
}
