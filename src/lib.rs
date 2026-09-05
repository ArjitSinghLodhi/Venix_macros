#![deprecated(since = "1.1.1", note = "use the avenix crate instead")]

extern crate proc_macro;
mod attributes;
mod macros {
    pub mod component_bundle;
    pub mod query_filter;
    pub mod query_data;
    pub mod system_param;
}



use proc_macro::TokenStream;

#[proc_macro_derive(ComponentBundle, attributes(venix))]
pub fn derive_bundle(input: TokenStream) -> TokenStream {
    macros::component_bundle::derive_bundle_impl(input)
}

#[proc_macro_derive(QueryFilter, attributes(venix))]
pub fn derive_filter(input: TokenStream) -> TokenStream {
    macros::query_filter::derive_filter_impl(input)
}

#[proc_macro_derive(QueryData, attributes(venix))]
pub fn derive_world_query(input: TokenStream) -> TokenStream {
    macros::query_data::derive_query_data_impl(input)
}

#[proc_macro_derive(SystemParam, attributes(venix))]
pub fn derive_system_param(input: TokenStream) -> TokenStream {
    macros::system_param::derive_system_param_impl(input)
}
