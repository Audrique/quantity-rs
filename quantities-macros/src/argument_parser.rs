use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Path, Token};

use crate::error::QuantityError;

enum QuantityArg {
	Unit(Path),
	ValueType(syn::Type),
}

impl Parse for QuantityArg {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let ident: syn::Ident = input.parse()?;
		let _: Token![=] = input.parse()?;

		match ident.to_string().as_str() {
			"unit" => Ok(QuantityArg::Unit(input.parse()?)),
			"value_type" => {
				Ok(QuantityArg::ValueType(input.parse()?))
			},
			other => Err(syn::Error::new(
				ident.span(),
				QuantityError::UnknownArgument(other.to_string()),
			)),
		}
	}
}

pub struct QuantityArgs {
	pub unit: Path,
	pub default_type: syn::Type,
}

impl Parse for QuantityArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let args: Punctuated<QuantityArg, Token![,]> =
			input.parse_terminated(QuantityArg::parse, Token![,])?;

		let mut unit = None;
		let mut default_type = None;

		for arg in args {
			match arg {
				QuantityArg::Unit(p) => {
					if unit.is_some() {
						return Err(syn::Error::new(
							p.span(),
							QuantityError::DuplicateArgument(
								"unit".into(),
							),
						));
					}
					unit = Some(p);
				},
				QuantityArg::ValueType(t) => {
					if default_type.is_some() {
						return Err(syn::Error::new(
							t.span(),
							QuantityError::DuplicateArgument(
								"value_type".into(),
							),
						));
					}
					default_type = Some(t);
				},
			}
		}

		let unit = unit.ok_or_else(|| {
			syn::Error::new(
				Span::call_site(),
				QuantityError::MissingUnit,
			)
		})?;
		Ok(QuantityArgs {
			unit,
			default_type: default_type
				.unwrap_or_else(|| syn::parse_quote!(f64)),
		})
	}
}
