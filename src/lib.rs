use proc_macro::TokenStream;

mod component;

#[proc_macro]
pub fn component(input: TokenStream) -> TokenStream {
    component::parse(input)
}
