use yew::prelude::*;

use crate::components::{form::FormComponent, navbar::NavBar};

#[function_component]
pub fn App() -> Html {
    html! {
        <>
            <NavBar />
            <main class="pt-16">
                <FormComponent />
            </main>
        </>
    }
}
