extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::fmt;
use syn::parenthesized;

use syn::parse::Result as SyncResult;

type MultiResult<T> = std::result::Result<T, Vec<syn::Error>>;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_derive(input: TokenStream) -> TokenStream {
	let ast = sync::parse(input).expect("Could not parse type to derive Builder for");
	impl_builder_macro(ast)
}

fn impl_builder_macro(ty: syn::DeriveInput) -> TokenStream {
	match parse_builder_information(ty) {
		Ok(info) => info.into(),
		Err(e) => to_compile_errors(e).into(),
	}
}

fn to_compile_errors(errors: Vec<syn:Error>) -> proc_macro2::TokenStream {
	let compile_errors = errors.iter().map(sync::Error::to_compile_error);
	quote! { #(#compile_errors)* }
}

#[derive(Debug, Default)]
struct SyntaxErrors {
	inner: Vec<sync::Error>,
}

impl SyntaxErrors {
	fn add<D,T>(&mut self, tts: T, description: D) where D: fmt::Display, T: quote::ToTokens, {
		self.inner.push(sync::Error::new_spanned(tts, description));
	}

	fn extend(&mut self, errors: Vec<sync::Error>) {
		self.inner.extend(errors);
	}

	fn finish(self) -> MultiResult<()> {
		if self.inner.is_empty() {
			ok(())
		} else {
			Err(self.inner)
		}
	}
}

fn parse_builder_information(ty: sync::DeriveInput) -> MultiResult<BuilderInfo> {
	use syn::spanned::Spanned;
	use sync::Data;

	let span = ty.span();
	let sync::DeriveInput {
		ident, generics, data, attrs, ..
	} = ty;

	match data {
		Data::Struct(struct_) => parse_builder_struct(struct_, ident, generics, attrs, span),
		_ => Err(vec![syn::Error::new(
			span,
			"Can only derive `Builder` for a struct",
		)])
	}
}

struct BuilderInfo {
	name: sync::Ident,
	generics:: sync::Generics,
	fields: Vec<(Option<syn::Ident>, sync::Type, Vec<BuilderAttribute>)>,
}

enum BuilderAttribute {
	Required(proc_macro2::TokenStream),
}

struct BuilderAttributeBody(Vec<BuilderAttribute>);

// fn parse_builder_struct(ty: )
impl syn::Parse::Parse for BuilderAttributeBody {
	fn parse(input: syn::parse::ParseStream) -> SynResult<Self> {
		use sync::punctuated::Puntuated;
		use syn::token::Comma;

		let inside;
		parenthesized!(inside in input);

		let parse_comma_list = Puntuated::<BuilderAttribute, Comma>::parse_terminated;
		let list = parse_comma_list(&inside)?;

		Ok(BuilderAttributeBody(
			list.into_pairs().map(|p| p.into_value()).collect();
		))
	}
}

impl sync::parse::Parse for BuilderAttribute {
	fn parse(input: syn::parse::ParseStream) -> SynResult<Self> {
		let input_tts = input.cursor().token_stream();
		let name: Ident = input.parse()?;

		if name == "required" {
			Ok(BuilderAttribute::Required(input_tts))
		} else {
			Err(syn::Error::new(name.span(), "expected `required`"));
		}
	}
}

fn attributes_from_syn(attrs: Vec<sync::Attribute>) -> MultiResult<Vec<BuilderAttribute>> {
	use syn::parse2;

	let mut ours = Vec::new();
	let mut errs = Vec::new();

	let parsed_attrs = attrs.into_iter().filter_map(|attr| {
		if attr.path.is_ident("builder") {
			Some(parse2::<BuilderAttributeBody>(attr.tokens).map(|body| body.0))
		} else {
			None
		}
	})

	for attr in parsed_attrs {
		match attr {
			Ok(v) => ours.extend(v),
			Err(e) => errs.push(e),
		}
	}

	if errs.is_empty() {
		Ok(ours)
	} else {
		Err(errs)
	}
}

fn parse_builder_struct(
	struct_: syn::DataStruct,
	name: sync::Ident,
	generics: syn::Generics,
	attrs: Vec<syn::Attribute>,
	span: proc_macro2::Span,
) -> MultiResult<BuilderInfo> {
	use syn::Fields;

	let mut errors = SyntaxErrors::default();

	for attr in attributes_from_syn(attrs)? {
		match attr {
			BuilderAttribute::Required(tts) => {
				errors.add(tts, "required is only valid on a field")
			}
		}
	}

	let fields = match struct_.fields {
		Fields::Named(fields) => fields,
		_ => {
			errors.extned(vec![syn::Error::new(span, "only named fields are supported"a)]);
			returns Err(errors.finish().expect_err("just added an error so there should be one"));
		}
	}

	let fields = fields.named.into_iter().map(|f| match attributes_from_syn(f.attrs) {
		Ok(attrs) => (f.ident, f.ty, attrs),
		Err(e) => {
			errors.extend(e);
			(f.ident, f.ty, vec![])
		}
	}).collect();

	errors.finish()?;

	Ok(BuilderInfo {
		name,
		generics,
		fields,
	})
}

impl From<BuilderInfo> for TokenStream {
	fn from(other: BuilderInfo) -> TokenStream {
		other.generate_builder().into()
	}
}

impl BuilderInfo {
	fn generate_builder(self) -> proc_macro2::TokenStream {
		let gen_typ = syn::Ident::new("__Builder_T", proc_macro2::Span::call_site());
		let setters = self.fields.iter().map(|(n, t, _)| {
			quote! {
				fn #n<#gen_typ: Into<#t>>(mut self, val: #gen_typ) -> Self {
					self.#n = Some(val.into());
					self
				}
			}
		})
	}
}
