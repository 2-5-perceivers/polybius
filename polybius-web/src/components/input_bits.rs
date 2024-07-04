use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BitsInput {
    pub bits: usize,
    pub oninput: Callback<InputEvent>,
}

#[function_component(InputBits)]
pub fn input_bits(props: &BitsInput) -> Html {
    let value = props.bits.clone();
    let oninput = props.oninput.clone();

    html! {
        <div class="border-b border-gray-900/10 dark:border-gray-100/10 pb-8">
            <div class="grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                <div class="sm:col-span-4">
                    <label for="bits_count" class="block text-sm font-medium leading-6 text-gray-900 dark:text-gray-100">{"Password bits"}</label>
                    <span class="text-sm text-gray-600 dark:text-gray-400">{"Select the lenght of the generated passwords. A bit does not necesarly have a lenght of one character (default is 8)"}</span>
                    <div class="mt-2">
                        <div class="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 dark:ring-gray-700 focus-within:ring-2 focus-within:ring-inset focus-within:ring-primary-600 sm:max-w-md">
                            <input
                                id="bits_count"
                                name="bits_count"
                                type="number"
                                class="block flex-1 pl-3 border-0 bg-transparent py-1.5 pl-1 text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-600 focus:ring-0 sm:text-sm sm:leading-6"
                                placeholder="Number of bits"
                                value={value.to_string()}
                                oninput={oninput}
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
