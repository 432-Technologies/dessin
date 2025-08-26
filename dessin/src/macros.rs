//! DSL definition
//!
//! ## Components
//!
//! ### Basic use, with variable or expression:
//! ```
//! # use dessin::prelude::*;
//! let r = 2.;
//!
//! dessin!(Circle(
//! 	radius=r,
//! 	translate=[r * 2., 0.],
//! ));
//! ```
//!
//! ### With a function that takes no argument:
//!
//! ```
//! # use dessin::prelude::*;
//! dessin!(Curve(
//! 	closed,
//! ));
//! ```
//!
//! ### With a function that has the same name as a variable:
//!
//! ```
//! # use dessin::prelude::*;
//! let text = "my string";
//! dessin!(Text(
//! 	{text},
//! ));
//! ```
//!
//! ### With component in a mod:
//!
//! ```
//! # use dessin::prelude::dessin;
//! dessin!(dessin::prelude::Text());
//! ```
//!
//! ## Group
//!
//! ```
//! # use dessin::prelude::*;
//! dessin!([
//!	 Circle(),
//!	 Text(),
//! ]);
//! ```
//!
//! ## Erase type
//!
//! Useful to access certain function only availiable in Shape (related to transform).
//! Also useful also for branches with different components (see [If else](#with-different-components)),
//!
//! ```
//! # use dessin::prelude::*;
//! dessin!(Text(
//!	 // here type is `Text`
//! ) > (
//!	 // here type is `Shape`
//! ));
//!
//! dessin!([
//!	 Circle(),
//!	 Text(),
//! ] > (
//!	 // Transform this group
//! ));
//! ```
//!
//! ## For loop
//!
//! ```
//! # use dessin::prelude::*;
//! dessin!(for x in 0..10 {
//!	 // Here, rust code is expected. But return type must be a `Shape`
//!	 let x = x as f32;
//!
//!	 dessin!(Circle(
//!		 radius=x,
//!		 translate=[x, x * 2.]
//!	 ))
//! });
//!
//! // Same as before, we can transform the group after
//! dessin!(for text in ["Hello", "World"] {
//!	 dessin!(Text(
//!		 { text },
//!	 ))
//! } > (
//!	 scale=[2., 2.],
//! ));
//! ```
//!
//! ## If else
//!
//! ```
//! # use dessin::prelude::*;
//! dessin!(if true {
//!	 Circle()
//! });
//!
//! // Both side must return the same type
//! dessin!(if true {
//!	 Circle()
//! } else {
//!	 Circle()
//! });
//!
//! // That's why the type of each branch can be erased
//! dessin!(if true {
//!	 Circle() > ()
//! } else {
//!	 Text() > ()
//! });
//! ```
