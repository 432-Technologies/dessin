use std::ops::{Add, Mul};

#[inline]
pub fn lerp<X: Add<Output = X> + Mul<f32, Output = X>>(t: f32, a: X, b: X) -> X {
	a * (1. - t) + b * t
}

#[macro_export]
macro_rules! lerp {
	(
		@call $t:expr, $a:expr => $b:expr => $c:expr => $d:expr
	) => {
		crate::lerp!(@call $t, crate::lerp!(@call $t, $a => $b) => crate::lerp!(@call $t, $b => $c) => crate::lerp!(@call $t, $c => $d))
	};
	(
		@call $t:expr, $a:expr => $b:expr => $c:expr
	) => {
		crate::lerp!(@call $t, crate::lerp!(@call $t, $a => $b) => crate::lerp!(@call $t, $b => $c))
	};
	(
		@call $t:expr, $a:expr => $b:expr
	) => {
		crate::macros::lerp($t, $a, $b)
	};
	(
		@call $t:expr, $a:expr
	) => {
		$a
	};
	(
		$t:expr,
		Bezier {
			start_control: $($sc:expr)=>+,
			end_control: $($ec:expr)=>+,
			end: $($e:expr)=>+,
		}
	) => {
		dessin::prelude::Keypoint::Bezier(Bezier {
			start_control: dessin::nalgebra::Point2::from(lerp!(@call $t, $(dessin::nalgebra::Vector2::from($sc))=>+)),
			end_control: dessin::nalgebra::Point2::from(lerp!(@call $t, $(dessin::nalgebra::Vector2::from($ec))=>+)),
			end: dessin::nalgebra::Point2::from(lerp!(@call $t, $(dessin::nalgebra::Vector2::from($e))=>+)),
			..Default::default()
		})
	};
	(
		$t:expr,
		Point ($($e:expr)=>+)
	) => {
		dessin::prelude::Keypoint::Point(dessin::nalgebra::Point2::from(lerp!(@call $t, $(dessin::nalgebra::Vector2::from($e))=>+)))
	};
	(
		$t:expr, f32($($e:expr)=>+)
	) => {
		lerp!(@call $t, $($e)=>+) as f32
	};
	(
		$t:expr, $($e:expr)=>+
	) => {
		dessin::nalgebra::Point2::from(lerp!(@call $t, $(dessin::nalgebra::Vector2::from($e))=>+))
	};
}
