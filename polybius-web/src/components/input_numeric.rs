use polybius_lib::password_data::{Number, NumberType};
use yew::prelude::*;

use crate::traits::data_serialization::DataSerialization;

#[derive(Properties, PartialEq, Clone)]
pub struct NumericInput {
    pub number: Number,
    pub oninput: Callback<InputEvent>,
    pub onselect: Callback<InputEvent>,
}

#[function_component(InputNumeric)]
pub fn input_numeric(props: &NumericInput) -> Html {
    let value = props.number.clone();
    let oninput = props.oninput.clone();
    let onselect = props.onselect.clone();

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
            <div class="absolute inset-y-0 right-0 flex items-center">
                <label for="type" class="sr-only">{"Type"}</label>
                <select id="type" name="type" class="polybius-input-select" oninput={onselect}>
                    {for num_type_options}
                </select>
            </div>
        </div>
    }
}
