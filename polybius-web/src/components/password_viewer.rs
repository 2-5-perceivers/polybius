use polybius_lib::password_bits::PasswordBits;
use web_sys::window;
use yew::prelude::*;

use crate::components::copy_icon::CopyIcon;

#[derive(Properties, PartialEq, Clone)]
pub struct PasswordViewerProps {
    pub password: PasswordBits,
}

#[function_component]
pub fn PasswordViewer(props: &PasswordViewerProps) -> Html {
    let copy_to_clipboard = {
        let password = props
            .password
            .clone()
            .iter()
            .map(move |bit| bit.bits.as_str())
            .collect::<String>();
        Callback::from(move |_| {
            if let Some(window) = window() {
                let clipboard = window.navigator().clipboard();
                // Copy text to the clipboard
                let _ = clipboard.write_text(&password);
            }
        })
    };

    html! {
        <div class="flex my-4 justify-between max-w-full h-fit">
            <div class="polybius-password-bits-flex">
                { for props.password.iter().map(|bit| html! {
                    <div class="polybius-password-bit" title={bit.importance.clone()}>
                        <span class="dark:text-neutral-100 text-center">{&bit.bits}</span>
                    </div>
                })}
            </div>
            <div onclick={copy_to_clipboard} class="polybius-password-bit hover:bg-primary-200 hover:dark:bg-primary-800 text-primary-900 dark:text-primary-200 ml-2" title="Copy password">
                <CopyIcon />
            </div>
        </div>
    }
}
