extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(JsonTracing)]
pub fn json_tracing(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let expanded = match &input.data {
        syn::Data::Struct(_s) => {
            // let field_names: Vec<_> = s.fields.iter().map(|f| &f.ident).collect();
            quote! {
                impl std::fmt::Debug for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let mut json = serde_json::to_value(&self).unwrap();

                        let json = serde_json::json!({
                            "type" : stringify!(#name),
                            "value" : json
                        });
                        let json = serde_json::to_string(&json).unwrap();
                        write!(f, "{}", json)

                        // write!(f, "{} {{ ", stringify!(#name))?;
                        // #(
                        //     write!(f, "{}, ", stringify!(#field_names))?;
                        // )*
                        // write!(f, "}}")
                    }
                }
            }
        }
        _ => panic!("CustomDebug is only defined for structs"),
    };

    TokenStream::from(expanded)
}
