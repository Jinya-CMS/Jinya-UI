use yew::{Component, ComponentLink, Html};
use yew::prelude::*;

pub fn get_css<'a>() -> &'a str {
    // language=CSS
    "
.jinya-page {
    margin-left: 10%;
    margin-right: 10%;
    margin-top: 1.5rem;
}
"
}

pub struct Page {
    children: Children,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PageProps {
    pub children: Children
}

impl Component for Page {
    type Message = ();
    type Properties = PageProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Page {
            children: props.children,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        self.children = _props.children;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="jinya-page">
                {for self.children.iter().enumerate().map(|(_, mut child)| {
                    child
                })}
            </div>
        }
    }
}