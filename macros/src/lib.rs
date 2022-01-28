use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn request_obj(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let output = quote! {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        #input
    };

    output.into()
}

#[proc_macro_attribute]
pub fn response_obj(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let output = quote! {
        #[derive(Serialize, Clone, Debug)]
        #input
    };

    output.into()
}

#[proc_macro_derive(Request)]
pub fn derive_request(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl server::model::Request for #ident {
            fn string_body_to_obj(body: String) -> Self
                where Self: ::serde::Serialize + ::serde::Deserialize<'static> + Sized + Clone {
                let b = &body[..];
                serde_json::from_str(b).unwrap()
            }
        }
    };

    output.into()
}

#[proc_macro_derive(Response)]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl server::model::Response for #ident {
            fn to_string_json(&self) -> String {
                serde_json::to_string_pretty(&self.clone()).unwrap()
            }
        }
    };

    output.into()
}