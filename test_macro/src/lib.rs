use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(VisitFields)]
pub fn visit_fields_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // 提取结构体字段
    TokenStream::from(match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| &f.ident);
                let field_types = fields.named.iter().map(|f| &f.ty);

                quote! {
                    impl #struct_name {
                        pub fn visit_fields(&self) {
                            #(
                                println!(
                                    "{}: {} = {:?}",
                                    stringify!(#field_names),
                                    stringify!(#field_types),
                                    self.#field_names
                                );
                            )*
                        }
                    }
                }
            },
            Fields::Unnamed(fields) => {
                let field_indices = 0..fields.unnamed.len();
                let field_types = fields.unnamed.iter().map(|f| &f.ty);

                quote! {
                    impl #struct_name {
                        pub fn visit_fields(&self) {
                            #(
                                println!(
                                    "{}: {} = {:?}",
                                    #field_indices,
                                    stringify!(#field_types),
                                    self.#field_indices
                                );
                            )*
                        }
                    }
                }
            },
            Fields::Unit => {
                quote! {
                    impl #struct_name {
                        pub fn visit_fields(&self) {}
                    }
                }
            }
        },
        _ => panic!("仅支持结构体"),
    })
}