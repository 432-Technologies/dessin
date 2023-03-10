use crate::Color;

#[macro_export]
macro_rules! dessin {
	() => {$crate::Shape::default()};
	(var |$v:ident|: ($($fn_name:ident={$value:expr})*)) => {
		{
			#[allow(unused_mut)]
			let mut shape = $v.clone();
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
			let mut group = $crate::Shape::Group(acc);
			$(group.$fn_name($value);)*

			group
		}
	};
	($shape:ty: style=($($style_fn_name:ident={$style_value:expr})*) ($($fn_name:ident={$value:expr})*)) => {
        {
			#[allow(unused_mut)]
			let mut shape = $crate::Style::<$shape>::default();

			$(shape.$fn_name($value);)*
			$(shape.$style_fn_name($style_value);)*

			shape
		}
    };
    ($shape:ty: ($($fn_name:ident={$value:expr})*)) => {
        {
			#[allow(unused_mut)]
			let mut shape = <$shape>::default();
			$(shape.$fn_name($value);)*
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
                Ellipse: style=(
                    stroke={crate::Stroke::Full { color: Color::RED, width: 1. }}
                ) (
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
