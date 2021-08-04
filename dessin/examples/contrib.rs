use dessin::{contrib::*, shape::*, *};
use std::{f32::consts::PI, fs::write};

pub fn main() {
    let ks: Keypoints = Arc::new()
        .with_radius(100.)
        .with_start_angle(Angle::Radians(PI / 4.))
        .with_end_angle(Angle::Radians(PI / 3.))
        .into();

    // let ks: Keypoints = QuarterCircle::new(Quarter::BottomRight)
    //     .at(vec2(100., 100.))
    //     .with_radius(100.)
    //     .into();

    let f = format!(
        r#"
<html>
    <body>
        <svg height="300" viewBox="-200 -200 400 400">
            {}
        </svg>
    </body>
</html>"#,
        (0..ks.0.len())
            .step_by(4)
            .map(|idx| {
                let k1 = match ks.0[idx] {
                    Keypoint::Point(p) | Keypoint::Bezier(p) => p,
                };
                let k2 = match ks.0[idx + 1] {
                    Keypoint::Point(p) | Keypoint::Bezier(p) => p,
                };
                let k3 = match ks.0[idx + 2] {
                    Keypoint::Point(p) | Keypoint::Bezier(p) => p,
                };
                let k4 = match ks.0[idx + 3] {
                    Keypoint::Point(p) | Keypoint::Bezier(p) => p,
                };

                format!(
                    r#"<path d="M{},{} C{},{} {},{} {},{}"/>"#,
                    k1.x, k1.y, k2.x, k2.y, k3.x, k3.y, k4.x, k4.y
                )
            })
            .collect::<String>()
    );

    write("./a.html", f).unwrap();
}
