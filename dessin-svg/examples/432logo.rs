use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI};

use dessin::prelude::*;
use dessin_svg::ToSVG;
use nalgebra::{Point2, Rotation2, Translation2};

const C: Color = rbg(0x3b, 0x54, 0x85);
fn c(a: u8) -> Color {
    rgba(0x3b, 0x54, 0x85, a)
}

#[derive(Default)]
struct InnerBubbleRing;
impl From<InnerBubbleRing> for Shape {
    fn from(_: InnerBubbleRing) -> Self {
        let ring_strip = dessin!(
            group: ( translate={Translation2::new(14., 0.)} ) [
                {
                    Circle: #(
                        stroke={Stroke::Full { color: c(200), width: 0.1 }}
                        radius={ 1. }
                    )
                }
                {
                    Circle: #(
                        stroke={Stroke::Full { color: c(150), width: 0.1 }}
                        radius={ 0.5 }
                        translate={Translation2::new(2., 0.)}
                    )
                }
                {
                    Circle: #(
                        stroke={Stroke::Full { color: c(100), width: 0.1 }}
                        radius={ 0.25 }
                        translate={Translation2::new(3.2, 0.)}
                    )
                }
            ]
        );

        let angle = PI / 14_f32;
        dessin!(for n in {0..28}: {
            dessin!(var |ring_strip|: ( rotate={Rotation2::new(n as f32 * angle)} ))
        })
    }
}

#[derive(Default)]
struct BinaryRing(pub f32);
impl BinaryRing {
    #[inline]
    pub fn radius(&mut self, radius: f32) -> &mut Self {
        self.0 = radius;
        self
    }
}
impl From<BinaryRing> for Shape {
    fn from(BinaryRing(radius): BinaryRing) -> Self {
        const T: &str = "10001011101001011000101110001010010101110100111010010101110010101001110010100101011010100101111101001011011100001110001110001011100000101011100101000101110100101100010111000101001010111010011101001010101100010111000101001010111010011101001010111001010100111001010010101101010010111110100101101110000111000111000101110000010101110010100010111010010110001011100010100101011101001110100101011100101010011100101001010110101001011111010010110111000011100011100010111000001010111001010001011101001011000101110001010010101110100111010010101110010101001110010100101011010100101111101001011011100001110001110001011100000101011100101000101110100101100010111000101001010111010011101001010111001010100111001010010101101010010111110100101101110000111000111000101110000010101110010";
        dessin!(Text: #(
            text={T}
            // on_curve={Rectangle::default().with_width(30.).with_height(30.).as_curve()}
            // on_curve={Curve::default().with_then(Point2::new(0., 0.)).with_then(Point2::new(100., 100.))}
            on_curve={Circle::default().with_radius(radius).into()}
            font_size={1.}
            fill={Fill::Color(C)}
        ))
        .into()
        // Style::new(Circle::default().with_radius(radius).as_curve())
        //     .with_fill(Fill::Color(rgba(128, 0, 0, 128)))
        //     .into()
    }
}

#[derive(Default)]
struct TimerRing;
impl From<TimerRing> for Shape {
    fn from(_: TimerRing) -> Self {
        let long_line = dessin!(Line: (
            from={Point2::new(36., 0.)}
            to={Point2::new(28., 0.)}
        ));
        let short_line = dessin!(Line: (
            from={Point2::new(36., 0.)}
            to={Point2::new(32., 0.)}
            rotate={Rotation2::new(FRAC_PI_8)}
        ));
        let small_line = dessin!(Line: (
            from={Point2::new(36., 0.)}
            to={Point2::new(35., 0.)}
        ));

        dessin!(group: #( stroke={Stroke::Full { color: C, width: 0.2 }} ) [
            { Circle: ( radius={36.} ) }
            { for x in {0..8}: {
                    dessin!(var |long_line|: (
                        rotate={Rotation2::new(x as f32 * FRAC_PI_4)}
                    ))
            } }
            { for x in {0..8}: {
                    dessin!(var |short_line|: (
                        rotate={Rotation2::new(x as f32 * FRAC_PI_4)}
                    ))
            } }
            { for x in {0..160}: {
                    dessin!(var |small_line|: (
                        rotate={Rotation2::new(x as f32 * PI / 160.)}
                    ))
            } }
        ])
        .into()
    }
}

#[derive(Default)]
struct ThreeColoredRing;
impl From<ThreeColoredRing> for Shape {
    fn from(_: ThreeColoredRing) -> Self {
        dessin!(group: [
            {
                Circle: #(
                    stroke={Stroke::Full { color: rbg(0x96, 0x96, 0x96), width: 0.2 }}
                    radius={40.}
                )
            }
            {
                Circle: #(
                    stroke={Stroke::Full { color: rbg(0x2e, 0x2e, 0x2e), width: 0.2 }}
                    radius={42.}
                )
            }
            {
                Circle: #(
                    stroke={Stroke::Full { color: C, width: 0.2 }}
                    radius={44.}
                )
            }
        ])
    }
}

#[derive(Default)]
struct Squares;
impl From<Squares> for Shape {
    fn from(_: Squares) -> Self {
        let square_line = dessin!(
            group: (
                translate={Translation2::new(50., 0.)}
            ) [
                {
                    Rectangle: #(
                        stroke={Stroke::Full { color: C, width: 0.1 }}
                        width={2.5}
                        height={2.5}
                    )
                }
                {
                    Rectangle: #(
                        stroke={Stroke::Full { color: c(200), width: 0.1 }}
                        width={1.8}
                        height={1.8}
                        translate={Translation2::new(2.8, 0.)}
                    )
                }
                {
                    Rectangle: #(
                        stroke={Stroke::Full { color: c(150), width: 0.1 }}
                        width={1.2}
                        height={1.2}
                        translate={Translation2::new(4.8, 0.)}
                    )
                }
                {
                    Rectangle: #(
                        stroke={Stroke::Full { color: c(100), width: 0.1 }}
                        width={0.8}
                        height={0.8}
                        translate={Translation2::new(6.2, 0.)}
                    )
                }
                {
                    Rectangle: #(
                        stroke={Stroke::Full { color: c(50), width: 0.1 }}
                        width={0.4}
                        height={0.4}
                        translate={Translation2::new(7.2, 0.)}
                    )
                }
                {
                    Rectangle: #(
                        stroke={Stroke::Full { color: c(25), width: 0.1 }}
                        width={0.2}
                        height={0.2}
                        translate={Translation2::new(7.8, 0.)}
                    )
                }
            ]
        );

        let angle = 150_f32.to_radians() / 36.;
        let quarter = dessin!(
            for x in {0..36}: {
                dessin!(
                    var |square_line|: (
                        rotate={Rotation2::new(x as f32 * angle)}
                    )
                )
            }
        );

        dessin!(group: [
            {
                var |quarter|: ( rotate={Rotation2::new(15_f32.to_radians())} )
            }
            {
                use |quarter|: ( rotate={Rotation2::new(195_f32.to_radians())} )
            }
        ])
    }
}

fn main() {
    let logo = dessin!(
        group: [
            {
                InnerBubbleRing: ()
            }
            {
                BinaryRing: (
                    radius={10.}
                )
            }
            {
                TimerRing: ()
            }
            {
                ThreeColoredRing: ()
            }
            {
                Squares: ()
            }
            {
                BinaryRing: (
                    radius={30.}
                )
            }
            {
                Circle: #(
                    stroke={Stroke::Full { color: rbg(0x96, 0x96, 0x96), width: 0.2 }}
                    radius={70.}
                )
            }
        ]
    )
    .to_svg()
    .unwrap();
    println!("{logo}");
}
