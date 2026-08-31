use syn::{Attribute, Result};

#[derive(Default, Clone)]
pub struct SystemParamFieldArgs {
    pub ignore: bool,
}

impl SystemParamFieldArgs {
    pub fn parse_from_field_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut args = Self::default();

        for attr in attrs {
            if attr.path().is_ident("venix") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("system_param") {
                        meta.parse_nested_meta(|inner| {
                            if inner.path.is_ident("ignore") {
                                args.ignore = true;
                                Ok(())
                            } else {
                                Err(inner.error("unrecognized parameter inside venix(system_param(...))"))
                            }
                        })?;
                        Ok(())
                    } else {
                        if meta.input.peek(syn::token::Paren) {
                            meta.parse_nested_meta(|_| Ok(()))?;
                        }
                        Ok(())
                    }
                })?;
            }
        }

        Ok(args)
    }
}
