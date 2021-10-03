use quote::quote;
use std::collections::HashSet;

use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, Field, Ident, ItemStruct, LitStr,
    Token,
};

struct Args {
    new_struct_name: LitStr,
    fields: HashSet<Ident>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let new_struct_name: LitStr = input.parse()?;
        let _ = input.parse::<Token![,]>()?;
        let fields = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

        Ok(Args {
            new_struct_name,
            fields: fields.into_iter().collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn simple_struct(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let args = parse_macro_input!(args as Args);

    let copy_fields = input
        .clone()
        .fields
        .into_iter()
        .filter(|original_field| {
            args.fields
                .iter()
                .find(|field| original_field.ident.as_ref().unwrap() == *field)
                .is_some()
        })
        .collect::<Vec<Field>>();

    let new_struct_ident = Ident::new(&args.new_struct_name.value(), args.new_struct_name.span());

    let output = quote! {
        #input

        #[derive(Debug)]
        struct #new_struct_ident {
            #(#copy_fields),*
        }
    };

    output.into()
}
