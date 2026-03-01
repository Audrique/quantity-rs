mod argument_parser;
mod error;
mod expand;

use proc_macro::TokenStream;

use crate::expand::try_expand;

#[proc_macro_attribute]
pub fn quantity(
	attribute: TokenStream,
	item: TokenStream,
) -> TokenStream {
	match try_expand(attribute, item) {
		Ok(tokens) => tokens.into(),
		Err(err) => err.to_compile_error().into(),
	}
}
