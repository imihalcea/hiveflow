use proc_macro::TokenStream;
use syn::parse_macro_input;

mod sequential;
mod parallel;
mod flow;

#[proc_macro]
pub fn flow(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as flow::parser::Flow);
    flow::generate::generate_flow(parsed).into()
}

#[proc_macro]
pub fn sequential(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as sequential::parser::SequentialBlock);
    sequential::generate::generate_sequential(parsed).into()
}

#[proc_macro]
pub fn parallel(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as parallel::parser::ParallelBlock);
    parallel::generate::generate_parallel(parsed).into()
}