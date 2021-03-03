use proc_macro::TokenStream;

mod yew_component;

#[proc_macro]
pub fn yew_component(input: TokenStream) -> TokenStream {
    yew_component::parse(input)
}
