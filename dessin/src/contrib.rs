macro_rules! auto_import {
    {$($v:ident,)*} => {
		$(
			mod $v;
			pub use $v::*;
		)*
	};
}

auto_import! {
    anchor,
    arc,
    circle,
    fit,
    layout,
    line,
    padding,
    polygone,
    rectangle,
    textbox,
    thick_arc,
    triangle,
}
