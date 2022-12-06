use crate::ast;
use crate::backends::rust::types;
use quote::{format_ident, quote};

pub struct FieldDeclarations {
    code: Vec<proc_macro2::TokenStream>,
}

impl FieldDeclarations {
    pub fn new() -> FieldDeclarations {
        FieldDeclarations { code: Vec::new() }
    }

    pub fn add(&mut self, field: &ast::Field) {
        self.code.push(match field {
            ast::Field::Scalar { id, width, .. } => {
                let id = format_ident!("{id}");
                let field_type = types::Integer::new(*width);
                quote! {
                    #id: #field_type,
                }
            }
            _ => todo!(),
        });
    }
}

impl quote::ToTokens for FieldDeclarations {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let code = &self.code;
        tokens.extend(quote! {
            #(#code)*
        });
    }
}
