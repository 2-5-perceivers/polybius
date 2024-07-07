use chrono::Datelike;
use polybius_lib::{
    password_bits::PasswordBits,
    password_data::{Number, NumberType, PasswordData},
    password_generation::{GenerationSettings, PasswordGeneration},
};
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

use crate::{
    components::{
        input_bits::InputBits, input_numeric::InputNumeric, input_string::InputString,
        list_tile_switch::ListTileSwitch,
    },
    traits::data_serialization::DataSerialization,
};

pub enum Msg {
    AddNumericInput,
    AddStringInput,
    RemoveNumericInput(usize),
    RemoveStringInput(usize),
    UpdateNumericValueInput(usize, u16),
    UpdateNumericTypeInput(usize, NumberType),
    UpdateStringInput(usize, String),
    UpdatePasswordBits(usize),
    FlipAddYear,
    FlipAddSymbols,
    GeneratePasswords,
}

pub struct FormComponent {
    pub numeric_values: Vec<Number>,
    pub string_values: Vec<String>,
    pub add_year: bool,
    pub add_symbols: bool,
    pub password_bits: usize,
    pub passwords: Option<Vec<PasswordBits>>,
}

impl Component for FormComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            numeric_values: vec![],
            string_values: vec![],
            add_year: false,
            add_symbols: true,
            password_bits: 8,
            passwords: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddNumericInput => {
                self.numeric_values.push(Number::default());
            }
            Msg::AddStringInput => {
                self.string_values.push(String::new());
            }
            Msg::RemoveNumericInput(index) => {
                self.numeric_values.remove(index);
            }
            Msg::RemoveStringInput(index) => {
                self.string_values.remove(index);
            }
            Msg::UpdateNumericValueInput(index, value) => {
                self.numeric_values[index].value = value;
                console::log_1(&format!("{:?}", self.numeric_values).as_str().into());
            }
            Msg::UpdateNumericTypeInput(index, value) => {
                self.numeric_values[index].num_type = value;
                console::log_1(&format!("{:?}", self.numeric_values).as_str().into());
            }
            Msg::UpdateStringInput(index, value) => {
                self.string_values[index] = value;
            }
            Msg::FlipAddYear => {
                self.add_year = !self.add_year;
            }
            Msg::FlipAddSymbols => {
                self.add_symbols = !self.add_symbols;
            }
            Msg::UpdatePasswordBits(vaue) => {
                self.password_bits = vaue;
            }
            Msg::GeneratePasswords => {
                let mut passwords: Vec<PasswordBits> = vec![];
                let numbers: Vec<Number> = {
                    if self.add_year {
                        let current_year = chrono::Local::now().year();
                        vec![
                            vec![Number::new(current_year as u16, NumberType::CurrentYear)],
                            self.numeric_values.clone(),
                        ]
                        .concat()
                    } else {
                        self.numeric_values.clone()
                    }
                };
                let password_data = PasswordData::new(numbers, self.string_values.clone());
                let password_settings = GenerationSettings {
                    length: self.password_bits, // Test value
                    symbols: self.add_symbols,
                };

                // Generate the passwords
                for _ in 0..10 {
                    passwords.push(password_data.generate_password(&password_settings));
                }

                self.passwords = Some(passwords);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let numeric_inputs = self
            .numeric_values
            .iter()
            .enumerate()
            .map(|(index, number)| {
                html! {
                    <InputNumeric
                        number={number.clone()}
                        oninput={ctx.link().callback(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateNumericValueInput(index, input.value().parse().unwrap_or_default())
                        })}
                        onselect={ctx.link().callback(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateNumericTypeInput(index, NumberType::from_string(&input.value()))
                        })}
                        ondelete={ctx.link().callback(move |_| Msg::RemoveNumericInput(index))}
                    />
                }
            });

        let string_inputs = self
            .string_values
            .iter()
            .enumerate()
            .map(|(index, string)| {
                html! {
                    <InputString
                        string={string.clone()}
                        oninput={ctx.link().callback(move |e: InputEvent| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Msg::UpdateStringInput(index, input.value())
                        })}
                        ondelete={ctx.link().callback(move |_| Msg::RemoveStringInput(index))}
                    />
                }
            });

        html! {
            <form onsubmit={|e: SubmitEvent| e.prevent_default()} class="w-full sm:max-w-screen-sm md:max-w-screen-md lg:max-w-screen-lg mx-auto px-4 sm:px-5 lg:px-8">
                <div class="space-y-8">
                    <div class="border-b border-gray-900/10 dark:border-gray-100/10 pb-8">
                        <h2 class="text-base font-semibold leading-7 text-gray-900 dark:text-gray-100">{"Information"}</h2>
                        <p class="mt-1 text-sm leading-6 text-gray-600 dark:text-gray-400">{"This information will be used to generate a memorable random password for you. This data is not leaving the browser."}</p>

                        <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                            <div class="sm:col-span-6 lg:col-span-3">
                                <h3 class="text-base font-semibold leading-7 text-gray-900 dark:text-gray-100">{"Numeric information"}</h3>

                                <div class="flex flex-col gap-2">
                                    { for numeric_inputs }
                                </div>

                                <button type="button" class="w-full polybius-button" onclick={ctx.link().callback(|_| Msg::AddNumericInput)}>
                                    {"Add numeric value"}
                                    <svg class="w-4 h-4 ml-2 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                                    </svg>
                                </button>

                                <ListTileSwitch title={"Add current year"} subtitle={"This will add the current year to the password numeric information"} checked={self.add_year} onclick={ctx.link().callback(|_| Msg::FlipAddYear)} />
                            </div>

                            <div class="sm:col-span-6 lg:col-span-3">
                                <h3 class="text-base font-semibold leading-7 text-gray-900 dark:text-gray-100">{"Text information"}</h3>

                                <div class="flex flex-col gap-2">
                                    { for string_inputs }
                                </div>

                                <button type="button" class="w-full polybius-button" onclick={ctx.link().callback(|_| Msg::AddStringInput)}>
                                    {"Add text value"}
                                    <svg class="w-4 h-4 ml-2 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"></path>
                                    </svg>
                                </button>

                                <ListTileSwitch title={"Add symbols"} subtitle={"This will add symbols like !@#$%^&*(). For more control you can add them manually"} checked={self.add_symbols} onclick={ctx.link().callback(|_| Msg::FlipAddSymbols)} />
                            </div>
                        </div>
                    </div>

                    <InputBits bits={self.password_bits} oninput={ctx.link().callback(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Msg::UpdatePasswordBits(input.value().parse().unwrap_or(8))
                    })}/>

                    <div class="pb-8 flex justify-end">
                        <button
                            class="polybius-button "
                            onclick={ctx.link().callback(|_| {
                                Msg::GeneratePasswords
                            })}
                        >
                            {"Generate passwords"}
                        </button>
                    </div>

                    if let Some(passwords) = &self.passwords {
                        <div>
                            { for passwords.iter().map(|password| html! { <p class="dark:text-gray-100">{ password.iter().map(|e| e.bits.to_string()).collect::<String>() }</p> }) }
                        </div>
                    }

                </div>
            </form>
        }
    }
}
