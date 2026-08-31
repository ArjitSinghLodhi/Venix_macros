use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};
use crate::attributes::system_param::SystemParamFieldArgs;

pub fn derive_system_param_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return syn::Error::new_spanned(name, "SystemParam can only be derived on structs")
                .to_compile_error()
                .into();
        }
    };

    let mut param_types = Vec::new();
    let mut param_idents = Vec::new();
    let mut ignored_idents = Vec::new();

    let is_named = matches!(fields, Fields::Named(_));
    let is_unnamed = matches!(fields, Fields::Unnamed(_));

    match fields {
        Fields::Named(fields_named) => {
            for field in &fields_named.named {
                let ident = field.ident.clone().unwrap();
                let field_args = match SystemParamFieldArgs::parse_from_field_attributes(&field.attrs) {
                    Ok(args) => args,
                    Err(err) => return err.to_compile_error().into(),
                };

                if field_args.ignore {
                    ignored_idents.push(ident);
                } else {
                    param_types.push(field.ty.clone());
                    param_idents.push(ident);
                }
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                let dummy_ident =
                    syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site());
                let field_args = match SystemParamFieldArgs::parse_from_field_attributes(&field.attrs) {
                    Ok(args) => args,
                    Err(err) => return err.to_compile_error().into(),
                };

                if field_args.ignore {
                    ignored_idents.push(dummy_ident);
                } else {
                    param_types.push(field.ty.clone());
                    param_idents.push(dummy_ident);
                }
            }
        }
        Fields::Unit => {}
    }

    let extract_fields = if is_named {
        quote! {
            #name {
                #( #param_idents: <#param_types as ::venix::extensions::SystemParam>::extract(world, system_data), )*
                #( #ignored_idents: ::std::default::Default::default(), )*
            }
        }
    } else if is_unnamed {
        let mut tuple_extractors = Vec::new();
        let mut param_idx = 0;

        for field in fields.iter() {
            let field_args = match SystemParamFieldArgs::parse_from_field_attributes(&field.attrs) {
                Ok(args) => args,
                Err(err) => return err.to_compile_error().into(),
            };

            if field_args.ignore {
                tuple_extractors.push(quote! { ::std::default::Default::default() });
            } else {
                let ty = &param_types[param_idx];
                tuple_extractors.push(quote! { <#ty as ::venix::extensions::SystemParam>::extract(world, system_data) });
                param_idx += 1;
            }
        }

        quote! {
            #name( #( #tuple_extractors ),* )
        }
    } else {
        quote! { #name }
    };

    let expanded = quote! {
        impl #impl_generics ::venix::extensions::SystemParam for #name #ty_generics #where_clause {
            #[inline(always)]
            fn get_access() -> ::venix::extensions::ParamAccess {
                let mut access = ::venix::extensions::ParamAccess::default();

                #(
                    let mut field_access = <#param_types as ::venix::extensions::SystemParam>::get_access();
                    access.extend(&mut field_access);
                )*

                access
            }

            #[inline(always)]
            fn extract(
                world: &mut ::venix::prelude::World,
                system_data: &mut ::venix::extensions::FunctionData,
            ) -> Self {
                #extract_fields
            }
        }
    };

    TokenStream::from(expanded)
}
