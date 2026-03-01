use proc_macro_crate::Error as CrateError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuantityError {
	#[error(
		"the `#[quantity(...)]` attribute can only be used on unit \
		 structs (zero fields)"
	)]
	NotUnitStruct,

	#[error("missing `unit` argument in `#[quantity(...)]`")]
	MissingUnit,

	#[error(
		"unknown argument `{0}` in `#[quantity(...)]`. Valid \
		 arguments: `unit`, `value_type`"
	)]
	UnknownArgument(String),

	#[error("duplicate `{0}` argument in `#[quantity(...)]`")]
	DuplicateArgument(String),

	#[error(
		"`quantities` crate must be a dependency in Cargo.toml: {0}"
	)]
	CrateNotFound(#[from] CrateError),
}
