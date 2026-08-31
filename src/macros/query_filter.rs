use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn derive_filter_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data_struct) => &data_struct.fields,
        _ => {
            return syn::Error::new_spanned(name, "Filter can only be derived on structs")
                .to_compile_error()
                .into();
        }
    };

    let mut field_types = Vec::new();
    let mut scratch_idents = Vec::new();

    let is_named = matches!(fields, Fields::Named(_));
    let is_unnamed = matches!(fields, Fields::Unnamed(_));

    match fields {
        Fields::Named(fields_named) => {
            for field in &fields_named.named {
                field_types.push(field.ty.clone());
                let ident = field.ident.clone().unwrap();
                scratch_idents.push(syn::Ident::new(&format!("_{}", ident), proc_macro2::Span::call_site()));
            }
        }
        Fields::Unnamed(fields_unnamed) => {
            for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                field_types.push(field.ty.clone());
                scratch_idents.push(syn::Ident::new(&format!("_field_{}", i), proc_macro2::Span::call_site()));
            }
        }
        Fields::Unit => {}
    }
    let link_fields_check = if is_named {
        let mut field_mappings = Vec::new();
        match fields {
            Fields::Named(fields_named) => {
                for (real_ident, scratch_ident) in fields_named.named.iter().map(|f| f.ident.as_ref().unwrap()).zip(&scratch_idents) {
                    field_mappings.push(quote! { #real_ident: #scratch_ident });
                }
            }
            _ => {}
        }
        
        quote! {
            if false {
                let #name { #( #field_mappings ),* } = unsafe { ::std::mem::zeroed() };
            }
        }
    } else if is_unnamed {
        quote! {
            if false {
                let #name ( #( #scratch_idents, )* ) = unsafe { ::std::mem::zeroed() };
            }
        }
    } else {
        quote! {}
    };


    let expanded = quote! {
        impl #impl_generics ::venix::query::filter::QueryFilter for #name #ty_generics #where_clause {
            #[inline(always)]
            fn matches(types: &::venix::extensions::AccessHashSet<::std::any::TypeId>) -> bool {
                #link_fields_check

                <( #(#field_types,)* ) as ::venix::query::filter::QueryFilter>::matches(types)
            }

            #[inline(always)]
            fn matches_negated(types: &::venix::extensions::AccessHashSet<::std::any::TypeId>) -> bool {
                <( #(#field_types,)* ) as ::venix::query::filter::QueryFilter>::matches_negated(types)
            }

            #[inline(always)]
            fn collect_filter(
                withs: &mut ::venix::extensions::AccessVec<::std::any::TypeId>,
                withouts: &mut ::venix::extensions::AccessVec<::std::any::TypeId>,
            ) {
                <( #(#field_types,)* ) as ::venix::query::filter::QueryFilter>::collect_filter(withs, withouts);
            }

            #[inline(always)]
            fn filter_indices(
                archetype: &::venix::extensions::Archetype,
                indices: &mut ::std::vec::Vec<usize>,
                system_data: &mut ::venix::extensions::FunctionData,
            ) {
                <( #(#field_types,)* ) as ::venix::query::filter::QueryFilter>::filter_indices(archetype, indices, system_data);
            }
        }
    };

    TokenStream::from(expanded)
}
