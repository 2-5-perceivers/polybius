use polybius_lib::password_data::{Number, NumberType};
use yew::prelude::*;

use crate::traits::data_serialization::DataSerialization;

#[derive(Properties, PartialEq, Clone)]
pub struct NumericInput {
    pub number: Number,
    pub oninput: Callback<InputEvent>,
    pub onselect: Callback<InputEvent>,
    pub ondelete: Callback<MouseEvent>,
}

#[function_component(InputNumeric)]
pub fn input_numeric(props: &NumericInput) -> Html {
    let value = props.number.clone();
    let oninput = props.oninput.clone();
    let onselect = props.onselect.clone();
    let ondelete = props.ondelete.clone();

    let num_type_options = [NumberType::RelevantNumber, NumberType::BirthYear, NumberType::BirthMonth, NumberType::BirthDay].into_iter().map(|num_type| {
        html! {
            <option class="text-primary-900" value={num_type.into_string()} selected={value.num_type == num_type}>{num_type.to_string()}</option>
        }
    });

    html! {
        <div class="relative ">
            <input type="number"
            name="number"
            class="polybius-input"
            placeholder="0000"
            value={value.value.to_string()}
            oninput={oninput}
            />
            <div class="absolute inset-y-0 right-10 flex items-center">
                <label for="type" class="sr-only">{"Type"}</label>
                <select id="type" name="type" class="polybius-input-select" oninput={onselect}>
                    {for num_type_options}
                </select>
            </div>

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
