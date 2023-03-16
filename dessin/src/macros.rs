#[macro_export]
macro_rules! dessin {
	() => {$crate::Shape::default()};
	(do {$range:expr}: { $x:ident => $f:expr}) => {
		{
			#[allow(unused_mut)]
			let mut shapes = vec![];

			for $x in $range {
				shapes.push($crate::Shape::from($f))
			}

			$crate::Shape::Group {
				local_transform: ::nalgebra::Transform2::default(),
				shapes,
			}
		}
	};
	(var |$v:ident|: ($($fn_name:ident={$value:expr})*)) => {
		{
			#[allow(unused_mut)]
			let mut shape = $v.clone();
			$(shape.$fn_name($value);)*
			shape
		}
	};
	(var |$v:ident|: #($($fn_name:ident={$value:expr})*)) => {
		{
			#[allow(unused_mut)]
			let mut shape = $crate::Style::new($v.clone());
			$(shape.$fn_name($value);)*
			shape
		}
	};
	(style: ($($fn_name:ident={$value:expr})*) { $($rest:tt)* }) => {
		{
			#[allow(unused_mut)]
			let mut style = $crate::Style::new(
				dessin! ($($rest)*)
			);

			$(style.$fn_name($value);)*

			style
		}
	};
	(group: ($($fn_name:ident={$value:expr})*) [$( { $($rest:tt)* } )*]) => {
		{
			#[allow(unused_mut)]
			let mut acc = Vec::new();

			$(
				acc.push(
					$crate::Shape::from(
						dessin! ($($rest)*)
					)
				);
			)*

			#[allow(unused_mut)]
			let mut group = $crate::Shape::Group {
				local_transform: ::nalgebra::Transform2::default(),
				shapes: acc,
			};
			$(group.$fn_name($value);)*

			group
		}
	};
	(group: #($($fn_name:ident={$value:expr})*) [$( { $($rest:tt)* } )*]) => {
		{
			#[allow(unused_mut)]
			let mut acc = Vec::new();

			$(
				acc.push(
					$crate::Shape::from(
						dessin! ($($rest)*)
					)
				);
			)*

			#[allow(unused_mut)]
			let mut group = $crate::Style::new($crate::Shape::Group {
				local_transform: ::nalgebra::Transform2::default(),
				shapes: acc,
			});
			$(group.$fn_name($value);)*

			group
		}
	};
	($shape:ty: #($($fn_name:ident $( ={$value:expr} )? )*)) => {
        {
			#[allow(unused_mut)]
			let mut shape = $crate::Style::<$shape>::default();
			$(shape.$fn_name($($value)?);)*
			shape
		}
    };
    ($shape:ty: ($($fn_name:ident $( ={$value:expr} )? )*)) => {
        {
			#[allow(unused_mut)]
			let mut shape = <$shape>::default();
			$(shape.$fn_name($($value)?);)*
			shape
		}
    };
}

// #[cfg(test)]
fn test() {
    use crate::prelude::*;
    use nalgebra::{Rotation2, Translation2};

    let ellipse = dessin! {
        style: (
            stroke={crate::Stroke::Full { color: Color::RED, width: 1. }}
        ) {
            Ellipse: (
                translate={ Translation2::new(0., 0.) }
                rotate={ Rotation2::new(0.) }
            )
        }
    };

    let g = dessin! {
        group: (
            translate={ Translation2::new(0., 0.) }
            rotate={ Rotation2::new(0.) }
        ) [
            {
                Ellipse: #(
                    stroke={crate::Stroke::Full { color: Color::RED, width: 1. }}
                    semi_major_axis={10.}
                    semi_minor_axis={5.}
                )
            }
            {
                var |ellipse|: (
                    semi_major_axis={10.}
                )
            }
        ]
    };
}
