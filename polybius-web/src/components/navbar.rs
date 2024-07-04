use yew::prelude::*;

use crate::components::github_logo::GitHubLogo;

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <header class="absolute inset-x-0 top-0 z-50">
            <nav class="bg-transparent">
                <div class="mx-auto sm:max-w-screen-sm md:max-w-screen-md lg:max-w-screen-lg px-2 sm:px-5 lg:px-8">
                    <div class="relative flex h-16 items-center justify-between">
                        <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                            <div class="flex flex-shrink-0 items-center">
                                <span class="text-primary-1000 dark:text-primary-50 text-2xl font-display font-bold hover:text-3xl transition-all ease"> { "Polybius" } </span>
                                <span class="text-primary-400 dark:text-primary-400 text-2xl font-display font-bold hover:text-3xl transition-all ease px-2"> { "#%&*" } </span>
                            </div>
                        </div>
                        <a class="text-primary-1000 dark:text-primary-50 absolute right-2 flex items-center justify-center h-6 w-6" href="https://github.com/2-5-perceivers/polybius">
                            <GitHubLogo />
                        </a>
                    </div>
                </div>
            </nav>
        </header>
    }
}
