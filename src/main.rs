use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ButtonProps {
    n: u32,
    onclick: Callback<u32>,
}

fn format_money(amount: u32) -> String {
    if amount < 100 {
        format!("{amount}ct")
    } else {
        let amount = amount / 100;
        format!("{amount}€")
    }
}

enum ButtonMsg {
    Clicked,
}

struct Button;

impl Component for Button {
    type Message = ButtonMsg;
    type Properties = ButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button onclick={ctx.link().callback(|_|ButtonMsg::Clicked)} style="font-size: 150%"> {format_money(ctx.props().n)} </button>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg { ButtonMsg::Clicked => { ctx.props().onclick.emit(ctx.props().n) } };
        false
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |n| {
            let value = *counter + n;
            counter.set(value);
        }
    };

    html! {
        <div style="font-size: 200%;font-family: sans-serif">
            <h1 style="text-align: center">{ format!("{:.2}€", *counter as f32 / 100.0) }</h1>
            <div style="display: grid;grid-template-columns: auto auto;height: 80vh;">
                <Button n=10 onclick={onclick.clone()} />
                <Button n=100 onclick={onclick.clone()} />
                <Button n=20 onclick={onclick.clone()} />
                <Button n=200 onclick={onclick.clone()} />
                <Button n=50 onclick={onclick.clone()} />
                <Button n=30 onclick={onclick.clone()} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
