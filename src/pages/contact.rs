use yew::prelude::*;

pub struct Contact;

impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"Contact"}</h1>
        }
    }
}
