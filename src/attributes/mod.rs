pub mod query_data;
pub mod system_param;

use syn::{Attribute, Result};

#[derive(Default)]
pub struct VenixArgs {
    pub query_data: query_data::QueryArgs,
}

impl VenixArgs {
    pub fn parse_from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut settings = Self::default();

        for attr in attrs {
            if attr.path().is_ident("venix") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("query_data") {
                        settings.query_data.parse_nested(&meta)?;
                        Ok(())
                    } else if meta.path.is_ident("system_param") 
                    {
                        if meta.input.peek(syn::token::Paren) {
                            meta.parse_nested_meta(|_| Ok(()))?;
                        }
                        Ok(())
                    } else {
                        Err(meta.error("unrecognized venix attribute category"))
                    }
                })?;
            }
        }

        Ok(settings)
    }
}

