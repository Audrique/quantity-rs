use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, parse_macro_input, DeriveInput, Path, Token};

struct QuantityAttr {
    unit: Path,
    default_type: syn::Type,
}

impl Parse for QuantityAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut unit = None;
        let mut default_type: Option<syn::Type> = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let _: Token![=] = input.parse()?;

            match ident.to_string().as_str() {
                "unit" => unit = Some(input.parse::<Path>()?),
                "value_type" => default_type = Some(input.parse::<syn::Type>()?),
                other => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unknown key `{}`", other),
                    ))
                }
            }

            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(QuantityAttr {
            unit: unit
                .ok_or_else(|| syn::Error::new(proc_macro2::Span::call_site(), "missing `unit`"))?,
            default_type: default_type.unwrap_or_else(|| syn::parse_quote!(f64)),
        })
    }
}

fn quantities_crate() -> proc_macro2::TokenStream {
    match crate_name("quantities").expect("quantities must be in Cargo.toml") {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let ident = syn::Ident::new(&name, Span::call_site());
            quote! { #ident }
        }
    }
}

#[proc_macro_attribute]
pub fn quantity(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as QuantityAttr);
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;
    let vis = &input.vis;
    let unit_path = &attr.unit;
    let default_type = &attr.default_type;

    let krate = quantities_crate();

    quote! {
            #[derive(
                Clone,
                PartialEq, PartialOrd, Debug,
                derive_more::Add,
                derive_more::Sub,
                derive_more::Neg,
            )]
            #vis struct #name<T = #default_type> {
                value: T,
            }

            impl<T> #krate::common::Quantity<T> for #name<T>
            where
                T: Clone,
                #unit_path: #krate::common::UnitValue<T>,
            {
                type Unit = #unit_path;
                fn raw(value: T) -> Self { Self { value } }
                fn raw_value(&self) -> T { self.value.clone() }
            }

            impl<T> std::ops::Mul<T> for #name<T>
            where
                T: std::ops::Mul<Output = T>,
            {
                type Output = Self;
                fn mul(self, rhs: T) -> Self {
                    Self { value: self.value * rhs }
                }
            }

            impl<T> std::ops::Div<T> for #name<T>
            where
                T: std::ops::Div<Output = T>,
            {
                type Output = Self;
                fn div(self, rhs: T) -> Self {
                    Self { value: self.value / rhs }
                }
            }
    // These are impl for users such that they can use these without importing Quantity, but we leave those trait methods
    // if users want to add generic methods with Quantity and still want to use the 'new' or 'to'
    impl<T> #name<T> {
        pub fn new(value: T, unit: #unit_path) -> Self
        where
            T: std::ops::Mul<Output = T>,
            #unit_path: #krate::common::UnitValue<T>,
        {
            #name {
                value: value * unit.value(),
            }
        }

        pub fn to(&self, unit: #unit_path) -> T
        where
            T: std::ops::Div<Output = T> + Clone,
            #unit_path: #krate::common::UnitValue<T>,
        {
            self.value.clone() / unit.value()
        }
    }
        }
    .into()
}
