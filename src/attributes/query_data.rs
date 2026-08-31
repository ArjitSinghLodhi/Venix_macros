use syn::{Result, meta::ParseNestedMeta, punctuated::Punctuated, token::Comma};

#[derive(Default)]
pub struct QueryArgs {
    pub derives: Vec<syn::Path>,
}

impl QueryArgs {
    pub fn parse_nested(&mut self, meta: &ParseNestedMeta) -> Result<()> {
        meta.parse_nested_meta(|inner| {
            if inner.path.is_ident("derive") {
                let content;
                syn::parenthesized!(content in inner.input);
                
                let paths: Punctuated<syn::Path, Comma> = Punctuated::parse_terminated(&content)?;
                for path in paths {
                    self.derives.push(path);
                }
                Ok(())
            } else {
                Err(inner.error("unrecognized parameter inside venix(query_data(...))"))
            }
        })
    }
}
