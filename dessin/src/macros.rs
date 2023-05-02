/// The `dessin` macro makes it easier to build drawings.
///
/// Example:
/// ```
/// # fn main() {
/// use dessin::prelude::*;
///
/// let dessin = dessin!(group: [
/// 	{ Circle: #(
/// 		fill={Color::RED}
/// 		radius={10.}
/// 		translate={[10., 10.]}
/// 	) }
/// 	{ for x in {0..10}: {
/// 		dessin!(Line: #(
/// 			stroke={(Color::BLUE, 1.)}
/// 			from={[x as f32 * 10., 0.]}
/// 			to={[x as f32 * 10., 10.]}
/// 		))
/// 	} }
/// ]);
/// # }
/// ```
#[macro_export]
macro_rules! dessin {
	() => {$crate::shapes::Shape::default()};
	(for $x:ident in {$range:expr}: $($f:block)?) => {
		{
			#[allow(unused_mut)]
			let mut shapes = vec![];

			$(
				for $x in $range {
					shapes.push($crate::shapes::Shape::from($f))
				}
			)?

			$crate::shapes::Shape::Group {
				local_transform: ::nalgebra::Transform2::default(),
				shapes,
			}
		}
	};
	(for $x:ident in {$range:expr}: #($($fn_name:ident={$value:expr})*) $($f:block)?) => {
		{
			#[allow(unused_mut)]
			let mut shapes = vec![];

			$(
				for $x in $range {
					shapes.push($crate::shapes::Shape::from($f))
				}
			)?

			#[allow(unused_mut)]
			let mut shape = $crate::style::Style::new($crate::shapes::Shape::Group {
				local_transform: ::nalgebra::Transform2::default(),
				shapes,
			});
			$(shape.$fn_name($value);)*
			shape
		}
	};
	(for $x:ident in {$range:expr}: ($($fn_name:ident={$value:expr})*) $($f:block)?) => {
		{
			#[allow(unused_mut)]
			let mut shapes = vec![];

			$(
				for $x in $range {
					shapes.push($crate::shapes::Shape::from($f))
				}
			)?

			#[allow(unused_mut)]
			let mut shape = $crate::shapes::Shape::Group {
				local_transform: ::nalgebra::Transform2::default(),
				shapes,
			};
			$(shape.$fn_name($value);)*
			shape
		}
	};
	(if ($e:expr) { $($rest:tt)* } else { $($else_rest:tt)* }) => {
		{
			if $e {
				$crate::shapes::Shape::from($crate::dessin! ($($rest)*))
			} else {
				$crate::shapes::Shape::from($crate::dessin! ($($else_rest)*))
			}
		}
	};
	(if ($e:expr) { $($rest:tt)* }) => {
		{
			Shape::from(
				if $e {
					$crate::shapes::Shape::from($crate::dessin! ($($rest)*))
				} else {
					$crate::dessin! ()
				}
			)
		}
	};
	(var |$v:ident|: #($($fn_name:ident={$value:expr})*)) => {
		{
			#[allow(unused_mut)]
			let mut shape = $crate::style::Style::new($v);
			$(shape.$fn_name($value);)*
			shape
		}
	};
	(var |$v:ident|: ($($fn_name:ident={$value:expr})*)) => {
		{
			#[allow(unused_mut)]
			let mut shape = $v;
			$(shape.$fn_name($value);)*
			shape
		}
	};
	(var {$v:expr}: #($($fn_name:ident={$value:expr})*)) => {
		{
			#[allow(unused_mut)]
			let mut shape = $crate::style::Style::new($v);
			$(shape.$fn_name($value);)*
			shape
		}
	};
	(var {$v:expr}: ($($fn_name:ident={$value:expr})*)) => {
		{
			#[allow(unused_mut)]
			let mut shape = $v;
			$(shape.$fn_name($value);)*
			shape
		}
	};
	(group: [$( { $($rest:tt)* } )*]) => {
		{
			#[allow(unused_mut)]
			let mut acc = Vec::new();

			$(
				acc.push(
					$crate::shapes::Shape::from(
						$crate::dessin! ($($rest)*)
					)
				);
			)*

			$crate::shapes::Shape::Group {
				local_transform: ::nalgebra::Transform2::default(),
				shapes: acc,
			}
		}
	};
	(group: #($($fn_name:ident={$value:expr})*) [$( { $($rest:tt)* } )*]) => {
		{
			#[allow(unused_mut)]
			let mut acc = Vec::new();

			$(
				acc.push(
					$crate::shapes::Shape::from(
						$crate::dessin! ($($rest)*)
					)
				);
			)*

			#[allow(unused_mut)]
			let mut group = $crate::style::Style::new($crate::shapes::Shape::Group {
				local_transform: ::nalgebra::Transform2::default(),
				shapes: acc,
			});
			$(group.$fn_name($value);)*

			group
		}
	};
	(group: ($($fn_name:ident={$value:expr})*) [$( { $($rest:tt)* } )*]) => {
		{
			#[allow(unused_mut)]
			let mut acc = Vec::new();

			$(
				acc.push(
					$crate::shapes::Shape::from(
						$crate::dessin! ($($rest)*)
					)
				);
			)*

			#[allow(unused_mut)]
			let mut group = $crate::shapes::Shape::Group {
				local_transform: ::nalgebra::Transform2::default(),
				shapes: acc,
			};
			$(group.$fn_name($value);)*

			group
		}
	};
	($shape:ty: !#( $($fn_name_shape:ident $( ={$value_shape:expr} )? )* )) => {
        {
			#[allow(unused_mut)]
			let mut shape = $crate::style::Style::new(
				$crate::shapes::Shape::from(<$shape>::default())
			);
			$(shape.$fn_name_shape($($value_shape)?);)*

			shape
		}
    };
	($shape:ty: !( $($fn_name_shape:ident $( ={$value_shape:expr} )? )* )) => {
        {
			#[allow(unused_mut)]
			let mut shape = $crate::shapes::Shape::from(<$shape>::default());
			$(shape.$fn_name_shape($($value_shape)?);)*

			shape
		}
    };
	($shape:ty: #($($fn_name:ident $( ={$value:expr} )? )*) -> !#( $($fn_name_shape:ident $( ={$value_shape:expr} )? )* )) => {
        {
			#[allow(unused_mut)]
			let mut shape = $crate::style::Style::<$shape>::default();
			$(shape.$fn_name($($value)?);)*

			#[allow(unused_mut)]
			let mut shape = $crate::style::Style::new(shape);
			$(shape.$fn_name_shape($($value_shape)?);)*

			shape
		}
    };
	($shape:ty: ($($fn_name:ident $( ={$value:expr} )? )*) -> !#( $($fn_name_shape:ident $( ={$value_shape:expr} )? )* )) => {
        {
			#[allow(unused_mut)]
			let mut shape = <$shape>::default();
			$(shape.$fn_name($($value)?);)*

			#[allow(unused_mut)]
			let mut shape = $crate::style::Style::new(shape);
			$(shape.$fn_name_shape($($value_shape)?);)*

			shape
		}
    };
	($shape:ty: #($($fn_name:ident $( ={$value:expr} )? )*) -> !( $($fn_name_shape:ident $( ={$value_shape:expr} )? )* )) => {
        {
			#[allow(unused_mut)]
			let mut shape = $crate::style::Style::<$shape>::default();
			$(shape.$fn_name($($value)?);)*

			#[allow(unused_mut)]
			let mut shape = $crate::style::Shape::from(shape);
			$(shape.$fn_name_shape($($value_shape)?);)*

			shape
		}
    };
    ($shape:ty: ($($fn_name:ident $( ={$value:expr} )? )*) -> !( $($fn_name_shape:ident $( ={$value_shape:expr} )? )* )) => {
        {
			#[allow(unused_mut)]
			let mut shape = <$shape>::default();
			$(shape.$fn_name($($value)?);)*

			#[allow(unused_mut)]
			let mut shape = $crate::style::Shape::from(shape);
			$(shape.$fn_name_shape($($value_shape)?);)*

			shape
		}
    };
	($shape:ty: #($($fn_name:ident $( ={$value:expr} )? )*)) => {
        {
			#[allow(unused_mut)]
			let mut shape = $crate::style::Style::<$shape>::default();
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
