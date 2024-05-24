use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ListTileSwitchProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub subtitle: Option<AttrValue>,
    pub checked: bool,
    pub onclick: Callback<MouseEvent>,
}

#[function_component]
pub fn ListTileSwitch(props: &ListTileSwitchProps) -> Html {
    // Button clases

    let color_classes = if props.checked {
        classes!("bg-primary-500 dark:bg-primary-300 border-4".to_string())
    } else {
        classes!("bg-gray-300 dark:bg-gray-800 border-8".to_string())
    };

    let mut button_classes = classes!("relative inline-flex h-8 w-13 shrink-0 cursor-pointer rounded-full border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2".to_string());

    button_classes.extend(color_classes);

    // Toggle classes

    let position_classes = if props.checked {
        classes!("translate-x-5 w-6 h-6 bg-primary-100 dark:bg-primary-800".to_string())
    } else {
        classes!("translate-x-0 w-4 h-4 bg-gray-700 dark:bg-gray-400".to_string())
    };

    let mut toggle_classes = classes!(String::from("pointer-events-none inline-block rounded-full shadow ring-0 transition duration-200 ease-in-out"));

    toggle_classes.extend(position_classes);

    html! {
        <div class="flex items-center justify-between mt-8 gap-2">
            <span class="flex grow flex-col">
                <span class="text-sm font-medium leading-6 text-gray-900 dark:text-gray-100">
                    {props.title.clone()}
                </span>
                if let Some(subtitle) = &props.subtitle {
                    <span class="text-sm text-gray-600 dark:text-gray-400">
                        {subtitle.clone()}
                    </span>
                }
            </span>
            <button onclick={&props.onclick} aria-checked={props.checked.to_string()} class={button_classes} role="switch" type="button" tabindex="0">
                <span aria-hidden="true" class={toggle_classes} />
            </button>
        </div>
    }
}
