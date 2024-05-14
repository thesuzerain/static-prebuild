use proc_macro::TokenStream;
use std::fs;
use syn_helpers::{
    quote,
    syn::{parse_macro_input, Data, DeriveInput, Fields},
};

#[proc_macro_derive(TextBuildable)]
pub fn derive_text_buildable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = match input.data {
        Data::Struct(ref data) => {
            let fields = match data.fields {
                Fields::Named(ref fields) => fields.named.iter().collect::<Vec<_>>(),
                Fields::Unnamed(ref fields) => fields.unnamed.iter().collect::<Vec<_>>(),
                Fields::Unit => vec![],
            };

            let field_names = fields
                .iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect::<Vec<_>>();

            let fake_struct = quote! {
                #(
                    #field_names: {},
                )*
            };

            let name_str = name.to_string();
            let fake_struct = fake_struct.to_string();

            // TODO: should this be String or &str?
            quote! {
                impl TextBuildable for #name {
                    fn to_code_text(&self) -> String {
                        let inner = format!(#fake_struct,
                            #(
                                self.#field_names.to_code_text(),
                            )*
                        );

                        format!(r#"{} {{ {} }}"#, #name_str, inner)
                    }
                }
            }
        }
        _ => panic!(),
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn include_json(path: TokenStream) -> TokenStream {
    // Convert the input TokenStream to a string
    let path_string = path.to_string().trim_matches('"').to_string();

    // Read the JSON file at the given path
    let text = fs::read_to_string(path_string).expect("Failed to read file");

    // Deserialize the JSON text into Rust data structure
    // TODO: needs to be T
    let deserialized: serde_json::Value =
        serde_json::from_str(&text).expect("Failed to deserialize");

    // Convert the deserialized data back into a Rust source code representation
    let generated_code = serde_json::to_string(&deserialized).expect("Failed to serialize");

    // Output the Rust code that, when compiled, will recreate the deserialized object
    let expanded = quote! {
        {
            const DATA: &str = #generated_code;
            let deserialized: TestStruct = serde_json::from_str(DATA).unwrap();
            deserialized
        }
    };

    // Convert the expanded code into a TokenStream
    TokenStream::from(expanded)
}
