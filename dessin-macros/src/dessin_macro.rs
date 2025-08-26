use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
	braced, bracketed, parenthesized,
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
	token::{Brace, Bracket, Comma, Paren},
	Expr, ExprAssign, ExprForLoop, ExprLet, Pat, Path, Result, Token,
};

enum Action {
	WithArgs(ExprAssign),
	WithoutArgs(Ident),
	SameName(Ident),
}
impl Parse for Action {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Brace) {
			let arg;
			let _ = braced!(arg in input);
			Ok(Action::SameName(arg.parse()?))
		} else {
			match input.fork().parse::<ExprAssign>() {
				Ok(_) => input.parse().map(Action::WithArgs),
				Err(_) => input.parse().map(Action::WithoutArgs),
			}
		}
	}
}
impl From<Action> for TokenStream {
	fn from(value: Action) -> Self {
		match value {
			Action::WithArgs(ExprAssign {
				attrs: _,
				left,
				eq_token: _,
				right,
			}) => quote!(__current_shape__.#left(#right);),
			Action::WithoutArgs(member) => quote!(__current_shape__.#member();),
			Action::SameName(name) => quote!(__current_shape__.#name(#name);),
		}
	}
}

struct Actions {
	actions: Punctuated<Action, Comma>,
}
impl Parse for Actions {
	fn parse(input: ParseStream) -> Result<Self> {
		let actions;
		let _ = parenthesized!(actions in input);
		let actions = actions.parse_terminated(Action::parse, Comma)?;

		Ok(Actions { actions })
	}
}
impl From<Actions> for TokenStream {
	fn from(Actions { actions }: Actions) -> Self {
		actions
			.into_iter()
			.map(TokenStream::from)
			.collect::<TokenStream>()
	}
}

struct DessinItem {
	add_style: bool,
	item: Path,
	actions: Actions,
}
impl Parse for DessinItem {
	fn parse(input: ParseStream) -> Result<Self> {
		let add_style = input.peek(Token![*]);
		if add_style {
			input.parse::<Token![*]>()?;
		}

		let item = input.parse::<Path>()?;
		let actions = input.parse::<Actions>()?;

		Ok(DessinItem {
			add_style,
			item,
			actions,
		})
	}
}
impl From<DessinItem> for TokenStream {
	fn from(
		DessinItem {
			add_style,
			item,
			actions,
		}: DessinItem,
	) -> Self {
		let base = if add_style {
			quote!(::dessin::prelude::Style::new(<#item>::default()))
		} else {
			quote!(<#item>::default())
		};

		if actions.actions.is_empty() {
			return base;
		}

		let actions = TokenStream::from(actions);

		quote!({
			let mut __current_shape__ = #base;
			#actions
			__current_shape__
		})
	}
}

struct DessinVar {
	var: Expr,
	actions: Option<Actions>,
}
impl Parse for DessinVar {
	fn parse(input: ParseStream) -> Result<Self> {
		let var;
		let _ = braced!(var in input);
		let var = var.parse::<Expr>()?;
		let actions = if input.peek(Paren) {
			Some(input.parse::<Actions>()?)
		} else {
			None
		};

		Ok(DessinVar { var, actions })
	}
}
impl From<DessinVar> for TokenStream {
	fn from(DessinVar { var, actions }: DessinVar) -> Self {
		let Some(actions) = actions else {
			return quote!(#var);
		};

		if actions.actions.is_empty() {
			quote!(#var)
		} else {
			let actions = TokenStream::from(actions);

			quote!({
				let mut __current_shape__ = #var;
				#actions
				__current_shape__
			})
		}
	}
}

struct DessinFor {
	expr: ExprForLoop,
}
impl Parse for DessinFor {
	fn parse(input: ParseStream) -> Result<Self> {
		let expr = input.parse::<ExprForLoop>()?;

		Ok(DessinFor { expr })
	}
}
impl From<DessinFor> for TokenStream {
	fn from(
		DessinFor {
			expr:
				ExprForLoop {
					attrs: _,
					label: _,
					for_token: _,
					pat,
					in_token: _,
					expr,
					body,
				},
		}: DessinFor,
	) -> Self {
		quote!(::dessin::prelude::Shape::Group(::dessin::prelude::Group {
			metadata: ::std::vec::Vec::new(),
			local_transform: ::dessin::nalgebra::Transform2::default(),
			shapes: {
				let __current_iterator__ = (#expr).into_iter();
				let mut __current_shapes__ = ::std::vec::Vec::with_capacity(__current_iterator__.size_hint().0);
				for #pat in __current_iterator__ {
					let __current_shape__ = ::dessin::prelude::Shape::from(#body);
					__current_shapes__.push(__current_shape__);
				}
				__current_shapes__
			},
		}))
	}
}

enum DessinIfElseArg {
	Let(ExprLet),
	Ident(Ident),
	Expr(Expr),
}
impl Parse for DessinIfElseArg {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Token![let]) {
			let let_exp = ExprLet {
				attrs: vec![],
				let_token: input.parse().unwrap(),
				pat: Box::new(Pat::parse_single(&input).unwrap()),
				eq_token: input.parse().unwrap(),
				expr: Box::new(Expr::parse_without_eager_brace(&input).unwrap()),
			};
			return Ok(DessinIfElseArg::Let(let_exp));
		}

		let is_ident = input.peek(syn::Ident) && input.peek2(Brace);
		if is_ident {
			let ident: Ident = input.parse()?;
			return Ok(DessinIfElseArg::Ident(ident));
		}

		let expr: Expr = input.parse()?;
		Ok(DessinIfElseArg::Expr(expr))
	}
}
impl From<DessinIfElseArg> for TokenStream {
	fn from(dessin_arg: DessinIfElseArg) -> Self {
		match dessin_arg {
			DessinIfElseArg::Let(v) => quote!(#v),
			DessinIfElseArg::Ident(v) => quote!(#v),
			DessinIfElseArg::Expr(v) => quote!(#v),
		}
	}
}

struct DessinIfElse {
	condition: DessinIfElseArg,
	if_body: Box<Dessin>,
	else_body: Option<Box<Dessin>>,
}
impl Parse for DessinIfElse {
	fn parse(input: ParseStream) -> Result<Self> {
		let _ = input.parse::<Token![if]>()?;
		let condition = input.parse::<DessinIfElseArg>()?;

		let if_body;
		let _ = braced!(if_body in input);
		let if_body: Dessin = if_body.parse()?;
		let else_body = if input.parse::<Token![else]>().is_ok() {
			let else_body;
			let _ = braced!(else_body in input);
			Some(Box::new(else_body.parse()?))
		} else {
			None
		};

		Ok(DessinIfElse {
			condition,
			if_body: Box::new(if_body),
			else_body,
		})
	}
}
impl From<DessinIfElse> for TokenStream {
	fn from(
		DessinIfElse {
			condition,
			if_body,
			else_body,
		}: DessinIfElse,
	) -> Self {
		let else_body = if let Some(else_body) = else_body {
			TokenStream::from(*else_body)
		} else {
			TokenStream::from(DessinType::Empty)
		};

		let condition = TokenStream::from(condition);
		let if_body = TokenStream::from(*if_body);

		quote!(
			if #condition {
				::dessin::prelude::Shape::from(#if_body)
			} else {
				::dessin::prelude::Shape::from(#else_body)
			}
		)
	}
}

struct DessinGroup(Punctuated<Dessin, Token![,]>);
impl Parse for DessinGroup {
	fn parse(input: ParseStream) -> Result<Self> {
		let children;
		let _ = bracketed!(children in input);

		let children = children.parse_terminated(Dessin::parse, Token![,])?;

		Ok(DessinGroup(children))
	}
}
impl From<DessinGroup> for TokenStream {
	fn from(DessinGroup(children): DessinGroup) -> Self {
		let children = children
			.into_iter()
			.map(TokenStream::from)
			.collect::<Vec<_>>();

		quote!(::dessin::prelude::Shape::Group(::dessin::prelude::Group {
			local_transform: ::dessin::nalgebra::Transform2::default(),
			metadata: ::std::vec::Vec::new(),
			shapes: ::std::vec![
				#(::dessin::prelude::Shape::from(#children)),*
			],
		}))
	}
}

enum DessinType {
	Empty,
	Item(DessinItem),
	Var(DessinVar),
	Group(DessinGroup),
	For(DessinFor),
	IfElse(DessinIfElse),
}
impl Parse for DessinType {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.is_empty() {
			Ok(DessinType::Empty)
		} else if input.peek(Brace) {
			input.parse().map(DessinType::Var)
		} else if input.peek(Token![for]) {
			input.parse().map(DessinType::For)
		} else if input.peek(Token![if]) {
			input.parse().map(DessinType::IfElse)
		} else if input.peek(Bracket) {
			input.parse().map(DessinType::Group)
		} else {
			input.parse().map(DessinType::Item)
		}
	}
}
impl From<DessinType> for TokenStream {
	fn from(value: DessinType) -> Self {
		match value {
			DessinType::Empty => quote!(::dessin::prelude::Shape::default()),
			DessinType::Item(i) => i.into(),
			DessinType::Group(g) => g.into(),
			DessinType::Var(v) => v.into(),
			DessinType::For(f) => f.into(),
			DessinType::IfElse(i) => i.into(),
		}
	}
}

///
pub struct Dessin {
	dessin_type: DessinType,
	erased_type_shape_add_style: bool,
	erased_type_shape_actions: Option<Actions>,
}
impl Parse for Dessin {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut erased_type_shape_add_style = false;

		let dessin_type: DessinType = input.parse()?;
		let erased_type_shape_actions = if input.peek(Token![>]) {
			let _: Token![>] = input.parse()?;

			if input.peek(Token![*]) {
				let _: Token![*] = input.parse()?;
				erased_type_shape_add_style = true;
			}

			let actions = input.parse::<Actions>()?;

			Some(actions)
		} else {
			None
		};

		Ok(Dessin {
			dessin_type,
			erased_type_shape_add_style,
			erased_type_shape_actions,
		})
	}
}
impl From<Dessin> for TokenStream {
	fn from(
		Dessin {
			dessin_type,
			erased_type_shape_add_style,
			erased_type_shape_actions,
		}: Dessin,
	) -> Self {
		let base = TokenStream::from(dessin_type);

		let Some(actions) = erased_type_shape_actions else {
			return base;
		};

		let base = if erased_type_shape_add_style {
			quote!(::dessin::prelude::Style::new(
				::dessin::prelude::Shape::from(#base)
			))
		} else {
			quote!(::dessin::prelude::Shape::from(#base))
		};

		if actions.actions.is_empty() {
			return base;
		}

		if actions.actions.is_empty() {
			return base;
		}

		let actions = TokenStream::from(actions);
		return quote!({
			let mut __current_shape__ = #base;
			#actions
			__current_shape__
		});
	}
}

#[test]
fn simple() {
	syn::parse_str::<Dessin>("Item()").unwrap();
}
#[test]
fn simple_with_style() {
	syn::parse_str::<Dessin>("*Item()").unwrap();
}
#[test]
fn simple_with_style_and_generic() {
	syn::parse_str::<Dessin>("*Item<GenA, GenB<GenC>>()").unwrap();
}
#[test]
fn complex_with_style() {
	syn::parse_str::<Dessin>("*Item() > *()").unwrap();
}
#[test]
fn simple_and_actions() {
	syn::parse_str::<Dessin>("Item( my_fn=(1., 1.), {close}, closed )").unwrap();
}
#[test]
fn var_no_args() {
	syn::parse_str::<Dessin>("{ v }").unwrap();
}
#[test]
fn var_args() {
	syn::parse_str::<Dessin>("{ v }( my_fn=(1., 1.), {close}, closed )").unwrap();
}
#[test]
fn group() {
	syn::parse_str::<Dessin>("[ Item(), Item() ]").unwrap();
}
#[test]
fn as_shape() {
	syn::parse_str::<Dessin>("Item() > ()").unwrap();
}
#[test]
fn group_complex() {
	syn::parse_str::<Dessin>("[ Item(), Item() ] > ()").unwrap();
}
#[test]
fn for_loop() {
	syn::parse_str::<Dessin>(
		"for x in 0..10 {
			let y = x as f32 * 2.;
			dessin!(Circle( radius={y}) )
		}",
	)
	.unwrap();
}
#[test]
fn for_loop_par() {
	syn::parse_str::<Dessin>(
		"for x in (it) {
			let y = x as f32 * 2.;
			dessin!(Circle( radius={y}) )
		}",
	)
	.unwrap();
}
#[test]
fn for_loop_var() {
	syn::parse_str::<Dessin>(
		"for x in it {
			let y = x as f32 * 2.;
			dessin!(Circle ( radius={y}) )
		}",
	)
	.unwrap();
}
// #[test]
// fn for_loop_range_var() {
//	 syn::parse_str::<Dessin>(
//		 "for x in 0..n {
//			 let y = x as f32 * 2.;
//			 dessin!(Circle: ( radius={y}) )
//		 }",
//	 )
//	 .unwrap();
// }
#[test]
fn simple_for_loop() {
	syn::parse_str::<Dessin>(
		"for x in xs {
			let y = x as f32 * 2.;
			dessin!(Circle( radius={y}) )
		}",
	)
	.unwrap();
}
#[test]
fn for_loop_range_var_par() {
	syn::parse_str::<Dessin>(
		"for x in 0..(n) {
			let y = x as f32 * 2.;
			dessin!(Circle( radius={y}) )
		}",
	)
	.unwrap();
}
#[test]
fn branch_if() {
	syn::parse_str::<Dessin>(
		"if test_fn() == 2 {
			Circle()
		}",
	)
	.unwrap();
}
#[test]
fn branch_if_else() {
	syn::parse_str::<Dessin>(
		"if test_fn() == 2 {
			Circle()
		} else {
			Ellipse()
		}",
	)
	.unwrap();
}
#[test]
fn combined_group_erased() {
	syn::parse_str::<Dessin>(
		"[
			Shape(),
			Shape() > (),
			{ var } > (),
		] > ()",
	)
	.unwrap();
}
#[test]
fn simple_if() {
	syn::parse_str::<Dessin>(
		"if my_condition {
			Circle()
		}",
	)
	.unwrap();
}
#[test]
fn if_let() {
	syn::parse_str::<Dessin>(
		"if let Some(x) = my_condition {
			Circle()
		}",
	)
	.unwrap();
}
#[test]
fn combined_if() {
	syn::parse_str::<Dessin>(
		"if test_fn() == 2 {
			Circle() > ()
		}",
	)
	.unwrap();
}
#[test]
fn mod_if() {
	syn::parse_str::<Dessin>(
		"if test_fn() == 2 {
			my_mod::Circle() > ()
		}",
	)
	.unwrap();
}
#[test]
fn var_if() {
	syn::parse_str::<Dessin>(
		"if test_fn() == 2 {
			{ circle } > ()
		}",
	)
	.unwrap();
}
#[test]
fn if_if_group() {
	syn::parse_str::<Dessin>(
		"[
			{ circle }(),
			if test_fn() == 2 {
				{ circle } > ()
			},
			for x in 0..1 {
				dessin!()
			},
			Circle(),
		]",
	)
	.unwrap();
}
#[test]
fn group_in_group() {
	syn::parse_str::<Dessin>(
		"[
			[
				Circle(),
				{ circle } > (),
				if test_fn() == 2 {
					{ circle } > ()
				},
				{ circle },
			],
			{ circle },
			for x in (var) {
				dessin!()
			},
			[],
			if test_fn() == 2 {
				[
					[],
					{ circle },
				]
			},
			Circle(),
		]",
	)
	.unwrap();
}
