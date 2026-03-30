//! Macros for the [dessin](https://docs.rs/dessin/latest/dessin/) crate.

#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

extern crate proc_macro;

mod dessin_macro;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
	parse_macro_input, punctuated::Punctuated, spanned::Spanned as _, DataStruct, DeriveInput,
	Fields, FieldsNamed, Ident, Token, Type,
};

/// Entry point to build drawings
/// ```ignore
/// dessin!([
/// 	*Text(
/// 		text = "Hi",
/// 		fill = Srgba::new(255, 0, 0, 255),
/// 	),
/// 	Line(
/// 		from = [0., 10.],
/// 		to = [10., 0.],
/// 	),
/// ] > *(
/// 	translate = [-5., 5.],
/// 	fill = Srgba::new(0, 255, 0, 255),
/// ))
#[proc_macro]
pub fn dessin(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let dessin = parse_macro_input!(tokens as dessin_macro::Dessin);

	TokenStream::from(dessin).into()
}

/// Auto implements setter for each members
///
/// ```rust
/// # #[macro_use] extern crate dessin_macros;
/// # use std::sync::{Arc, RwLock};
///
/// #[derive(Shape)]
/// struct MyShape {
/// 	// fn my_parameter(&mut self, v: u32)
/// 	my_parameter: u32,
///
/// 	// fn my_bool(&mut self)
/// 	// set my_bool to true if called
/// 	#[shape(bool)]
/// 	my_bool: bool,
///
/// 	// No fn generated
/// 	#[shape(skip)]
/// 	skip_this: Arc<RwLock<Vec<u8>>>,
///
/// 	// fn skip_option(&mut self, v: u32)
/// 	// set skip_option to Some(v) if called
/// 	#[shape(some)]
/// 	skip_option: Option<u32>,
///
/// 	// fn or_not(&mut self, v: Option<u32>)
/// 	or_not: Option<u32>,
///
/// 	// fn or_both(&mut self, v: u32)
/// 	// fn maybe_or_both(&mut self, v: Option<u32>)
/// 	#[shape(some, option_fn)]
/// 	or_both: Option<u32>,
///
/// 	// fn into_string<V: Into<String>>(&mut self, v: V)
/// 	#[shape(into)]
/// 	into_string: String,
///
/// 	// fn maybe_into_string<V: Into<String>>(&mut self, v: V)
/// 	// set maybe_into_string to Some(v.into()) if called
/// 	#[shape(into, some)]
/// 	maybe_into_string: Option<String>,
/// }
/// ```
#[proc_macro_derive(Shape, attributes(shape, local_transform))]
pub fn shape(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	// let vis = input.vis;

	let mut local_transform: Option<Ident> = None;

	let fields = match input.data {
		syn::Data::Struct(DataStruct {
			fields: Fields::Named(FieldsNamed { named: fields, .. }),
			..
		}) => fields
			.iter()
			.map(|field| {
				let ident = field.ident.as_ref().unwrap();
				let ty = &field.ty;

				let mut skip: Option<_> = None;
				let mut into: Option<_> = None;
				let mut boolean: Option<_> = None;
				let mut some: Option<_> = None;
				let mut option_fn: Option<_> = None;

				let mut doc = None;

				for attr in &field.attrs {
					if attr.path().is_ident("doc") {
						doc = Some(attr);
						continue;
					}

					if attr.path().is_ident("local_transform") {
						if let Some(local_transform) = &local_transform {
							return quote_spanned! {
								local_transform.span() =>
								compile_error!("Only one field can be a local_transform")
							}
						}

						local_transform = Some(ident.clone());
						return quote! {};
					}

					if attr.path().is_ident("shape") {
						let Ok(nested) = attr
							.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated) else {
								return quote_spanned! {
									attr.span() =>
									compile_error!("shape attribute expect a comma separated list of args from `skip`, `into`, `bool`, `some`, and `option_fn`")
								};
							};

						for value in nested {
							if value == Ident::new("skip", value.span()) {
								skip = Some(value.span());
							} else if value == Ident::new("into", value.span()) {
								into = Some(value.span());
							} else if value == Ident::new("bool", value.span()) {
								boolean = Some(value.span());
							} else if value == Ident::new("some", value.span()) {
								some = Some(value.span());
							} else if value == Ident::new("option_fn", value.span()) {
								option_fn = Some(value.span());
							} else {
								let err = syn::Error::new(value.span(), format!("Unknown attribute `{}`. Exect `skip`, `into`, `bool`, `some` or `option_fn`", value)).to_compile_error();

								return quote_spanned! {
									value.span() =>
									#err
								}
							}
						}

					}
				}

				if let Some(skip) = skip {
					if [into, boolean, some, option_fn].into_iter().any(|v| v.is_some()) {
						return quote_spanned! {
							skip =>
							pub fn #ident() {
								compile_error!("skip is not compatible with any other attributes")
							}
						};
					} else {
						return quote!();
					}
				}

				let with_ident = Ident::new(&format!("with_{ident}"), field.span());

				let generated_tokens = match (
					into,
					boolean,
					some,
				) {
					(None, None, None) => {
						quote!(
							#doc
							#[inline]
							pub fn #ident(&mut self, value: #ty) -> &mut Self {
								self.#ident = value;
								self
							}

							#doc
							#[inline]
							pub fn #with_ident(mut self, value: #ty) -> Self {
								self.#ident(value);
								self
							}
						)
					}

					(None, Some(_), None) => {
						quote!(
							#doc
							#[inline]
							pub fn #ident(&mut self) -> &mut Self {
								self.#ident = true;
								self
							}

							#doc
							#[inline]
							pub fn #with_ident(mut self) -> Self {
								self.#ident();
								self
							}
						)
					}
					(_, Some(span), _) => {
						return quote_spanned! {
							span =>
							compile_error!("bool is not compatible with `into` or `some`")
						};
					}

					(Some(_), None, None) => {
						quote!(
							#doc
							#[inline]
							pub fn #ident<__INTO__T: Into<#ty>>(&mut self, value: __INTO__T) -> &mut Self {
								self.#ident = value.into();
								self
							}

							#doc
							#[inline]
							pub fn #with_ident<__INTO__T: Into<#ty>>(mut self, value: __INTO__T) -> Self {
								self.#ident(value);
								self
							}
						)
					}

					(None, None, Some(span)) => {
						let err_msg = syn::Error::new(span, "Not supported").to_compile_error();
						let Type::Path(syn::TypePath {
							path: syn::Path { segments, .. },
							..
						}) = ty
						else {
							return err_msg;
						};

						let ty = match segments.iter().next() {
							Some(syn::PathSegment {
								arguments:
									syn::PathArguments::AngleBracketed(
										syn::AngleBracketedGenericArguments { args, .. },
									),
								..
							}) => match args.iter().next() {
								Some(syn::GenericArgument::Type(t)) => t,
								_ => return err_msg,
							},
							_ => return err_msg,
						};


						quote!(
							#doc
							#[inline]
							pub fn #ident(&mut self, value: #ty) -> &mut Self {
								self.#ident = Some(value);
								self
							}

							#doc
							#[inline]
							pub fn #with_ident(mut self, value: #ty) -> Self {
								self.#ident(value);
								self
							}
						)
					}
					(Some(_), None, Some(span)) => {
						let err_msg = syn::Error::new(span, "Not supported").to_compile_error();
						let Type::Path(syn::TypePath {
							path: syn::Path { segments, .. },
							..
						}) = ty
						else {
							return err_msg;
						};

						let ty = match segments.iter().next() {
							Some(syn::PathSegment {
								arguments:
									syn::PathArguments::AngleBracketed(
										syn::AngleBracketedGenericArguments { args, .. },
									),
								..
							}) => match args.iter().next() {
								Some(syn::GenericArgument::Type(t)) => t,
								_ => return err_msg,
							},
							_ => return err_msg,
						};

						quote!(
							#doc
							#[inline]
							pub fn #ident<__INTO__T: Into<#ty>>(&mut self, value: __INTO__T) -> &mut Self {
								self.#ident = Some(value.into());
								self
							}

							#doc
							#[inline]
							pub fn #with_ident<__INTO__T: Into<#ty>>(mut self, value: __INTO__T) -> Self {
								self.#ident(value);
								self
							}
						)
					}
				};

				let generated_tokens = match (option_fn, into) {
					(Some(_), None) => {
						let option_ident = Ident::new(&format!("maybe_{ident}"), field.span());
						let with_option_ident = Ident::new(&format!("with_maybe_{ident}"), field.span());

						quote!(
							#generated_tokens

							#doc
							#[inline]
							pub fn #option_ident(&mut self, value: #ty) -> &mut Self {
								self.#ident = value;
								self
							}

							#doc
							#[inline]
							pub fn #with_option_ident(mut self, value: #ty) -> Self {
								self.#option_ident(value);
								self
							}
						)
					}
					(Some(span), Some(_)) => {
						let option_ident = Ident::new(&format!("maybe_{ident}"), field.span());
						let with_option_ident = Ident::new(&format!("with_maybe_{ident}"), field.span());

						let err_msg = syn::Error::new(span, "Not supported").to_compile_error();
						let Type::Path(syn::TypePath {
							path: syn::Path { segments, .. },
							..
						}) = ty
						else {
							return err_msg;
						};

						let ty = match segments.iter().next() {
							Some(syn::PathSegment {
								arguments:
									syn::PathArguments::AngleBracketed(
										syn::AngleBracketedGenericArguments { args, .. },
									),
								..
							}) => match args.iter().next() {
								Some(syn::GenericArgument::Type(t)) => t,
								_ => return err_msg,
							},
							_ => return err_msg,
						};

						quote!(
							#generated_tokens

							#doc
							#[inline]
							pub fn #option_ident<__INTO__T: Into<#ty>>(&mut self, value: Option<__INTO__T>) -> &mut Self {
								self.#ident = value.map(Into::into);
								self
							}

							#doc
							#[inline]
							pub fn #with_option_ident<__INTO__T: Into<#ty>>(mut self, value: Option<__INTO__T>) -> Self {
								self.#option_ident(value);
								self
							}
						)
					}
					_ => generated_tokens
				};

				generated_tokens
			})
			.collect::<Vec<_>>(),
		syn::Data::Struct(_) => {
			unreachable!()
		}
		syn::Data::Enum(_) => {
			unreachable!()
		}
		syn::Data::Union(_) => {
			unreachable!()
		}
	};

	let shape_op_impl = if let Some(lt) = local_transform {
		quote!(
			impl #impl_generics ::dessin::prelude::ShapeOp for #name #ty_generics #where_clause {
				#[inline]
				fn transform(&mut self, transform_matrix: ::dessin::nalgebra::Transform2<f32>) -> &mut Self {
					self.#lt = transform_matrix * self.#lt;
					self
				}

				#[inline]
				fn local_transform(&self) -> &::dessin::nalgebra::Transform2<f32> {
					&self.#lt
				}
			}
		)
	} else {
		quote!()
	};

	proc_macro::TokenStream::from(quote! {
		impl #impl_generics #name #ty_generics #where_clause {
			#(#fields)*
		}

		#shape_op_impl
	})
}
