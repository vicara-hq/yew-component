use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
	braced,
	parse::{Parse, ParseStream},
	parse_macro_input,
	punctuated::Punctuated,
	Error, Field, Ident, ItemFn, Result, Token, Variant,
	Visibility,
};

#[allow(dead_code)]
struct YewComponent {
	visibility: Visibility,
	name: Ident,
	message_variants: Punctuated<Variant, Token![,]>,
	props_fields: Punctuated<Field, Token![,]>,
	state_fields: Punctuated<Field, Token![,]>,
	update_fn: ItemFn,
	create_fn: ItemFn,
	view_fn: ItemFn,
}

impl Parse for YewComponent {
	fn parse(input: ParseStream) -> Result<Self> {
		let visibility = input.parse()?;
		input.parse::<Token![struct]>()?;
		let name: Ident = input.parse()?;

		let content;
		braced!(content in input);

		let mut message_variants = None;
		let mut props_fields = None;
		let mut state_fields = None;
		let mut update_fn = None;
		let mut create_fn = None;
		let mut view_fn = None;

		while !content.is_empty() {
			if content.peek(Token![type]) {
				content.parse::<Token![type]>()?;
				let type_name: Ident = content.parse()?;
				match type_name.to_string().as_str() {
					"Message" => {
						content.parse::<Token![=]>()?;
						let variants;
						braced!(variants in content);
						if message_variants.is_some() {
							return Err(Error::new(
								content.span(),
								"type Message defined twice",
							));
						}
						message_variants =
							Some(variants.parse_terminated(Variant::parse)?);
					}
					"Props" => {
						content.parse::<Token![=]>()?;
						let fields;
						braced!(fields in content);
						if props_fields.is_some() {
							return Err(Error::new(
								content.span(),
								"type Props defined twice",
							));
						}
						props_fields =
							Some(fields.parse_terminated(Field::parse_named)?);
					}
					"State" => {
						content.parse::<Token![=]>()?;
						let state;
						braced!(state in content);
						if state_fields.is_some() {
							return Err(Error::new(
								content.span(),
								"type State defined twice",
							));
						}
						state_fields =
							Some(state.parse_terminated(Field::parse_named)?);
					}
					_ => {
						return Err(Error::new(content.span(), "type Message, type Props, type State are the only allowed params"));
					}
				}
			} else if content.peek(Token![fn]) {
				let fn_type: ItemFn = content.parse()?;
				match fn_type.sig.ident.to_string().as_str() {
					"update" => {
						update_fn = Some(fn_type);
					}
					"create" => {
						create_fn = Some(fn_type);
					}
					"view" => {
						view_fn = Some(fn_type);
					}
					_ => {
						return Err(Error::new(content.span(), "fn update, fn create or fn view are the only allowed params"));
					}
				}
			} else {
				return Err(Error::new(content.span(), "type Message, type Props, type State, fn update, fn create or fn view are the only allowed params"));
			}
		}

		if message_variants.is_none() {
			return Err(Error::new(content.span(), "type Message not defined"));
		}
		if props_fields.is_none() {
			return Err(Error::new(content.span(), "type Props not defined"));
		}
		if state_fields.is_none() {
			return Err(Error::new(content.span(), "type State not defined"));
		}
		if update_fn.is_none() {
			return Err(Error::new(content.span(), "fn update not defined"));
		}
		if create_fn.is_none() {
			return Err(Error::new(content.span(), "fn create not defined"));
		}
		if view_fn.is_none() {
			return Err(Error::new(content.span(), "fn view not defined"));
		}

		Ok(Self {
			visibility,
			name,
			message_variants: message_variants.unwrap(),
			props_fields: props_fields.unwrap(),
			state_fields: state_fields.unwrap(),
			update_fn: update_fn.unwrap(),
			create_fn: create_fn.unwrap(),
			view_fn: view_fn.unwrap(),
		})
	}
}

#[allow(unused_variables)]
pub fn parse(input: TokenStream) -> TokenStream {
	let YewComponent {
		visibility,
		name,
		message_variants,
		props_fields,
		state_fields,
		update_fn,
		view_fn,
		create_fn,
	} = parse_macro_input!(input as YewComponent);

	let props_name = format_ident!("{}Props", name.to_string());
	let state_name = format_ident!("{}State", name.to_string());

	let expanded = quote! {
		#visibility enum Message {
			#message_variants
		}

		#[derive(yew::Properties, Clone, Debug, PartialEq)]
		#visibility struct #props_name {
			#props_fields
		}

		#[derive(Clone, Debug, PartialEq)]
		#visibility struct #state_name {
			#state_fields
		}

		impl #state_name {
			#create_fn
		}

		#visibility struct #name {
			link: yew::ComponentLink<Self>,
			props: #props_name,
			state: #state_name,
		}

		impl yew::Component for #name {
			type Properties = #props_name;
			type Message = Message;

			fn create(props: Self::Properties, mut link: yew::ComponentLink<Self>) -> Self {
				let state = #state_name ::create(&mut link);
				Self {
					link,
					props,
					state,
				}
			}

			#update_fn

			fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
				use yewtil::NeqAssign;

				self.props.neq_assign(props)
			}

			#view_fn
		}
	};

	TokenStream::from(expanded)
}
