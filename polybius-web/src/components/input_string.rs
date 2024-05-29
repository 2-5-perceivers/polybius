use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct StringInput {
    pub string: String,
    pub oninput: Callback<InputEvent>,
    pub ondelete: Callback<MouseEvent>,
}

#[function_component(InputString)]
pub fn input_string(props: &StringInput) -> Html {
    let value = props.string.clone();
    let oninput = props.oninput.clone();
    let ondelete = props.ondelete.clone();

    html! {
        <div class="relative ">
            <input type="text"
                class="polybius-input"
                placeholder="Text value"
                value={value.clone()}
                oninput={oninput}
            />
            // An x button to remove the input
            <button
                class="absolute inset-y-0 right-0 aspect-square flex items-center justify-center"
                onclick={ondelete}
            >
                <svg class="w-4 h-4 text-primary-500" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                </svg>
            </button>
        </div>
    }
}
