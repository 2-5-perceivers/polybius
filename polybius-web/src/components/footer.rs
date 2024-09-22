use yew::prelude::*;

use crate::components::github_logo::GitHubLogo;

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer>
            <div class="mx-auto sm:max-w-screen-sm md:max-w-screen-md lg:max-w-screen-lg px-6 py-12 md:flex md:items-center md:justify-between lg:px-8">
                <div class="flex justify-center order-2">
                    <a href="https://github.com/2-5-perceivers/polybius" class="text-neutral-500 hover:text-neutral-700">
                        <span class="footer-brand-name">{"GitHub"}</span>
                        <div class="footer-brand-icon">
                            <GitHubLogo  />
                        </div>
                    </a>
                </div>
                <div class="mt-8 md:order-1 md:mt-0">
                    <p  class="text-center text-xs leading-5 text-neutral-500"> {"© 2024 2.5 Perceivers | Under Apache License"} </p>
                </div>
            </div>
        </footer>
    }
}
