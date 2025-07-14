use proc_macro::TokenStream;
use syn::parse_macro_input;

mod parser;
mod generate;

#[proc_macro]
pub fn flow(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as parser::Flow);
    generate::generate_flow(parsed).into()
}

#[proc_macro]
pub fn sequential(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as parser::SequentialBlock);
    generate::generate_sequential(parsed).into()
}