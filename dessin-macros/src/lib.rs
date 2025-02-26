//! Macros for the [dessin](https://docs.rs/dessin/latest/dessin/) crate.

#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

extern crate proc_macro;

mod dessin_macro;

use proc_macro2::TokenStream;
use quote::{__private::mk_ident, quote, spanned::Spanned};
use syn::{parse_macro_input, DataStruct, DeriveInput, Fields, FieldsNamed, Type};

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
/// 	// fn or_not(&mut self, v: Option<u32>)
/// 	or_not: Option<u32>,
///
/// 	// fn into_string<V: Into<String>>(&mut self, v: V)
/// 	#[shape(into)]
/// 	into_string: String,
///
/// 	// fn maybe_into_string<V: Into<String>>(&mut self, v: V)
/// 	// set maybe_into_string to Some(v.into()) if called
/// 	#[shape(into_some)]
/// 	maybe_into_string: Option<String>,
/// }
/// ```
#[proc_macro_derive(Shape, attributes(shape, local_transform))]
pub fn shape(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let name = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	// let vis = input.vis;

	let mut local_transform = None;

	let fields = match input.data {
		syn::Data::Struct(DataStruct {
			fields: Fields::Named(FieldsNamed { named: fields, .. }),
			..
		}) => fields
			.into_iter()
			.map(|field| {
				let ident = field.ident.unwrap();
				let ty = field.ty;

				let mut skip = false;
				let mut into = false;
				let mut boolean = false;
				let mut some = false;
				let mut into_some = false;
				let mut doc = None;
				for attr in field.attrs {
					if attr.path().is_ident("doc") {
						doc = Some(attr);
						continue;
					}

					if attr.path().is_ident("local_transform") {
						if local_transform.is_some() {
							panic!("Only one field can be a local_transform");
						}

						local_transform = Some(ident.clone());
						skip = true;
					}

					if attr.path().is_ident("shape") {
						attr.parse_nested_meta(|meta| {
							if meta.path.is_ident("skip") {
								skip = true;
							}

							if meta.path.is_ident("into") {
								into = true;
							}

							if meta.path.is_ident("bool") {
								boolean = true;
							}

							if meta.path.is_ident("some") {
								some = true;
							}

							if meta.path.is_ident("into_some") {
								into_some = true;
							}

							Ok(())
						})
						.unwrap()
					}
				}

				if skip {
					return quote!();
				}

				let with_ident = mk_ident(&format!("with_{ident}"), None);
				if boolean {
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
				} else if into {
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
				} else if some {
					let err_msg = syn::Error::new(ty.__span(), "Not supported").to_compile_error();
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
				} else if into_some {
					let err_msg = syn::Error::new(ty.__span(), "Not supported").to_compile_error();
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
				} else {
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
