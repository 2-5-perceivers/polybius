use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct StringInput {
    pub string: String,
    pub oninput: Callback<InputEvent>,
}

#[function_component(InputString)]
pub fn input_string(props: &StringInput) -> Html {
    let value = props.string.clone();
    let oninput = props.oninput.clone();

    html! {
        <div class="relative ">
            <input type="text"
                class="polybius-input"
                placeholder="Text value"
                value={value.clone()}
                oninput={oninput}
            />
        </div>
    }
}
