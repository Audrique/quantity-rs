use crate::argument_parser::QuantityArgs;
use crate::error::QuantityError;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse2, DeriveInput, Path};

fn quantities_crate() -> Result<TokenStream2, QuantityError> {
    match crate_name("quantities")? {
        FoundCrate::Itself => Ok(quote! { crate }),
        FoundCrate::Name(name) => {
            let ident = syn::Ident::new(&name, Span::call_site());
            Ok(quote! { #ident })
        }
    }
}

fn is_unit_struct(data: &syn::Data) -> bool {
    match data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Unit => true,
            syn::Fields::Named(fields) if fields.named.is_empty() => true,
            _ => false,
        },
        _ => false,
    }
}

fn generate_struct_def(
    name: &syn::Ident,
    vis: &syn::Visibility,
    default_type: &syn::Type,
) -> TokenStream2 {
    quote! {
        #[derive(
            Clone,
            PartialEq,
            PartialOrd,
            Debug,
            derive_more::Add,
            derive_more::Sub,
            derive_more::Neg,
        )]
        #vis struct #name<T = #default_type> {
            value: T,
        }
    }
}

fn generate_unit_value_assertion(
    krate: &TokenStream2,
    unit_path: &Path,
    default_type: &syn::Type,
) -> TokenStream2 {
    quote! {
    const _: () = {
                const fn assert_implemented<U>()
                where
                    U: #krate::common::UnitValue<#default_type>,
                {}

                assert_implemented::<#unit_path>();
            };
    }
}

fn generate_quantity_trait_impl(
    name: &syn::Ident,
    krate: &TokenStream2,
    unit_path: &Path,
) -> TokenStream2 {
    quote! {
        impl<T> #krate::common::Quantity<T> for #name<T>
        where
            T: Clone,
            #unit_path: #krate::common::UnitValue<T>,
        {
            type Unit = #unit_path;
            fn raw(value: T) -> Self { Self { value } }
            fn raw_value(&self) -> T { self.value.clone() }
        }
    }
}

fn generate_mul_div_impls(name: &syn::Ident) -> TokenStream2 {
    quote! {
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
    }
}

fn generate_user_methods(
    name: &syn::Ident,
    krate: &TokenStream2,
    unit_path: &Path,
) -> TokenStream2 {
    // Note that we use UFCS syntax to make missing_unitvalue_impl.rs pass
    quote! {
        impl<T> #name<T> {
            pub fn new(value: T, unit: #unit_path) -> Self
            where
                T: std::ops::Mul<Output = T>,
                #unit_path: #krate::common::UnitValue<T>,
            {
                #name {
                    value: value * <#unit_path as #krate::common::UnitValue<T>>::value(&unit),
                }
            }

            pub fn to(&self, unit: #unit_path) -> T
            where
                T: std::ops::Div<Output = T> + Clone,
                #unit_path: #krate::common::UnitValue<T>,
            {
                self.value.clone() / <#unit_path as #krate::common::UnitValue<T>>::value(&unit)
            }
        }
    }
}

pub fn try_expand(attribute: TokenStream, item: TokenStream) -> Result<TokenStream2, syn::Error> {
    let arguments = parse2::<QuantityArgs>(TokenStream2::from(attribute))?;
    let input = parse2::<DeriveInput>(TokenStream2::from(item))?;

    let krate = quantities_crate().map_err(|e| syn::Error::new(Span::call_site(), e))?;

    if !is_unit_struct(&input.data) {
        return Err(syn::Error::new(
            input.ident.span(),
            QuantityError::NotUnitStruct,
        ));
    }

    let name = &input.ident;
    let unit_path = &arguments.unit;
    let default_type = &arguments.default_type;

    let struct_def = generate_struct_def(name, &input.vis, default_type);
    let unit_value_check = generate_unit_value_assertion(&krate, unit_path, default_type);
    let quantity_impl = generate_quantity_trait_impl(name, &krate, unit_path);
    let ops_impls = generate_mul_div_impls(name);
    let conv_methods = generate_user_methods(name, &krate, unit_path);

    let expanded = quote! {
        #struct_def
        #unit_value_check
        #quantity_impl
        #ops_impls
        #conv_methods
    };

    Ok(expanded)
}
