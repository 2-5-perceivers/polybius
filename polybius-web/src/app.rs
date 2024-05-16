use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "Polybius!" }</h1>
            <span class="subtitle">{ "from 2.5 Perceivers with " }<i class="heart" /></span>
        </main>
    }
}
