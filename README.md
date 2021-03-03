# Yew component

## A simple macro to generate yew components

A simple macro to generate yew components from a single struct declaration

## How to use

1. Import the macro in your `Cargo.toml`

```toml
yew-component = "<version-number-here>"
yewtil = "<version-number-here>"
```

2. Declare your component

```rust
use yew_component::component;
use yew::{ShouldRender, Html, html};

component! {
    pub struct SampleComponent {
        // Declare your Message variants here
        type Message = {
            Login,
            SignUp,
            OnDataLoad
        }

        // Declare your props here
        type Props = {
            enabled: bool,
            error_message: Option<String>,
        }

        // Declare your state here
        type State = {
            value: u32,
            username: String,
        }

        // Declare your update function as well
        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            match msg {
                Self::Message::Login => {
                    // self.state contains the state value
                    self.state.value = 2;
                    true
                }
                Self::Message::SignUp => {
                    self.state.value = 1;
                    true
                }
                Self::Message::OnDataLoad => {
                    self.state.username = get_data();
                    false
                }
            }
        }

        // Declare the function that will create your state
        fn create(link: &mut ComponentLink<SampleComponent>) -> Self {
            Self {
                value: 0,
                username: String::default()
            }
        }

        fn view(&self) -> Html {
            html! {
                <div>
                    {
                        format!(
                            "My sample component. Value is: {}",
                            self.state.value
                        )
                    }
                </div>
            }
        }
    }
}
```

3. It will automatically expand to the following:

```rust

pub enum SampleComponentMessage {
    Login,
    SignUp,
    OnDataLoad
}

#[derive(yew::Properties, Clone, Debug, PartialEq)]
pub struct SampleComponentProps {
    enabled: bool,
    error_message: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SampleComponentState {
    value: u32,
    username: String,
}

impl SampleComponentState {
    fn create(link: &mut ComponentLink<SampleComponent>) -> Self {
        Self {
            value: 0,
            username: String::default()
        }
    }
}

pub struct SampleComponent {
    link: yew::ComponentLink<Self>,
    props: SampleComponentProps,
    state: SampleComponentState,
}

impl yew::Component for SampleComponent {
    type Properties = SampleComponentProps;
    type Message = SampleComponentMessage;

    fn create(props: Self::Properties, mut link: yew::ComponentLink<Self>) -> Self {
        let state = SampleComponentState::create(&mut link);
        Self {
            link,
            props,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::Login => {
                self.state.value = 2;
                true
            }
            Self::Message::SignUp => {
                self.state.value = 1;
                true
            }
            Self::Message::OnDataLoad => {
                self.state.username = get_data();
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        use yewtil::NeqAssign;

        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {
                    format!(
                        "My sample component. Value is: {}",
                        self.state.value
                    )
                }
            </div>
        }
    }
}
```

This makes it much more easier to generate a new component
